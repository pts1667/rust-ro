//! Quest log (`quest.cpp`): adding, completing and erasing quests, hunting counters and the quest icons of NPCs.
//! Quest content (the NPCs that hand out quests) is written with the quest functions of the script SDK.

use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::quest::{
    LocalClock, QuestCheck, QuestDefinition, QuestEntry, QuestError, QuestKill, QuestLog, QuestObjective, QuestState, quest_definition,
};
use crate::server::model::quest_info::{Condition, ConditionEnvironment, QuestInfoEntry};
use crate::server::script::ScriptRequest;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::quest_packets::{self as wire, ObjectiveView, QuestView};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// `AREA_SIZE` of rathena: party members this close to the monster share its quest credit.
const PARTY_SHARE_RANGE: u16 = 14;
/// Shown by the client when an objective has no monster of its own ("Poring").
const FALLBACK_MOB_ID: u32 = 1002;
const ICON_QUEST: u16 = 0;
const MARK_MAX: i32 = 4;

/// The `questinfo` icon ids of the 2012 clients are the script ids shifted by one (0 shows nothing).
fn client_icon(icon: i32) -> u16 {
    if (0..=7).contains(&icon) { icon as u16 + 1 } else { ICON_QUEST }
}

fn objective_text(objective: &QuestObjective) -> String {
    let element = match objective.element.as_str() {
        "All" => String::new(),
        "Dark" => "Shadow Element".into(),
        other => format!("{other} Element"),
    };
    let parts = [objective.map_name.as_str(), objective_filter(&objective.race), objective_filter(&objective.size), element.as_str()];
    parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(", ")
}

fn objective_filter(value: &str) -> &str {
    if value == "All" { "" } else { value }
}

fn objective_view(objective: &QuestObjective, done: u16) -> ObjectiveView {
    let mob = (objective.mob != 0).then(|| GlobalConfigService::instance().get_mob_safe(i32::from(objective.mob))).flatten();
    ObjectiveView {
        mob_id: mob.map_or(FALLBACK_MOB_ID, |mob| mob.id as u32),
        name: mob.map_or_else(|| objective_text(objective), |mob| mob.name_english.clone()),
        done,
        total: objective.count,
    }
}

fn quest_view(entry: &QuestEntry, definition: &QuestDefinition) -> QuestView {
    QuestView {
        quest_id: entry.quest_id,
        active: entry.state != QuestState::Inactive,
        start_time: entry.expires_at.wrapping_sub(definition.time as u32),
        expire_time: entry.expires_at,
        objectives: definition.objectives.iter().zip(entry.counts).map(|(objective, done)| objective_view(objective, done)).collect(),
    }
}

fn views(log: &QuestLog) -> Vec<QuestView> {
    log.open_entries().iter().filter_map(|entry| Some(quest_view(entry, quest_definition(entry.quest_id)?))).collect()
}

struct CharacterConditions<'a> {
    server: &'a Server,
    character: &'a Character,
}

impl ConditionEnvironment for CharacterConditions<'_> {
    fn name(&self, name: &str) -> Result<i64, String> {
        let status = &self.character.status;
        Ok(match name {
            "BaseLevel" => i64::from(status.base_level),
            "JobLevel" => i64::from(status.job_level),
            "Class" => i64::from(status.job),
            "Zeny" => i64::from(self.character.get_zeny()),
            _ => match crate::server::script::constant::load_constant(&name.to_string()) {
                Some(Value::Number(value)) => i64::from(value),
                _ => i64::from(self.server.repository.script_variable_char_num_fetch_one(self.character.char_id, name.to_string(), 0)),
            },
        })
    }

    fn call(&self, function: &str, arguments: &[i64]) -> Result<i64, String> {
        let quest_id = |index: usize| arguments.get(index).copied().and_then(|id| u32::try_from(id).ok()).ok_or("quest id is required");
        let log = &self.character.game_systems.quests;
        Ok(match function {
            "isbegin_quest" => {
                let have = i64::from(log.check(quest_id(0)?, QuestCheck::Have, LocalClock::current().now));
                have + i64::from(have < 1)
            }
            "checkquest" => {
                let kind = arguments.get(1).copied().map_or(Some(QuestCheck::Have), |kind| QuestCheck::from_value(kind as i32)).ok_or("unknown checkquest type")?;
                i64::from(log.check(quest_id(0)?, kind, LocalClock::current().now))
            }
            "countitem" => {
                let item_id = arguments.first().copied().ok_or("item id is required")?;
                self.character.inventory.iter().flatten().filter(|item| i64::from(item.item_id) == item_id).map(|item| i64::from(item.amount)).sum()
            }
            other => return Err(format!("function {other} is not supported in conditions")),
        })
    }
}

