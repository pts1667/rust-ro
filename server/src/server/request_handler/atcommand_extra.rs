use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::look::LookType;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusTypes;

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterLook, CharacterZeny, GameEvent};
use crate::server::model::events::map_event::{AdminKillAllMobs, MapEvent, ScriptSpawn};
use crate::server::model::map::RANDOM_CELL;
use crate::server::model::permission_groups::CommandKind;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::script_character_service::{self, learned_level};
use crate::server::state::character::{Character, CharacterAction};
use crate::server::state::server::ServerState;

const MAX_ZENY: i64 = 1_000_000_000;
const MAX_SPAWN_AMOUNT: u16 = 100;
const SPAWN_SPREAD: i32 = 5;
const MAX_HAIR_STYLE: u16 = 29;
const MAX_HAIR_COLOR: u16 = 8;
const MAX_CLOTHES_COLOR: u16 = 4;
const LISTED_NAMES: usize = 50;
const LISTED_DROPPERS: usize = 10;
const RESURRECTION_EFFECT_PACKET: u16 = 0x0148;

fn number(args: &[&str], index: usize) -> Option<i64> {
    args.get(index).and_then(|arg| arg.parse::<i64>().ok())
}

fn lines(text: impl IntoIterator<Item = String>) -> Option<Vec<String>> {
    Some(text.into_iter().collect())
}

fn say(text: &str) -> Option<Vec<String>> {
    Some(vec![text.to_string()])
}

/// GM commands that act on the caller or the world around them. `None` means the command is not handled here.
pub fn handle(server: &Server, state: &mut ServerState, char_id: u32, command: &str, args: &[&str]) -> Option<Vec<String>> {
    let args: Vec<&str> = args.iter().copied().filter(|arg| !arg.is_empty()).collect();
    let args = args.as_slice();
    match command {
        "zeny" => zeny(server, state, char_id, args),
        "statuspoint" => points(server, state, char_id, args, true),
        "skillpoint" => points(server, state, char_id, args, false),
        "allskill" => all_skills(server, state, char_id),
        "skilltree" => skill_tree(state, args),
        "feelreset" => star_memory_reset(server, state, char_id, false),
        "hatereset" => star_memory_reset(server, state, char_id, true),
        "str" | "agi" | "vit" | "int" | "dex" | "luk" => stat(server, state, char_id, command, args),
        "stat_all" => stat_all(server, state, char_id, args),
        "heal" => heal(server, state, char_id, args),
        "alive" => alive(server, state, char_id),
        "jump" => jump(server, state, char_id, args),
        "recall" => recall(server, state, char_id, args),
        "partyrecall" => recall_group(server, state, char_id, true),
        "guildrecall" => recall_group(server, state, char_id, false),
        "storage" => storage(server, state, char_id, false),
        "guildstorage" => storage(server, state, char_id, true),
        "monster" => monster(state, char_id, args),
        "killmonster" => kill_monsters(state, char_id),
        "mobsearch" => mob_search(state, char_id, args),
        "mobinfo" => mob_info(args),
        "whodrops" => who_drops(args),
        "day" | "night" => time_of_day(server, state, command == "night"),
        "users" => say(&format!("{} user(s) online.", online(state).count())),
        "who" => who(state, args, None),
        "whomap" => {
            let map = state.get_character(char_id).map(|character| character.current_map_name().clone());
            who(state, args, map)
        }
        "mapinfo" => map_info(state, char_id),
        "hair_style" => look(server, state, char_id, args, LookType::Hair, MAX_HAIR_STYLE),
        "hair_color" => look(server, state, char_id, args, LookType::HairColor, MAX_HAIR_COLOR),
        "dye" => look(server, state, char_id, args, LookType::ClothesColor, MAX_CLOTHES_COLOR),
        "save" => save(server, state, char_id),
        "commands" => commands(state, char_id),
        "servertime" => say(&format!(
            "Server time: {} ({}).",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            if server.day_night().is_night() { "night" } else { "day" }
        )),
        _ => None,
    }
}

