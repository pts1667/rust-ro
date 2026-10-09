//! Script functions that NPC dialogue scripts converted from rathena rely on: NPC effects, sounds, map-wide
//! announcements, counters and weight checks.

use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU64, EnumWithNumberValue};
use packets::packets::{Packet, PacketZcCongratulation, PacketZcPlayNpcBgm};
use models::enums::item::{EquipmentLocation, ItemType};
use models::enums::look::LookType;
use script_sdk::{Function, Reply, Value};

use crate::server::model::events::map_event::{MapEvent, ScriptMapCommand};
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
            | Function::ConvertPcInfo
            | Function::Nude
            | Function::SetNpcDisplay
            | Function::ConsumeItem
            | Function::MakeItem
            | Function::ReadBook
            | Function::GetGmLevel
            | Function::OpenMail
            | Function::OpenAuction
            | Function::CountItem2
            | Function::DelItem2
            | Function::GetInventoryList
            | Function::PushPc
            | Function::AreaAnnounce
            | Function::MercenaryGetFaith
            | Function::UnloadNpc
            | Function::MoveNpc
            | Function::GetMonsterInfo
            | Function::NpcTalk
            | Function::WaitingRoom
            | Function::DelWaitingRoom
            | Function::WaitingRoomKick
            | Function::WaitingRoomKickAll
            | Function::EnableWaitingRoomEvent
            | Function::DisableWaitingRoomEvent
            | Function::GetWaitingRoomState
            | Function::WarpWaitingPc
            | Function::PlayBgm
            | Function::Wedding
            | Function::RequestGuildInfo
            | Function::SetItemScript
    )
}

const EMOTION_PACKET: u16 = 0x00c0;
const SOUND_PACKET: u16 = 0x01d3;
const COMPASS_PACKET: u16 = 0x0144;
const READ_BOOK_PACKET: u16 = 0x0294;
const NPC_SPRITE_PACKET: u16 = 0x01b0;
const NOTIFY_CHAT_PACKET: u16 = 0x008d;
const BGM_NAME_BYTES: usize = 24;
const SOUND_NAME_BYTES: usize = 24;
/// What rathena answers `openauction` with while `feature.auction` is off; there is no auction house here.
const AUCTION_DISABLED: &str = "Auction System is disabled.";

const EMOTIONS: [&str; 64] = [
    "SURPRISE", "QUESTION", "DELIGHT", "THROB", "SWEAT", "AHA", "FRET", "ANGER", "MONEY", "THINK", "SCISSOR", "ROCK", "WRAP", "FLAG", "BIGTHROB", "THANKS", "KEK",
    "SORRY", "SMILE", "PROFUSELY_SWEAT", "SCRATCH", "BEST", "STARE_ABOUT", "HUK", "O", "X", "HELP", "GO", "CRY", "KIK", "CHUP", "CHUPCHUP", "HNG", "OK",
    "CHAT_PROHIBIT", "INDONESIA_FLAG", "STARE", "HUNGRY", "COOL", "MERONG", "SHY", "GOODBOY", "SPTIME", "SEXY", "COMEON", "SLEEPY", "CONGRATULATION", "HPTIME", "PH_FLAG", "MY_FLAG", "SI_FLAG", "BR_FLAG", "SPARK", "CONFUSE", "OHNO", "HUM", "BLABLA", "OTL", "DICE1", "DICE2", "DICE3", "DICE4", "DICE5", "DICE6",
];

const HIGH_JOBS: [&str; 22] = [
    "NOVICE_HIGH", "SWORDMAN_HIGH", "MAGE_HIGH", "ARCHER_HIGH", "ACOLYTE_HIGH", "MERCHANT_HIGH", "THIEF_HIGH", "LORD_KNIGHT", "HIGH_PRIEST", "HIGH_WIZARD", "WHITESMITH",
    "SNIPER", "ASSASSIN_CROSS", "LORD_KNIGHT2", "PALADIN", "CHAMPION", "PROFESSOR", "STALKER", "CREATOR", "CLOWN", "GYPSY", "PALADIN2",
];

const CELLS: [&str; 11] = ["WALKABLE", "SHOOTABLE", "WATER", "NPC", "BASILICA", "LANDPROTECTOR", "NOVENDING", "NOCHAT", "MAELSTROM", "ICEWALL", "NOBUYINGSTORE"];