impl Server {
    /// Loads the quest log when the character first enters a map; the icons of the NPCs are rebuilt on every map.
    pub(crate) fn quest_login(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        character.game_systems.quest_icons.clear();
        if !character.game_systems.quests_loaded {
            let entries = self.repository.quest_entries(char_id).unwrap_or_else(|error| {
                warn!("Quest log of {char_id} could not be loaded: {error}");
                Vec::new()
            });
            character.game_systems.quests = QuestLog::from_entries(entries);
            character.game_systems.quests_loaded = true;
            let quests = views(&character.game_systems.quests);
            if !quests.is_empty() {
                self.send_raw(char_id, wire::quest_list(&quests));
                self.send_raw(char_id, wire::quest_missions(&quests));
                for quest in &quests {
                    self.send_raw(char_id, wire::quest_progress(quest));
                }
            }
        }
        self.show_quest_icons(state, char_id);
    }

    fn save_quests(&self, character: &Character) {
        if let Err(error) = self.repository.quest_entries_save(character.char_id, character.game_systems.quests.entries()) {
            error!("Quest log of {} could not be saved: {error}", character.char_id);
        }
    }

    fn announce_new_quest(&self, char_id: u32, entry: &QuestEntry) {
        if let Some(definition) = quest_definition(entry.quest_id) {
            let view = quest_view(entry, definition);
            self.send_raw(char_id, wire::quest_added(&view));
            self.send_raw(char_id, wire::quest_progress(&view));
        }
    }

    /// `setquest`
    pub(crate) fn quest_add(&self, state: &mut ServerState, char_id: u32, quest_id: u32) -> Result<(), QuestError> {
        let character = state.characters_mut().get_mut(&char_id).ok_or(QuestError::Missing(quest_id))?;
        let entry = character.game_systems.quests.add(quest_id, LocalClock::current())?;
        self.save_quests(character);
        self.announce_new_quest(char_id, &entry);
        self.show_quest_icons(state, char_id);
        Ok(())
    }

    /// `changequest`
    pub(crate) fn quest_change(&self, state: &mut ServerState, char_id: u32, old_id: u32, new_id: u32) -> Result<(), QuestError> {
        let character = state.characters_mut().get_mut(&char_id).ok_or(QuestError::Missing(old_id))?;
        let entry = character.game_systems.quests.change(old_id, new_id, LocalClock::current())?;
        self.save_quests(character);
        self.send_raw(char_id, wire::quest_deleted(old_id));
        self.announce_new_quest(char_id, &entry);
        self.show_quest_icons(state, char_id);
        Ok(())
    }

    /// `erasequest`
    pub(crate) fn quest_erase(&self, state: &mut ServerState, char_id: u32, quest_id: u32) -> Result<(), QuestError> {
        let character = state.characters_mut().get_mut(&char_id).ok_or(QuestError::Missing(quest_id))?;
        character.game_systems.quests.delete(quest_id)?;
        self.save_quests(character);
        self.send_raw(char_id, wire::quest_deleted(quest_id));
        self.show_quest_icons(state, char_id);
        Ok(())
    }

    /// `completequest`: the quest leaves the client log but stays recorded as completed.
    pub(crate) fn quest_complete(&self, state: &mut ServerState, char_id: u32, quest_id: u32) -> Result<(), QuestError> {
        let character = state.characters_mut().get_mut(&char_id).ok_or(QuestError::Missing(quest_id))?;
        character.game_systems.quests.set_state(quest_id, QuestState::Complete)?;
        self.save_quests(character);
        self.send_raw(char_id, wire::quest_deleted(quest_id));
        self.show_quest_icons(state, char_id);
        Ok(())
    }

