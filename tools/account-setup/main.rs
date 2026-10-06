use std::collections::{BTreeMap, HashMap};
use std::{env, fs};

use configuration::configuration::Config;
use database::Database;
use database::model::{AccountRecord, CharacterInventory, IpBanRecord, CharacterRecord, CharacterSkills, InventoryRecord, SeedData};
use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use rand::{Rng, thread_rng};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Character {
    job: String,
    hair: i16,
    hair_color: i16,
    clothes_color: i16,
    max_hp: i32,
    max_sp: i32,
    agi: i16,
    dex: i16,
    str: i16,
    vit: i16,
    int: i16,
    luk: i16,
    base_level: i32,
    job_level: i32,
    equipments: HashMap<String, Equipment>,
    save_y: i16,
    save_x: i16,
    save_map: String,
    last_y: i16,
    last_x: i16,
    last_map: String,
    status_point: i16,
    skill_point: i16,
    skills: Vec<Skill>,
}

#[derive(Deserialize)]
struct Equipment {
    #[serde(rename = "itemId")]
    item_id: i32,
}

#[derive(Deserialize)]
struct Skill {
    name: String,
    lvl: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("").map_err(std::io::Error::other)?;
    let database = Database::open(&config.database.path)?;
    let args: Vec<String> = env::args().skip(1).collect();
    let replace = args.iter().any(|arg| arg == "--replace");
    let args: Vec<_> = args.iter().filter(|arg| arg.as_str() != "--replace").collect();
    match args.first().map(|arg| arg.as_str()).unwrap_or("seed") {
        "seed" => {
            let path = args
                .get(1)
                .map(|arg| arg.as_str())
                .or(config.database.seed_path.as_deref())
                .ok_or("No seed file configured")?;
            let seed: SeedData = serde_json::from_slice(&fs::read(path)?)?;
            database.seed(&seed, replace)?;
            println!(
                "Seeded {} accounts and {} characters in {}",
                seed.accounts.len(),
                seed.characters.len(),
                config.database.path
            );
        }
        "create" => {
            let username = args.get(1).ok_or("Usage: account-setup create USERNAME [--sex M|F] [--group ID] (set ACCOUNT_PASSWORD)")?;
            let password = env::var("ACCOUNT_PASSWORD").map_err(|_| "Set ACCOUNT_PASSWORD to the new account password")?;
            let sex = option_value(&args, "--sex").unwrap_or("M").to_uppercase();
            if !matches!(sex.as_str(), "M" | "F") {
                return Err("--sex must be M or F".into());
            }
            let group_id = option_value(&args, "--group").map(str::parse).transpose()?.unwrap_or(0);
            let account = AccountRecord { sex, group_id, ..AccountRecord::new(0, (*username).clone(), password) };
            let id = database.create_account_record(account)?;
            println!("Created account {id}");
        }
        "group" => {
            let (username, group) = (args.get(1).ok_or(GROUP_USAGE)?, args.get(2).ok_or(GROUP_USAGE)?);
            let group_id: u32 = group.parse()?;
            update_account(&database, username, |account| account.group_id = group_id)?;
            println!("Account {username} is now in group {group_id}");
        }
        "ban" => {
            let (username, until) = (args.get(1).ok_or(BAN_USAGE)?, args.get(2).ok_or(BAN_USAGE)?);
            let unban_time: i64 = until.parse()?;
            update_account(&database, username, |account| account.unban_time = unban_time)?;
            println!("Account {username} is banned until {unban_time} (0 lifts the ban)");
        }
        "ipban" => {
            let (pattern, minutes) = (args.get(1).ok_or(IPBAN_USAGE)?, args.get(2).ok_or(IPBAN_USAGE)?);
            let minutes: i64 = minutes.parse()?;
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;
            let ban = IpBanRecord { list: (*pattern).clone(), begin: now, release: now + minutes * 60, reason: "account-setup".into() };
            database.ip_bans.insert(pattern.as_bytes(), serde_json::to_vec(&ban)?)?;
            println!("IP {pattern} is banned for {minutes} minutes");
        }
        "unipban" => {
            let pattern = args.get(1).ok_or("Usage: account-setup unipban PATTERN")?;
            database.ip_bans.remove(pattern.as_bytes())?;
            println!("IP ban {pattern} removed");
        }
        "characters" => {
            let path = args.get(1).map(|arg| arg.as_str()).unwrap_or("tools/account-setup/characters.json");
            let presets: Vec<Character> = serde_json::from_slice(&fs::read(path)?)?;
            let catalog: Value = serde_json::from_slice(&fs::read(&config.database.items_path)?)?;
            let items = catalog["items"].as_array().ok_or("Invalid item catalog")?;
            let account_id = 2_000_002;
            let mut seed = SeedData {
                accounts: vec![AccountRecord {
                    account_id,
                    username: "test".into(),
                    password: env::var("ACCOUNT_PASSWORD").unwrap_or_else(|_| "qwertz".into()),
                    ..AccountRecord::default()
                }],
                ..SeedData::default()
            };
            let mut rng = thread_rng();
            for (index, preset) in presets.into_iter().enumerate() {
                let char_id = 160_100 + i32::try_from(index)?;
                let equipment_id = |slot: &str| -> i16 { preset.equipments.get(slot).map_or(0, |equipment| equipment.item_id as i16) };
                let character = CharacterRecord {
                    char_id,
                    account_id: account_id as i32,
                    char_num: i16::try_from(index)?,
                    name: format!("{}{}", generate_player_name(), index),
                    class: JobName::from_string(&preset.job).value() as i16,
                    zeny: 11_127_525,
                    hair: preset.hair,
                    hair_color: preset.hair_color,
                    clothes_color: preset.clothes_color,
                    hp: preset.max_hp,
                    max_hp: preset.max_hp,
                    sp: preset.max_sp,
                    max_sp: preset.max_sp,
                    agi: preset.agi,
                    dex: preset.dex,
                    str: preset.str,
                    vit: preset.vit,
                    int: preset.int,
                    luk: preset.luk,
                    base_level: preset.base_level,
                    job_level: preset.job_level,
                    inventory_slots: 100,
                    last_map: preset.last_map.clone(),
                    last_x: preset.last_x,
                    last_y: preset.last_y,
                    save_map: preset.save_map.clone(),
                    save_x: preset.save_x,
                    save_y: preset.save_y,
                    status_point: preset.status_point,
                    skill_point: preset.skill_point,
                    sex: "M".into(),
                    body: equipment_id("body"),
                    weapon: equipment_id("weapon"),
                    shield: equipment_id("shield"),
                    head_top: equipment_id("head_top"),
                    head_mid: equipment_id("head_mid"),
                    head_bottom: equipment_id("head_low"),
                    ..CharacterRecord::default()
                };
                let mut inventory = Vec::new();
                for equipment in preset.equipments.values() {
                    let item = items
                        .iter()
                        .find(|item| item["id"].as_i64() == Some(equipment.item_id as i64))
                        .ok_or_else(|| format!("Unknown equipment {}", equipment.item_id))?;
                    let location = item["location"].as_u64().ok_or("Invalid equipment location")?;
                    inventory.push(InventoryRecord {
                        item_id: equipment.item_id,
                        amount: 1,
                        equip: i32::try_from(location)?,
                        is_identified: true,
                        unique_id: rng.gen_range(1..i64::MAX),
                        ..InventoryRecord::default()
                    });
                }
                let skills: BTreeMap<u32, u8> = preset
                    .skills
                    .iter()
                    .map(|skill| (SkillEnum::from_name(&skill.name).id(), skill.lvl))
                    .collect();
                seed.characters.push(character);
                seed.inventories.push(CharacterInventory { char_id, items: inventory });
                seed.skills.push(CharacterSkills { char_id, skills });
            }
            database.seed(&seed, replace)?;
            println!("Seeded {} preset characters for account {account_id}", seed.characters.len());
        }
        _ => {
            return Err(
                "Usage: account-setup [seed [FILE] | create USERNAME | group USERNAME ID | ban USERNAME UNIX_TIME | ipban PATTERN MINUTES | unipban PATTERN | characters [FILE]] [--replace]"
                    .into(),
            );
        }
    }
    Ok(())
}