fn online(state: &ServerState) -> impl Iterator<Item = &Character> {
    state.characters().values().filter(|character| character.loaded_from_client_side)
}

fn zeny(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let Some(amount) = number(args, 0).filter(|amount| *amount != 0) else {
        return say("Please, enter an amount (usage: @zeny <amount>).");
    };
    let current = i64::from(state.get_character(char_id)?.get_zeny());
    let updated = (current + amount).clamp(0, MAX_ZENY);
    server.add_to_next_tick(GameEvent::CharacterUpdateZeny(CharacterZeny { char_id, zeny: Some(updated as u32) }));
    say(if amount > 0 { "Zeny gained." } else { "Zeny taken." })
}

fn points(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str], status: bool) -> Option<Vec<String>> {
    let Some(amount) = number(args, 0).filter(|amount| *amount != 0) else {
        return say("Please, enter a number (usage: @statpoint/@skpoint <number of points>).");
    };
    let character = state.characters_mut().get_mut(&char_id)?;
    let service = server.character_service();
    if status {
        let updated = (i64::from(character.status.status_point) + amount).clamp(0, i64::from(u16::MAX));
        service.update_status_point(character, updated as u32);
        say("Status points changed.")
    } else {
        let updated = (i64::from(character.status.skill_point) + amount).clamp(0, i64::from(u16::MAX));
        service.update_skill_point(character, updated as u32, true);
        say("Skill points changed.")
    }
}

/// `pc_allskillup`: the caller's job tree at maximum levels, through the same commit as a skill reset.
fn all_skills(server: &Server, state: &mut ServerState, char_id: u32) -> Option<Vec<String>> {
    let character = state.characters_mut().get_mut(&char_id)?;
    let plan = match script_character_service::plan_all_skills(character) {
        Ok(plan) => plan,
        Err(error) => return say(&error),
    };
    if let Err(error) = server.repository.character_commit_skill_reset(character.char_id, character.account_id, &plan) {
        return say(&error.to_string());
    }
    script_character_service::apply_reset_skills(server, character, &plan);
    say("All skills have been added to your skill tree.")
}

/// Shows the caller's requirements for a skill of the target's job tree, or that the target cannot use it.
fn skill_tree(state: &ServerState, args: &[&str]) -> Option<Vec<String>> {
    let usage = "Please, enter a skill ID and a character name (usage: @skilltree <skill ID> <char name>).";
    let Some(skill_id) = number(args, 0).and_then(|id| u32::try_from(id).ok()) else {
        return say(usage);
    };
    if args.len() < 2 {
        return say(usage);
    }
    let wanted = args[1..].join(" ");
    let Some(target) = online(state).find(|character| character.name.eq_ignore_ascii_case(&wanted)) else {
        return say("Character not found.");
    };
    let Ok(job) = JobName::try_from_value(target.status.job as usize) else {
        return say("Character has an unknown job.");
    };
    let tree = GlobalConfigService::instance().get_job_skilltree(job);
    let basic = learned_level(&target.status, SkillEnum::NvBasic.id());
    let mut out = vec![format!("Player is using {} skill tree ({basic} basic points).", tree.name())];
    let Some(entry) = script_character_service::class_tree_entries(tree).find(|entry| SkillEnum::from_name(entry.name()).id() == skill_id) else {
        out.push("The player cannot use that skill.".into());
        return lines(out);
    };
    let unmet: Vec<String> = entry
        .requires()
        .into_iter()
        .flatten()
        .filter(|requirement| learned_level(&target.status, SkillEnum::from_name(requirement.name()).id()) < requirement.level())
        .map(|requirement| format!("Player requires level {} of skill {}.", requirement.level(), requirement.name()))
        .collect();
    if unmet.is_empty() {
        out.push("The player meets all the requirements for that skill.".into());
    } else {
        out.extend(unmet);
    }
    lines(out)
}

