use models::enums::cell::CellType;
use models::enums::EnumWithMaskValueU16;
use script_sdk::Value;

use crate::server::Server;
use crate::server::model::events::game_event::{ScriptPartyWarp, ScriptWarp};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub(crate) fn party_warp_request(character: &Character, args: &[Value]) -> Result<ScriptPartyWarp, String> {
    if !(3..=7).contains(&args.len()) { return Err("Party warp requires three to seven arguments".into()); }
    let coordinate = |index: usize| args.get(index).map(Value::number_value).transpose()?.map_or(Ok(0), |value| u16::try_from(value).map_err(|_| "Invalid party warp coordinate".to_string()));
    let party_id = args.get(3).map(Value::number_value).transpose()?.map_or(Ok(character.game_systems.party_id), |id| u32::try_from(id).map_err(|_| "Invalid party identifier"))?;
    if party_id == 0 { return Err("Party warp requires a party".into()); }
    Ok(ScriptPartyWarp { char_id: character.char_id, party_id, map: args[0].string_value()?.to_string(),
        x: coordinate(1)?, y: coordinate(2)?, source_map: args.get(4).map(Value::string_value).transpose()?.map(|name| normalize_map(name)),
        range_x: coordinate(5)?, range_y: coordinate(6)? })
}

fn walkable(cells: &[u16], width: u16, height: u16, x: u16, y: u16) -> bool {
    x < width && y < height && cells.get(usize::from(y) * usize::from(width) + usize::from(x)).is_some_and(|cell| cell & CellType::Walkable.as_flag() != 0)
}

fn random_cell(cells: &[u16], width: u16, height: u16) -> Result<(u16, u16), String> {
    if width < 3 || height < 3 { return Err("Party teleport map has no interior cells".into()); }
    let mut selected = None;
    let mut count = 0_u32;
    for (index, cell) in cells.iter().enumerate() {
        let x = (index % usize::from(width)) as u16;
        let y = (index / usize::from(width)) as u16;
        if x == 0 || y == 0 || x >= width - 1 || y >= height - 1 || cell & CellType::Walkable.as_flag() == 0 { continue; }
        count += 1;
        if fastrand::u32(0..count) == 0 { selected = Some((x, y)); }
    }
    selected.ok_or("Party teleport map has no walkable destination".into())
}

fn destination_cell(state: &ServerState, key: &MapInstanceKey, center: Option<(u16, u16)>, range: (u16, u16)) -> Result<(u16, u16), String> {
    let instance = state.get_map_instance(key.map_name(), key.map_instance()).ok_or("Party warp destination instance is unavailable")?;
    let map = instance.state();
    let (width, height) = (instance.x_size(), instance.y_size());
    let Some((x, y)) = center else { return random_cell(map.cells(), width, height); };
    if !walkable(map.cells(), width, height, x, y) { return Err("Party warp destination is not walkable".into()); }
    if range != (0, 0) {
        let x_range = x.saturating_sub(range.0)..=x.saturating_add(range.0).min(width.saturating_sub(1));
        let y_range = y.saturating_sub(range.1)..=y.saturating_add(range.1).min(height.saturating_sub(1));
        for _ in 0..10 {
            let (x, y) = (fastrand::u16(x_range.clone()), fastrand::u16(y_range.clone()));
            if walkable(map.cells(), width, height, x, y) { return Ok((x, y)); }
        }
    }
    Ok((x, y))
}