    /// `CZ_ACTIVE_QUEST`: the player toggles a quest between active and inactive.
    pub(crate) fn quest_set_active(&self, state: &mut ServerState, char_id: u32, quest_id: u32, active: bool) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        let target = if active { QuestState::Active } else { QuestState::Inactive };
        match character.game_systems.quests.set_state(quest_id, target) {
            Ok(()) => {
                self.save_quests(character);
                self.send_raw(char_id, wire::quest_activated(quest_id, active));
            }
            Err(error) => warn!("Quest activation of {char_id} refused: {error}"),
        }
    }

    pub(crate) fn quest_check(state: &ServerState, char_id: u32, quest_id: u32, check: QuestCheck) -> i32 {
        state.get_character(char_id).map_or(-1, |character| character.game_systems.quests.check(quest_id, check, LocalClock::current().now))
    }

    /// A monster died: counts it for the killer, or for the party members standing near it.
    pub(crate) fn quest_kill(&self, state: &mut ServerState, killer_id: u32, mob_id: u16, key: &MapInstanceKey, mob_x: u16, mob_y: u16) {
        let Some(killer) = state.get_character(killer_id) else { return };
        let party_id = killer.game_systems.party_id;
        let near = |character: &&Character| {
            character.map_instance_key == *key && character.x().abs_diff(mob_x) <= PARTY_SHARE_RANGE && character.y().abs_diff(mob_y) <= PARTY_SHARE_RANGE
        };
        let credited: Vec<u32> = if party_id != 0 {
            state.characters().values().filter(|character| character.game_systems.party_id == party_id).filter(near).map(|character| character.char_id).collect()
        } else {
            vec![killer_id]
        };
        let Some(mob) = GlobalConfigService::instance().get_mob_safe(i32::from(mob_id)) else { return };
        for char_id in credited {
            let Some(character) = state.characters_mut().get_mut(&char_id) else { continue };
            if character.game_systems.quests.open_entries().is_empty() {
                continue;
            }
            let map = character.current_map_name().clone();
            let kill = QuestKill { mob_id, level: mob.level as u16, race: &mob.race, size: &mob.size, element: &mob.element, map: &map };
            let (updated, drops) = character.game_systems.quests.record_kill(&kill);
            if !updated.is_empty() {
                self.save_quests(character);
            }
            for quest_id in updated {
                let view = character.game_systems.quests.find(quest_id).zip(quest_definition(quest_id)).map(|(entry, definition)| quest_view(entry, definition));
                if let Some(view) = view {
                    self.send_raw(char_id, wire::quest_progress(&view));
                }
            }
            for drop in drops {
                if drop.rate < 10_000 && fastrand::u16(0..10_000) >= drop.rate {
                    continue;
                }
                if let Err(error) = self.grant_quest_item(character, drop.item, drop.count) {
                    warn!("Quest item {} was not given to {char_id}: {error}", drop.item);
                }
            }
            self.show_quest_icons(state, char_id);
        }
    }

    fn grant_quest_item(&self, character: &mut Character, item_id: u32, amount: u16) -> Result<(), String> {
        use crate::repository::model::item_model::InventoryItemModel;
        use crate::server::model::events::game_event::CharacterAddItems;
        let item = GlobalConfigService::instance().find_item(item_id as i32).ok_or("unknown item")?;
        let model = InventoryItemModel::from_item_model(item, amount.min(i16::MAX as u16) as i16, true);
        self.inventory_service().try_add_items_in_inventory(
            self.runtime(),
            CharacterAddItems { char_id: character.char_id, should_perform_check: true, buy: false, items: vec![model] },
            character,
        )
    }

    /// Shows or hides the quest icons above the NPCs of the map the character stands on (`pc_show_questinfo`).
    pub(crate) fn show_quest_icons(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.get_character(char_id) else { return };
        let key = character.map_instance_key.clone();
        let Some(instance) = state.get_map_instance(key.map_name(), key.map_instance()) else { return };
        let conditions = CharacterConditions { server: self, character };
        let mut wanted = Vec::new();
        for (npc_id, entries) in state.quest_infos.on_map(key.map_name(), key.map_instance()) {
            let Some((x, y, hidden)) = instance.state().script_skill_state.npcs.get(&npc_id).map(|npc| (npc.x, npc.y, npc.hidden)) else { continue };
            let shown = (!hidden)
                .then(|| entries.iter().find(|entry| entry.condition.as_ref().map_or(true, |condition| condition.evaluate(&conditions).is_ok_and(|value| value != 0))))
                .flatten();
            wanted.push((npc_id, x, y, shown.map(|entry| (entry.icon, entry.color))));
        }
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        for (npc_id, x, y, icon) in wanted {
            let shown = character.game_systems.quest_icons.get(&npc_id).copied();
            if shown == icon {
                continue;
            }
            match icon {
                Some((icon, color)) => {
                    character.game_systems.quest_icons.insert(npc_id, (icon, color));
                    self.send_raw(char_id, wire::quest_icon(npc_id, x, y, icon, u16::from(color)));
                }
                None => {
                    character.game_systems.quest_icons.remove(&npc_id);
                    self.send_raw(char_id, wire::quest_icon(npc_id, x, y, 0, 0));
                }
            }
        }
    }

    /// `showevent`: one icon for one player, outside the `questinfo` bookkeeping.
    fn show_event(&self, char_id: u32, npc: (u32, u16, u16), icon: i32, color: i32) {
        let icon = if (0..=7).contains(&icon) { icon as u16 + 1 } else { 0 };
        self.send_raw(char_id, wire::quest_icon(npc.0, npc.1, npc.2, icon, color as u16));
    }
}

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::SetQuest
            | Function::CompleteQuest
            | Function::EraseQuest
            | Function::ChangeQuest
            | Function::CheckQuest
            | Function::IsBeginQuest
            | Function::QuestInfo
            | Function::QuestInfoRefresh
            | Function::ShowEvent
    )
}

