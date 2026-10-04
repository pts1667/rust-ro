use std::collections::BTreeMap;

use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, tx_read, tx_required, tx_write};
use models::skill_grant::ScriptSkillGrant;
use models::status::KnownSkill;
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use crate::repository::game_system_repository::character_key;
use crate::repository::{Error, SledRepository};
use crate::server::model::game_systems::CharacterGameSystems;

#[derive(Debug, Clone)]
pub struct ScriptSkillResetPlan {
    pub expected_skills: BTreeMap<u32, u8>,
    pub persisted_skills: BTreeMap<u32, u8>,
    pub permanent_grants: BTreeMap<u32, u8>,
    pub known_skills: Vec<KnownSkill>,
    pub temporary_grants: BTreeMap<u32, ScriptSkillGrant>,
    pub refund: u32,
    pub expected_skill_points: u32,
    pub skill_points: u32,
    pub options: u64,
}

#[derive(Debug, Clone)]
pub struct ScriptSkillGrantMutation {
    pub skill_id: u32,
    pub level: u8,
    pub expected_level: u8,
    pub permanent: bool,
    pub expected_grants: BTreeMap<u32, u8>,
}

#[derive(Debug, Clone)]
pub struct ScriptLevelResetPlan {
    pub action: u8,
    pub expected_class: u32,
    pub base_level: u32,
    pub job_level: u32,
    pub base_exp: u32,
    pub job_exp: u32,
    pub skill_points: u32,
    pub status_points: u32,
    pub attributes: [u16; 6],
    pub options: u64,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub reset_skills: Option<ScriptSkillResetPlan>,
    pub additional_skills: BTreeMap<u32, u8>,
}

#[derive(Debug, Clone)]
pub struct ScriptExperiencePlan {
    pub expected_job: u32,
    pub expected: [u32; 8],
    pub base_level: u32,
    pub job_level: u32,
    pub base_exp: u32,
    pub job_exp: u32,
    pub status_points: u32,
    pub skill_points: u32,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
}

#[derive(Debug, Clone)]
pub struct ScriptExperienceAward {
    pub char_id: u32,
    pub account_id: u32,
    pub plan: ScriptExperiencePlan,
}

pub fn apply_experience_tx(character: &mut CharacterRecord, plan: &ScriptExperiencePlan) -> ConflictableTransactionResult<(), Error> {
    let current = [
        character.base_level as u32,
        character.job_level as u32,
        character.base_exp as u32,
        character.job_exp as u32,
        character.status_point as u32,
        character.skill_point as u32,
        character.hp as u32,
        character.sp as u32,
    ];
    if character.class as u32 != plan.expected_job || current[..6] != plan.expected[..6] {
        return abort("Character experience changed before the grant committed");
    }
    macro_rules! field {
        ($name:ident, $value:expr) => {
            character.$name = $value.try_into().map_err(|_| {
                sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput(
                    concat!(stringify!($name), " is out of bounds").into(),
                ))
            })?;
        };
    }
    field!(base_level, plan.base_level);
    field!(job_level, plan.job_level);
    field!(base_exp, plan.base_exp);
    field!(job_exp, plan.job_exp);
    field!(status_point, plan.status_points);
    field!(skill_point, plan.skill_points);
    field!(hp, plan.hp);
    field!(sp, plan.sp);
    field!(max_hp, plan.max_hp);
    field!(max_sp, plan.max_sp);
    Ok(())
}

pub trait ScriptCharacterRepository: Send + Sync {
    fn character_commit_experience_awards(&self, _awards: &[ScriptExperienceAward]) -> Result<(), Error> {
        Err(Error::InvalidInput("Character experience transactions are unavailable".into()))
    }

    fn character_script_grant_skill(
        &self,
        _char_id: u32,
        _account_id: u32,
        _change: &ScriptSkillGrantMutation,
    ) -> Result<CharacterGameSystems, Error> {
        Err(Error::InvalidInput("Character skill transactions are unavailable".into()))
    }

