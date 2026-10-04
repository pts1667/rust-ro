use std::sync::Arc;

use database::model::{AccountRecord, CharacterRecord, InventoryRecord};
use sled::transaction::Transactional;

use super::ServerServiceTestContext;
use crate::repository::SledRepository;
use crate::server::model::action::DamageContribution;
use crate::server::model::events::game_event::CharacterKillMonster;
use crate::server::model::game_systems::PartyRecord;
use crate::server::service::script_experience_service::monster_experience_awards;

fn fixture() -> (ServerServiceTestContext, Arc<SledRepository>, Vec<u32>) {
    let (context, repository, leader) = super::native_payment_tests::fixture(false, true);
    context
        .server
        .state_mut()
        .runtime_map_flags
        .insert(("empty".into(), 0), Default::default());
    let ids: Vec<_> = (0..3).map(|offset| leader.char_id + offset).collect();
    let party = PartyRecord {
        id: 1,
        revision: 0,
        name: "Reward party".into(),
        leader_char_id: ids[0],
        members: ids.clone(),
        exp_share: true,
        item_pickup: true,
        item_share: true,
    };
    for (offset, id) in ids.iter().copied().enumerate() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.char_id = id;
        character.account_id = leader.account_id + offset as u32;
        character.name = format!("Reward player {offset}");
        character.map_instance_key = leader.map_instance_key.clone();
        character.x = 50;
        character.y = 50;
        character.loaded_from_client_side = true;
        character.status.hp = 30;
        character.status.sp = 5;
        character.game_systems.party_id = party.id;
        character.game_systems.party = Some(party.clone());
        (
            &repository.database.accounts,
            &repository.database.characters,
            &repository.database.inventories,
        )
            .transaction(|(accounts, characters, inventories)| {
                database::tx_write(accounts, &character.account_id.to_be_bytes(), &AccountRecord {
                    account_id: character.account_id,
                    username: character.name.clone(),
                    password: "secret".into(),
                })?;
                database::tx_write(characters, &id.to_be_bytes(), &CharacterRecord {
                    char_id: id as i32,
                    account_id: character.account_id as i32,
                    name: character.name.clone(),
                    class: character.status.job as i16,
                    base_level: character.status.base_level as i32,
                    job_level: character.status.job_level as i32,
                    status_point: character.status.status_point as i16,
                    hp: 30,
                    sp: 5,
                    max_hp: 40,
                    max_sp: 10,
                    inventory_slots: 100,
                    ..CharacterRecord::default()
                })?;
                database::tx_write(inventories, &id.to_be_bytes(), &Vec::<InventoryRecord>::new())
            })
            .unwrap();
        context.server.state_mut().insert_character(character);
    }
    repository
        .database
        .items
        .transaction(|tree| {
            for id in [501_i32, 1201] {
                database::tx_write(
                    tree,
                    &id.to_be_bytes(),
                    crate::server::service::global_config_service::GlobalConfigService::instance().get_item(id),
                )?;
            }
            Ok(())
        })
        .unwrap();
    (context, repository, ids)
}

fn kill(context: &ServerServiceTestContext, ids: &[u32]) -> CharacterKillMonster {
    CharacterKillMonster {
        attacker_id: ids[0],
        char_id: ids[0],
        mob_id: 1002,
        mob_x: 50,
        mob_y: 50,
        map_instance_key: context.server.state().get_character(ids[0]).unwrap().map_instance_key.clone(),
        mob_base_exp: 8,
        mob_job_exp: 4,
        mob_max_hp: 2,
        contributions: ids[..2]
            .iter()
            .map(|id| DamageContribution {
                actor_id: *id,
                owner_id: *id,
                damage: 1,
            })
            .collect(),
    }
}

#[test]
fn party_kill_combines_contributions_before_dividing_and_persists_every_recipient() {
    let (context, repository, ids) = fixture();
    let kill = kill(&context, &ids);
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        ids.iter().map(|id| (*id, (3, 1))).collect()
    );
    context
        .server
        .reward_monster_kill(context.server.state_mut().as_mut(), kill, 100)
        .unwrap();
    for id in ids {
        let state = context.server.state();
        let live = state.get_character(id).unwrap();
        let stored: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
        assert_eq!((live.status.base_exp, live.status.job_exp), (3, 1));
        assert_eq!((stored.base_exp, stored.job_exp), (3, 1));
        assert_eq!(stored.base_level as u32, live.status.base_level);
        assert_eq!((stored.hp as u32, stored.sp as u32), (live.status.hp, live.status.sp));
    }
}

