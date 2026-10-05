use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use models::enums::skill_enums::SkillEnum;
use script_sdk::{Reply, Value};

use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::persistence_event::SavePositionUpdate;
use crate::server::model::game_systems::MemoPoint;
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub(crate) fn position_for_save(character: &Character, flags: &MapFlags) -> (String, u16, u16) {
    if character.status.hp == 0 {
        return (normalize_map(&character.save_map), character.save_x, character.save_y);
    }
    if !flags.enabled(MapFlag::NoSave) {
        return (normalize_map(character.current_map_name()), character.x(), character.y());
    }
    if let Some((map, x, y)) = &flags.save {
        if !map.eq_ignore_ascii_case("SavePoint") {
            return (normalize_map(map), *x, *y);
        }
    }
    (normalize_map(&character.save_map), character.save_x, character.save_y)
}

pub(crate) fn position_update(character: &Character, flags: &MapFlags) -> SavePositionUpdate {
    let (map_name, x, y) = position_for_save(character, flags);
    SavePositionUpdate {
        char_id: character.char_id,
        account_id: character.account_id,
        map_name,
        x,
        y,
        revision: character.next_position_revision(),
    }
}

pub(crate) fn configured_position_update(character: &Character) -> SavePositionUpdate {
    let flags = GlobalConfigService::instance()
        .find_map(&normalize_map(character.current_map_name()))
        .map(|map| map.flags().clone())
        .unwrap_or_default();
    position_update(character, &flags)
}

pub(crate) fn restore_login_position(state: &ServerState, character: &mut Character) -> Result<(), String> {
    let current = normalize_map(character.current_map_name());
    let flags = state.map_flags(&character.map_instance_key);
    let unavailable = GlobalConfigService::instance().find_map(&current).is_none();
    let dead = character.status.hp == 0;
    let position = if unavailable || dead {
        Some((normalize_map(&character.save_map), character.save_x, character.save_y))
    } else if flags.enabled(MapFlag::NoSave) && current != "sec_pri" {
        Some(position_for_save(character, &flags))
    } else {
        None
    };
    if let Some((map, x, y)) = position {
        if GlobalConfigService::instance().find_map(&map).is_none() {
            return Err("Character relogin destination is unavailable".into());
        }
        character.map_instance_key = MapInstanceKey::new(map, 0);
        character.update_position(x, y);
        character.refresh_script_context();
    }
    if dead {
        let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
        let (hp, sp) = respawn_resources(&snapshot, &GlobalConfigService::instance().config().game);
        character.status.hp = hp;
        character.status.sp = sp;
        character.refresh_script_context();
    }
    Ok(())
}

pub(crate) fn respawn_resources(status: &models::status::StatusSnapshot, config: &configuration::configuration::GameConfig) -> (u32, u32) {
    use models::enums::EnumWithNumberValue;
    use models::enums::class::JobName;
    if status
        .bonuses_raw()
        .iter()
        .any(|bonus| matches!(bonus, models::enums::bonus::BonusType::EnableFullHpSpRecoverOnResurrect))
    {
        return (status.max_hp(), status.max_sp());
    }
    let hp_rate = if JobName::try_from_value(status.job() as usize).is_ok_and(|job| job.is_novice()) {
        config.restart_hp_rate.max(50)
    } else {
        config.restart_hp_rate
    }
    .min(100);
    let hp = (u64::from(status.max_hp()) * u64::from(hp_rate) / 100).max(1) as u32;
    let sp = (u64::from(status.max_sp()) * u64::from(config.restart_sp_rate.min(100)) / 100).max(1) as u32;
    (hp.min(status.max_hp()), sp.max(status.sp()).min(status.max_sp()))
}

pub(crate) fn set_save_point(server: &Server, state: &mut ServerState, character: &mut Character, arguments: &[Value]) -> Reply {
    if !(3..=6).contains(&arguments.len()) {
        return Err("SavePoint requires map, x, y, optional ranges, and optional character ID".into());
    }
    let name = normalize_map(arguments[0].string_value()?);
    let map = GlobalConfigService::instance()
        .find_map(&name)
        .ok_or("Save point map is unavailable")?;
    let instance = state
        .get_map_instance(&name, 0)
        .unwrap_or_else(|| server.server_service().create_map_instance(state, map, 0));
    let center = (arguments[1].number_value()?, arguments[2].number_value()?);
    let range = if arguments.len() > 4 {
        (arguments[3].number_value()?, arguments[4].number_value()?)
    } else {
        (0, 0)
    };
    if range.0 < 0 || range.1 < 0 || range.0 > i32::from(u16::MAX) || range.1 > i32::from(u16::MAX) {
        return Err("Invalid save point range".into());
    }
    let state = instance.state();
    let walkable = |x: i32, y: i32| {
        x >= 0
            && y >= 0
            && x < i32::from(instance.x_size())
            && y < i32::from(instance.y_size())
            && state
                .cells()
                .get(y as usize * usize::from(instance.x_size()) + x as usize)
                .is_some_and(|cell| cell & CellType::Walkable.as_flag() != 0)
    };
    let mut coordinates = None;
    for _ in 0..if range == (0, 0) { 1 } else { 10 } {
        let x = center
            .0
            .checked_add(fastrand::i32(-range.0..=range.0))
            .ok_or("Save point x coordinate overflow")?;
        let y = center
            .1
            .checked_add(fastrand::i32(-range.1..=range.1))
            .ok_or("Save point y coordinate overflow")?;
        if walkable(x, y) {
            coordinates = Some((x as u16, y as u16));
            break;
        }
    }
    let (x, y) = coordinates.ok_or("Save point is not walkable")?;
    let expected = (normalize_map(&character.save_map), character.save_x, character.save_y);
    server
        .repository
        .character_set_save_point(character.char_id, character.account_id, &expected, &(name.clone(), x, y))
        .map_err(|error| error.to_string())?;
    character.save_map = name;
    character.save_x = x;
    character.save_y = y;
    Ok(Value::default())
}