impl Server {
    pub(crate) fn plan_party_warp(&self, state: &mut ServerState, event: &ScriptPartyWarp) -> Result<Vec<ScriptWarp>, String> {
        let party = self.repository.party(event.party_id).map_err(|error| error.to_string())?.ok_or("Party no longer exists")?;
        let source = state.get_character(event.char_id);
        let source_key = source.map(|source| source.map_instance_key.clone());
        let source_save = source.map(|source| (normalize_map(&source.save_map), source.save_x, source.save_y));
        let common = match event.map.as_str() {
            "RandomAll" => {
                let key = source_key.clone().ok_or("Party teleport source disconnected")?;
                let flags = state.map_flags(&key);
                if flags.enabled(MapFlag::NoWarp) || flags.enabled(MapFlag::NoTeleport) { return Err("Party teleport is not allowed on this map".into()); }
                let (x, y) = destination_cell(state, &key, None, (0, 0))?;
                Some((key, x, y))
            }
            "Leader" => {
                let leader = state.get_character(party.leader_char_id).filter(|leader| leader.game_systems.party_id == party.id).ok_or("Party leader is offline")?;
                Some((leader.map_instance_key.clone(), leader.x, leader.y))
            }
            "SavePoint" => {
                let (map, x, y) = source_save.ok_or("Party warp source disconnected")?;
                Some((MapInstanceKey::new(map, 0), x, y))
            }
            "Random" | "SavePointAll" => None,
            map => {
                let name = normalize_map(map);
                let instance = source_key.as_ref().filter(|key| normalize_map(key.map_name()) == name).map_or(0, MapInstanceKey::map_instance);
                Some((MapInstanceKey::new(name, instance), event.x, event.y))
            }
        };
        if let Some((key, _, _)) = &common { self.ensure_party_warp_instance(state, key)?; }
        let recipients: Vec<_> = state.characters().values().filter(|character| character.game_systems.party_id == party.id
            && party.members.contains(&character.char_id) && character.status.hp > 0 && !character.is_dead()
            && event.source_map.as_ref().is_none_or(|map| normalize_map(character.current_map_name()) == *map
                && source_key.as_ref().filter(|key| normalize_map(key.map_name()) == *map).is_none_or(|key| character.map_instance_key == *key)))
            .map(|character| (character.char_id, character.map_instance_key.clone(), normalize_map(&character.save_map), character.save_x, character.save_y))
            .collect();
        let mut warps = Vec::new();
        for (char_id, origin, save_map, save_x, save_y) in recipients {
            let flags = state.map_flags(&origin);
            let (key, center, range) = match event.map.as_str() {
                "Random" => {
                    if flags.enabled(MapFlag::NoWarp) || flags.enabled(MapFlag::NoTeleport) { continue; }
                    (origin, None, (0, 0))
                }
                "SavePointAll" => {
                    if flags.enabled(MapFlag::NoReturn) { continue; }
                    (MapInstanceKey::new(save_map, 0), Some((save_x, save_y)), (0, 0))
                }
                "SavePoint" => {
                    if flags.enabled(MapFlag::NoReturn) { continue; }
                    let (key, x, y) = common.as_ref().unwrap();
                    (key.clone(), Some((*x, *y)), (0, 0))
                }
                "Leader" if char_id == party.leader_char_id => continue,
                _ => {
                    let (key, x, y) = common.as_ref().unwrap();
                    if event.map != "RandomAll" || char_id != event.char_id {
                        if flags.enabled(MapFlag::NoReturn) || flags.enabled(MapFlag::NoWarp) { continue; }
                    }
                    let range = if event.map == "RandomAll" && char_id == event.char_id { (0, 0) } else { (event.range_x, event.range_y) };
                    (key.clone(), Some((*x, *y)), range)
                }
            };
            self.ensure_party_warp_instance(state, &key)?;
            let (x, y) = destination_cell(state, &key, center, range)?;
            warps.push(ScriptWarp { char_id, map: normalize_map(key.map_name()), x, y, destination_instance: Some(key.map_instance()) });
        }
        Ok(warps)
    }

    fn ensure_party_warp_instance(&self, state: &mut ServerState, key: &MapInstanceKey) -> Result<(), String> {
        if state.get_map_instance(key.map_name(), key.map_instance()).is_none() {
            let map = GlobalConfigService::instance().find_map(&normalize_map(key.map_name())).ok_or("Party warp destination map is unavailable")?;
            self.server_service().create_map_instance(state, map, key.map_instance());
        }
        Ok(())
    }
}