/// `@feelreset` forgets the Sun, Moon and Star places, `@hatereset` the hated monsters.
fn star_memory_reset(server: &Server, state: &mut ServerState, char_id: u32, hatred: bool) -> Option<Vec<String>> {
    let character = state.characters_mut().get_mut(&char_id)?;
    if character.status.job != JobName::StarGladiator.value() as u32 {
        return say("You can't use this command with this class.");
    }
    if hatred {
        character.game_systems.star_hates = [0; 3];
    } else {
        character.game_systems.star_places = Default::default();
    }
    character.refresh_script_context();
    if let Err(error) = server.script_world_service().persist(character) {
        return say(&error);
    }
    say(if hatred { "Reset 'Hatred' monsters." } else { "Reset 'Feeling' maps." })
}

fn stat_type(name: &str) -> Option<StatusTypes> {
    Some(match name {
        "str" => StatusTypes::Str,
        "agi" => StatusTypes::Agi,
        "vit" => StatusTypes::Vit,
        "int" => StatusTypes::Int,
        "dex" => StatusTypes::Dex,
        "luk" => StatusTypes::Luk,
        _ => return None,
    })
}

fn stat(server: &Server, state: &mut ServerState, char_id: u32, name: &str, args: &[&str]) -> Option<Vec<String>> {
    let Some(amount) = number(args, 0).filter(|amount| *amount != 0) else {
        return say(&format!("Please, enter a number (usage: @{name} <amount>)."));
    };
    let character = state.characters_mut().get_mut(&char_id)?;
    server.character_service().add_stat_without_cost(character, stat_type(name)?, amount.clamp(-1000, 1000) as i32);
    say("Stat changed.")
}

fn stat_all(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let character = state.characters_mut().get_mut(&char_id)?;
    let max = i32::from(GlobalConfigService::instance().config().game.max_stat_level);
    let amount = number(args, 0).map_or(max, |amount| amount.clamp(-1000, 1000) as i32);
    for name in ["str", "agi", "vit", "int", "dex", "luk"] {
        server.character_service().add_stat_without_cost(character, stat_type(name)?, amount);
    }
    say("All stats changed.")
}

/// Both amounts missing restores everything; a negative amount takes HP or SP away (never killing).
fn heal(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let character = state.characters_mut().get_mut(&char_id)?;
    if character.status.hp == 0 {
        return say("You cannot heal a dead character; use @alive.");
    }
    let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
    let (hp, sp) = if args.is_empty() {
        (snapshot.max_hp() as i64, snapshot.max_sp() as i64)
    } else {
        (
            i64::from(character.status.hp) + number(args, 0).unwrap_or(0),
            i64::from(character.status.sp) + number(args, 1).unwrap_or(0),
        )
    };
    let hp = hp.clamp(1, i64::from(snapshot.max_hp())) as u32;
    let sp = sp.clamp(0, i64::from(snapshot.max_sp())) as u32;
    server.character_service().update_hp_sp(character, hp, sp);
    say("HP, SP recovered.")
}

fn alive(server: &Server, state: &mut ServerState, char_id: u32) -> Option<Vec<String>> {
    let character = state.characters_mut().get_mut(&char_id)?;
    if character.status.hp > 0 {
        return say("You're not dead.");
    }
    let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
    server.character_service().update_hp_sp(character, snapshot.max_hp(), snapshot.max_sp());
    character.action = CharacterAction::Idle;
    let mut packet = RESURRECTION_EFFECT_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&character.char_id.to_le_bytes());
    packet.extend_from_slice(&0u16.to_le_bytes());
    server.send_area_raw(character, packet, true);
    say("You've been revived! It's a miracle!")
}