impl Server {
    pub(crate) fn respawn_character(
        &self,
        state: &mut ServerState,
        request: crate::server::model::character_lifecycle::CharacterRespawn,
    ) -> Result<(), String> {
        if !state
            .find_session(request.session.account_id)
            .is_some_and(|current| std::sync::Arc::ptr_eq(&current, &request.session))
        {
            return Err("Respawn session expired".into());
        }
        let char_id = request.session.char_id.ok_or("Respawn requires a character")?;
        let character = state.get_character(char_id).ok_or("Respawn character disconnected")?;
        if character.account_id != request.session.account_id || !character.loaded_from_client_side || character.status.hp != 0 {
            return Ok(());
        }
        let map = normalize_map(&character.save_map);
        if GlobalConfigService::instance().find_map(&map).is_none() {
            return Err("Respawn destination is unavailable".into());
        }
        let (x, y) = (character.save_x, character.save_y);
        let mut character = state.characters_mut().remove(&char_id).ok_or("Respawn character disconnected")?;
        let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
        let (hp, sp) = respawn_resources(&snapshot, &self.configuration.game);
        self.script_skill_service().cancel_queued_cast(&mut character);
        character.pending_item_skill = None;
        character.movements.clear();
        character.clear_attack();
        character.clear_pending_skill();
        character.clear_skill_in_use();
        character.script_skill_state.casting_until = 0;
        character.script_skill_state.running = false;
        character.action = crate::server::state::character::CharacterAction::Idle;
        character.loaded_from_client_side = false;
        self.character_service().update_hp_sp(&mut character, hp, sp);
        character.refresh_script_context();
        state.insert_character(character);
        self.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, char_id);
        Ok(())
    }

    pub(crate) fn memo_location(
        &self,
        state: &mut ServerState,
        request: crate::server::model::character_lifecycle::CharacterMemo,
    ) -> Result<(), String> {
        if !state
            .find_session(request.session.account_id)
            .is_some_and(|current| std::sync::Arc::ptr_eq(&current, &request.session))
        {
            return Err("Memo session expired".into());
        }
        let char_id = request.session.char_id.ok_or("Memo requires a character")?;
        let character = state.get_character(char_id).ok_or("Memo character disconnected")?;
        if character.account_id != request.session.account_id
            || !character.loaded_from_client_side
            || character.status.hp == 0
            || character.is_dead()
        {
            return Err("Character cannot memorize a location".into());
        }
        let notify = |data| {
            self.server_service()
                .notification_sender()
                .send(Notification::Char(CharNotification::new(char_id, data)))
                .map_err(|_| "Memo notification failed".to_string())
        };
        let flags = state.map_flags(&character.map_instance_key);
        if flags.enabled(MapFlag::NoMemo) || flags.enabled(MapFlag::NoWarpTo) {
            return notify([0x0189_u16.to_le_bytes(), 1_u16.to_le_bytes()].concat());
        }
        let level = character
            .status
            .known_skills
            .iter()
            .filter(|skill| skill.value == SkillEnum::AlWarp)
            .map(|skill| skill.level)
            .max()
            .unwrap_or(0);
        if level < 2 {
            return notify([0x011E_u16.to_le_bytes().to_vec(), vec![if level == 0 { 2 } else { 1 }]].concat());
        }
        if character.current_map_instance() != 0 {
            let message = b"You cannot create a memo in an instance.\0";
            return notify(
                [
                    0x008E_u16.to_le_bytes().to_vec(),
                    ((message.len() + 4) as u16).to_le_bytes().to_vec(),
                    message.to_vec(),
                ]
                .concat(),
            );
        }
        let point = MemoPoint {
            map: normalize_map(character.current_map_name()),
            x: character.x(),
            y: character.y(),
        };
        let mut character = state.characters_mut().remove(&char_id).ok_or("Memo character disconnected")?;
        let mut points: Vec<_> = character
            .game_systems
            .memo_points
            .iter()
            .flatten()
            .filter(|saved| saved.map != point.map)
            .cloned()
            .collect();
        points.insert(0, point);
        character.game_systems.memo_points = std::array::from_fn(|index| points.get(index).cloned());
        let result = self.script_world_service().persist(&mut character);
        state.insert_character(character);
        result?;
        notify([0x011E_u16.to_le_bytes().to_vec(), vec![0]].concat())
    }
}
