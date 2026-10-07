//! Script functions that NPC dialogue scripts converted from rathena rely on: NPC effects, sounds, map-wide
//! announcements, counters and weight checks.

use script_sdk::{Function, Reply, Value};

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::script::Script;
use crate::server::script::ScriptRequest;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::state::server::ServerState;
use crate::server::Server;

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::Emotion
            | Function::MapAnnounce
            | Function::SoundEffect
            | Function::SoundEffectAll
            | Function::ViewPoint
            | Function::NpcSpecialEffect
            | Function::CheckWeight
            | Function::StrNpcInfo
            | Function::GetMapUsers
            | Function::GetAreaUsers
            | Function::GetTimeTick
            | Function::GetTimeStr
            | Function::GetMapXy
            | Function::GetPartyMember
            | Function::IsPartyLeader
    )
}

const EMOTION_PACKET: u16 = 0x00c0;
const SOUND_PACKET: u16 = 0x01d3;
const COMPASS_PACKET: u16 = 0x0144;
const SOUND_NAME_BYTES: usize = 24;

const EMOTIONS: [&str; 62] = [
    "SURPRISE", "QUESTION", "DELIGHT", "THROB", "SWEAT", "AHA", "FRET", "ANGER", "MONEY", "THINK", "SCISSOR", "ROCK", "WRAP", "FLAG", "BIGTHROB", "THANKS", "KEK",
    "SORRY", "SMILE", "PROFUSELY_SWEAT", "SCRATCH", "BEST", "STARE_ABOUT", "HUK", "O", "X", "HELP", "GO", "CRY", "KIK", "CHUP", "CHUPCHUP", "HNG", "OK",
    "CHAT_PROHIBIT", "INDONESIA_FLAG", "STARE", "HUNGRY", "COOL", "MERONG", "SHY", "GOODBOY", "SPTIME", "SEXY", "COMEON", "SLEEPY", "CONGRATULATION", "HPTIME", "PH_FLAG", "MY_FLAG", "SI_FLAG", "BR_FLAG", "SPARK", "CONFUSE", "OHNO", "HUM", "BLABLA", "OTL", "DICE1", "DICE2", "DICE3", "DICE4",
];

const HIGH_JOBS: [&str; 22] = [
    "NOVICE_HIGH", "SWORDMAN_HIGH", "MAGE_HIGH", "ARCHER_HIGH", "ACOLYTE_HIGH", "MERCHANT_HIGH", "THIEF_HIGH", "LORD_KNIGHT", "HIGH_PRIEST", "HIGH_WIZARD", "WHITESMITH",
    "SNIPER", "ASSASSIN_CROSS", "LORD_KNIGHT2", "PALADIN", "CHAMPION", "PROFESSOR", "STALKER", "CREATOR", "CLOWN", "GYPSY", "PALADIN2",
];

const CELLS: [&str; 11] = ["WALKABLE", "SHOOTABLE", "WATER", "NPC", "BASILICA", "LANDPROTECTOR", "NOVENDING", "NOCHAT", "MAELSTROM", "ICEWALL", "NOBUYINGSTORE"];

/// Constants rathena scripts spell in upper case (`ET_*`, `CELL_*`, `JOB_*`, `BC_*`) that the host tables only know in another spelling.
pub(crate) fn rathena_constant(name: &str) -> Option<Value> {
    if let Some(emotion) = name.strip_prefix("ET_") {
        return EMOTIONS.iter().position(|known| *known == emotion).map(|index| Value::Number(index as i32));
    }
    if let Some(cell) = name.strip_prefix("CELL_") {
        return CELLS.iter().position(|known| *known == cell).map(|index| Value::Number(index as i32));
    }
    if name == "EAJL_THIRD" {
        return Some(Value::Number(0x4000));
    }
    if let Some(job) = name.strip_prefix("JOB_") {
        match job {
            "BABY" => return Some(Value::Number(4023)),
            "SUPER_NOVICE" => return Some(Value::Number(23)),
            "SUMMONER" => return Some(Value::Number(4218)),
            "TAEKWON" => return Some(Value::Number(4046)),
            "STAR_GLADIATOR" => return Some(Value::Number(4047)),
            "STAR_GLADIATOR2" => return Some(Value::Number(4048)),
            "SOUL_LINKER" => return Some(Value::Number(4049)),
            _ => {}
        }
        if let Some(index) = HIGH_JOBS.iter().position(|known| *known == job) {
            return Some(Value::Number(4001 + index as i32));
        }
        let mut letters = job.chars();
        let title: String = letters.next()?.to_ascii_uppercase().to_string() + &letters.as_str().to_ascii_lowercase();
        return crate::server::script::constant::load_constant(&format!("Job_{title}"));
    }
    if name.starts_with("BC_") {
        return crate::server::service::script_presentation_service::presentation_constant(&name.to_ascii_lowercase());
    }
    None
}

