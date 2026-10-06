use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use database::{abort, character_slot_key, read, required, tx_read, tx_required, tx_write};
use models::enums::skill_enums::SkillEnum;
use models::enums::EnumWithMaskValueU64;
use models::status::{KnownSkill, Status, StatusSnapshot};
use models::status_bonus::{
    BonusExpiry, StatusBonusFlag, StatusBonusSource, StructuredBonus, TemporaryStatusBonus, TemporaryStatusBonuses,
};
use serde::{Deserialize, Serialize};
use sled::transaction::Transactional;

use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::repository::{CharacterRepository, Error, SledRepository};
use crate::util::tick::get_tick;

#[derive(Serialize, Deserialize)]
struct StoredBonus {
    remaining_ms: u32,
    bonus_type: i32,
    flags: u64,
    source: Option<(String, i32)>,
    val1: i32,
    val2: i32,
    #[serde(default)]
    structured: Option<StructuredBonus>,
}

fn position_coordinates(x: u16, y: u16) -> Result<(i16, i16), Error> {
    if (x, y) == crate::server::model::map::RANDOM_CELL {
        return Ok((-1, -1));
    }
    Ok((
        i16::try_from(x).map_err(|_| Error::new("Map x coordinate is out of bounds".into()))?,
        i16::try_from(y).map_err(|_| Error::new("Map y coordinate is out of bounds".into()))?,
    ))
}