#[test]
fn live_map_exp_rates_preserve_fractional_contributions_before_party_division() {
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, _, ids) = fixture();
    let mut kill = kill(&context, &ids);
    kill.mob_base_exp = 13;
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Bexp, true, &[800]).unwrap();
    flags.set(MapFlag::Jexp, true, &[200]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        ids.iter().map(|id| (*id, (43, 3))).collect()
    );
}

#[test]
fn no_base_exp_suppresses_every_party_base_award_and_keeps_job_exp_atomic() {
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, repository, ids) = fixture();
    let kill = kill(&context, &ids);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::NoBaseExp, true, &[]).unwrap();
    flags.set(MapFlag::Jexp, true, &[200]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    context
        .server
        .reward_monster_kill(context.server.state_mut().as_mut(), kill, 100)
        .unwrap();
    for id in ids {
        let live = context.server.state().get_character(id).unwrap();
        let stored: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
        assert_eq!((live.status.base_exp, live.status.job_exp), (0, 3));
        assert_eq!((stored.base_exp, stored.job_exp), (0, 3));
    }
}

#[test]
fn pvp_maps_do_not_award_player_experience_when_pvp_exp_is_disabled() {
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, repository, ids) = fixture();
    let kill = kill(&context, &ids);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Pvp, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    context
        .server
        .reward_monster_kill(context.server.state_mut().as_mut(), kill, 100)
        .unwrap();
    for id in ids {
        let live = context.server.state().get_character(id).unwrap();
        let stored: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
        assert_eq!((live.status.base_exp, live.status.job_exp), (0, 0));
        assert_eq!((stored.base_exp, stored.job_exp), (0, 0));
    }
}

#[test]
fn pet_exp_to_master_filters_only_pet_awards_and_keeps_the_shared_damage_denominator() {
    let (context, _, ids) = fixture();
    let mut kill = kill(&context, &ids);
    kill.contributions.push(DamageContribution {
        actor_id: crate::server::service::script_world_service::pet_world_id(10),
        owner_id: ids[0],
        damage: 1,
    });
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        ids.iter().map(|id| (*id, (2, 1))).collect()
    );
    let enabled =
        crate::server::service::script_experience_service::monster_experience_awards_with_pets(context.server.state(), &kill, 0, true, 100);
    assert_eq!(enabled, ids.iter().map(|id| (*id, (4, 2))).collect());
    let half =
        crate::server::service::script_experience_service::monster_experience_awards_with_pets(context.server.state(), &kill, 0, true, 50);
    assert_eq!(half, ids.iter().map(|id| (*id, (3, 1))).collect());
    kill.contributions.retain(|entry| entry.actor_id >= 1_000_000_000);
    assert!(monster_experience_awards(context.server.state(), &kill, 0).is_empty());
}

#[test]
fn no_drop_map_preserves_the_exact_equipment_record_and_rejects_before_floor_admission() {
    use crate::server::model::events::game_event::CharacterRemoveItem;
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, repository, ids) = fixture();
    let record = InventoryRecord {
        id: 211,
        item_id: 1201,
        amount: 1,
        unique_id: 779,
        refine: 7,
        is_identified: true,
        is_damaged: true,
        card0: 255,
        card1: 3843,
        card2: 1,
        card3: 2,
        ..Default::default()
    };
    (&repository.database.inventories, &repository.database.inventory_owners)
        .transaction(|(inventories, owners)| {
            database::tx_write(inventories, &ids[0].to_be_bytes(), &vec![record.clone()])?;
            database::tx_write(owners, &record.id.to_be_bytes(), &(ids[0] as i32))
        })
        .unwrap();
    let inventory = context
        .runtime()
        .block_on(crate::repository::InventoryRepository::character_inventory_fetch(
            repository.as_ref(),
            ids[0] as i32,
        ))
        .unwrap();
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&ids[0])
        .unwrap()
        .add_items(inventory);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::NoDrop, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    assert!(
        !context
            .server
            .server_service()
            .character_drop_item(&context.server, context.server.state_mut().as_mut(), CharacterRemoveItem {
                char_id: ids[0],
                index: 0,
                amount: 1,
                price: 0
            })
            .unwrap()
    );
    assert_eq!(self::inventory(&repository, ids[0]), vec![record]);
    let character = context.server.state().get_character(ids[0]).unwrap();
    assert_eq!(character.get_item_from_inventory(0).unwrap().unique_id, 779);
    assert!(
        context
            .server
            .state()
            .get_map_instance_from_character(character)
            .unwrap()
            .task_queue()
            .is_empty()
    );
}