fn emotion_packet(actor_id: u32, emotion: u8) -> Vec<u8> {
    let mut packet = EMOTION_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&actor_id.to_le_bytes());
    packet.push(emotion);
    packet
}

fn sound_packet(name: &str, kind: u8, source_id: u32) -> Result<Vec<u8>, String> {
    if name.len() >= SOUND_NAME_BYTES {
        return Err("Sound file name is too long".into());
    }
    let mut packet = SOUND_PACKET.to_le_bytes().to_vec();
    let mut padded = [0u8; SOUND_NAME_BYTES];
    padded[..name.len()].copy_from_slice(name.as_bytes());
    packet.extend_from_slice(&padded);
    packet.push(kind);
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet.extend_from_slice(&source_id.to_le_bytes());
    Ok(packet)
}

fn compass_packet(npc_id: u32, action: u32, x: u32, y: u32, number: u8, color: u32) -> Vec<u8> {
    let mut packet = COMPASS_PACKET.to_le_bytes().to_vec();
    for value in [npc_id, action, x, y] {
        packet.extend_from_slice(&value.to_le_bytes());
    }
    packet.push(number);
    packet.extend_from_slice(&color.to_le_bytes());
    packet
}

/// Name parts of an NPC as `strnpcinfo` reports them.
fn npc_info(script: &Script, kind: i32) -> Value {
    let (visible, hidden) = script.name.split_once('#').unwrap_or((script.name.as_str(), ""));
    match kind {
        0 => script.name.clone().into(),
        1 => visible.into(),
        2 => hidden.into(),
        3 => script.name.clone().into(),
        4 => normalize_map(&script.map_name).into(),
        _ => Value::String(String::new()),
    }
}

fn calling_script(state: &ServerState, context: &ScriptRequest) -> Option<std::sync::Arc<Script>> {
    state.map_instances().values().flatten().find_map(|instance| {
        let map = instance.state();
        map.script_skill_state
            .npcs
            .get(&context.npc_id)
            .filter(|npc| npc.script.scope_instance == context.npc_scope_instance && npc.script.entry_id == context.npc_entry)
            .map(|npc| npc.script.clone())
    })
}

