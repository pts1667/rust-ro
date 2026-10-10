use database::model::InventoryRecord;
use script_sdk::{Function, Value};

use crate::repository::{GameSystemRepository, SledRepository};
use crate::server::model::game_systems::GuildRecord;
use crate::server::model::events::game_event::CastleLifecycle;
use crate::server::service::castle_service::{CD_CURRENT_DEFENSE, CD_ENABLED_GUARDIAN00, CD_GUILD_ID};

const CASTLE: &str = "aldeg_cas01";
const EMPERIUM_ITEM: i32 = 714;

/// Creating a guild consumes an Emperium, so the master is given one first.
fn guild_of(repository: &SledRepository, master: u32) -> GuildRecord {
    let key = (master as i32).to_be_bytes();
    let mut inventory: Vec<InventoryRecord> = database::read(&repository.database.inventories, &key).unwrap().unwrap_or_default();
    inventory.push(InventoryRecord { id: 900, item_id: EMPERIUM_ITEM, amount: 1, is_identified: true, ..InventoryRecord::default() });
    repository.database.inventories.transaction(|tree| database::tx_write(tree, &key, &inventory)).unwrap();
    repository.create_guild(master, "Holders".into()).unwrap()
}

#[test]
fn a_script_guardian_takes_the_castle_owner_and_defense() {
    let (context, repository, character) = super::native_payment_tests::fixture(false, false);
    let guild = guild_of(&repository, character.char_id);
    repository.set_castle_value(CASTLE, CD_GUILD_ID, guild.id as i32).unwrap();
    repository.set_castle_value(CASTLE, CD_CURRENT_DEFENSE, 30).unwrap();

    let spawn = context.server.script_guardian(CASTLE, Some(2)).unwrap();
    assert_eq!((spawn.owner_guild, spawn.defense, spawn.slot, spawn.emperium), (guild.id, 30, Some(2), false));
    assert!(spawn.friendly_guilds.contains(&guild.id));
    assert_eq!(context.server.script_guardian(CASTLE, None).unwrap().slot, None);
}

#[test]
fn a_script_guardian_needs_a_castle_and_a_valid_index() {
    let (context, _, _) = super::native_payment_tests::fixture(false, false);
    assert!(context.server.script_guardian("prontera", None).is_err());
    assert!(context.server.script_guardian(CASTLE, Some(8)).is_err());
    assert!(context.server.script_guardian(CASTLE, Some(-1)).is_err());
}

#[test]
fn a_slain_guardian_clears_its_castle_slot_only() {
    let (context, repository, _) = super::native_payment_tests::fixture(false, false);
    for slot in 0..3 {
        repository.set_castle_value(CASTLE, CD_ENABLED_GUARDIAN00 + slot, 1).unwrap();
    }
    let mut state = context.server.state_mut();
    context.server.handle_castle_lifecycle(&mut state, CastleLifecycle::GuardianSlain { map: CASTLE.into(), slot: 1 }, 0);
    let enabled: Vec<i32> = (0..3).map(|slot| repository.castle_value(CASTLE, CD_ENABLED_GUARDIAN00 + slot).unwrap()).collect();
    assert_eq!(enabled, vec![1, 0, 1]);
}

#[test]
fn guild_skill_levels_are_found_by_id_or_name() {
    let (context, repository, character) = super::native_payment_tests::fixture(false, false);
    let guild = guild_of(&repository, character.char_id);
    let level = |skill: Value| context.server.castle_script_call(character.char_id, Function::GetGuildSkillLevel, &[Value::Number(guild.id as i32), skill]).unwrap();
    assert_eq!(level(Value::Number(10002)), Value::Number(0));
    assert_eq!(level(Value::String("GD_GUARDRESEARCH".into())), Value::Number(0));
    assert_eq!(level(Value::String("gd_unknown".into())), Value::Number(0));
    let missing = context.server.castle_script_call(character.char_id, Function::GetGuildSkillLevel, &[Value::Number(999_999), Value::String("GD_GUARDUP".into())]);
    assert_eq!(missing.unwrap(), Value::Number(-1));
}