fn number(arguments: &[Value], index: usize) -> Result<i32, String> {
    arguments.get(index).ok_or("Missing argument")?.number_value()
}

fn quest_id(arguments: &[Value], index: usize) -> Result<u32, String> {
    u32::try_from(number(arguments, index)?).map_err(|_| "Invalid quest id".to_string())
}

fn color(arguments: &[Value], index: usize) -> Result<i32, String> {
    let color = arguments.get(index).map(Value::number_value).transpose()?.unwrap_or(0);
    if (0..MARK_MAX).contains(&color) {
        return Ok(color);
    }
    warn!("Invalid quest mark colour {color}, using none");
    Ok(0)
}

impl Server {
    /// Script functions of the quest log. Quest errors are logged and the script goes on, as in rathena.
    pub(crate) fn quest_script_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let character_argument = match function {
            Function::SetQuest | Function::CompleteQuest | Function::EraseQuest | Function::IsBeginQuest => 1,
            Function::ChangeQuest | Function::CheckQuest | Function::ShowEvent => 2,
            _ => 0,
        };
        let char_id = match function {
            Function::QuestInfo => context.char_id,
            _ => match arguments.get(character_argument).map(Value::number_value).transpose()? {
                Some(id) if id != 0 => u32::try_from(id).map_err(|_| "Invalid script target character")?,
                _ => context.char_id,
            },
        };
        if function != Function::QuestInfo && !state.characters().contains_key(&char_id) {
            return Err("Script target character disconnected".into());
        }
        let report = |result: Result<(), QuestError>| -> Reply {
            if let Err(error) = result {
                warn!("{function:?} of NPC {} failed: {error}", context.npc_id);
            }
            Ok(Value::default())
        };
        match function {
            Function::SetQuest => report(self.quest_add(state, char_id, quest_id(arguments, 0)?)),
            Function::EraseQuest => report(self.quest_erase(state, char_id, quest_id(arguments, 0)?)),
            Function::CompleteQuest => report(self.quest_complete(state, char_id, quest_id(arguments, 0)?)),
            Function::ChangeQuest => report(self.quest_change(state, char_id, quest_id(arguments, 0)?, quest_id(arguments, 1)?)),
            Function::CheckQuest => {
                let check = arguments.get(1).map(Value::number_value).transpose()?.map_or(Some(QuestCheck::Have), QuestCheck::from_value);
                let check = check.ok_or("Unknown checkquest type")?;
                Ok(Value::Number(Self::quest_check(state, char_id, quest_id(arguments, 0)?, check)))
            }
            Function::IsBeginQuest => {
                let have = Self::quest_check(state, char_id, quest_id(arguments, 0)?, QuestCheck::Have);
                Ok(Value::Number(have + i32::from(have < 1)))
            }
            Function::QuestInfoRefresh => {
                self.show_quest_icons(state, char_id);
                Ok(Value::default())
            }
            Function::QuestInfo => {
                let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("questinfo requires an NPC")?;
                let condition = match arguments.get(2) {
                    Some(text) if !text.text().trim().is_empty() => Some(Condition::parse(&text.text())?),
                    _ => None,
                };
                let entry = QuestInfoEntry { icon: client_icon(number(arguments, 0)?), color: color(arguments, 1)? as u8, condition };
                state.quest_infos.register((npc.map, npc.instance, npc.id), entry);
                Ok(Value::default())
            }
            Function::ShowEvent => {
                let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("showevent requires an NPC")?;
                self.show_event(char_id, (npc.id, npc.x, npc.y), number(arguments, 0)?, color(arguments, 1)?);
                Ok(Value::default())
            }
            _ => Err("Unsupported quest function".into()),
        }
    }
}