fn jump(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let (map, key) = {
        let character = state.get_character(char_id)?;
        (character.current_map_name().clone(), character.map_instance_key.clone())
    };
    if state.map_flags(&key).enabled(crate::server::model::map_flags::MapFlag::NoTeleport) {
        return say("You are not authorized to teleport on this map.");
    }
    let (x, y) = match (number(args, 0), number(args, 1)) {
        (Some(x), Some(y)) if x >= 0 && y >= 0 => (x.min(i64::from(u16::MAX)) as u16, y.min(i64::from(u16::MAX)) as u16),
        _ => RANDOM_CELL,
    };
    server.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, char_id);
    say("Jumped.")
}

fn recall(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    if args.is_empty() {
        return say("Please, enter a character name (usage: @recall <character name>).");
    }
    let wanted = args.join(" ");
    let target = online(state).find(|character| character.name.eq_ignore_ascii_case(&wanted)).map(|character| character.char_id);
    let Some(target) = target else {
        return say("Character not found.");
    };
    if target == char_id {
        return say("You are already where you are.");
    }
    let (map, x, y) = {
        let me = state.get_character(char_id)?;
        (me.current_map_name().clone(), me.x(), me.y())
    };
    if state.map_flags_for(&map, 0).enabled(crate::server::model::map_flags::MapFlag::NoWarpTo) {
        return say("You are not authorized to warp somebody to your current map.");
    }
    server.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, target);
    say(&format!("{wanted} recalled!"))
}

fn monster(state: &mut ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let Some(query) = args.first() else {
        return say("Please, enter a monster name or id (usage: @monster <name/id> {<amount>}).");
    };
    let Some(model) = GlobalConfigService::instance().find_mob(query) else {
        return say("Invalid monster ID or name.");
    };
    let amount = number(args, 1).map_or(1, |amount| amount.clamp(1, i64::from(MAX_SPAWN_AMOUNT)) as u16);
    let character = state.get_character(char_id)?;
    let instance = state.get_map_instance_from_character(character)?;
    let (x, y) = (i32::from(character.x()), i32::from(character.y()));
    instance.add_to_next_tick(MapEvent::ScriptSpawn(ScriptSpawn {
        mob_id: model.id,
        x: x - SPAWN_SPREAD,
        y: y - SPAWN_SPREAD,
        name: "--ja--".into(),
        amount,
        event: String::new(),
        event_npc: None,
        size: None,
        ai: None,
        owner_id: 0,
        guardian: None,
        bg_id: 0,
        max_hp: None,
        lifetime_ms: None,
        reserved_id: None,
        area_end: Some((x + SPAWN_SPREAD, y + SPAWN_SPREAD)),
    }));
    say(&format!("{} monster '{}' spawned.", amount, model.name_english))
}

fn kill_monsters(state: &mut ServerState, char_id: u32) -> Option<Vec<String>> {
    let instance = state.get_map_instance_from_character(state.get_character(char_id)?)?;
    instance.add_to_next_tick(MapEvent::AdminKillAllMobs(AdminKillAllMobs { char_id }));
    say("All monsters have been killed!")
}

fn mob_search(state: &ServerState, char_id: u32, args: &[&str]) -> Option<Vec<String>> {
    let Some(model) = GlobalConfigService::instance().find_mob(&args.join(" ")) else {
        return say("Please, enter a monster name or id (usage: @mobsearch <monster name or id>).");
    };
    let character = state.get_character(char_id)?;
    let instance = state.get_map_instance_from_character(character)?;
    let mut found: Vec<(u32, u16, u16)> = instance
        .state()
        .mobs()
        .values()
        .filter(|mob| i32::from(mob.mob_id) == model.id)
        .map(|mob| (mob.id, mob.x, mob.y))
        .collect();
    found.sort_unstable();
    let mut out = vec![format!("Mob Search... '{}' on {}:", model.name_english, character.current_map_name())];
    out.extend(found.iter().take(LISTED_NAMES).map(|(id, x, y)| format!("{id} [{x},{y}]")));
    out.push(format!("Number of monsters: {}", found.len()));
    lines(out)
}