    fn character_script_reset_level(
        &self,
        _char_id: u32,
        _account_id: u32,
        _plan: &ScriptLevelResetPlan,
    ) -> Result<CharacterGameSystems, Error> {
        Err(Error::InvalidInput("Character reset transactions are unavailable".into()))
    }
}

pub fn apply_skill_reset_tx(
    character: &mut CharacterRecord,
    skills: &TransactionalTree,
    game_systems: &TransactionalTree,
    char_id: u32,
    plan: &ScriptSkillResetPlan,
) -> ConflictableTransactionResult<(), Error> {
    let mut known: BTreeMap<u32, u8> = tx_read(skills, &char_id.to_be_bytes())?.unwrap_or_default();
    known.retain(|_, level| *level > 0);
    let systems: CharacterGameSystems = tx_read(game_systems, &character_key(char_id))?.unwrap_or_default();
    if known != plan.expected_skills
        || systems.permanent_skill_grants != plan.permanent_grants
        || character.skill_point as u32 != plan.expected_skill_points
    {
        return abort("Character skills changed before the reset committed");
    }
    if systems
        .permanent_skill_grants
        .iter()
        .any(|(id, level)| plan.persisted_skills.get(id) != Some(level))
    {
        return abort("A skill reset cannot remove a permanent script grant");
    }
    character.skill_point = i16::try_from(plan.skill_points).map_err(|_| {
        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Skill points are out of bounds".into()))
    })?;
    character.option = i32::try_from(plan.options).map_err(|_| {
        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Character options are out of bounds".into()))
    })?;
    tx_write(skills, &char_id.to_be_bytes(), &plan.persisted_skills)
}

impl ScriptCharacterRepository for SledRepository {
    fn character_commit_experience_awards(&self, awards: &[ScriptExperienceAward]) -> Result<(), Error> {
        let unique_ids: std::collections::BTreeSet<_> = awards.iter().map(|award| award.char_id).collect();
        if unique_ids.len() != awards.len() {
            return Err(Error::InvalidInput("Experience recipients must be unique".into()));
        }
        Ok(self.database.characters.transaction(|characters| {
            for award in awards {
                let key = award.char_id.to_be_bytes();
                let mut character: CharacterRecord = tx_required(characters, &key)?;
                if character.account_id as u32 != award.account_id {
                    return abort("Experience recipient belongs to another account");
                }
                apply_experience_tx(&mut character, &award.plan)?;
                tx_write(characters, &key, &character)?;
            }
            Ok(())
        })?)
    }