#[test]
fn stale_party_recipient_rolls_back_every_award_and_preserves_live_progression() {
    let (context, repository, ids) = fixture();
    repository
        .database
        .characters
        .transaction(|tree| {
            let mut record: CharacterRecord = database::tx_required(tree, &ids[2].to_be_bytes())?;
            record.base_exp = 1;
            database::tx_write(tree, &ids[2].to_be_bytes(), &record)
        })
        .unwrap();
    let kill = kill(&context, &ids);
    assert!(
        context
            .server
            .reward_monster_kill(context.server.state_mut().as_mut(), kill, 100)
            .is_err()
    );
    for (index, id) in ids.into_iter().enumerate() {
        let state = context.server.state();
        let live = state.get_character(id).unwrap();
        let stored: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
        assert_eq!((live.status.base_exp, live.status.job_exp), (0, 0));
        assert_eq!((stored.base_exp, stored.job_exp), (i32::from(index == 2), 0));
    }
}

#[test]
fn dead_offline_and_other_instance_members_receive_no_party_rewards() {
    let (context, _, ids) = fixture();
    let kill = kill(&context, &ids);
    context.server.state_mut().characters_mut().get_mut(&ids[2]).unwrap().status.hp = 0;
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        [(ids[0], (5, 2)), (ids[1], (5, 2))].into_iter().collect()
    );
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&ids[1])
        .unwrap()
        .loaded_from_client_side = false;
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        [(ids[0], (10, 4))].into_iter().collect()
    );
    context.server.state_mut().characters_mut().get_mut(&ids[2]).unwrap().status.hp = 30;
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&ids[2])
        .unwrap()
        .map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 1);
    assert_eq!(
        monster_experience_awards(context.server.state(), &kill, 0),
        [(ids[0], (10, 4))].into_iter().collect()
    );
}

fn capacity(repository: &SledRepository, ids: &[u32], slots: i16) {
    repository
        .database
        .characters
        .transaction(|tree| {
            for id in ids {
                let mut record: CharacterRecord = database::tx_required(tree, &id.to_be_bytes())?;
                record.inventory_slots = slots;
                database::tx_write(tree, &id.to_be_bytes(), &record)?;
            }
            Ok(())
        })
        .unwrap();
}

fn inventory(repository: &SledRepository, id: u32) -> Vec<InventoryRecord> {
    database::required(&repository.database.inventories, &id.to_be_bytes()).unwrap()
}

fn floor_item(context: &ServerServiceTestContext, picker: u32, amount: u16) -> models::item::DroppedItem {
    use crate::server::model::map_item::ToMapItem;
    let item = models::item::DroppedItem {
        map_item_id: 1000,
        item_id: 501,
        location: movement::position::Position { x: 50, y: 50, dir: 0 },
        sub_location: movement::position::Position { x: 3, y: 3, dir: 0 },
        owner_id: Some(picker + 1),
        dropped_at: crate::util::tick::get_tick(),
        amount,
        is_identified: true,
        attributes: Default::default(),
        player_dropped: false,
    };
    let instance = context
        .server
        .state()
        .get_map_instance_from_character(context.server.state().get_character(picker).unwrap())
        .unwrap();
    instance.state_mut().insert_dropped_item(item);
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&picker)
        .unwrap()
        .map_view
        .insert(item.to_map_item());
    item
}

fn pick_up(context: &ServerServiceTestContext, picker: u32, item: u32) -> Result<bool, String> {
    let mut state = context.server.state_mut();
    let mut character = state.characters_mut().remove(&picker).unwrap();
    let instance = state.get_map_instance_from_character(&character).unwrap();
    let result =
        context
            .server
            .server_service()
            .character_pickup_item(&context.server, state.as_mut(), &mut character, item, instance.as_ref());
    state.insert_character(character);
    result
}

#[test]
fn party_loot_skips_full_members_and_commits_to_one_eligible_recipient() {
    let (context, repository, ids) = fixture();
    capacity(&repository, &ids[..2], 0);
    let item = floor_item(&context, ids[0], 2);
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(true));
    assert!(inventory(&repository, ids[0]).is_empty());
    assert!(inventory(&repository, ids[1]).is_empty());
    let saved = inventory(&repository, ids[2]);
    assert_eq!((saved.len(), saved[0].item_id, saved[0].amount), (1, 501, 2));
    assert!(context.server.state().contains_locked_map_item(item.map_item_id));
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(false));
    assert_eq!(inventory(&repository, ids[2])[0].amount, 2);
}