/// Classes of later episodes: no player has them before renewal, the ids keep comparisons against them false.
const LATER_JOBS: &[(&str, i32)] = &[
    ("RUNE_KNIGHT", 4054), ("WARLOCK", 4055), ("RANGER", 4056), ("ARCH_BISHOP", 4057),
    ("MECHANIC", 4058), ("GUILLOTINE_CROSS", 4059), ("RUNE_KNIGHT_T", 4060), ("WARLOCK_T", 4061),
    ("RANGER_T", 4062), ("ARCH_BISHOP_T", 4063), ("MECHANIC_T", 4064), ("GUILLOTINE_CROSS_T", 4065),
    ("ROYAL_GUARD", 4066), ("SORCERER", 4067), ("MINSTREL", 4068), ("WANDERER", 4069),
    ("SURA", 4070), ("GENETIC", 4071), ("SHADOW_CHASER", 4072), ("ROYAL_GUARD_T", 4073),
    ("SORCERER_T", 4074), ("MINSTREL_T", 4075), ("WANDERER_T", 4076), ("SURA_T", 4077),
    ("GENETIC_T", 4078), ("SHADOW_CHASER_T", 4079), ("RUNE_KNIGHT2", 4080), ("RUNE_KNIGHT_T2", 4081),
    ("ROYAL_GUARD2", 4082), ("ROYAL_GUARD_T2", 4083), ("RANGER2", 4084), ("RANGER_T2", 4085),
    ("MECHANIC2", 4086), ("MECHANIC_T2", 4087), ("BABY_RUNE_KNIGHT", 4096), ("BABY_WARLOCK", 4097),
    ("BABY_RANGER", 4098), ("BABY_ARCH_BISHOP", 4099), ("BABY_MECHANIC", 4100), ("BABY_GUILLOTINE_CROSS", 4101),
    ("BABY_ROYAL_GUARD", 4102), ("BABY_SORCERER", 4103), ("BABY_MINSTREL", 4104), ("BABY_WANDERER", 4105),
    ("BABY_SURA", 4106), ("BABY_GENETIC", 4107), ("BABY_SHADOW_CHASER", 4108), ("BABY_RUNE_KNIGHT2", 4109),
    ("BABY_ROYAL_GUARD2", 4110), ("BABY_RANGER2", 4111), ("BABY_MECHANIC2", 4112), ("SUPER_NOVICE_E", 4190),
    ("SUPER_BABY_E", 4191), ("KAGEROU", 4211), ("OBORO", 4212), ("REBELLION", 4215),
    ("BABY_SUMMONER", 4220), ("BABY_NINJA", 4222), ("BABY_KAGEROU", 4223), ("BABY_OBORO", 4224),
    ("BABY_TAEKWON", 4225), ("BABY_STAR_GLADIATOR", 4226), ("BABY_SOUL_LINKER", 4227), ("BABY_GUNSLINGER", 4228),
    ("BABY_REBELLION", 4229), ("BABY_STAR_GLADIATOR2", 4238), ("STAR_EMPEROR", 4239), ("SOUL_REAPER", 4240),
    ("BABY_STAR_EMPEROR", 4241), ("BABY_SOUL_REAPER", 4242), ("STAR_EMPEROR2", 4243), ("BABY_STAR_EMPEROR2", 4244),
    ("DRAGON_KNIGHT", 4252), ("MEISTER", 4253), ("SHADOW_CROSS", 4254), ("ARCH_MAGE", 4255),
    ("CARDINAL", 4256), ("WINDHAWK", 4257), ("IMPERIAL_GUARD", 4258), ("BIOLO", 4259),
    ("ABYSS_CHASER", 4260), ("ELEMENTAL_MASTER", 4261), ("INQUISITOR", 4262), ("TROUBADOUR", 4263),
    ("TROUVERE", 4264), ("WINDHAWK2", 4278), ("MEISTER2", 4279), ("DRAGON_KNIGHT2", 4280),
    ("IMPERIAL_GUARD2", 4281), ("SKY_EMPEROR", 4302), ("SOUL_ASCETIC", 4303), ("SHINKIRO", 4304),
    ("SHIRANUI", 4305), ("NIGHT_WATCH", 4306), ("HYPER_NOVICE", 4307), ("SPIRIT_HANDLER", 4308),
    ("SKY_EMPEROR2", 4316), ("SECOND_JOB_START", 4331), ("RUNE_KNIGHT_2ND", 4332), ("MECHANIC_2ND", 4333),
    ("GUILLOTINE_CROSS_2ND", 4334), ("WARLOCK_2ND", 4335), ("ARCHBISHOP_2ND", 4336), ("RANGER_2ND", 4337),
    ("ROYAL_GUARD_2ND", 4338), ("GENETIC_2ND", 4339), ("SHADOW_CHASER_2ND", 4340), ("SORCERER_2ND", 4341),
    ("SURA_2ND", 4342), ("MINSTREL_2ND", 4343), ("WANDERER_2ND", 4344), ("SECOND_JOB_END", 4350),
    ("MAX", 4351),
];