impl Server {
    pub(crate) fn script_npc_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let text = |index: usize| -> Result<String, String> { Ok(arguments.get(index).ok_or("Missing argument")?.text()) };
        match function {
            Function::NpcSpecialEffect => self.script_npc_effect(state, context, arguments),
            Function::StrNpcInfo => {
                let script = calling_script(state, context).ok_or("strnpcinfo needs an NPC")?;
                Ok(npc_info(&script, number(0)?))
            }
            Function::Emotion => {
                let emotion = u8::try_from(number(0)?).map_err(|_| "Invalid emotion")?;
                let target = arguments.get(1).map(Value::number_value).transpose()?.filter(|id| *id != 0);
                let (actor_id, map, instance, x, y) = match target {
                    Some(id) => {
                        let character = state.characters().values().find(|character| character.account_id == id as u32 || character.char_id == id as u32)
                            .ok_or("Emotion target is unavailable")?;
                        (character.char_id, character.current_map_name().clone(), character.current_map_instance(), character.x(), character.y())
                    }
                    None => {
                        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("Emotion needs an NPC or a target")?;
                        (npc.id, npc.map.clone(), npc.instance, npc.x, npc.y)
                    }
                };
                let range = AreaNotificationRangeType::Fov { x, y, exclude_id: None };
                let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(map, instance, range, emotion_packet(actor_id, emotion))));
                Ok(Value::default())
            }
            Function::SoundEffect => {
                if context.char_id == 0 {
                    return Ok(Value::default());
                }
                let packet = sound_packet(&text(0)?, u8::try_from(number(1)?).map_err(|_| "Invalid sound type")?, context.npc_id)?;
                let _ = self.server_service().notification_sender().try_send(Notification::Char(CharNotification::new(context.char_id, packet)));
                Ok(Value::default())
            }
            Function::SoundEffectAll => {
                let packet = sound_packet(&text(0)?, u8::try_from(number(1)?).map_err(|_| "Invalid sound type")?, context.npc_id)?;
                if let Some(map) = arguments.get(2).map(Value::text) {
                    let map = normalize_map(&map);
                    let area = if arguments.len() >= 7 { Some((number(3)?, number(4)?, number(5)?, number(6)?)) } else { None };
                    let recipients: Vec<u32> = state.characters().values()
                        .filter(|character| normalize_map(character.current_map_name()) == map)
                        .filter(|character| area.is_none_or(|(x0, y0, x1, y1)| (x0..=x1).contains(&i32::from(character.x())) && (y0..=y1).contains(&i32::from(character.y()))))
                        .map(|character| character.char_id).collect();
                    self.map_notifications.extend(recipients.into_iter().map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))));
                    self.drain_map_notifications();
                } else if let Some(npc) = crate::server::script::unit_data::script_actor(state, context)? {
                    let range = AreaNotificationRangeType::Fov { x: npc.x, y: npc.y, exclude_id: None };
                    let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(npc.map.clone(), npc.instance, range, packet)));
                } else if context.char_id != 0 {
                    let _ = self.server_service().notification_sender().try_send(Notification::Char(CharNotification::new(context.char_id, packet)));
                }
                Ok(Value::default())
            }
            Function::ViewPoint => {
                if context.char_id == 0 {
                    return Err("viewpoint needs an attached player".into());
                }
                let packet = compass_packet(
                    context.npc_id,
                    u32::try_from(number(0)?).map_err(|_| "Invalid viewpoint action")?,
                    u32::try_from(number(1)?).map_err(|_| "Invalid viewpoint x")?,
                    u32::try_from(number(2)?).map_err(|_| "Invalid viewpoint y")?,
                    u8::try_from(number(3)?).map_err(|_| "Invalid viewpoint number")?,
                    number(4)? as u32,
                );
                let _ = self.server_service().notification_sender().try_send(Notification::Char(CharNotification::new(context.char_id, packet)));
                Ok(Value::default())
            }
            Function::MapAnnounce => {
                let map = text(0)?;
                let map = if map.eq_ignore_ascii_case("this") {
                    crate::server::script::unit_data::script_actor(state, context)?.map(|npc| normalize_map(&npc.map)).ok_or("mapannounce \"this\" needs an NPC")?
                } else {
                    normalize_map(&map)
                };
                let mut announcement = vec![arguments.get(1).cloned().ok_or("Missing announcement")?, arguments.get(2).cloned().ok_or("Missing announcement flags")?];
                announcement.extend(arguments.iter().skip(3).cloned());
                let packet = super::script_presentation_service::announcement_packet(&announcement)?;
                let recipients: Vec<u32> = state.characters().values().filter(|character| normalize_map(character.current_map_name()) == map).map(|character| character.char_id).collect();
                self.map_notifications.extend(recipients.into_iter().map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))));
                self.drain_map_notifications();
                Ok(Value::default())
            }
            Function::GetMapUsers => {
                let map = normalize_map(&text(0)?);
                Ok(Value::Number(state.characters().values().filter(|character| normalize_map(character.current_map_name()) == map).count() as i32))
            }
            Function::GetAreaUsers => {
                let map = normalize_map(&text(0)?);
                let (x0, y0, x1, y1) = (number(1)?, number(2)?, number(3)?, number(4)?);
                Ok(Value::Number(state.characters().values().filter(|character| {
                    normalize_map(character.current_map_name()) == map
                        && (x0..=x1).contains(&i32::from(character.x()))
                        && (y0..=y1).contains(&i32::from(character.y()))
                }).count() as i32))
            }
            Function::GetTimeTick => Ok(Value::Number(match number(0)? {
                0 => crate::util::tick::get_tick() as u32 as i32,
                1 => {
                    use chrono::Timelike;
                    chrono::Local::now().num_seconds_from_midnight() as i32
                }
                _ => chrono::Utc::now().timestamp() as i32,
            })),
            Function::GetTimeStr => {
                let format = text(0)?;
                let limit = usize::try_from(number(1)?).map_err(|_| "Invalid time string size")?;
                let mut formatted = String::new();
                std::fmt::write(&mut formatted, format_args!("{}", chrono::Local::now().format(&format))).map_err(|_| "Invalid time format")?;
                formatted.truncate(limit.saturating_sub(1));
                Ok(Value::String(formatted))
            }
            Function::GetMapXy => {
                const BL_PC: i32 = 1;
                const BL_NPC: i32 = 128;
                let (map, x, y) = match number(0)? {
                    BL_PC => {
                        let character = state.characters().get(&context.char_id).ok_or("getmapxy needs an attached player")?;
                        (normalize_map(character.current_map_name()), character.x(), character.y())
                    }
                    BL_NPC => {
                        let npc = match arguments.get(1) {
                            Some(name) => crate::server::service::npc_event_service::named_npc(state, context, &name.text())?.map(|(actor, _)| actor),
                            None => crate::server::script::unit_data::script_actor(state, context)?,
                        };
                        match npc {
                            Some(npc) => (normalize_map(&npc.map), npc.x, npc.y),
                            None => return Ok(Value::Array(vec![Value::String(String::new()), (-1).into(), (-1).into()])),
                        }
                    }
                    _ => return Err("getmapxy supports players and NPCs only".into()),
                };
                Ok(Value::Array(vec![map.into(), i32::from(x).into(), i32::from(y).into()]))
            }
            Function::GetPartyMember => {
                let party_id = u32::try_from(number(0)?).map_err(|_| "Invalid party")?;
                let kind = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(0);
                Ok(Value::Array(state.characters().values().filter(|character| party_id != 0 && character.game_systems.party_id == party_id).map(|character| match kind {
                    1 => Value::Number(character.char_id as i32),
                    2 => Value::Number(character.account_id as i32),
                    _ => Value::String(character.name.clone()),
                }).collect()))
            }
            Function::IsPartyLeader => {
                let character = state.characters().get(&context.char_id).ok_or("is_party_leader needs an attached player")?;
                let party_id = u32::try_from(number(0)?).unwrap_or(0);
                let leader = party_id != 0 && character.game_systems.party_id == party_id
                    && character.game_systems.party.as_ref().is_some_and(|party| party.id == party_id && party.leader_char_id == character.char_id);
                Ok(Value::Number(i32::from(leader)))
            }
            Function::CheckWeight => {
                if arguments.is_empty() || arguments.len() % 2 != 0 {
                    return Err("checkweight needs item and amount pairs".into());
                }
                let configuration = GlobalConfigService::instance();
                let mut added = 0u64;
                for pair in arguments.chunks(2) {
                    let item = crate::server::script::utilities::find_item(configuration, &pair[0]).ok_or("Unknown item")?;
                    let amount = pair[1].number_value()?;
                    if amount <= 0 {
                        return Ok(Value::Number(0));
                    }
                    added += item.weight.max(0) as u64 * amount as u64;
                }
                let character = state.characters().get(&context.char_id).ok_or("checkweight needs an attached player")?;
                let limit = u64::from(self.character_service().max_weight(character));
                Ok(Value::Number(i32::from(u64::from(character.weight()) + added <= limit)))
            }
            _ => Err(format!("{function:?} is not an NPC command")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packets_have_the_client_layout() {
        assert_eq!(emotion_packet(5, 1), vec![0xc0, 0x00, 5, 0, 0, 0, 1]);
        let sound = sound_packet("a.wav", 0, 9).unwrap();
        assert_eq!(sound.len(), 35);
        assert_eq!(&sound[2..7], b"a.wav");
        assert_eq!(&sound[31..35], &9u32.to_le_bytes());
        assert!(sound_packet(&"x".repeat(24), 0, 0).is_err());
        assert_eq!(compass_packet(1, 2, 3, 4, 5, 6).len(), 23);
    }
}