impl Server {
    /// `@setquest`, `@erasequest`, `@completequest`, `@checkquest`, `@questskill` and `@lostskill`; the reply is one line per message.
    pub(crate) fn quest_command(&self, state: &mut ServerState, char_id: u32, command: &str, argument: &str) -> String {
        let id = argument.split_whitespace().next().and_then(|text| text.parse::<i64>().ok()).filter(|id| *id > 0);
        if matches!(command, "questskill" | "lostskill") {
            return self.quest_skill_command(state, char_id, command == "questskill", id);
        }
        let Some(quest_id) = id.and_then(|id| u32::try_from(id).ok()) else { return format!("Usage: @{command} <quest ID>") };
        if quest_definition(quest_id).is_none() {
            return format!("Quest {quest_id} not found in DB.");
        }
        let have = |state: &ServerState| Self::quest_check(state, char_id, quest_id, QuestCheck::Have);
        let result = match command {
            "setquest" if have(state) >= 0 => return format!("Character already has quest {quest_id}."),
            "setquest" => self.quest_add(state, char_id, quest_id),
            "erasequest" if have(state) < 0 => return format!("Character doesn't have quest {quest_id}."),
            "erasequest" => self.quest_erase(state, char_id, quest_id),
            "completequest" => {
                let added = if have(state) < 0 { self.quest_add(state, char_id, quest_id) } else { Ok(()) };
                added.and_then(|()| if have(state) < 2 { self.quest_complete(state, char_id, quest_id) } else { Ok(()) })
            }
            "checkquest" => {
                let value = |check| Self::quest_check(state, char_id, quest_id, check);
                return format!(
                    "Checkquest value for quest {quest_id}
>    HAVEQUEST : {}
>    HUNTING   : {}
>    PLAYTIME  : {}",
                    value(QuestCheck::Have),
                    value(QuestCheck::Hunting),
                    value(QuestCheck::PlayTime)
                );
            }
            _ => return String::new(),
        };
        result.err().map(|error| error.to_string()).unwrap_or_default()
    }

    fn quest_skill_command(&self, state: &mut ServerState, char_id: u32, grant: bool, skill_id: Option<i64>) -> String {
        use crate::server::service::script_character_service::{grant_skill, is_quest_skill, learned_level};
        let Some(skill_id) = skill_id.and_then(|id| i32::try_from(id).ok()) else { return "Please enter a quest skill number.".into() };
        if GlobalConfigService::instance().find_skill_config(&Value::Number(skill_id)).is_none() {
            return "This skill number doesn't exist.".into();
        }
        if !is_quest_skill(skill_id as u32) {
            return "This skill number doesn't exist or isn't a quest skill.".into();
        }
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return String::new() };
        let known = learned_level(&character.status, skill_id as u32) > 0;
        match (grant, known) {
            (true, true) => return "You already have this quest skill.".into(),
            (false, false) => return "You don't have this quest skill.".into(),
            _ => {}
        }
        let level = i32::from(grant);
        match grant_skill(self, character, &[Value::Number(skill_id), Value::Number(level), Value::Number(0)]) {
            Ok(_) if grant => "You have learned the skill.".into(),
            Ok(_) => "You have forgotten the skill.".into(),
            Err(error) => error,
        }
    }
}