const GROUP_USAGE: &str = "Usage: account-setup group USERNAME GROUP_ID";
const BAN_USAGE: &str = "Usage: account-setup ban USERNAME UNIX_TIME";
const IPBAN_USAGE: &str = "Usage: account-setup ipban PATTERN MINUTES (PATTERN like 1.2.3.4 or 1.2.3.*)";

fn option_value<'a>(args: &'a [&String], name: &str) -> Option<&'a str> {
    args.iter().position(|arg| arg.as_str() == name).and_then(|index| args.get(index + 1)).map(|value| value.as_str())
}

fn update_account(database: &Database, username: &str, update: impl FnOnce(&mut AccountRecord)) -> Result<(), Box<dyn std::error::Error>> {
    let id: u32 = database::read(&database.account_names, username.as_bytes())?.ok_or("No such account")?;
    let mut account: AccountRecord = database::required(&database.accounts, &id.to_be_bytes())?;
    update(&mut account);
    database.accounts.insert(id.to_be_bytes(), serde_json::to_vec(&account)?)?;
    Ok(())
}

fn generate_player_name() -> String {
    let syllables = [
        "ar", "el", "ka", "an", "ra", "na", "to", "li", "ma", "in", "er", "la", "do", "sa", "vi", "no", "mi", "al", "es", "ro",
    ];
    let mut rng = thread_rng();
    let mut name = String::new();
    for _ in 0..rng.gen_range(2..=4) {
        name.push_str(syllables[rng.gen_range(0..syllables.len())]);
    }
    name.truncate(name.len().min(10));
    name
}