fn update_character_snapshots(
    repository: &SledRepository,
    statuses: Vec<&Status>,
    snapshots: Vec<StatusSnapshot>,
    positions: Vec<crate::server::model::events::persistence_event::SavePositionUpdate>,
    guarded: bool,
) -> Result<(), Error> {
    if statuses.len() != positions.len() || snapshots.len() != positions.len() {
        return Err(Error::new("Character snapshot lengths do not match".into()));
    }
    let coordinates = positions
        .iter()
        .map(|position| {
            if guarded {
                position_coordinates(position.x, position.y)
            } else {
                Ok((position.x as i16, position.y as i16))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    repository.database.characters.transaction(|tree| {
        for (i, position) in positions.iter().enumerate() {
            let mut character: CharSelectModel = tx_required(tree, &position.char_id.to_be_bytes())?;
            if guarded && character.account_id as u32 != position.account_id {
                return abort("Character snapshot owner changed");
            }
            let status = statuses[i];
            let snapshot = &snapshots[i];
            character.class = snapshot.job() as i16;
            character.base_level = status.base_level() as i32;
            character.job_level = status.job_level() as i32;
            character.base_exp = status.base_exp() as i32;
            character.job_exp = status.job_exp() as i32;
            character.str = snapshot.base_str() as i16;
            character.agi = snapshot.base_agi() as i16;
            character.vit = snapshot.base_vit() as i16;
            character.int = snapshot.base_int() as i16;
            character.dex = snapshot.base_dex() as i16;
            character.luk = snapshot.base_luk() as i16;
            character.max_hp = snapshot.max_hp() as i32;
            character.hp = snapshot.hp() as i32;
            character.max_sp = snapshot.max_sp() as i32;
            character.sp = snapshot.sp() as i32;
            character.status_point = status.status_point() as i16;
            character.skill_point = status.skill_point() as i16;
            character.hair = status.look().map_or(0, |look| look.hair() as i16);
            character.hair_color = status.look().map_or(0, |look| look.hair_color() as i16);
            character.clothes_color = status.look().map_or(0, |look| look.clothes_color() as i16);
            character.body = status.armor().map_or(0, |item| item.item_id as i16);
            character.weapon = status.right_hand_weapon().map_or(0, |item| item.item_id() as i16);
            character.shield = status.shield().map_or(0, |item| item.item_id() as i16);
            character.head_top = status.head_top().map_or(0, |item| item.item_id as i16);
            character.head_mid = status.head_mid().map_or(0, |item| item.item_id as i16);
            character.head_bottom = status.head_low().map_or(0, |item| item.item_id as i16);
            character.robe = status.look().map_or(0, |look| look.robe() as i32);
            if guarded && position.revision > character.position_revision || !guarded && character.position_revision == 0 {
                character.last_map = position.map_name.clone();
                character.last_x = coordinates[i].0;
                character.last_y = coordinates[i].1;
                if guarded {
                    character.position_revision = position.revision;
                }
            }
            tx_write(tree, &position.char_id.to_be_bytes(), &character)?;
        }
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod structured_bonus_persistence_tests {
    use models::enums::EnumWithMaskValueU32;
    use models::enums::bonus::BonusType;
    use models::status_bonus::{AutoBonus, BattleFlag, CombatProc, CombatProcKind, CombatTrigger};

    use super::*;

    #[test]
    fn structured_proc_and_autobonus_roundtrip_all_fields_with_account_isolation() {
        let repository = SledRepository::temporary().unwrap();
        let mut proc = CombatProc::new(CombatTrigger::Skill, CombatProcKind::Spell, 10000);
        proc.value = SkillEnum::MgFirebolt.id();
        proc.level = 10;
        proc.trigger_skill = SkillEnum::SmBash.id();
        proc.duration = 1234;
        proc.battle_flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        proc.flags =
            models::status_bonus::AutoSpellFlag::OtherTarget.as_flag() | models::status_bonus::AutoSpellFlag::RandomLevel.as_flag();
        let auto = AutoBonus {
            trigger: CombatTrigger::Hit,
            rate: 1500,
            duration: 5000,
            battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag(),
            trigger_skill: 0,
            program_id: 42001,
            visual_program_id: 42002,
            source_item_id: 4001,
            source_location: models::enums::item::EquipmentLocation::HandRight.as_flag(),
            source_pet_id: 0,
        };
        let types = [BonusType::CombatProc(proc, 2), BonusType::AutoBonus(auto, 1)];
        let flags = StatusBonusFlag::Persist.as_flag() | StatusBonusFlag::Unique.as_flag();
        let now = get_tick();
        let bonuses = TemporaryStatusBonuses::new(
            types
                .iter()
                .map(|bonus| TemporaryStatusBonus::with_duration_and_source(*bonus, flags, now, 60000, Some(StatusBonusSource::Item(4001))))
                .collect(),
        );
        futures::executor::block_on(repository.character_save_temporary_bonus(150000, 2000000, &bonuses)).unwrap();
        let stored: Vec<StoredBonus> = required(&repository.database.bonuses, &bonus_key(150000, 2000000)).unwrap();
        assert!(stored.iter().all(|bonus| bonus.remaining_ms > 0 && bonus.remaining_ms <= 60000));
        assert!(
            futures::executor::block_on(repository.character_load_temporary_bonus(150000, 2000001))
                .unwrap()
                .iter()
                .next()
                .is_none()
        );
        let before_load = get_tick();
        let loaded = futures::executor::block_on(repository.character_load_temporary_bonus(150000, 2000000)).unwrap();
        let after_load = get_tick();
        assert_eq!(loaded.iter().map(|bonus| *bonus.bonus()).collect::<Vec<_>>(), types);
        for (loaded, stored) in loaded.iter().zip(stored) {
            assert_eq!(loaded.flags(), flags);
            assert_eq!(loaded.source(), &Some(StatusBonusSource::Item(4001)));
            assert!(matches!(loaded.expirency(),BonusExpiry::Time(until)
                if *until >= before_load + u128::from(stored.remaining_ms) && *until <= after_load + u128::from(stored.remaining_ms)));
        }
        assert!(
            futures::executor::block_on(repository.character_load_temporary_bonus(150000, 2000000))
                .unwrap()
                .iter()
                .next()
                .is_none()
        );
    }

    #[test]
    fn legacy_scalar_bonuses_load_without_structured_payload() {
        let repository = SledRepository::temporary().unwrap();
        let (bonus_type, val1, val2) = BonusType::Str(7).serialize_to_sc_data();
        repository.database.bonuses.insert(bonus_key(150000,2000000),serde_json::to_vec(&serde_json::json!([{"remaining_ms":60000,"bonus_type":bonus_type,"flags":StatusBonusFlag::Persist.as_flag(),"source":["Skill",SkillEnum::AlBlessing.id()],"val1":val1,"val2":val2}])).unwrap()).unwrap();
        let loaded = futures::executor::block_on(repository.character_load_temporary_bonus(150000, 2000000)).unwrap();
        assert_eq!(*loaded.iter().next().unwrap().bonus(), BonusType::Str(7));
        assert_eq!(
            *loaded.iter().next().unwrap().source(),
            Some(StatusBonusSource::Skill(SkillEnum::AlBlessing.id() as u16))
        );
    }
}

fn bonus_key(char_id: u32, account_id: u32) -> [u8; 8] {
    let mut key = [0; 8];
    key[..4].copy_from_slice(&char_id.to_be_bytes());
    key[4..].copy_from_slice(&account_id.to_be_bytes());
    key
}

#[async_trait]
impl CharacterRepository for SledRepository {
    fn character_commit_skill_reset(
        &self,
        char_id: u32,
        account_id: u32,
        plan: &crate::repository::script_character_repository::ScriptSkillResetPlan,
    ) -> Result<(), Error> {
        (&self.database.characters, &self.database.skills, &self.database.game_systems).transaction(|(characters, skills, systems)| {
            let mut character: CharSelectModel = tx_required(characters, &char_id.to_be_bytes())?;
            if character.account_id as u32 != account_id {
                return abort("Character belongs to another account");
            }
            crate::repository::script_character_repository::apply_skill_reset_tx(&mut character, skills, systems, char_id, plan)?;
            tx_write(characters, &char_id.to_be_bytes(), &character)
        })?;
        Ok(())
    }

    fn character_commit_skill_allocation(
        &self,
        char_id: u32,
        account_id: u32,
        skill_id: u32,
        expected_level: u8,
        expected_points: u32,
        max_level: u8,
    ) -> Result<u8, Error> {
        Ok(
            (&self.database.characters, &self.database.skills, &self.database.game_systems).transaction(
                |(characters, skills, systems)| {
                    let mut character: CharSelectModel = tx_required(characters, &char_id.to_be_bytes())?;
                    let mut known: BTreeMap<u32, u8> = tx_read(skills, &char_id.to_be_bytes())?.unwrap_or_default();
                    let state: crate::server::model::game_systems::CharacterGameSystems =
                        tx_read(systems, &crate::repository::game_system_repository::character_key(char_id))?.unwrap_or_default();
                    if character.account_id as u32 != account_id
                        || character.skill_point as u32 != expected_points
                        || expected_points == 0
                        || known.get(&skill_id).copied().unwrap_or(0) != expected_level
                        || state.permanent_skill_grants.contains_key(&skill_id)
                    {
                        return abort("Character skill allocation is stale or unauthorized");
                    }
                    let level = expected_level.checked_add(1).filter(|level| *level <= max_level).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput(
                            "Skill has reached its maximum level".into(),
                        ))
                    })?;
                    known.insert(skill_id, level);
                    character.skill_point -= 1;
                    tx_write(skills, &char_id.to_be_bytes(), &known)?;
                    tx_write(characters, &char_id.to_be_bytes(), &character)?;
                    Ok(level)
                },
            )?,
        )
    }

    async fn character_insert(&self, char_model: &CharInsertModel) -> Result<(), Error> {
        self.database.insert_character(&CharSelectModel::try_from(char_model)?)?;
        Ok(())
    }

    async fn character_info(&self, account_id: i32, char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error> {
        Ok(
            (&self.database.character_names, &self.database.characters).transaction(|(names, characters)| {
                let id: i32 = tx_required(names, char_name.as_bytes())?;
                let character: CharSelectModel = tx_required(characters, &id.to_be_bytes())?;
                if character.account_id != account_id {
                    return Err(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound));
                }
                Ok(CharacterInfoNeoUnionWrapped::from(&character))
            })?,
        )
    }

    async fn characters_info(&self, account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped> {
        let result = self
            .database
            .character_slots
            .scan_prefix(account_id.to_be_bytes())
            .map(|entry| {
                let (_, bytes) = entry?;
                let id: i32 = serde_json::from_slice(&bytes)?;
                let character: CharSelectModel = required(&self.database.characters, &id.to_be_bytes())?;
                Ok(CharacterInfoNeoUnionWrapped::from(&character))
            })
            .collect::<Result<Vec<_>, Error>>();
        result.unwrap_or_else(|error| {
            error!("Failed to load account characters: {error}");
            Vec::new()
        })
    }

    async fn character_delete_reserved(&self, account_id: u32, char_id: u32) -> Result<(), Error> {
        let delete_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| Error::new(error.to_string()))?
            .as_secs()
            .checked_add(24 * 60 * 60)
            .and_then(|value| u32::try_from(value).ok())
            .ok_or_else(|| Error::new("Deletion date is out of bounds".into()))?;
        self.database.characters.transaction(|tree| {
            let mut character: CharSelectModel = tx_required(tree, &char_id.to_be_bytes())?;
            if character.account_id as u32 != account_id {
                return abort("Character belongs to another account");
            }
            character.delete_date = delete_date;
            tx_write(tree, &char_id.to_be_bytes(), &character)
        })?;
        Ok(())
    }

    async fn character_save_position(&self, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        let x = i16::try_from(x).map_err(|_| Error::new("Map x coordinate is out of bounds".into()))?;
        let y = i16::try_from(y).map_err(|_| Error::new("Map y coordinate is out of bounds".into()))?;
        self.database.characters.transaction(|tree| {
            let mut character: CharSelectModel = tx_required(tree, &char_id.to_be_bytes())?;
            if character.position_revision != 0 {
                return Ok(());
            }
            character.last_map = map_name.clone();
            character.last_x = x;
            character.last_y = y;
            tx_write(tree, &char_id.to_be_bytes(), &character)
        })?;
        Ok(())
    }

    async fn character_save_position_guarded(
        &self,
        position: crate::server::model::events::persistence_event::SavePositionUpdate,
    ) -> Result<(), Error> {
        let (x, y) = position_coordinates(position.x, position.y)?;
        self.database.characters.transaction(|tree| {
            let mut character: CharSelectModel = tx_required(tree, &position.char_id.to_be_bytes())?;
            if character.account_id as u32 != position.account_id {
                return abort("Character position owner changed");
            }
            if position.revision <= character.position_revision {
                return Ok(());
            }
            character.last_map = position.map_name.clone();
            character.last_x = x;
            character.last_y = y;
            character.position_revision = position.revision;
            tx_write(tree, &position.char_id.to_be_bytes(), &character)
        })?;
        Ok(())
    }

    async fn character_update_status(&self, char_id: u32, field: String, value: u32) -> Result<(), Error> {
        self.database.characters.transaction(|tree| {
            let mut character: CharSelectModel = tx_required(tree, &char_id.to_be_bytes())?;
            macro_rules! set_field {
                ($($name:ident),*) => {
                    match field.as_str() {
                        $(stringify!($name) => {
                            character.$name = value.try_into().map_err(|_| {
                                sled::transaction::ConflictableTransactionError::Abort(Error::new(
                                    format!("Character field {} is out of bounds", field)
                                ))
                            })?;
                        },)*
                        _ => return abort(format!("Unknown character field {field}")),
                    }
                }
            }
            set_field!(
                class,
                zeny,
                status_point,
                skill_point,
                str,
                agi,
                vit,
                int,
                dex,
                luk,
                max_hp,
                hp,
                max_sp,
                sp,
                hair,
                hair_color,
                clothes_color,
                body,
                weapon,
                shield,
                head_top,
                head_mid,
                head_bottom,
                robe,
                base_level,
                job_level,
                base_exp,
                job_exp,
                option,
                karma,
                manner,
                rename,
                delete_date
            );
            tx_write(tree, &char_id.to_be_bytes(), &character)
        })?;
        Ok(())
    }

    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        Ok(required::<CharSelectModel>(&self.database.characters, &char_id.to_be_bytes())?.zeny)
    }

    async fn character_allocated_skill_points(&self, char_id: u32) -> Result<i32, Error> {
        Ok(
            (&self.database.skills, &self.database.game_systems).transaction(|(skills, systems)| {
                let known: BTreeMap<u32, u8> = tx_read(skills, &char_id.to_be_bytes())?.unwrap_or_default();
                let state: crate::server::model::game_systems::CharacterGameSystems =
                    tx_read(systems, &crate::repository::game_system_repository::character_key(char_id))?.unwrap_or_default();
                Ok(known
                    .iter()
                    .filter(|(id, _)| {
                        !state.permanent_skill_grants.contains_key(id)
                            && SkillEnum::try_from_value(**id).map_or(false, |skill| !skill.is_platinium())
                    })
                    .map(|(_, level)| i32::from(*level))
                    .sum())
            })?,
        )
    }

    async fn character_skills(&self, char_id: u32) -> Result<Vec<KnownSkill>, Error> {
        let skills: BTreeMap<u32, u8> = read(&self.database.skills, &char_id.to_be_bytes())?.unwrap_or_default();
        Ok(skills
            .into_iter()
            .map(|(id, level)| KnownSkill {
                value: SkillEnum::from_id(id),
                level,
            })
            .collect())
    }

    async fn character_fetch(&self, account_id: u32, char_num: u8) -> Result<CharSelectModel, Error> {
        Ok(
            (&self.database.character_slots, &self.database.characters).transaction(|(slots, characters)| {
                let id: i32 = tx_required(slots, &character_slot_key(account_id as i32, char_num as i16))?;
                tx_required(characters, &id.to_be_bytes())
            })?,
        )
    }

    async fn character_with_id_fetch(&self, char_id: u32) -> Result<CharSelectModel, Error> {
        required(&self.database.characters, &char_id.to_be_bytes())
    }

    async fn character_reset_skills(&self, char_id: i32, skills: Vec<i32>) -> Result<(), Error> {
        (&self.database.skills, &self.database.game_systems).transaction(|(tree, systems)| {
            let mut known: BTreeMap<u32, u8> = tx_read(tree, &char_id.to_be_bytes())?.unwrap_or_default();
            let state: crate::server::model::game_systems::CharacterGameSystems = tx_read(
                systems,
                &crate::repository::game_system_repository::character_key(char_id as u32),
            )?
            .unwrap_or_default();
            for id in &skills {
                if !state.permanent_skill_grants.contains_key(&(*id as u32)) {
                    known.remove(&(*id as u32));
                }
            }
            tx_write(tree, &char_id.to_be_bytes(), &known)
        })?;
        Ok(())
    }

    async fn character_allocate_skill_point(&self, char_id: i32, skill_id: i32, increment: u8) -> Result<(), Error> {
        if skill_id <= 0 {
            return Err(Error::new("Skill ID must be positive".into()));
        }
        self.database.skills.transaction(|tree| {
            let mut known: BTreeMap<u32, u8> = tx_read(tree, &char_id.to_be_bytes())?.unwrap_or_default();
            let level = known.entry(skill_id as u32).or_default();
            *level = level
                .checked_add(increment)
                .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Skill level is out of bounds".into())))?;
            tx_write(tree, &char_id.to_be_bytes(), &known)
        })?;
        Ok(())
    }

    async fn characters_update(
        &self,
        statuses: Vec<&Status>,
        snapshots: Vec<StatusSnapshot>,
        char_ids: Vec<i32>,
        x: Vec<i16>,
        y: Vec<i16>,
        maps: Vec<String>,
    ) -> Result<(), Error> {
        let count = char_ids.len();
        if [statuses.len(), snapshots.len(), x.len(), y.len(), maps.len()]
            .iter()
            .any(|len| *len != count)
        {
            return Err(Error::new("Character snapshot lengths do not match".into()));
        }
        let positions = (0..count)
            .map(|i| crate::server::model::events::persistence_event::SavePositionUpdate {
                char_id: char_ids[i] as u32,
                account_id: 0,
                map_name: maps[i].clone(),
                x: x[i] as u16,
                y: y[i] as u16,
                revision: 0,
            })
            .collect();
        update_character_snapshots(self, statuses, snapshots, positions, false)
    }

    async fn characters_update_with_positions(
        &self,
        statuses: Vec<&Status>,
        snapshots: Vec<StatusSnapshot>,
        positions: Vec<crate::server::model::events::persistence_event::SavePositionUpdate>,
    ) -> Result<(), Error> {
        update_character_snapshots(self, statuses, snapshots, positions, true)
    }

    async fn characters_list_for_simulator(&self) -> Result<Vec<CharSelectModel>, Error> {
        self.database
            .characters
            .iter()
            .take(100)
            .map(|entry| {
                let (_, bytes) = entry?;
                Ok(serde_json::from_slice(&bytes)?)
            })
            .collect()
    }

    async fn character_save_temporary_bonus(
        &self,
        char_id: u32,
        account_id: u32,
        temporary_bonuses: &TemporaryStatusBonuses,
    ) -> Result<(), Error> {
        let now = get_tick();
        let mut stored = Vec::new();
        for bonus in temporary_bonuses
            .iter()
            .filter(|bonus| bonus.flags() & StatusBonusFlag::Persist.as_flag() != 0)
        {
            let remaining_ms = match bonus.expirency() {
                BonusExpiry::Time(until) => until.saturating_sub(now),
                _ => 0,
            };
            if remaining_ms == 0 {
                continue;
            }
            let remaining_ms = u32::try_from(remaining_ms).map_err(|_| Error::new("Persisted bonus duration is out of bounds".into()))?;
            let (bonus_type, val1, val2) = bonus.bonus().serialize_to_sc_data();
            let source = bonus.source().map(|source| {
                let (name, value) = source.serialize_to_sc_data();
                (name.to_string(), value)
            });
            stored.push(StoredBonus {
                remaining_ms,
                bonus_type,
                val1,
                val2,
                flags: bonus.flags(),
                source,
                structured: bonus.bonus().structured_payload(),
            });
        }
        self.database
            .bonuses
            .transaction(|tree| tx_write(tree, &bonus_key(char_id, account_id), &stored))?;
        Ok(())
    }

    async fn character_load_temporary_bonus(&self, char_id: u32, account_id: u32) -> Result<TemporaryStatusBonuses, Error> {
        let stored: Vec<StoredBonus> = self.database.bonuses.transaction(|tree| {
            let key = bonus_key(char_id, account_id);
            let stored = tx_read(tree, &key)?.unwrap_or_default();
            tree.remove(key.to_vec())?;
            Ok::<_, sled::transaction::ConflictableTransactionError<Error>>(stored)
        })?;
        let now = get_tick();
        let mut bonuses = TemporaryStatusBonuses::default();
        for stored in stored {
            let source = stored
                .source
                .and_then(|(name, value)| StatusBonusSource::deserialize_sc_data(&name, value));
            let bonus = stored
                .structured
                .map(models::enums::bonus::BonusType::from_structured_payload)
                .or_else(|| models::enums::bonus::BonusType::deserialize_from_sc_data(stored.bonus_type, stored.val1, stored.val2));
            if let Some(bonus) = bonus {
                bonuses.add(TemporaryStatusBonus::with_duration_and_source(
                    bonus,
                    stored.flags,
                    now,
                    stored.remaining_ms,
                    source,
                ));
            }
        }
        Ok(bonuses)
    }
}