const TARGET_PACKETVER: i32 = 20120307;

fn equipment_location(name: &str) -> Option<EquipmentLocation> {
    Some(match name {
        "HEAD_LOW" => EquipmentLocation::HeadLow,
        "HEAD_MID" => EquipmentLocation::HeadMid,
        "HEAD_TOP" => EquipmentLocation::HeadTop,
        "HAND_R" => EquipmentLocation::HandRight,
        "HAND_L" => EquipmentLocation::HandLeft,
        "ARMOR" => EquipmentLocation::Armor,
        "SHOES" => EquipmentLocation::Shoes,
        "GARMENT" => EquipmentLocation::Garment,
        "ACC_L" => EquipmentLocation::AccessoryLeft,
        "ACC_R" => EquipmentLocation::AccessoryRight,
        "ACC_RL" => EquipmentLocation::Accessory,
        "COSTUME_HEAD_TOP" => EquipmentLocation::CostumeHeadTop,
        "COSTUME_HEAD_MID" => EquipmentLocation::CostumeHeadMid,
        "COSTUME_HEAD_LOW" => EquipmentLocation::CostumeHeadLow,
        "COSTUME_GARMENT" => EquipmentLocation::CostumeGarment,
        "AMMO" => EquipmentLocation::Ammo,
        "SHADOW_ARMOR" => EquipmentLocation::ShadowArmor,
        "SHADOW_WEAPON" => EquipmentLocation::ShadowWeapon,
        "SHADOW_SHIELD" => EquipmentLocation::ShadowShield,
        "SHADOW_SHOES" => EquipmentLocation::ShadowShoes,
        "SHADOW_ACC_R" => EquipmentLocation::ShadowAccR,
        "SHADOW_ACC_L" => EquipmentLocation::ShadowAccL,
        _ => return None,
    })
}