#[test]
fn failed_party_loot_keeps_the_floor_item_available_for_a_later_pickup() {
    let (context, repository, ids) = fixture();
    capacity(&repository, &ids, 0);
    let item = floor_item(&context, ids[0], 2);
    assert!(pick_up(&context, ids[0], item.map_item_id).is_err());
    assert!(!context.server.state().contains_locked_map_item(item.map_item_id));
    let instance = context
        .server
        .state()
        .get_map_instance_from_character(context.server.state().get_character(ids[0]).unwrap())
        .unwrap();
    assert!(instance.state().get_dropped_item(item.map_item_id).is_some());
    assert!(instance.task_queue().is_empty());
    for id in &ids {
        assert!(inventory(&repository, *id).is_empty());
    }
    capacity(&repository, &ids[2..], 1);
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(true));
    assert_eq!(inventory(&repository, ids[2])[0].amount, 2);
}

#[test]
fn overweight_party_loot_rolls_back_every_candidate() {
    let (context, repository, ids) = fixture();
    let item = floor_item(&context, ids[0], 30_000);
    assert!(pick_up(&context, ids[0], item.map_item_id).is_err());
    assert!(!context.server.state().contains_locked_map_item(item.map_item_id));
    for id in ids {
        assert!(inventory(&repository, id).is_empty());
    }
}

#[test]
fn party_pickup_option_controls_access_to_a_members_protected_drop() {
    let (context, repository, ids) = fixture();
    let item = floor_item(&context, ids[0], 2);
    {
        let mut state = context.server.state_mut();
        let party = state
            .characters_mut()
            .get_mut(&ids[0])
            .unwrap()
            .game_systems
            .party
            .as_mut()
            .unwrap();
        party.item_pickup = false;
        party.item_share = false;
    }
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(false));
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&ids[0])
        .unwrap()
        .game_systems
        .party
        .as_mut()
        .unwrap()
        .item_pickup = true;
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(true));
    assert_eq!(inventory(&repository, ids[0])[0].amount, 2);
}

#[test]
fn real_player_drop_and_party_pickup_preserve_the_equipment_instance() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;

    use crate::repository::InventoryRepository;
    use crate::server::model::events::map_event::CharacterDropItems;
    use crate::server::model::map_item::ToMapItem;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::service::map_instance_service::MapInstanceService;
    use crate::server::service::mob_service::MobService;
    let (context, repository, ids) = fixture();
    let original = InventoryRecord {
        id: 211,
        item_id: 1201,
        amount: 1,
        unique_id: 779,
        refine: 7,
        is_identified: true,
        is_damaged: true,
        card0: 255,
        card1: 3843,
        card2: 1,
        card3: 2,
        ..InventoryRecord::default()
    };
    (&repository.database.inventories, &repository.database.inventory_owners)
        .transaction(|(inventories, owners)| {
            database::tx_write(inventories, &ids[0].to_be_bytes(), &vec![original.clone()])?;
            database::tx_write(owners, &original.id.to_be_bytes(), &(ids[0] as i32))
        })
        .unwrap();
    let mut source = context.server.state_mut().characters_mut().remove(&ids[0]).unwrap();
    source.add_items(
        context
            .runtime()
            .block_on(repository.character_inventory_fetch(ids[0] as i32))
            .unwrap(),
    );
    let instance = context.server.state().get_map_instance_from_character(&source).unwrap();
    instance
        .state_mut()
        .cells_mut()
        .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    let (removed, removal) = context
        .server
        .inventory_service()
        .remove_single_item_from_inventory(context.runtime(), 0, &mut source, false)
        .unwrap();
    assert!(inventory(&repository, ids[0]).is_empty());
    let map_service = MapInstanceService::new(
        context.client_notification_sender.clone(),
        GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
        context.server_task_queue.clone(),
    );
    map_service.character_drop_items_and_send_packet(instance.state_mut().as_mut(), CharacterDropItems {
        owner_id: ids[0],
        char_x: source.x,
        char_y: source.y,
        item_removal_info: vec![(removed, removal)],
    });
    let item = {
        let state = instance.state();
        *state.map_items().keys().find_map(|id| state.get_dropped_item(*id)).unwrap()
    };
    assert!(item.player_dropped);
    assert_eq!(item.attributes.unique_id, original.unique_id);
    source.map_view.insert(item.to_map_item());
    context.server.state_mut().insert_character(source);
    assert_eq!(pick_up(&context, ids[0], item.map_item_id), Ok(true));
    let saved: Vec<_> = ids.into_iter().flat_map(|id| inventory(&repository, id)).collect();
    assert_eq!(saved.len(), 1);
    assert_eq!((saved[0].unique_id, saved[0].refine, saved[0].is_damaged), (779, 7, true));
    assert_eq!([saved[0].card0, saved[0].card1, saved[0].card2, saved[0].card3], [
        255, 3843, 1, 2
    ]);
}
