use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, ScriptSkillCast, ScriptWarp};
use models::enums::cell::CellType;

use crate::server::model::events::map_event::{CellArea, MapEvent, ScriptMapCommand, ScriptMobCommand};
use crate::server::script::ScriptRequest;
use crate::server::script::item_script_handler::ItemEffect;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::npc_event_service::named_npc;
use crate::server::service::script_service::ScriptService;
use crate::server::state::server::ServerState;

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::BgMonster
            | Function::BgMonsterSetTeam
            | Function::AreaMobUseSkill
            | Function::KillMonster
            | Function::MobCount
            | Function::SetMobImmunity
            | Function::MapWarp
            | Function::AreaWarp
            | Function::AreaPercentHeal
            | Function::PercentHeal
            | Function::EnableNpc
            | Function::DisableNpc
            | Function::SetCell
            | Function::MapRespawnGuildId
    )
}

/// rathena's `guild_maprespawn_clones: no`: `maprespawnguildid` with flag 4 keeps clones standing.
const RESPAWN_REMOVES_CLONES: bool = false;

struct Area {
    map: String,
    instance: Option<u8>,
    x: (i32, i32),
    y: (i32, i32),
}

impl Area {
    fn contains(&self, map: &str, instance: u8, x: u16, y: u16) -> bool {
        normalize_map(map) == self.map
            && self.instance.is_none_or(|expected| expected == instance)
            && (self.x.0..=self.x.1).contains(&i32::from(x))
            && (self.y.0..=self.y.1).contains(&i32::from(y))
    }
}

impl Server {
    pub(crate) fn script_npc_effect(&self, state: &ServerState, context: &ScriptRequest, arguments: &[Value]) -> Reply {
        use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
        use packets::packets::{Packet, PacketZcNotifyEffect};
        let effect = arguments.first().ok_or("Missing effect")?.number_value()?;
        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC source is unavailable")?;
        let mut packet = PacketZcNotifyEffect::new(GlobalConfigService::instance().packetver());
        packet.set_aid(npc.id);
        packet.set_effect_id(effect);
        packet.fill_raw();
        let notification = AreaNotification::new(
            npc.map.clone(),
            npc.instance,
            AreaNotificationRangeType::Fov { x: npc.x, y: npc.y, exclude_id: None },
            packet.raw,
        );
        let _ = self.server_service().notification_sender().try_send(Notification::Area(notification));
        Ok(Value::default())
    }