    fn character_script_grant_skill(
        &self,
        char_id: u32,
        account_id: u32,
        change: &ScriptSkillGrantMutation,
    ) -> Result<CharacterGameSystems, Error> {
        Ok(
            (&self.database.characters, &self.database.skills, &self.database.game_systems).transaction(
                |(characters, skills, systems)| {
                    let character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
                    if character.account_id as u32 != account_id {
                        return abort("Character belongs to another account");
                    }
                    let mut known: BTreeMap<u32, u8> = tx_read(skills, &char_id.to_be_bytes())?.unwrap_or_default();
                    let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                    if known.get(&change.skill_id).copied().unwrap_or(0) != change.expected_level
                        || state.permanent_skill_grants != change.expected_grants
                    {
                        return abort("Character skills changed before the grant committed");
                    }
                    if change.level == 0 {
                        known.remove(&change.skill_id);
                    } else {
                        known.insert(change.skill_id, change.level);
                    }
                    if change.permanent && change.level > 0 {
                        state.permanent_skill_grants.insert(change.skill_id, change.level);
                    } else {
                        state.permanent_skill_grants.remove(&change.skill_id);
                    }
                    state.revision = state.revision.checked_add(1).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Game system revision overflow".into()))
                    })?;
                    tx_write(skills, &char_id.to_be_bytes(), &known)?;
                    tx_write(systems, &character_key(char_id), &state)?;
                    Ok(state)
                },
            )?,
        )
    }

    fn character_script_reset_level(
        &self,
        char_id: u32,
        account_id: u32,
        plan: &ScriptLevelResetPlan,
    ) -> Result<CharacterGameSystems, Error> {
        Ok((
            &self.database.characters,
            &self.database.skills,
            &self.database.game_systems,
            &self.database.inventories,
        )
            .transaction(|(characters, skills, systems, inventories)| {
                let key = char_id.to_be_bytes();
                let mut character: CharacterRecord = tx_required(characters, &key)?;
                if character.account_id as u32 != account_id || character.class as u32 != plan.expected_class {
                    return abort("Reset character account or class changed");
                }
                if !(1..=4).contains(&plan.action) {
                    return abort("Invalid level reset action");
                }
                if let Some(reset) = &plan.reset_skills {
                    apply_skill_reset_tx(&mut character, skills, systems, char_id, reset)?;
                }
                if !plan.additional_skills.is_empty() {
                    let mut known: BTreeMap<u32, u8> = tx_read(skills, &key)?.unwrap_or_default();
                    known.extend(plan.additional_skills.clone());
                    tx_write(skills, &key, &known)?;
                }
                let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                if plan.action == 1 && state.mounting {
                    state.mounting = false;
                    state.revision = state.revision.checked_add(1).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Game system revision overflow".into()))
                    })?;
                    tx_write(systems, &character_key(char_id), &state)?;
                }
                macro_rules! field {
                    ($name:ident, $value:expr) => {
                        character.$name = $value.try_into().map_err(|_| {
                            sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput(
                                concat!(stringify!($name), " is out of bounds").into(),
                            ))
                        })?;
                    };
                }
                field!(base_level, plan.base_level);
                field!(job_level, plan.job_level);
                field!(base_exp, plan.base_exp);
                field!(job_exp, plan.job_exp);
                field!(skill_point, plan.skill_points);
                field!(status_point, plan.status_points);
                field!(str, plan.attributes[0]);
                field!(agi, plan.attributes[1]);
                field!(vit, plan.attributes[2]);
                field!(int, plan.attributes[3]);
                field!(dex, plan.attributes[4]);
                field!(luk, plan.attributes[5]);
                field!(option, plan.options);
                field!(hp, plan.hp);
                field!(sp, plan.sp);
                field!(max_hp, plan.max_hp);
                field!(max_sp, plan.max_sp);
                character.weapon = 0;
                character.shield = 0;
                character.head_top = 0;
                character.head_mid = 0;
                character.head_bottom = 0;
                character.robe = 0;
                let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &key)?.unwrap_or_default();
                for item in &mut inventory {
                    item.equip = 0;
                }
                tx_write(inventories, &key, &inventory)?;
                tx_write(characters, &key, &character)?;
                Ok(state)
            })?)
    }
}

#[cfg(test)]
mod tests {
    use database::model::{AccountRecord, CharacterInventory, CharacterSkills, SeedData};
    use database::required;
    use models::enums::skill_enums::SkillEnum;
    use models::status::Status;

    use super::*;
    use crate::repository::CharacterRepository;
    use crate::server::service::script_character_service::{grant_temporary_skill, plan_reset_skills};
    use crate::server::state::character::Character;

    fn setup() -> (SledRepository, Character) {
        let repository = SledRepository::temporary().unwrap();
        let known = BTreeMap::from([(SkillEnum::NvBasic.id(), 5), (SkillEnum::MgFirebolt.id(), 2)]);
        repository
            .database
            .seed(
                &SeedData {
                    accounts: vec![AccountRecord {
                        account_id: 2_000_000,
                        username: "Player".into(),
                        password: "secret".into(),
                    }],
                    characters: vec![CharacterRecord {
                        char_id: 150_000,
                        account_id: 2_000_000,
                        name: "Player".into(),
                        inventory_slots: 100,
                        skill_point: 3,
                        ..Default::default()
                    }],
                    skills: vec![CharacterSkills {
                        char_id: 150_000,
                        skills: known,
                    }],
                    inventories: vec![CharacterInventory {
                        char_id: 150_000,
                        items: vec![InventoryRecord {
                            id: 10,
                            item_id: 1201,
                            amount: 1,
                            equip: 2,
                            ..Default::default()
                        }],
                    }],
                    ..Default::default()
                },
                false,
            )
            .unwrap();
        let status = Status {
            base_level: 40,
            job_level: 20,
            skill_point: 3,
            known_skills: vec![
                KnownSkill {
                    value: SkillEnum::NvBasic,
                    level: 5,
                },
                KnownSkill {
                    value: SkillEnum::MgFirebolt,
                    level: 2,
                },
            ],
            ..Default::default()
        };
        (
            repository,
            Character::new(
                "Player".into(),
                150_000,
                2_000_000,
                status,
                1,
                1,
                0,
                "prontera".into(),
                1,
                vec![],
            ),
        )
    }

