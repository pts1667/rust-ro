use std::collections::BTreeMap;

use database::model::CharacterRecord;
use database::{abort, read, tx_read, tx_required, tx_write};
use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::item::ItemType;
use models::enums::skill_enums::SkillEnum;
use models::status::Status;
use serde::{Deserialize, Serialize};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use crate::repository::model::item_model::ItemModel;
use crate::repository::{Error, SledRepository};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FameCategory {
    Blacksmith,
    Alchemist,
    Taekwon,
}

impl FameCategory {
    fn key(self, prefix: &[u8]) -> Vec<u8> {
        let category = match self {
            Self::Blacksmith => 0,
            Self::Alchemist => 1,
            Self::Taekwon => 2,
        };
        [prefix, &[category]].concat()
    }

    pub fn for_job(job: u32) -> Option<Self> {
        match JobName::try_from_value(job as usize).ok()? {
            JobName::Blacksmith | JobName::Whitesmith | JobName::BabyBlacksmith => Some(Self::Blacksmith),
            JobName::Alchemist | JobName::Creator | JobName::BabyAlchemist => Some(Self::Alchemist),
            JobName::Taekwon => Some(Self::Taekwon),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FameEntry {
    pub char_id: u32,
    pub name: String,
    pub points: u32,
    pub updated_order: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct FameBoard {
    generation: u64,
    entries: BTreeMap<u32, FameEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CharacterFame {
    entry: FameEntry,
    category: Option<FameCategory>,
}

fn score_key(char_id: u32) -> Vec<u8> {
    [b"character_fame/".as_slice(), &char_id.to_be_bytes()].concat()
}

#[derive(Debug, Clone)]
pub struct CraftingFamePlan {
    pub item_id: i32,
    pub skill_id: u32,
    pub success: bool,
    pub additive_slots: u8,
    pub previous_potion_streak: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftingFameOutcome {
    pub potion_streak: u8,
    pub category: Option<FameCategory>,
    pub points: u32,
    pub gained: u32,
    pub rank: u8,
    pub rankings: Vec<FameEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FameUpdate {
    pub category: FameCategory,
    pub points: u32,
    pub rank: u8,
    pub rankings: Vec<FameEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaekwonMission {
    pub mob_id: u32,
    pub kills: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissionKillOutcome {
    pub mission: TaekwonMission,
    pub fame: Option<FameUpdate>,
}

fn mission_key(char_id: u32) -> Vec<u8> {
    [b"taekwon_mission/".as_slice(), &char_id.to_be_bytes()].concat()
}

fn store_mission_tx(
    systems: &TransactionalTree,
    variables: &TransactionalTree,
    char_id: u32,
    mission: TaekwonMission,
) -> ConflictableTransactionResult<(), Error> {
    tx_write(systems, &mission_key(char_id), &mission)?;
    for (name, value) in [
        ("TK_MISSION_ID", mission.mob_id as i32),
        ("TK_MISSION_COUNT", i32::from(mission.kills)),
    ] {
        let key = crate::repository::script_variable_repository::variable_key(0, char_id, name);
        let mut values: BTreeMap<u32, i32> = tx_read(variables, &key)?.unwrap_or_default();
        values.insert(0, value);
        tx_write(variables, &key, &values)?;
    }
    Ok(())
}

fn read_mission_tx(variables: &TransactionalTree, char_id: u32) -> ConflictableTransactionResult<Option<TaekwonMission>, Error> {
    let values: BTreeMap<u32, i32> = tx_read(
        variables,
        &crate::repository::script_variable_repository::variable_key(0, char_id, "TK_MISSION_ID"),
    )?
    .unwrap_or_default();
    let id = values.get(&0).copied().unwrap_or(0);
    if id == 0 {
        return Ok(None);
    }
    let values: BTreeMap<u32, i32> = tx_read(
        variables,
        &crate::repository::script_variable_repository::variable_key(0, char_id, "TK_MISSION_COUNT"),
    )?
    .unwrap_or_default();
    let kills = values.get(&0).copied().unwrap_or(0);
    if id < 0 || !(0..=99).contains(&kills) {
        return abort("Invalid persisted Taekwon mission");
    }
    Ok(Some(TaekwonMission {
        mob_id: id as u32,
        kills: kills as u8,
    }))
}

pub trait FameRepository: Send + Sync {
    fn character_change_fame_class(&self, _char_id: u32, _account_id: u32, _new_job: u32) -> Result<Vec<FameUpdate>, Error> {
        Err(Error::InvalidInput("Character job transactions are unavailable".into()))
    }
    fn taekwon_mission(&self, _char_id: u32) -> Result<Option<TaekwonMission>, Error> {
        Err(Error::InvalidInput("Taekwon missions are unavailable".into()))
    }
    fn set_taekwon_mission(
        &self,
        _char_id: u32,
        _account_id: u32,
        _expected: Option<TaekwonMission>,
        _mob_id: u32,
    ) -> Result<TaekwonMission, Error> {
        Err(Error::InvalidInput("Taekwon missions are unavailable".into()))
    }
    fn record_taekwon_kill(
        &self,
        _char_id: u32,
        _account_id: u32,
        _mob_id: u32,
        _next_target: Option<u32>,
    ) -> Result<Option<MissionKillOutcome>, Error> {
        Err(Error::InvalidInput("Taekwon missions are unavailable".into()))
    }
    fn character_fame(&self, _char_id: u32) -> Result<u32, Error> {
        Err(Error::InvalidInput("Fame points are unavailable".into()))
    }
    fn add_fame(&self, _char_id: u32, _account_id: u32, _amount: i32) -> Result<FameUpdate, Error> {
        Err(Error::InvalidInput("Fame updates are unavailable".into()))
    }
    fn fame_rank(&self, category: FameCategory, char_id: u32) -> Result<u8, Error> {
        Ok(self
            .fame_rankings(category)?
            .iter()
            .position(|entry| entry.char_id == char_id)
            .map_or(0, |rank| rank as u8 + 1))
    }
    fn fame_rankings(&self, _category: FameCategory) -> Result<Vec<FameEntry>, Error> {
        Err(Error::InvalidInput("Fame rankings are unavailable".into()))
    }
    fn fame_points(&self, _category: FameCategory, _char_id: u32) -> Result<u32, Error> {
        Err(Error::InvalidInput("Fame points are unavailable".into()))
    }
}

impl FameRepository for SledRepository {
    fn character_change_fame_class(&self, char_id: u32, account_id: u32, new_job: u32) -> Result<Vec<FameUpdate>, Error> {
        if JobName::try_from_value(new_job as usize).is_err() {
            return Err(Error::InvalidInput("Unknown classic job".into()));
        }
        Ok(
            (&self.database.characters, &self.database.game_systems).transaction(|(characters, systems)| {
                let mut character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
                if character.account_id as u32 != account_id {
                    return abort("Job character belongs to another account");
                }
                let mut score: Option<CharacterFame> = tx_read(systems, &score_key(char_id))?;
                let old_category = score
                    .as_ref()
                    .and_then(|score| score.category)
                    .or_else(|| FameCategory::for_job(character.class as u32));
                character.class = i16::try_from(new_job).map_err(|_| {
                    sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Job is out of bounds".into()))
                })?;
                let category = FameCategory::for_job(new_job);
                let mut updates = Vec::new();
                if category.is_some() {
                    updates.push(update_fame_tx(&character, systems, 0)?);
                } else if let Some(score) = &mut score {
                    if let Some(old_category) = score.category {
                        let mut board: FameBoard = tx_read(systems, &old_category.key(b"fame_board/"))?.unwrap_or_default();
                        board.entries.remove(&char_id);
                        tx_write(systems, &old_category.key(b"fame_board/"), &board)?;
                        tx_write(systems, &old_category.key(b"rankings/"), &top_ten(&board))?;
                    }
                    score.category = None;
                    tx_write(systems, &score_key(char_id), score)?;
                }
                if let Some(old_category) = old_category.filter(|old| Some(*old) != category) {
                    let rankings: Vec<FameEntry> = tx_read(systems, &old_category.key(b"rankings/"))?.unwrap_or_default();
                    updates.push(FameUpdate {
                        category: old_category,
                        points: 0,
                        rank: 0,
                        rankings,
                    });
                }
                tx_write(characters, &char_id.to_be_bytes(), &character)?;
                Ok(updates)
            })?,
        )
    }

    fn taekwon_mission(&self, char_id: u32) -> Result<Option<TaekwonMission>, Error> {
        Ok(self
            .database
            .numeric_variables
            .transaction(|variables| read_mission_tx(variables, char_id))?)
    }

    fn set_taekwon_mission(
        &self,
        char_id: u32,
        account_id: u32,
        expected: Option<TaekwonMission>,
        mob_id: u32,
    ) -> Result<TaekwonMission, Error> {
        if mob_id == 0 {
            return Err(Error::InvalidInput("Mission target must be a real monster".into()));
        }
        Ok((
            &self.database.characters,
            &self.database.game_systems,
            &self.database.numeric_variables,
        )
            .transaction(|(characters, systems, variables)| {
                let character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
                if character.account_id as u32 != account_id {
                    return abort("Mission character belongs to another account");
                }
                let current = read_mission_tx(variables, char_id)?;
                if current != expected || current.is_some_and(|mission| mission.kills > 0) {
                    return abort("Taekwon mission target cannot change after progress");
                }
                let mission = TaekwonMission { mob_id, kills: 0 };
                store_mission_tx(systems, variables, char_id, mission)?;
                Ok(mission)
            })?)
    }

    fn record_taekwon_kill(
        &self,
        char_id: u32,
        account_id: u32,
        mob_id: u32,
        next_target: Option<u32>,
    ) -> Result<Option<MissionKillOutcome>, Error> {
        Ok((
            &self.database.characters,
            &self.database.game_systems,
            &self.database.numeric_variables,
        )
            .transaction(|(characters, systems, variables)| {
                let character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
                if character.account_id as u32 != account_id {
                    return abort("Mission character belongs to another account");
                }
                let Some(mut mission) = read_mission_tx(variables, char_id)? else {
                    return Ok(None);
                };
                if mission.mob_id != mob_id {
                    return Ok(None);
                }
                mission.kills = mission.kills.checked_add(1).ok_or_else(|| {
                    sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Mission progress overflow".into()))
                })?;
                let fame = if mission.kills >= 100 {
                    let Some(next) = next_target.filter(|id| *id > 0) else {
                        return abort("Completed mission requires a new valid target");
                    };
                    mission = TaekwonMission { mob_id: next, kills: 0 };
                    if FameCategory::for_job(character.class as u32).is_some() {
                        Some(update_fame_tx(&character, systems, 1)?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                store_mission_tx(systems, variables, char_id, mission)?;
                Ok(Some(MissionKillOutcome { mission, fame }))
            })?)
    }

    fn character_fame(&self, char_id: u32) -> Result<u32, Error> {
        Ok(read::<CharacterFame>(&self.database.game_systems, &score_key(char_id))?.map_or(0, |score| score.entry.points))
    }

    fn add_fame(&self, char_id: u32, account_id: u32, amount: i32) -> Result<FameUpdate, Error> {
        Ok(
            (&self.database.characters, &self.database.game_systems).transaction(|(characters, systems)| {
                let character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
                if character.account_id as u32 != account_id {
                    return abort("Fame character belongs to another account");
                }
                update_fame_tx(&character, systems, amount)
            })?,
        )
    }

    fn fame_rankings(&self, category: FameCategory) -> Result<Vec<FameEntry>, Error> {
        Ok(read(&self.database.game_systems, &category.key(b"rankings/"))?.unwrap_or_default())
    }

    fn fame_points(&self, category: FameCategory, char_id: u32) -> Result<u32, Error> {
        if let Some(score) = read::<CharacterFame>(&self.database.game_systems, &score_key(char_id))? {
            return Ok(if score.category == Some(category) { score.entry.points } else { 0 });
        }
        let key = [category.key(b"fame/"), char_id.to_be_bytes().to_vec()].concat();
        Ok(read::<FameEntry>(&self.database.game_systems, &key)?.map_or(0, |entry| entry.points))
    }
}

fn top_ten(board: &FameBoard) -> Vec<FameEntry> {
    let mut entries: Vec<_> = board.entries.values().filter(|entry| entry.points > 0).cloned().collect();
    entries.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then_with(|| b.updated_order.cmp(&a.updated_order))
            .then_with(|| a.char_id.cmp(&b.char_id))
    });
    entries.truncate(10);
    entries
}

pub(crate) fn forget_character_fame_tx(systems: &TransactionalTree, char_id: u32) -> ConflictableTransactionResult<(), Error> {
    for category in [FameCategory::Blacksmith, FameCategory::Alchemist, FameCategory::Taekwon] {
        let mut board: FameBoard = tx_read(systems, &category.key(b"fame_board/"))?.unwrap_or_default();
        if board.entries.remove(&char_id).is_some() {
            tx_write(systems, &category.key(b"fame_board/"), &board)?;
            tx_write(systems, &category.key(b"rankings/"), &top_ten(&board))?;
        }
        systems.remove([category.key(b"fame/"), char_id.to_be_bytes().to_vec()].concat())?;
    }
    systems.remove(score_key(char_id))?;
    systems.remove(mission_key(char_id))?;
    Ok(())
}

pub(crate) fn rename_character_fame_tx(systems: &TransactionalTree, char_id: u32, name: &str) -> ConflictableTransactionResult<(), Error> {
    for category in [FameCategory::Blacksmith, FameCategory::Alchemist, FameCategory::Taekwon] {
        let mut board: FameBoard = tx_read(systems, &category.key(b"fame_board/"))?.unwrap_or_default();
        if let Some(entry) = board.entries.get_mut(&char_id) {
            entry.name = name.to_string();
            tx_write(systems, &category.key(b"fame_board/"), &board)?;
            tx_write(systems, &category.key(b"rankings/"), &top_ten(&board))?;
        }
    }
    if let Some(mut score) = tx_read::<CharacterFame>(systems, &score_key(char_id))? {
        score.entry.name = name.to_string();
        tx_write(systems, &score_key(char_id), &score)?;
    }
    Ok(())
}

pub fn update_fame_tx(
    character: &CharacterRecord,
    systems: &TransactionalTree,
    amount: i32,
) -> ConflictableTransactionResult<FameUpdate, Error> {
    let Some(category) = FameCategory::for_job(character.class as u32) else {
        return abort("Character class does not use classic fame");
    };
    let mut board: FameBoard = tx_read(systems, &category.key(b"fame_board/"))?.unwrap_or_default();
    let id = character.char_id as u32;
    let existing: Option<CharacterFame> = tx_read(systems, &score_key(id))?;
    if let Some(previous) = existing.as_ref().filter(|score| score.category != Some(category)) {
        if let Some(old_category) = previous.category {
            let mut old: FameBoard = tx_read(systems, &old_category.key(b"fame_board/"))?.unwrap_or_default();
            old.entries.remove(&id);
            tx_write(systems, &old_category.key(b"fame_board/"), &old)?;
            tx_write(systems, &old_category.key(b"rankings/"), &top_ten(&old))?;
        }
        board.entries.insert(id, previous.entry.clone());
    }
    if amount != 0 {
        board.generation = board
            .generation
            .checked_add(1)
            .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Fame sequence overflow".into())))?;
        let entry = board.entries.entry(id).or_insert(FameEntry {
            char_id: id,
            name: character.name.clone(),
            points: 0,
            updated_order: 0,
        });
        entry.points = (i64::from(entry.points) + i64::from(amount)).clamp(0, 1_000_000_000) as u32;
        entry.name = character.name.clone();
        entry.updated_order = board.generation;
        let key = [category.key(b"fame/"), id.to_be_bytes().to_vec()].concat();
        tx_write(systems, &key, entry)?;
    }
    let rankings = top_ten(&board);
    let entry = board.entries.get(&id).cloned().unwrap_or(FameEntry {
        char_id: id,
        name: character.name.clone(),
        points: 0,
        updated_order: 0,
    });
    tx_write(systems, &score_key(id), &CharacterFame {
        entry,
        category: Some(category),
    })?;
    tx_write(systems, &category.key(b"fame_board/"), &board)?;
    tx_write(systems, &category.key(b"rankings/"), &rankings)?;
    Ok(FameUpdate {
        category,
        points: board.entries.get(&id).map_or(0, |entry| entry.points),
        rank: rankings
            .iter()
            .position(|entry| entry.char_id == id)
            .map_or(0, |index| index as u8 + 1),
        rankings,
    })
}

pub fn commit_crafting_fame_tx(
    character: &CharacterRecord,
    game_systems: &TransactionalTree,
    items: &TransactionalTree,
    plan: &CraftingFamePlan,
) -> ConflictableTransactionResult<CraftingFameOutcome, Error> {
    if plan.previous_potion_streak >= 10 || plan.additive_slots > 3 {
        return abort("Invalid crafting fame state");
    }
    let item: ItemModel = tx_required(items, &plan.item_id.to_be_bytes())?;
    let skill = SkillEnum::try_from_value(plan.skill_id).ok();
    let category = FameCategory::for_job(character.class as u32);
    let mut potion_streak = plan.previous_potion_streak;
    let mut gained = 0;
    if (545..=547).contains(&plan.item_id)
        && matches!(
            skill,
            Some(SkillEnum::AmPharmacy | SkillEnum::AmTwilight1 | SkillEnum::AmTwilight2 | SkillEnum::AmTwilight3)
        )
    {
        if plan.success {
            potion_streak += 1;
            if category == Some(FameCategory::Alchemist) {
                gained = match potion_streak {
                    3 => 1,
                    5 => 3,
                    7 => 10,
                    10 => 50,
                    _ => 0,
                };
            }
            if potion_streak == 10 {
                potion_streak = 0;
            }
        } else {
            potion_streak = 0;
        }
    }
    if category == Some(FameCategory::Blacksmith)
        && plan.success
        && item.item_type == ItemType::Weapon
        && item.weapon_level.unwrap_or(0) >= 3
        && plan.additive_slots == 3
    {
        if !skill.is_some_and(|skill| skill.to_name().starts_with("BS_")) {
            return abort("Forging fame requires a classic forging skill");
        }
        gained = 10;
    }
    let mut outcome = CraftingFameOutcome {
        potion_streak,
        category,
        points: 0,
        gained,
        rank: 0,
        rankings: vec![],
    };
    let Some(_category) = category else {
        return Ok(outcome);
    };
    let updated = update_fame_tx(character, game_systems, gained as i32)?;
    outcome.points = updated.points;
    outcome.rankings = updated.rankings;
    outcome.rank = updated.rank;
    Ok(outcome)
}

pub fn refresh_forged_rank(status: &mut Status, ranked_creators: &[u32]) -> bool {
    let mut changed = false;
    for weapon in &mut status.weapons {
        let ranked = weapon.card0 == 255 && weapon.creator_id().is_some_and(|creator| ranked_creators.contains(&creator));
        changed |= weapon.ranked_forged != ranked;
        weapon.ranked_forged = ranked;
    }
    changed
}

#[cfg(test)]
mod tests {
    use sled::transaction::Transactional;

    use super::*;

    fn setup() -> SledRepository {
        let repository = SledRepository::temporary().unwrap();
        repository
            .database
            .items
            .transaction(|items| {
                for (id, kind, level) in [(545, "Healing", 0), (547, "Healing", 0), (1201, "Weapon", 3), (1202, "Weapon", 2)] {
                    let item: ItemModel = serde_json::from_value(
                        serde_json::json!({"id":id,"name_aegis":format!("Item{id}"),"name_english":"Crafted","weight":10,
                    "item_type":kind,"weapon_level":level,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0}),
                    )
                    .unwrap();
                    tx_write(items, &(id as i32).to_be_bytes(), &item)?;
                }
                Ok(())
            })
            .unwrap();
        repository
    }
    fn commit(repository: &SledRepository, id: u32, class: JobName, plan: CraftingFamePlan) -> CraftingFameOutcome {
        let character = CharacterRecord {
            char_id: id as i32,
            name: format!("Creator{id}"),
            class: class.value() as i16,
            ..Default::default()
        };
        (&repository.database.game_systems, &repository.database.items)
            .transaction(|(systems, items)| commit_crafting_fame_tx(&character, systems, items, &plan))
            .unwrap()
    }
    fn forge(item_id: i32, success: bool, additives: u8) -> CraftingFamePlan {
        CraftingFamePlan {
            item_id,
            skill_id: SkillEnum::BsDagger.id(),
            success,
            additive_slots: additives,
            previous_potion_streak: 0,
        }
    }
    #[test]
    fn only_successful_level_three_weapons_with_three_additives_grant_fame() {
        let repository = setup();
        assert_eq!(commit(&repository, 1, JobName::Blacksmith, forge(1201, true, 2)).gained, 0);
        assert_eq!(commit(&repository, 1, JobName::Blacksmith, forge(1201, false, 3)).gained, 0);
        assert_eq!(commit(&repository, 1, JobName::Blacksmith, forge(1202, true, 3)).gained, 0);
        assert_eq!(commit(&repository, 1, JobName::Mage, forge(1201, true, 3)).gained, 0);
        assert_eq!(commit(&repository, 1, JobName::Whitesmith, forge(1201, true, 3)).gained, 10);
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 1).unwrap(), 1);
    }
    #[test]
    fn condensed_potion_streak_awards_all_classic_milestones_and_resets_on_failure() {
        let repository = setup();
        let mut streak = 0;
        let mut total = 0;
        for _ in 0..10 {
            let outcome = commit(&repository, 1, JobName::Alchemist, CraftingFamePlan {
                item_id: 545,
                skill_id: SkillEnum::AmPharmacy.id(),
                success: true,
                additive_slots: 0,
                previous_potion_streak: streak,
            });
            streak = outcome.potion_streak;
            total += outcome.gained;
        }
        assert_eq!((streak, total), (0, 64));
        assert_eq!(repository.fame_points(FameCategory::Alchemist, 1).unwrap(), 64);
        assert_eq!(
            commit(&repository, 1, JobName::Alchemist, CraftingFamePlan {
                item_id: 547,
                skill_id: SkillEnum::AmPharmacy.id(),
                success: false,
                additive_slots: 0,
                previous_potion_streak: 6
            })
            .potion_streak,
            0
        );
    }
    #[test]
    fn ranking_keeps_ten_and_promotes_the_creator_reaching_an_equal_score() {
        let repository = setup();
        for id in 1..=11 {
            commit(&repository, id, JobName::Blacksmith, forge(1201, true, 3));
        }
        assert_eq!(repository.fame_rankings(FameCategory::Blacksmith).unwrap().len(), 10);
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 1).unwrap(), 0);
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 11).unwrap(), 1);
        commit(&repository, 1, JobName::Blacksmith, forge(1201, true, 3));
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 1).unwrap(), 1);
        assert_eq!(repository.fame_points(FameCategory::Blacksmith, 1).unwrap(), 20);
    }
    #[test]
    fn aborted_outer_transaction_rolls_back_fame_and_rankings() {
        let repository = setup();
        let character = CharacterRecord {
            char_id: 1,
            class: JobName::Blacksmith.value() as i16,
            ..Default::default()
        };
        let result: Result<(), sled::transaction::TransactionError<Error>> =
            (&repository.database.game_systems, &repository.database.items).transaction(|(systems, items)| {
                commit_crafting_fame_tx(&character, systems, items, &forge(1201, true, 3))?;
                abort("Craft inventory did not commit")
            });
        assert!(result.is_err());
        assert_eq!(repository.fame_points(FameCategory::Blacksmith, 1).unwrap(), 0);
        assert_eq!(repository.fame_rankings(FameCategory::Blacksmith).unwrap(), vec![]);
    }

    fn seed_character(repository: &SledRepository, class: JobName) {
        repository
            .database
            .seed(
                &database::model::SeedData {
                    accounts: vec![database::model::AccountRecord {
                        account_id: 2_000_000,
                        username: "Creator".into(),
                        password: "secret".into(), ..Default::default()
                    }],
                    characters: vec![CharacterRecord {
                        char_id: 150_000,
                        account_id: 2_000_000,
                        name: "Creator".into(),
                        inventory_slots: 100,
                        class: class.value() as i16,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                false,
            )
            .unwrap();
    }

    #[test]
    fn class_changes_move_canonical_fame_and_rank_membership_atomically() {
        let repository = setup();
        seed_character(&repository, JobName::Blacksmith);
        repository.add_fame(150_000, 2_000_000, 10).unwrap();
        assert!(
            repository
                .character_change_fame_class(150_000, 9, JobName::Alchemist.value() as u32)
                .is_err()
        );
        let updates = repository
            .character_change_fame_class(150_000, 2_000_000, JobName::Alchemist.value() as u32)
            .unwrap();
        assert_eq!(updates.len(), 2);
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 150_000).unwrap(), 0);
        assert_eq!(repository.fame_rank(FameCategory::Alchemist, 150_000).unwrap(), 1);
        assert_eq!(repository.character_fame(150_000).unwrap(), 10);
        repository
            .character_change_fame_class(150_000, 2_000_000, JobName::Mage.value() as u32)
            .unwrap();
        assert_eq!(repository.fame_rank(FameCategory::Alchemist, 150_000).unwrap(), 0);
        assert_eq!(repository.character_fame(150_000).unwrap(), 10);
        repository
            .character_change_fame_class(150_000, 2_000_000, JobName::Blacksmith.value() as u32)
            .unwrap();
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 150_000).unwrap(), 1);
        let saved: CharacterRecord = database::required(&repository.database.characters, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!(saved.class, JobName::Blacksmith.value() as i16);
    }

    #[test]
    fn mission_completion_commits_progress_variables_and_fame_together() {
        let repository = setup();
        seed_character(&repository, JobName::Taekwon);
        let initial = repository.set_taekwon_mission(150_000, 2_000_000, None, 1002).unwrap();
        assert_eq!(repository.record_taekwon_kill(150_000, 2_000_000, 1003, None).unwrap(), None);
        for _ in 0..99 {
            repository.record_taekwon_kill(150_000, 2_000_000, 1002, None).unwrap();
        }
        assert!(repository.set_taekwon_mission(150_000, 2_000_000, Some(initial), 1004).is_err());
        assert!(repository.record_taekwon_kill(150_000, 2_000_000, 1002, None).is_err());
        assert_eq!(repository.taekwon_mission(150_000).unwrap().unwrap().kills, 99);
        assert_eq!(repository.character_fame(150_000).unwrap(), 0);
        let outcome = repository
            .record_taekwon_kill(150_000, 2_000_000, 1002, Some(1004))
            .unwrap()
            .unwrap();
        assert_eq!(outcome.mission, TaekwonMission { mob_id: 1004, kills: 0 });
        assert_eq!(outcome.fame.unwrap().points, 1);
        assert_eq!(
            repository.record_taekwon_kill(150_000, 2_000_000, 1002, Some(1004)).unwrap(),
            None
        );
        use crate::repository::ScriptVariableRepository;
        assert_eq!(
            repository.script_variable_char_num_fetch_one(150_000, "TK_MISSION_ID".into(), 0),
            1004
        );
        assert_eq!(
            repository.script_variable_char_num_fetch_one(150_000, "TK_MISSION_COUNT".into(), 0),
            0
        );
    }

    #[test]
    fn non_fame_crafts_with_zero_skill_succeed_without_altering_score_or_streak() {
        let repository = setup();
        let outcome = commit(&repository, 1, JobName::Alchemist, CraftingFamePlan {
            item_id: 545,
            skill_id: 0,
            success: true,
            additive_slots: 0,
            previous_potion_streak: 6,
        });
        assert_eq!((outcome.gained, outcome.points, outcome.potion_streak), (0, 0, 6));
    }
}
