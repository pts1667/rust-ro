use std::fmt::Write;
use std::time::Instant;

use configuration::configuration::CityConfig;
use lazy_static::lazy_static;
use models::enums::class::JobName;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use packets::packets::{Packet, PacketZcNotifyPlayerchat};
use regex_lite::Regex;

use crate::load_scripts;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterChangeJob, CharacterChangeJobLevel, CharacterChangeLevel, GameEvent, CharacterResetSkills, CharacterResetStats, CharacterRestoreAllHpAndSP, CharacterUpdateSpeed};
use crate::server::model::duel::{DuelAction, DuelCommand};
use crate::server::model::map::RANDOM_CELL;
use crate::server::model::map_flags::MapFlag;
use crate::server::model::autoloot::{AUTOLOOT_ITEM_SLOTS, AutoLoot};
use crate::server::model::permission_groups::CommandKind;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::state::server::ServerState;
use crate::server::script::Value;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::packet::playerchat_packet;

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^([@#!])([^\s]*)\s?(.*)?").unwrap();
}
pub fn handle_atcommand(server: &Server, state: &mut ServerState, char_id: u32, message: &str) {
    let index_of_colon = message.find(':').unwrap();
    let command_txt = &message[index_of_colon + 1..message.len()].trim();
    debug!("Received atcommand: {}", command_txt);
    let maybe_captures = COMMAND_REGEX.captures(command_txt);
    if maybe_captures.is_none() {
        return;
    }
    let captures = maybe_captures.unwrap();
    if captures.len() < 2 {
        return;
    }
    let symbol = captures.get(1).unwrap().as_str();
    let command = captures.get(2).unwrap().as_str();
    let mut args = Vec::<&str>::new();
    if captures.len() > 2 {
        args = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|arg| arg.trim_matches(char::from(0)))
            .collect();
    }
    let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new(GlobalConfigService::instance().packetver());
    let groups = state.permission_groups();
    let account_id = state.get_character(char_id).map_or(0, |character| character.account_id);
    let group_id = state.group_id_of(account_id);
    let kind = if symbol == "#" { CommandKind::Char } else { CommandKind::At };
    let canonical = groups.canonical_command(command);
    if !groups.group(group_id).can_use_command(&canonical, kind) {
        packet_zc_notify_playerchat.set_msg(format!("{symbol}{command} is an Unknown Command."));
        send_chat_reply(server, char_id, packet_zc_notify_playerchat);
        return;
    }
    if groups.group(group_id).log_commands {
        info!(target: "atcommand_log", "account {account_id} char {char_id} (group {group_id}): {symbol}{command} {}", args.join(" ").trim());
    }
    match canonical.as_str() {
        "go" => {
            debug!("{:?}", args);
            let result = handle_go(server, state, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "mapmove" | "jumpto" => {
            let result = handle_warp(server, state, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "item" => {
            let result = handle_item(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "inspect" | "i" => {
            let result = handle_inspect(server, state, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "baselevelup" => {
            let result = handle_base_level(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "setblvl" | "setblevel" | "setbaselvl" | "setbaselevel" => {
            let result = handle_set_base_level(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "joblevelup" | "jlvup" => {
            let result = handle_job_level(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "setjlvl" | "setjlevel" | "setjoblvl" | "setjoblevel" => {
            let result = handle_set_job_level(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "jobchange" => {
            let result = handle_set_job(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "rates" | "rate" => {
            let result = handle_rates(server);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "reload" => {
            let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
            let sender = server.server_service().notification_sender();
            let packetver = GlobalConfigService::instance().packetver();
            std::thread::spawn(move || {
                let result = handle_reload(args.iter().map(String::as_str).collect());
                let mut packet = playerchat_packet(packetver, &result);
                sender
                    .send(Notification::Char(CharNotification::new(char_id, std::mem::take(packet.raw_mut()))))
                    .unwrap_or_else(|_| error!("Failed to send notification packet_zc_notify_playerchat to client"));
            });
            return;
        }
        "resetskill" | "resetskills" => {
            let result = handle_reset_skills(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "resetstat" | "resetstats" => {
            let result = handle_reset_stats(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "speed" => {
            let result = handle_speed_change(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "duel" | "invite" | "accept" | "reject" | "leave" | "killer" | "pk" => {
            let action = match canonical.as_str() {
                "duel" => DuelAction::Create,
                "invite" => DuelAction::Invite,
                "accept" => DuelAction::Accept,
                "reject" => DuelAction::Reject,
                "killer" | "pk" => DuelAction::Killer,
                _ => DuelAction::Leave,
            };
            server.add_to_next_tick(GameEvent::Duel(DuelCommand {
                char_id: char_id,
                action,
                argument: args.join(" ").trim().to_string(),
            }));
            return;
        }
        "ban" | "unban" | "char_ban" | "char_unban" | "char_block" | "char_unblock" | "kick" => {
            for reply in super::atcommand_admin::handle(server, state, char_id, canonical.as_str(), &args.join(" ")) {
                let mut packet = PacketZcNotifyPlayerchat::new(GlobalConfigService::instance().packetver());
                packet.set_msg(reply);
                send_chat_reply(server, char_id, packet);
            }
            return;
        }
        "kami" | "kamib" | "kamic" | "lkami" => {
            let reply = server.command_broadcast(char_id, canonical.as_str(), &args.join(" "));
            if reply.is_empty() {
                return;
            }
            packet_zc_notify_playerchat.set_msg(reply);
        }
        "setquest" | "erasequest" | "completequest" | "checkquest" | "questskill" | "lostskill" => {
            let reply = server.quest_command(state, char_id, canonical.as_str(), &args.join(" "));
            for line in reply.lines() {
                server.tell(char_id, line);
            }
            return;
        }
        "channel" | "main" | "join" => {
            let reply = server.channel_command(state, char_id, canonical.as_str(), &args);
            for line in reply.lines() {
                server.tell(char_id, line);
            }
            return;
        }
        "mail" => {
            server.open_mail_window(state, char_id);
            return;
        }
        "noask" => {
            let enabled = state.characters_mut().get_mut(&char_id).is_some_and(|character| {
                character.game_systems.no_ask = !character.game_systems.no_ask;
                character.game_systems.no_ask
            });
            packet_zc_notify_playerchat.set_msg(if enabled { "Autorejecting is activated." } else { "Autorejecting is deactivated." }.to_string());
        }
        "autoloot" => {
            let reply = state.characters_mut().get_mut(&char_id).map(|character| handle_autoloot(&mut character.game_systems.autoloot, &args));
            packet_zc_notify_playerchat.set_msg(reply.unwrap_or_default());
        }
        "autolootitem" => {
            let reply = state.characters_mut().get_mut(&char_id).map(|character| handle_autolootitem(&mut character.game_systems.autoloot, &args));
            for line in reply.unwrap_or_default().lines() {
                server.tell(char_id, line);
            }
            return;
        }
        "reloadmotd" => {
            server.motd().reload(&server.configuration.server.motd_path);
            packet_zc_notify_playerchat.set_msg("Reloaded the Message of the Day.".to_string());
        }
        "heal" => {
            let result = handle_heal(server, char_id, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        _ => {
            packet_zc_notify_playerchat.set_msg(format!("{symbol}{command} is an Unknown Command."));
        }
    }
    send_chat_reply(server, char_id, packet_zc_notify_playerchat);
}

fn send_chat_reply(server: &Server, char_id: u32, reply: PacketZcNotifyPlayerchat) {
    let mut packet = playerchat_packet(GlobalConfigService::instance().packetver(), &reply.msg);
    server
        .server_service()
        .notification_sender()
        .send(Notification::Char(CharNotification::new(char_id, std::mem::take(packet.raw_mut()))))
        .unwrap_or_else(|_| error!("Failed to send notification packet_zc_notify_playerchat to client"));
}

pub fn handle_go(server: &Server, state: &mut ServerState, char_id: u32, args: Vec<&str>) -> String {
    let cities_len = server.configuration.maps.cities.len();
    let cleaned_arg = args[0].trim();
    let mut maybe_city: Option<&CityConfig> = None;
    if let Ok(index) = cleaned_arg.parse::<i8>() {
        if index < cities_len as i8 {
            maybe_city = unsafe { Some(server.configuration.maps.cities.get_unchecked(index as usize)) } // it safe, bounds are checked
        }
    }
    if maybe_city.is_none() {
        // aliases
        let name = match cleaned_arg {
            "old_moc" => "morroc".to_string(),
            "morocc" => "morroc".to_string(),
            "lutie" => "xmas".to_string(),
            "juno" => "yuno".to_string(),
            "kunlun" => "gornyun".to_string(),
            "luoyang" => "louyang".to_string(),
            "new1-1" => "novice".to_string(),
            "startpoint" => "novice".to_string(),
            "beginning" => "novice".to_string(),
            "prison" => "jail".to_string(),
            "sec_pri" => "jail".to_string(),
            "rael" => "rachel".to_string(),
            _ => cleaned_arg.to_string(),
        };
        maybe_city = server.configuration.maps.cities.iter().find(|city| city.name == name);
    }
    if maybe_city.is_none() {
        return format!("Can't find map by index or name with given argument: {cleaned_arg}");
    }
    let mut city = maybe_city.unwrap().clone();

    match city.name.as_str() {
        // To match client side name
        "morroc" => city.name = "old_moc".to_string(),
        "lutie" => city.name = "xmas".to_string(),
        "juno" => city.name = "yuno".to_string(),
        "kunlun" => city.name = "gornyun".to_string(),
        "luoyang" => city.name = "louyang".to_string(),
        "novice" => city.name = "new1-1".to_string(),
        "startpoint" => city.name = "new1-1".to_string(),
        "beginning" => city.name = "new1-1".to_string(),
        "prison" => city.name = "sec_pri".to_string(),
        "jail" => city.name = "sec_pri".to_string(),
        "rael" => city.name = "rachel".to_string(),
        _ => (),
    }

    if state.map_flags(&state.get_character_unsafe(char_id).map_instance_key).enabled(MapFlag::NoGo) {
        return "You cannot use @go on this map.".into();
    }
    if let Some(refusal) = admin_travel_blocked(state, char_id, &city.name) {
        return refusal;
    }
    server
        .server_service()
        .schedule_warp_to_walkable_cell(state, &city.name, city.x, city.y, char_id);
    format!("Warping at {} {},{}", city.name.clone(), city.x, city.y)
}

fn admin_travel_blocked(state: &ServerState, char_id: u32, destination: &str) -> Option<String> {
    let character = state.get_character_unsafe(char_id);
    if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoWarp) {
        return Some("You are not authorized to warp from your current map.".into());
    }
    if state.map_flags_for(destination, 0).enabled(MapFlag::NoWarpTo) {
        return Some("You are not authorized to warp to this destination map.".into());
    }
    None
}

pub fn handle_warp(server: &Server, state: &mut ServerState, char_id: u32, args: Vec<&str>) -> String {
    let map_name = args[0].to_string();
    if GlobalConfigService::instance().maps().contains_key(&map_name) {
        if let Some(refusal) = admin_travel_blocked(state, char_id, &map_name) {
            return refusal;
        }
        let mut x = RANDOM_CELL.0;
        let mut y = RANDOM_CELL.1;
        if args.len() > 2 {
            let parse_x_res = args[1].parse::<u16>();
            let parse_y_res = args[2].parse::<u16>();
            if let Ok(parse_x_res) = parse_x_res {
                x = parse_x_res;
            }
            if let Ok(parse_y_res) = parse_y_res {
                y = parse_y_res;
            }
        }
        server
            .server_service()
            .schedule_warp_to_walkable_cell(state, &map_name, x, y, char_id);
        let character = state.get_character_unsafe(char_id);
        return format!("Warp to map {} at {},{}", map_name, character.x(), character.y());
    }
    format!("Map not found: {map_name}")
}

pub fn handle_item(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return format!("@item command accept from 1 to 2 parameters but received {}", args.len());
    }
    server.script_service().schedule_get_items(
        char_id,
        server.runtime(),
        vec![(
            args[0]
                .parse::<i32>()
                .map(Value::Number)
                .unwrap_or(Value::String(args[0].to_string())),
            args.get(1).unwrap_or(&"1").parse::<i16>().unwrap_or(1),
        )],
        false,
    );

    String::new()
}

pub fn handle_inspect(server: &Server, state: &mut ServerState, char_id: u32, _args: Vec<&str>) -> String {
    let char_id = char_id;
    let character = state.get_character_unsafe(char_id);
    server.character_service().print(character);
    String::new()
}

pub fn handle_base_level(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@baselevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeLevel(CharacterChangeLevel {
        char_id: char_id,
        set_level: None,
        add_level: Some(args.first().unwrap().parse::<i32>().unwrap_or(0)),
    }));
    String::new()
}

pub fn handle_set_base_level(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@set_baselevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeLevel(CharacterChangeLevel {
        char_id: char_id,
        set_level: args.first().unwrap().parse::<u32>().ok(),
        add_level: None,
    }));
    String::new()
}

pub fn handle_job_level(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@joblevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeJobLevel(CharacterChangeJobLevel {
        char_id: char_id,
        set_level: None,
        add_level: Some(args.first().unwrap().parse::<i32>().unwrap_or(0)),
    }));
    String::new()
}

pub fn handle_set_job_level(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@set_joblevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeJobLevel(CharacterChangeJobLevel {
        char_id: char_id,
        set_level: args.first().unwrap().parse::<u32>().ok(),
        add_level: None,
    }));
    String::new()
}

pub fn handle_set_job(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@job command accept 1 parameters but received none".to_string();
    }
    let maybe_job = if let Ok(job_id) = args.first().unwrap().parse::<u32>() {
        JobName::try_from_value(job_id as usize)
    } else {
        JobName::try_from_string(args.first().unwrap())
    };
    if let Ok(job) = maybe_job {
        server.add_to_next_tick(GameEvent::CharacterChangeJob(CharacterChangeJob {
            char_id,
            job,
            should_reset_skills: true,
        }));
        return "Your job has been changed.".to_string();
    }
    format!("Job {} not found", args.first().unwrap())
}

pub fn handle_rates(server: &Server) -> String {
    let mut msg = String::new();
    writeln!(
        msg,
        "Experience rates: Base {:.2}x / Job {:.2}x",
        server.configuration.game.base_exp_rate, server.configuration.game.job_exp_rate
    )
    .unwrap();
    writeln!(
        msg,
        "Normal Drop Rates: Common {:.2}x / Healing {:.2}x / Usable {:.2}x Equipment {:.2}x / Card {:.2}x",
        server.configuration.game.drop_rate,
        server.configuration.game.drop_rate,
        server.configuration.game.drop_rate,
        server.configuration.game.drop_rate,
        server.configuration.game.drop_rate_card
    )
    .unwrap();
    writeln!(
        msg,
        "Boss  Drop Rates: Common {:.2}x / Healing {:.2}x / Usable {:.2}x Equipment {:.2}x / Card {:.2}x",
        server.configuration.game.drop_rate_mvp,
        server.configuration.game.drop_rate_mvp,
        server.configuration.game.drop_rate_mvp,
        server.configuration.game.drop_rate_mvp,
        server.configuration.game.drop_rate_card
    )
    .unwrap();
    msg
}

pub fn handle_reload(args: Vec<&str>) -> String {
    // TODO check if user privileges
    if args.is_empty() {
        return "@reload command accept 1 parameters but received none".to_string();
    }
    match args[0] {
        "script" => {
            let start = Instant::now();
            let scripts = load_scripts();
            format!(
                "{} scripts have been recompiled and reloaded in {} secs",
                scripts.len(),
                start.elapsed().as_millis() as f32 / 1000.0
            )
        }
        &_ => format!("@reload command accept a string value among: [script] but received {}", args[0]),
    }
}

pub fn handle_reset_skills(server: &Server, char_id: u32, _args: Vec<&str>) -> String {
    server.add_to_next_tick(GameEvent::CharacterResetSkills(CharacterResetSkills { char_id: char_id }));
    "Skills have been reset.".to_string()
}

pub fn handle_reset_stats(server: &Server, char_id: u32, _args: Vec<&str>) -> String {
    server.add_to_next_tick(GameEvent::CharacterResetStats(CharacterResetStats { char_id: char_id }));
    "Stats have been reset.".to_string()
}

pub fn handle_speed_change(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@speed command accept 1 parameters but received none".to_string();
    }
    let mut speed = args[0].parse::<u16>().unwrap_or(150_u16);
    if speed < 50 {
        speed = 50;
    } else if speed > 500 {
        speed = 500;
    }
    server.add_to_next_tick(GameEvent::CharacterUpdateSpeed(CharacterUpdateSpeed { char_id: char_id, speed }));
    format!("Speed has been set at {}.", speed)
}

/// `@autoloot [percent]`: no argument toggles between off and everything.
fn handle_autoloot(autoloot: &mut AutoLoot, args: &[&str]) -> String {
    let requested = args.first().filter(|arg| !arg.is_empty()).map(|arg| arg.parse::<f64>().map_or(0, |percent| (percent * 100.0) as i32));
    let rate = requested.unwrap_or(if autoloot.rate > 0 { 0 } else { 10_000 }).clamp(0, 10_000) as u16;
    autoloot.rate = rate;
    if rate == 0 {
        "Autoloot is now off.".to_string()
    } else if rate == 10_000 {
        "Autoloot is now on.".to_string()
    } else {
        format!("Autolooting items with drop rates of {:.2}% and below.", f64::from(rate) / 100.0)
    }
}

/// `@autolootitem <item>` adds, `-<item>` removes, `reset` clears and no argument lists.
fn handle_autolootitem(autoloot: &mut AutoLoot, args: &[&str]) -> String {
    let argument = args.join(" ").trim().to_string();
    let configuration = GlobalConfigService::instance();
    let describe = |id: i32| configuration.find_item(id).map_or_else(|| id.to_string(), |item| format!("{} {{{}}}", item.name_english, id));
    if argument == "reset" {
        autoloot.items = [0; AUTOLOOT_ITEM_SLOTS];
        return "Your autolootitem list has been reset.".to_string();
    }
    if argument.is_empty() {
        let listed: Vec<String> = autoloot.items.iter().filter(|id| **id != 0).map(|id| describe(*id)).collect();
        if listed.is_empty() {
            return "Your autolootitem list is empty.".to_string();
        }
        return format!("Items on your autolootitem list:\n{}", listed.join("\n"));
    }
    let (removing, name) = argument.strip_prefix('-').map_or((false, argument.as_str()), |name| (true, name.trim()));
    let name = name.strip_prefix('+').unwrap_or(name).trim();
    let item = name.parse::<i32>().ok().and_then(|id| configuration.find_item(id)).or_else(|| configuration.find_item_by_name(name));
    let Some(item) = item else {
        return format!("Item '{name}' not found.");
    };
    let slot = autoloot.items.iter().position(|id| *id == item.id);
    match (removing, slot) {
        (true, Some(slot)) => {
            autoloot.items[slot] = 0;
            format!("Removed item: '{}' from your autolootitem list.", describe(item.id))
        }
        (true, None) => "You're currently not autolooting this item.".to_string(),
        (false, Some(_)) => "You're already autolooting this item.".to_string(),
        (false, None) => match autoloot.items.iter().position(|id| *id == 0) {
            Some(free) => {
                autoloot.items[free] = item.id;
                format!("Autolooting item: '{}'.", describe(item.id))
            }
            None => format!("Your autolootitem list is full. Remove some items first with @autolootitem -<item name or ID>."),
        },
    }
}

pub fn handle_heal(server: &Server, char_id: u32, args: Vec<&str>) -> String {
    if args.is_empty() {
        return "@speed command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterRestoreAllHpAndSP(CharacterRestoreAllHpAndSP { char_id: char_id }));
    "Restored all HP and SP".to_string()
}