    #[test]
    fn permanent_grants_survive_reset_and_temporary_levels_do_not_generate_refunds() {
        let (repository, mut character) = setup();
        let mutation = ScriptSkillGrantMutation {
            skill_id: SkillEnum::MgFirebolt.id(),
            level: 2,
            expected_level: 2,
            permanent: true,
            expected_grants: BTreeMap::new(),
        };
        character.game_systems = repository
            .character_script_grant_skill(character.char_id, character.account_id, &mutation)
            .unwrap();
        grant_temporary_skill(&mut character.status, SkillEnum::NvBasic, 4, true);
        grant_temporary_skill(&mut character.status, SkillEnum::SmBash, 10, false);
        let plan = plan_reset_skills(&character, true).unwrap();
        assert_eq!((plan.refund, plan.skill_points), (5, 8));
        repository
            .character_commit_skill_reset(character.char_id, character.account_id, &plan)
            .unwrap();
        let known: BTreeMap<u32, u8> = required(&repository.database.skills, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!(known, BTreeMap::from([(SkillEnum::MgFirebolt.id(), 2)]));
        let saved: CharacterRecord = required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!(saved.skill_point, 8);
        assert_eq!(
            repository
                .character_script_grant_skill(character.char_id, character.account_id, &ScriptSkillGrantMutation {
                    skill_id: SkillEnum::MgFirebolt.id(),
                    level: 0,
                    expected_level: 2,
                    permanent: false,
                    expected_grants: character.game_systems.permanent_skill_grants.clone()
                })
                .unwrap()
                .permanent_skill_grants,
            BTreeMap::new()
        );
    }

    #[test]
    fn stale_reset_and_wrong_account_leave_both_trees_unchanged() {
        let (repository, character) = setup();
        let plan = plan_reset_skills(&character, true).unwrap();
        assert!(repository.character_commit_skill_reset(character.char_id, 7, &plan).is_err());
        repository
            .character_commit_skill_allocation(character.char_id, character.account_id, SkillEnum::NvBasic.id(), 5, 3, 9)
            .unwrap();
        assert!(
            repository
                .character_commit_skill_reset(character.char_id, character.account_id, &plan)
                .is_err()
        );
        let saved: CharacterRecord = required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
        let known: BTreeMap<u32, u8> = required(&repository.database.skills, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!(saved.skill_point, 2);
        assert_eq!(known[&SkillEnum::NvBasic.id()], 6);
    }

    #[test]
    fn level_reset_atomically_clears_every_equipment_slot_without_deleting_inventory() {
        let (repository, character) = setup();
        let plan = ScriptLevelResetPlan {
            action: 3,
            expected_class: 0,
            base_level: 1,
            job_level: 20,
            base_exp: 0,
            job_exp: 40,
            skill_points: 3,
            status_points: 12,
            attributes: [20, 10, 5, 7, 9, 1],
            options: 0,
            hp: 50,
            sp: 10,
            max_hp: 50,
            max_sp: 10,
            reset_skills: None,
            additional_skills: BTreeMap::new(),
        };
        repository
            .character_script_reset_level(character.char_id, character.account_id, &plan)
            .unwrap();
        let saved: CharacterRecord = required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!((saved.base_level, saved.job_level, saved.job_exp, saved.str), (1, 20, 40, 20));
        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory[0].equip, 0);
        let known: BTreeMap<u32, u8> = required(&repository.database.skills, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!(known[&SkillEnum::NvBasic.id()], 5);
    }
}