fn mob_info(args: &[&str]) -> Option<Vec<String>> {
    let Some(model) = GlobalConfigService::instance().find_mob(&args.join(" ")) else {
        return say("Monster not found (usage: @mobinfo <name or id>).");
    };
    let mut out = vec![
        format!("Monster: '{}'/'{}' (id {})", model.name, model.name_english, model.id),
        format!("Level: {}  HP: {}  SP: {}", model.level, model.hp, model.sp),
        format!("Atk: {}~{}  Def: {}  Mdef: {}", model.atk1, model.atk2, model.def, model.mdef),
        format!(
            "Str: {}  Agi: {}  Vit: {}  Int: {}  Dex: {}  Luk: {}",
            model.str, model.agi, model.vit, model.int, model.dex, model.luk
        ),
        format!("Base exp: {}  Job exp: {}  MVP exp: {}", model.exp, model.job_exp, model.mvp_exp),
        format!("Race: {}  Size: {}  Element: {} {}", model.race, model.size, model.element, model.element_level),
    ];
    let drops: Vec<String> = model
        .drops
        .iter()
        .chain(model.mvp_drops.iter())
        .map(|drop| format!("{} ({:.2}%)", drop.item_name, f64::from(drop.rate) / 100.0))
        .collect();
    out.push(if drops.is_empty() { "Drops: none".to_string() } else { format!("Drops: {}", drops.join(", ")) });
    lines(out)
}

fn who_drops(args: &[&str]) -> Option<Vec<String>> {
    if args.is_empty() {
        return say("Please, enter an item name or id (usage: @whodrops <item name or id>).");
    }
    let configuration = GlobalConfigService::instance();
    let query = args.join(" ");
    let Some(item) = query.parse::<i32>().ok().and_then(|id| configuration.find_item(id)).or_else(|| configuration.find_item_by_name(&query)) else {
        return say("Item not found.");
    };
    let mut droppers: Vec<_> = configuration
        .mobs()
        .filter_map(|mob| {
            mob.drops
                .iter()
                .chain(mob.mvp_drops.iter())
                .find(|drop| drop.item_id == item.id)
                .map(|drop| (mob.name_english.clone(), drop.rate))
        })
        .collect();
    if droppers.is_empty() {
        return say(&format!("Item '{}' is not dropped by any monster.", item.name_english));
    }
    droppers.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut out = vec![format!("Item '{}' ({}) is dropped by {} monster(s):", item.name_english, item.id, droppers.len())];
    out.extend(droppers.iter().take(LISTED_DROPPERS).map(|(name, rate)| format!("  {name} ({:.2}%)", f64::from(*rate) / 100.0)));
    lines(out)
}

fn time_of_day(server: &Server, state: &mut ServerState, night: bool) -> Option<Vec<String>> {
    if server.day_night().is_night() == night {
        return say(if night { "It is already night." } else { "It is already day." });
    }
    let day_ms = server.configuration.battle.get("day_duration") as u64;
    server.day_night().set(night, crate::util::tick::get_tick(), day_ms);
    server.announce_day_night(state, night, true);
    None
}

fn who(state: &ServerState, args: &[&str], map: Option<String>) -> Option<Vec<String>> {
    let filter = args.join(" ").to_ascii_lowercase();
    let mut names: Vec<&str> = online(state)
        .filter(|character| map.as_ref().is_none_or(|map| character.current_map_name() == map))
        .filter(|character| character.name.to_ascii_lowercase().contains(&filter))
        .map(|character| character.name.as_str())
        .collect();
    names.sort_unstable();
    let total = names.len();
    let mut out: Vec<String> = names.into_iter().take(LISTED_NAMES).map(str::to_string).collect();
    out.push(format!("{total} user(s) found."));
    lines(out)
}