/// Constants rathena scripts spell in upper case (`ET_*`, `CELL_*`, `JOB_*`, `BC_*`) that the host tables only know in another spelling.
pub(crate) fn rathena_constant(name: &str) -> Option<Value> {
    if let Some(emotion) = name.strip_prefix("ET_") {
        return EMOTIONS.iter().position(|known| *known == emotion).map(|index| Value::Number(index as i32));
    }
    if let Some(cell) = name.strip_prefix("CELL_") {
        return CELLS.iter().position(|known| *known == cell).map(|index| Value::Number(index as i32));
    }
    if let Some(location) = name.strip_prefix("EQP_").and_then(equipment_location) {
        return Some(Value::Number(location.as_flag() as i32));
    }
    match name {
        "SKILL_PERM" => return Some(Value::Number(0)),
        "SKILL_TEMP" => return Some(Value::Number(1)),
        "SKILL_TEMPLEVEL" => return Some(Value::Number(2)),
        "SKILL_PERM_GRANT" => return Some(Value::Number(3)),
        "VIP_SCRIPT" => return Some(Value::Number(0)),
        "MAX_LEVEL" => return Some(Value::Number(crate::server::model::waiting_room::MAX_LEVEL as i32)),
        "PACKETVER" => return Some(Value::Number(TARGET_PACKETVER)),
        "VAR_HEAD" => return Some(Value::Number(LookType::Hair.value() as i32)),
        "VAR_HEADPALETTE" => return Some(Value::Number(LookType::HairColor.value() as i32)),
        // rathena does not export it, so the script reads an empty variable: effect 0
        "PF_FOGWALL" => return Some(Value::Number(0)),
        "SPEAR_MERC_GUILD" => return Some(Value::Number(0)),
        "SWORD_MERC_GUILD" => return Some(Value::Number(1)),
        "ARCH_MERC_GUILD" => return Some(Value::Number(2)),
        "CPC_NAME" => return Some(Value::Number(0)),
        "CPC_CHAR" => return Some(Value::Number(1)),
        "CPC_ACCOUNT" => return Some(Value::Number(2)),
        "IM_NONE" | "IE_OK" | "IIT_ID" | "ILI_NAME" | "IWA_NONE" => return Some(Value::Number(0)),
        "IM_CHAR" | "IE_NOMEMBER" | "IIT_TIME_LIMIT" | "ILI_MODE" | "IWA_NOTDEAD" => return Some(Value::Number(1)),
        "IM_PARTY" | "IE_NOINSTANCE" | "IIT_IDLE_TIMEOUT" | "ILI_OWNER" => return Some(Value::Number(2)),
        "IM_GUILD" | "IE_OTHER" | "IIT_ENTER_MAP" => return Some(Value::Number(3)),
        "IM_CLAN" | "IIT_ENTER_X" => return Some(Value::Number(4)),
        "IIT_ENTER_Y" => return Some(Value::Number(5)),
        "IIT_MAPCOUNT" => return Some(Value::Number(6)),
        "IIT_MAP" => return Some(Value::Number(7)),
        "REFINE_COST_NORMAL" | "REFINE_MATERIAL_ID" => return Some(Value::Number(0)),
        "REFINE_COST_HD" | "REFINE_ZENY_COST" => return Some(Value::Number(1)),
        "REFINE_COST_ENRICHED" => return Some(Value::Number(2)),
        "IT_ARMOR" => return Some(Value::Number(ItemType::Armor.value() as i32)),
        "IT_WEAPON" => return Some(Value::Number(ItemType::Weapon.value() as i32)),
        "FW_THIN" => return Some(Value::Number(100)),
        "FW_NORMAL" => return Some(Value::Number(400)),
        "FW_BOLD" => return Some(Value::Number(700)),
        _ => {}
    }
    if name == "EAJL_THIRD" {
        return Some(Value::Number(0x4000));
    }
    if let Some(job) = name.strip_prefix("JOB_") {
        match job {
            "CRUSADER2" => return Some(Value::Number(21)),
            "BABY" => return Some(Value::Number(4023)),
            "BABY_ARCHER" => return Some(Value::Number(4026)),
            "SUPER_BABY" => return Some(Value::Number(4045)),
            "SUPER_NOVICE" => return Some(Value::Number(23)),
            "SUMMONER" => return Some(Value::Number(4218)),
            "TAEKWON" => return Some(Value::Number(4046)),
            "STAR_GLADIATOR" => return Some(Value::Number(4047)),
            "STAR_GLADIATOR2" => return Some(Value::Number(4048)),
            "SOUL_LINKER" => return Some(Value::Number(4049)),
            _ => {}
        }
        if let Some((_, id)) = LATER_JOBS.iter().find(|(known, _)| *known == job) {
            return Some(Value::Number(*id));
        }
        if let Some(index) = HIGH_JOBS.iter().position(|known| *known == job) {
            return Some(Value::Number(4001 + index as i32));
        }
        let mut letters = job.chars();
        let title: String = letters.next()?.to_ascii_uppercase().to_string() + &letters.as_str().to_ascii_lowercase();
        return crate::server::script::constant::load_constant(&format!("Job_{title}"));
    }
    if let Some(kind) = name.strip_prefix("MOB_") {
        return MONSTER_INFO.iter().position(|known| *known == kind).map(|index| Value::Number(index as i32 + 1));
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

/// `ZC_NOTIFY_CHAT`: the text appears over the speaker's head and in the chat window of everyone around.
fn npc_chat_packet(npc_id: u32, text: &str) -> Vec<u8> {
    let mut packet = NOTIFY_CHAT_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&((text.len() + 9) as u16).to_le_bytes());
    packet.extend_from_slice(&npc_id.to_le_bytes());
    packet.extend_from_slice(text.as_bytes());
    packet.push(0);
    packet
}

fn npc_sprite_packet(npc_id: u32, sprite: u32) -> Vec<u8> {
    let mut packet = NPC_SPRITE_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&npc_id.to_le_bytes());
    packet.push(0);
    packet.extend_from_slice(&sprite.to_le_bytes());
    packet
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

const PUSH_DX: [i32; 8] = [0, -1, -1, -1, 0, 1, 1, 1];
const PUSH_DY: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
/// `getmonsterinfo` types in rathena's order, starting from `MOB_NAME` = 1.
const MONSTER_INFO: [&str; 29] = [
    "NAME", "LV", "MAXHP", "MAXSP", "BASEEXP", "JOBEXP", "ATKMIN", "ATKMAX", "DEF", "MDEF", "RES", "MRES", "STR", "AGI", "VIT", "INT", "DEX", "LUK", "SPEED",
    "ATKRANGE", "SKILLRANGE", "CHASERANGE", "SIZE", "RACE", "ELEMENT", "ELEMENTLV", "MODE", "MVPEXP", "ID",
];

impl Server {
    /// `countitem2` and `delitem2`: only stacks with exactly this identification, refine, damage state and cards count.
    fn script_exact_items(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let item = crate::server::script::utilities::find_item(GlobalConfigService::instance(), arguments.first().ok_or("Missing item")?);
        let offset = usize::from(function == Function::DelItem2);
        let (identified, refine, damaged) = (number(1 + offset)? != 0, number(2 + offset)?, number(3 + offset)? != 0);
        let cards = [number(4 + offset)?, number(5 + offset)?, number(6 + offset)?, number(7 + offset)?];
        let character = state.characters_mut().get_mut(&context.char_id).ok_or("This item command needs an attached player")?;
        let matching: Vec<(usize, i16)> = item.map_or(vec![], |item| character.inventory.iter().enumerate().filter_map(|(index, slot)| {
            let entry = slot.as_ref()?;
            let same = entry.item_id == item.id && entry.is_identified == identified && i32::from(entry.refine) == refine && entry.is_damaged == damaged
                && [entry.card0, entry.card1, entry.card2, entry.card3].map(i32::from) == cards;
            same.then_some((index, entry.amount))
        }).collect());
        let owned: i32 = matching.iter().map(|(_, amount)| i32::from(*amount)).sum();
        if function == Function::CountItem2 {
            return Ok(Value::Number(owned));
        }
        let mut remaining = number(1)?;
        if remaining <= 0 || owned < remaining {
            return Err("Not enough items to delete".into());
        }
        let mut items = vec![];
        for (index, amount) in matching {
            let taken = remaining.min(i32::from(amount)) as i16;
            items.push(crate::server::model::events::game_event::CharacterRemoveItem { char_id: context.char_id, index, amount: taken, price: 0 });
            remaining -= i32::from(taken);
            if remaining == 0 {
                break;
            }
        }
        let removal = crate::server::model::events::game_event::CharacterRemoveItems { char_id: context.char_id, sell: false, items, notify_client: true };
        self.inventory_service().remove_item_from_inventory(self.runtime.as_ref(), removal, character)?;
        Ok(Value::default())
    }

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
            Function::PlayBgm => {
                if context.char_id == 0 {
                    return Ok(Value::default());
                }
                let name = text(0)?;
                if name.len() > BGM_NAME_BYTES {
                    return Err("BGM file name is too long".into());
                }
                let mut bgm = [' '; BGM_NAME_BYTES];
                for (slot, byte) in bgm.iter_mut().zip(name.bytes()) {
                    *slot = char::from(byte);
                }
                let mut packet = PacketZcPlayNpcBgm::new(GlobalConfigService::instance().packetver());
                packet.set_bgm(bgm);
                packet.fill_raw();
                let _ = self.server_service().notification_sender().try_send(Notification::Char(CharNotification::new(context.char_id, packet.raw)));
                Ok(Value::default())
            }
            Function::Wedding => {
                let (actor_id, map, instance, x, y) = match state.get_character(context.char_id) {
                    Some(character) => (character.char_id, character.current_map_name().clone(), character.current_map_instance(), character.x(), character.y()),
                    None => {
                        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("Wedding effect needs an NPC or a player")?;
                        (npc.id, npc.map.clone(), npc.instance, npc.x, npc.y)
                    }
                };
                let mut packet = PacketZcCongratulation::new(GlobalConfigService::instance().packetver());
                packet.set_aid(actor_id);
                packet.fill_raw();
                let range = AreaNotificationRangeType::Fov { x, y, exclude_id: None };
                let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(map, instance, range, packet.raw)));
                Ok(Value::default())
            }
            Function::RequestGuildInfo => {
                if arguments.len() > 1 {
                    return Err("requestguildinfo with an event callback is not supported".into());
                }
                Ok(Value::default())
            }
            Function::SetItemScript => Err("setitemscript is not supported: item bonus scripts are compiled into the item modules".into()),
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
                let (map, instance) = if map.eq_ignore_ascii_case("this") {
                    crate::server::script::unit_data::script_actor(state, context)?.map(|npc| (normalize_map(&npc.map), Some(npc.instance))).ok_or("mapannounce \"this\" needs an NPC")?
                } else {
                    self.resolve_script_map(context.npc_scope_instance, &map)
                };
                let mut announcement = vec![arguments.get(1).cloned().ok_or("Missing announcement")?, arguments.get(2).cloned().ok_or("Missing announcement flags")?];
                announcement.extend(arguments.iter().skip(3).cloned());
                let packet = super::script_presentation_service::announcement_packet(&announcement)?;
                let recipients: Vec<u32> = state.characters().values().filter(|character| normalize_map(character.current_map_name()) == map && instance.is_none_or(|instance| instance == character.current_map_instance())).map(|character| character.char_id).collect();
                self.map_notifications.extend(recipients.into_iter().map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))));
                self.drain_map_notifications();
                Ok(Value::default())
            }
            Function::GetMapUsers => {
                let (map, instance) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                Ok(Value::Number(state.characters().values().filter(|character| normalize_map(character.current_map_name()) == map && instance.is_none_or(|instance| instance == character.current_map_instance())).count() as i32))
            }
            Function::GetAreaUsers => {
                let (map, instance) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let (x0, y0, x1, y1) = (number(1)?, number(2)?, number(3)?, number(4)?);
                Ok(Value::Number(state.characters().values().filter(|character| {
                    normalize_map(character.current_map_name()) == map
                        && instance.is_none_or(|instance| instance == character.current_map_instance())
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
            Function::ConvertPcInfo => {
                let target = &arguments.first().ok_or("Missing player")?;
                let character = match target {
                    Value::Number(id) => {
                        let id = *id as u32;
                        state.characters().values().find(|character| character.char_id == id).or_else(|| state.characters().values().find(|character| character.account_id == id))
                    }
                    name => state.characters().values().find(|character| character.name == name.text()),
                };
                Ok(match (number(1)?, character) {
                    (0, Some(character)) => Value::String(character.name.clone()),
                    (1, Some(character)) => Value::Number(character.char_id as i32),
                    (2, Some(character)) => Value::Number(character.account_id as i32),
                    (0, None) => Value::String(String::new()),
                    _ => Value::Number(0),
                })
            }
            Function::Nude => {
                let mut character = state.characters_mut().remove(&context.char_id).ok_or("nude needs an attached player")?;
                let equipped: Vec<usize> = character.inventory_wearable().into_iter().filter(|(_, item)| item.equip != 0).map(|(index, _)| index).collect();
                for index in equipped {
                    self.inventory_service().takeoff_equip_item(&mut character, index);
                }
                state.insert_character(character);
                Ok(Value::default())
            }
            Function::SetNpcDisplay => {
                let (npc, _) = crate::server::service::npc_event_service::named_npc(state, context, &text(0)?)?.ok_or("setnpcdisplay: unknown NPC")?;
                let sprite = match &arguments[1] {
                    Value::Number(sprite) => *sprite,
                    name => crate::server::script::constant::load_constant(&name.text()).ok_or("setnpcdisplay: unknown sprite")?.number_value()?,
                };
                let range = AreaNotificationRangeType::Fov { x: npc.x, y: npc.y, exclude_id: None };
                let packet = npc_sprite_packet(npc.id, u32::try_from(sprite).map_err(|_| "Invalid NPC sprite")?);
                let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(npc.map.clone(), npc.instance, range, packet)));
                Ok(Value::default())
            }
            Function::ConsumeItem => {
                let item = crate::server::script::utilities::find_item(GlobalConfigService::instance(), arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                let mut character = state.characters_mut().remove(&context.char_id).ok_or("consumeitem needs an attached player")?;
                let result = self.item_service().run_item_script(self, state, &mut character, item.id as u32);
                state.insert_character(character);
                result.map(|_| Value::default())
            }
            Function::MakeItem => {
                use crate::server::model::events::map_event::{MapEvent, ScriptDropItem};
                let item = crate::server::script::utilities::find_item(GlobalConfigService::instance(), arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                let amount = i16::try_from(number(1)?).ok().filter(|amount| *amount > 0).ok_or("makeitem amount must be positive")?;
                let (x, y) = (u16::try_from(number(3)?).map_err(|_| "Invalid makeitem x")?, u16::try_from(number(4)?).map_err(|_| "Invalid makeitem y")?);
                let caller = crate::server::script::unit_data::script_actor(state, context)?;
                let (map_name, instance) = match (text(2)?, caller) {
                    (name, Some(npc)) if name.eq_ignore_ascii_case("this") => (npc.map.clone(), npc.instance),
                    (name, Some(npc)) => (name, npc.instance),
                    (name, None) => {
                        let character = state.characters().get(&context.char_id).ok_or("makeitem needs an NPC or a player")?;
                        if name.eq_ignore_ascii_case("this") { (character.current_map_name().clone(), character.current_map_instance()) } else { (name, character.current_map_instance()) }
                    }
                };
                let map = state.get_map_instance(&map_name, instance).ok_or("makeitem: map is unavailable")?;
                map.add_to_next_tick(MapEvent::ScriptDropItem(ScriptDropItem { owner_id: context.char_id, item_id: item.id, amount, x, y }));
                Ok(Value::default())
            }
            Function::ReadBook => {
                let mut packet = READ_BOOK_PACKET.to_le_bytes().to_vec();
                packet.extend_from_slice(&number(0)?.to_le_bytes());
                packet.extend_from_slice(&number(1)?.to_le_bytes());
                if context.char_id == 0 {
                    return Err("readbook needs an attached player".into());
                }
                let _ = self.server_service().notification_sender().try_send(Notification::Char(CharNotification::new(context.char_id, packet)));
                Ok(Value::default())
            }
            Function::GetGmLevel => {
                let character = state.characters().get(&context.char_id).ok_or("getgmlevel needs an attached player")?;
                Ok(Value::Number(i32::from(state.permission_groups().level(state.group_id_of(character.account_id)))))
            }
            Function::OpenMail => {
                if context.char_id == 0 {
                    return Err("openmail needs an attached player".into());
                }
                self.open_mail_window(state, context.char_id);
                Ok(Value::default())
            }
            Function::OpenAuction => {
                if context.char_id == 0 {
                    return Err("openauction needs an attached player".into());
                }
                self.tell(context.char_id, AUCTION_DISABLED);
                Ok(Value::default())
            }
            Function::WaitingRoom | Function::DelWaitingRoom | Function::WaitingRoomKick | Function::WaitingRoomKickAll | Function::EnableWaitingRoomEvent
            | Function::DisableWaitingRoomEvent | Function::GetWaitingRoomState | Function::WarpWaitingPc => self.script_waiting_room_call(state, context, function, arguments),
            Function::CountItem2 | Function::DelItem2 => self.script_exact_items(state, context, function, arguments),
            Function::GetInventoryList => {
                let character = state.characters().get(&context.char_id).ok_or("getinventorylist needs an attached player")?;
                Ok(Value::Array(character.inventory.iter().flatten().flat_map(|item| {
                    [item.item_id, i32::from(item.amount), item.equip, i32::from(item.refine), i32::from(item.is_identified), i32::from(item.is_damaged),
                        i32::from(item.card0), i32::from(item.card1), i32::from(item.card2), i32::from(item.card3)].map(Value::Number)
                }).collect()))
            }
            Function::PushPc => {
                let mut direction = usize::try_from(number(0)?).ok().filter(|direction| *direction < 8).ok_or("pushpc direction must be between 0 and 7")?;
                let mut cells = number(1)?;
                if cells < 0 {
                    direction = (direction + 4) % 8;
                    cells = -cells;
                }
                let character = state.characters().get(&context.char_id).ok_or("pushpc needs an attached player")?;
                let (map_name, instance) = (character.current_map_name().clone(), character.current_map_instance());
                let map = state.get_map_instance(&map_name, instance).ok_or("pushpc: map is unavailable")?;
                let map = map.state();
                let (mut x, mut y) = (i32::from(character.x()), i32::from(character.y()));
                for _ in 0..cells {
                    let (next_x, next_y) = (x + PUSH_DX[direction], y + PUSH_DY[direction]);
                    let walkable = u16::try_from(next_x).ok().zip(u16::try_from(next_y).ok()).is_some_and(|(next_x, next_y)| {
                        next_x < map.x_size() && next_y < map.y_size()
                            && map.cells().get(map.get_cell_index_of(next_x, next_y)).is_some_and(|cell| cell & models::enums::cell::CellType::Walkable.as_flag() != 0)
                    });
                    if !walkable {
                        break;
                    }
                    (x, y) = (next_x, next_y);
                }
                let destination = crate::server::model::map::Map::name_without_ext(&map_name).to_string();
                self.server_service().schedule_warp_to_walkable_cell_by_character_in_instance(&destination, x as u16, y as u16, context.char_id, instance);
                Ok(Value::default())
            }
            Function::AreaAnnounce => {
                let (map, instance) = self.resolve_script_map(context.npc_scope_instance, &text(0)?);
                let (x0, y0, x1, y1) = (number(1)?, number(2)?, number(3)?, number(4)?);
                let packet = super::script_presentation_service::announcement_packet(arguments.get(5..).ok_or("Missing announcement")?)?;
                let recipients: Vec<u32> = state.characters().values().filter(|character| {
                    normalize_map(character.current_map_name()) == map
                        && instance.is_none_or(|instance| instance == character.current_map_instance())
                        && (x0..=x1).contains(&i32::from(character.x()))
                        && (y0..=y1).contains(&i32::from(character.y()))
                }).map(|character| character.char_id).collect();
                self.map_notifications.extend(recipients.into_iter().map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))));
                self.drain_map_notifications();
                Ok(Value::default())
            }
            Function::MercenaryGetFaith => {
                let character = state.characters().get(&context.char_id).ok_or("mercenary_get_faith needs an attached player")?;
                let guild = u8::try_from(number(0)?).map_err(|_| "Invalid mercenary guild")?;
                Ok(Value::Number(i32::from(character.game_systems.mercenary_faith.get(&guild).copied().unwrap_or_default())))
            }
            Function::NpcTalk => {
                let (npc, script) = match arguments.get(1) {
                    Some(name) => crate::server::service::npc_event_service::named_npc(state, context, &name.text())?.ok_or("npctalk: unknown NPC")?,
                    None => {
                        let npc = crate::server::script::unit_data::script_actor(state, context)?.ok_or("npctalk needs an NPC")?;
                        let script = state.get_map_instance(&npc.map, npc.instance).and_then(|instance| instance.get_script(npc.id)).ok_or("npctalk: NPC is unavailable")?;
                        (npc, script)
                    }
                };
                let speaker = script.name.split('#').next().unwrap_or(&script.name);
                let packet = npc_chat_packet(npc.id, &format!("{speaker} : {}", text(0)?));
                let range = AreaNotificationRangeType::Fov { x: npc.x, y: npc.y, exclude_id: None };
                let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(npc.map.clone(), npc.instance, range, packet)));
                Ok(Value::default())
            }
            Function::UnloadNpc | Function::MoveNpc => {
                let (actor, script) = crate::server::service::npc_event_service::named_npc(state, context, &text(0)?)?.ok_or("NPC is not found")?;
                let map_instance = state.get_map_instance(&actor.map, actor.instance).ok_or("NPC map is unavailable")?;
                let command = if function == Function::UnloadNpc {
                    ScriptMapCommand::NpcRemove { npc_id: script.id }
                } else {
                    let coordinate = |index: usize| u16::try_from(number(index)?).map_err(|_| "NPC coordinate is outside the map".to_string());
                    let dir = arguments.get(3).map(Value::number_value).transpose()?.map(|dir| u16::try_from(dir).map_err(|_| "Invalid NPC direction".to_string())).transpose()?;
                    ScriptMapCommand::NpcMove { npc_id: script.id, x: coordinate(1)?, y: coordinate(2)?, dir }
                };
                map_instance.add_to_next_tick(MapEvent::ScriptMapCommand(command));
                Ok(Value::default())
            }
            Function::GetMonsterInfo => {
                let configuration = GlobalConfigService::instance();
                let mob = arguments.first().ok_or("Missing monster")?.text().parse::<i32>().ok().and_then(|id| configuration.get_mob_safe(id));
                let kind = number(1)?;
                Ok(match (mob, kind) {
                    (None, 1) => Value::String("null".into()),
                    (None, _) => Value::Number(-1),
                    (Some(mob), kind) => match MONSTER_INFO.get(usize::try_from(kind - 1).unwrap_or(usize::MAX)).copied() {
                        Some("NAME") => Value::String(mob.name.clone()),
                        Some("LV") => Value::Number(mob.level),
                        Some("MAXHP") => Value::Number(mob.hp),
                        Some("MAXSP") => Value::Number(mob.sp),
                        Some("BASEEXP") => Value::Number(mob.exp),
                        Some("JOBEXP") => Value::Number(mob.job_exp),
                        Some("ATKMIN") => Value::Number(mob.atk1),
                        Some("ATKMAX") => Value::Number(mob.atk2),
                        Some("DEF") => Value::Number(mob.def),
                        Some("MDEF") => Value::Number(mob.mdef),
                        Some("STR") => Value::Number(mob.str),
                        Some("AGI") => Value::Number(mob.agi),
                        Some("VIT") => Value::Number(mob.vit),
                        Some("INT") => Value::Number(mob.int),
                        Some("DEX") => Value::Number(mob.dex),
                        Some("LUK") => Value::Number(mob.luk),
                        Some("SPEED") => Value::Number(mob.speed),
                        Some("ATKRANGE") => Value::Number(i32::from(mob.range1)),
                        Some("SKILLRANGE") => Value::Number(i32::from(mob.range2)),
                        Some("CHASERANGE") => Value::Number(i32::from(mob.range3)),
                        Some("SIZE") => Value::Number(i32::from(mob.scale)),
                        Some("ELEMENTLV") => Value::Number(i32::from(mob.element_level)),
                        Some("MODE") => Value::Number(i32::from(mob.mode)),
                        Some("MVPEXP") => Value::Number(mob.mvp_exp),
                        Some("ID") => Value::Number(mob.id),
                        _ => Value::Number(-1),
                    },
                })
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