    pub(crate) fn script_map_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let text = |index: usize| -> Result<String, String> { Ok(arguments.get(index).ok_or("Missing argument")?.string_value()?.clone()) };
        let coordinate = |index: usize| -> Result<u16, String> { u16::try_from(number(index)?).map_err(|_| "Invalid coordinate".to_string()) };
        match function {
            Function::BgMonster => {
                let bg_id = u32::try_from(number(0)?).map_err(|_| "Invalid battleground")?;
                let spawn_arguments = [
                    arguments.get(1).cloned().ok_or("Missing map")?,
                    arguments.get(2).cloned().ok_or("Missing x")?,
                    arguments.get(3).cloned().ok_or("Missing y")?,
                    arguments.get(4).cloned().ok_or("Missing name")?,
                    arguments.get(5).cloned().ok_or("Missing class")?,
                    Value::Number(1),
                    arguments.get(6).cloned().unwrap_or_default(),
                ];
                let mut request = self.item_service().spawn_request(&spawn_arguments, 0)?;
                request.bg_id = bg_id;
                let id = self.spawn_script_monster(state, context, &text(1)?, request)?;
                Ok(Value::Number(id.map_or(0, |id| id as i32)))
            }
            Function::BgMonsterSetTeam => {
                let mob_id = u32::try_from(number(0)?).map_err(|_| "Invalid monster")?;
                let bg_id = u32::try_from(number(1)?).map_err(|_| "Invalid battleground")?;
                let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC source is unavailable")?;
                if let Some(map_instance) = state.get_map_instance(&npc.map, npc.instance) {
                    map_instance.add_to_next_tick(MapEvent::ScriptMobCommand(ScriptMobCommand::SetTeam { mob_id, bg_id }));
                }
                Ok(Value::default())
            }
            Function::AreaMobUseSkill => {
                let (map, resolved) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let (x, y, range) = (number(1)?, number(2)?, number(3)?);
                let class = number(4)?;
                let skill = GlobalConfigService::instance().find_skill_config(arguments.get(5).ok_or("Missing skill")?).ok_or("Unknown monster skill")?;
                let level = u16::try_from(number(6)?).ok().filter(|level| *level > 0).ok_or("Invalid monster skill level")?;
                let cancelable = number(8)? > 0;
                let target_type = number(10)?;
                let npc = crate::server::script::unit_data::script_actor(state, context)?;
                let instance = resolved.unwrap_or_else(|| npc.filter(|npc| normalize_map(&npc.map) == map).map_or(0, |npc| npc.instance));
                let Some(map_instance) = state.get_map_instance(&map, instance) else {
                    return Ok(Value::default());
                };
                let casters: Vec<(u32, Option<u32>)> = map_instance
                    .state()
                    .mobs()
                    .values()
                    .filter(|mob| {
                        mob.is_present()
                            && i32::from(mob.mob_id) == class
                            && (x - range..=x + range).contains(&i32::from(mob.x))
                            && (y - range..=y + range).contains(&i32::from(mob.y))
                    })
                    .map(|mob| {
                        let target = match target_type {
                            0 => Some(mob.id),
                            1 => mob.target_id,
                            2 => mob.summon_owner,
                            _ => None,
                        };
                        (mob.id, target)
                    })
                    .collect();
                let random_targets: Vec<u32> = if target_type == 3 {
                    state.characters().values().filter(|character| character.current_map_name() == &map && character.current_map_instance() == instance).map(|character| character.char_id).collect()
                } else {
                    vec![]
                };
                for (mob_id, target) in casters {
                    let target = if target_type == 3 { random_targets.get(fastrand::usize(0..random_targets.len().max(1))).copied() } else { target };
                    let Some(target_id) = target else { continue };
                    let request = ScriptSkillCast {
                        source_id: mob_id,
                        target_id,
                        skill_id: skill.id,
                        level,
                        cast_cancel: Some(cancelable),
                        source_map: Some(map.clone()),
                        source_instance: Some(instance),
                        ..Default::default()
                    };
                    if let Err(error) = self.script_skill_service().cast_script_unit_skill(self, state, request, crate::util::tick::get_tick()) {
                        warn!("areamobuseskill: monster {mob_id} could not use skill {}: {error}", skill.id);
                    }
                }
                Ok(Value::default())
            }
            Function::KillMonster | Function::MobCount | Function::SetMobImmunity => {
                let (map, resolved) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let label = text(1)?;
                let entry = if label.eq_ignore_ascii_case("all") {
                    None
                } else {
                    Some(ScriptService::event_entry(&label).ok_or("Monster event is not a compiled event")?)
                };
                let npc = crate::server::script::unit_data::script_actor(state, context)?;
                let instance = resolved.unwrap_or_else(|| npc.filter(|npc| normalize_map(&npc.map) == map).map_or(0, |npc| npc.instance));
                let Some(map_instance) = state.get_map_instance(&map, instance) else {
                    return Ok(Value::Number(0));
                };
                match function {
                    Function::KillMonster => {
                        map_instance.add_to_next_tick(MapEvent::ScriptMobCommand(ScriptMobCommand::Kill { event_entry: entry }));
                        Ok(Value::default())
                    }
                    Function::SetMobImmunity => {
                        let entry = entry.ok_or("Damage immunity needs a monster event")?;
                        let immune = number(2)? != 0;
                        map_instance.add_to_next_tick(MapEvent::ScriptMobCommand(ScriptMobCommand::SetDamageImmunity { event_entry: entry, immune }));
                        Ok(Value::default())
                    }
                    _ => {
                        let count = map_instance
                            .state()
                            .mobs()
                            .values()
                            .filter(|mob| mob.is_present() && entry.is_none_or(|entry| mob.event_entry == Some(entry)))
                            .count();
                        Ok(Value::Number(count as i32))
                    }
                }
            }
            Function::EnableNpc | Function::DisableNpc => {
                let name = text(0)?;
                let (actor, script) = if name.is_empty() {
                    let actor = crate::server::script::unit_data::script_actor(state, context)?.ok_or("NPC source is unavailable")?;
                    let script = state.get_map_instance(&actor.map, actor.instance).and_then(|instance| instance.get_script(actor.id)).ok_or("NPC is unavailable")?;
                    (actor, script)
                } else {
                    named_npc(state, context, &name)?.ok_or("NPC is not found")?
                };
                let map_instance = state.get_map_instance(&actor.map, actor.instance).ok_or("NPC map is unavailable")?;
                map_instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::NpcVisibility {
                    npc_id: script.id,
                    visible: function == Function::EnableNpc,
                }));
                Ok(Value::default())
            }
            Function::SetCell => {
                let (map, resolved) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let cell = match number(5)? {
                    0 => CellType::Walkable,
                    1 => CellType::Shootable,
                    2 => CellType::Water,
                    4 => CellType::Basilica,
                    5 => CellType::LandProtector,
                    6 => CellType::NoVending,
                    7 => CellType::NoChat,
                    9 => CellType::Icewall,
                    _ => return Err("Unsupported cell type".into()),
                };
                let area = CellArea { x1: coordinate(1)?, y1: coordinate(2)?, x2: coordinate(3)?, y2: coordinate(4)? };
                let npc = crate::server::script::unit_data::script_actor(state, context)?;
                let instance = resolved.unwrap_or_else(|| npc.filter(|npc| normalize_map(&npc.map) == map).map_or(0, |npc| npc.instance));
                let map_instance = state.get_map_instance(&map, instance).ok_or("Cell map is unavailable")?;
                map_instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::SetCell { area, cell, enabled: number(6)? != 0 }));
                Ok(Value::default())
            }
            Function::MapRespawnGuildId => {
                let (map, resolved) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let guild_id = i64::from(number(1)?);
                let flags = number(2)?;
                let instance = resolved.unwrap_or(0);
                let warps: Vec<ScriptWarp> = state
                    .characters()
                    .values()
                    .filter(|character| normalize_map(character.current_map_name()) == map && character.current_map_instance() == instance)
                    .filter(|character| {
                        let member = i64::from(character.game_systems.guild_id) == guild_id;
                        (member && flags & 1 != 0) || (!member && flags & 2 != 0) || (character.game_systems.guild_id == 0 && flags & 2 != 0)
                    })
                    .map(|character| ScriptWarp {
                        char_id: character.char_id,
                        map: normalize_map(&character.save_map),
                        x: character.save_x,
                        y: character.save_y,
                        destination_instance: None,
                    })
                    .collect();
                for warp in warps {
                    self.add_to_next_tick(GameEvent::ScriptWarp(warp));
                }
                if flags & 4 != 0 {
                    if let Some(map_instance) = state.get_map_instance(&map, instance) {
                        map_instance.add_to_next_tick(MapEvent::ScriptMobCommand(ScriptMobCommand::RemoveRespawnable { remove_clones: RESPAWN_REMOVES_CLONES }));
                    }
                }
                Ok(Value::default())
            }
            // rathena kills the player at -100; there is no script-caused death yet, so a full loss leaves 1 HP
            Function::PercentHeal => {
                if context.char_id == 0 {
                    return Err("percentheal requires an attached player".into());
                }
                let effect = ItemEffect::Heal { hp: number(0)?, sp: number(1)?, percentage: true, item_scaling: false, item_id: 0 };
                state
                    .with_character_taken(context.char_id, |state, character| {
                        self.item_service().apply_effects(self, state, self.runtime(), character, vec![effect])?;
                        if character.status.hp == 0 {
                            self.character_service().update_hp_sp(character, 1, character.status.sp);
                        }
                        Ok(Value::default())
                    })
                    .ok_or("percentheal target character disconnected")?
            }
            Function::MapWarp | Function::AreaWarp | Function::AreaPercentHeal => {
                let (source_map, source_instance) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let area = if function == Function::MapWarp {
                    Area { map: source_map, instance: source_instance, x: (0, i32::from(u16::MAX)), y: (0, i32::from(u16::MAX)) }
                } else {
                    Area { map: source_map, instance: source_instance, x: (number(1)?, number(3)?), y: (number(2)?, number(4)?) }
                };
                let members: Vec<u32> = state
                    .characters()
                    .values()
                    .filter(|character| area.contains(character.current_map_name(), character.current_map_instance(), character.x(), character.y()))
                    .map(|character| character.char_id)
                    .collect();
                if function == Function::AreaPercentHeal {
                    let effect = ItemEffect::Heal { hp: number(5)?, sp: number(6)?, percentage: true, item_scaling: false, item_id: 0 };
                    for char_id in members {
                        let Some(mut character) = state.characters_mut().remove(&char_id) else { continue };
                        if let Err(error) = self.item_service().apply_effects(self, state, self.runtime(), &mut character, vec![effect.clone()]) {
                            warn!("Area heal failed for {char_id}: {error}");
                        }
                        state.insert_character(character);
                    }
                    return Ok(Value::default());
                }
                let (destination, x, y) = if function == Function::MapWarp { (1, 2, 3) } else { (5, 6, 7) };
                let (map, destination_instance) = self.resolve_script_map(context.npc_scope_instance, &text(destination)?);
                if GlobalConfigService::instance().find_map(&map).is_none() {
                    return Err("Warp destination map is unavailable".into());
                }
                let (x, y) = (coordinate(x)?, coordinate(y)?);
                for char_id in members {
                    self.add_to_next_tick(GameEvent::ScriptWarp(ScriptWarp { char_id, map: map.clone(), x, y, destination_instance }));
                }
                Ok(Value::default())
            }
            _ => Err("Unsupported map command".into()),
        }
    }
}