fn map_info(state: &ServerState, char_id: u32) -> Option<Vec<String>> {
    let character = state.get_character(char_id)?;
    let instance = state.get_map_instance_from_character(character)?;
    let players = online(state).filter(|other| other.map_instance_key == character.map_instance_key).count();
    let mobs = instance.state().mobs().values().filter(|mob| mob.is_present()).count();
    lines([
        format!("Map: {} (instance {})", character.current_map_name(), character.current_map_instance()),
        format!("Size: {} x {}", instance.x_size(), instance.y_size()),
        format!("Players: {players}  Monsters: {mobs}"),
    ])
}

fn look(server: &Server, state: &mut ServerState, char_id: u32, args: &[&str], look_type: LookType, max: u16) -> Option<Vec<String>> {
    let Some(value) = number(args, 0).filter(|value| (0..=i64::from(max)).contains(value)) else {
        return say(&format!("Please, enter a value between 0 and {max}."));
    };
    state.get_character(char_id)?;
    server.add_to_next_tick(GameEvent::CharacterUpdateLook(CharacterLook { char_id, look_type, look_value: value as u16 }));
    say("Appearance changed.")
}

fn save(server: &Server, state: &mut ServerState, char_id: u32) -> Option<Vec<String>> {
    if state.map_flags(&state.get_character(char_id)?.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoSave) {
        return say("You cannot save on this map.");
    }
    let character = state.characters_mut().get_mut(&char_id)?;
    let map = normalize_map(character.current_map_name());
    let (x, y) = (character.x(), character.y());
    let expected = (normalize_map(&character.save_map), character.save_x, character.save_y);
    if server.repository.character_set_save_point(character.char_id, character.account_id, &expected, &(map.clone(), x, y)).is_err() {
        return say("Your save point could not be changed.");
    }
    character.save_map = map.clone();
    character.save_x = x;
    character.save_y = y;
    say(&format!("Saved at {map} {x},{y}."))
}

fn commands(state: &ServerState, char_id: u32) -> Option<Vec<String>> {
    let account_id = state.get_character(char_id)?.account_id;
    let groups = state.permission_groups();
    let names = groups.usable_commands(state.group_id_of(account_id), CommandKind::At);
    lines([format!("Available commands ({}): {}", names.len(), names.join(" "))])
}

/// Brings every online member of the caller's own party or guild to the caller.
fn recall_group(server: &Server, state: &mut ServerState, char_id: u32, party: bool) -> Option<Vec<String>> {
    let group_of = |character: &Character| if party { character.game_systems.party_id } else { character.game_systems.guild_id };
    let group = group_of(state.get_character(char_id)?);
    if group == 0 {
        return say(if party { "You are not in a party." } else { "You are not in a guild." });
    }
    let (map, x, y) = {
        let me = state.get_character(char_id)?;
        (me.current_map_name().clone(), me.x(), me.y())
    };
    if state.map_flags_for(&map, 0).enabled(crate::server::model::map_flags::MapFlag::NoWarpTo) {
        return say("You are not authorized to warp somebody to your current map.");
    }
    let members: Vec<u32> = online(state).filter(|other| other.char_id != char_id && group_of(other) == group).map(|other| other.char_id).collect();
    for member in &members {
        server.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, *member);
    }
    say(&format!("{} member(s) recalled.", members.len()))
}

fn storage(server: &Server, state: &mut ServerState, char_id: u32, guild: bool) -> Option<Vec<String>> {
    let (function, arguments) = if guild {
        (script_sdk::Function::GuildOpenStorage, Vec::new())
    } else {
        (script_sdk::Function::OpenStorage, vec![crate::server::script::Value::Number(0)])
    };
    let tick = crate::util::tick::get_tick() as u64;
    let result = state.with_character_taken(char_id, |state, character| {
        server.script_world_service().call(server, state, character, function, &arguments, tick)
    })?;
    result.err().map(|error| vec![error])
}
