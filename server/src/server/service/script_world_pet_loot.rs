use std::sync::atomic::Ordering;

use database::model::InventoryRecord;
use models::item::DroppedItem;
use script_sdk::{Function, Value};

use super::{ScriptWorldService, install_state, pet_world_id, plan_persistent_effects};
use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemGrant};
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, PetLootClaimResult, PetLootDropResult, ScriptWorld};
use crate::server::model::events::map_event::{MapEvent, PetLootClaimRequest, PetLootFinalize, PetLootDropRequest, PetLootDropFinalize};
use crate::server::model::game_systems::{CharacterGameSystems, PetLootCargo, PetLootDropReceipt, PetLootDropReservation,
    PetLootRuntime, PetSupportRuntime, PendingPetLootClaim, ScriptWorldRequest, PetRequest};
use crate::server::model::map_flags::MapFlag;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub(crate) fn loot_capacity(args: &[Value]) -> Result<u8, String> {
    Ok(args.first().ok_or("Pet loot capacity is missing")?.number_value()?.clamp(1, 30) as u8)
}

pub(crate) fn cargo_grants(items: &[InventoryRecord]) -> Result<Vec<ScriptItemGrant>, String> {
    if items.len() > 30 || items.iter().any(|item| item.amount <= 0 || item.equip != 0) { return Err("Invalid pet cargo".into()); }
    Ok(items.iter().map(|item| ScriptItemGrant { item_id: item.item_id, amount: item.amount,
        identified: item.is_identified, refine: item.refine, cards: [item.card0, item.card1, item.card2, item.card3],
        unique_id: Some(item.unique_id), damaged: item.is_damaged }).collect())
}

pub(crate) fn plan_reconfiguration(systems: &mut CharacterGameSystems, args: &[Value]) -> Result<(u8, Vec<ScriptItemGrant>), String> {
    systems.pet.as_ref().filter(|pet| !pet.incubating).ok_or("Pet looting requires an active pet")?;
    if systems.pending_pet_loot.is_some() || systems.pet_loot.as_ref().is_some_and(|cargo| cargo.pending_drop.is_some()) {
        return Err("Pet cargo is currently being transferred".into());
    }
    let capacity = loot_capacity(args)?;
    let grants = systems.pet_loot.as_ref().map(|cargo| cargo_grants(&cargo.items)).transpose()?.unwrap_or_default();
    systems.pet_loot = None;
    Ok((capacity, grants))
}

pub(crate) fn configure_runtime(character: &mut Character, capacity: u8, returned_cargo: bool, now: u64) {
    let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating) else { return; };
    let pet_id = pet.id;
    let support = character.game_systems.pet_support.get_or_insert_with(|| PetSupportRuntime { pet_id, ..Default::default() });
    if support.pet_id != pet_id { *support = PetSupportRuntime { pet_id, ..Default::default() }; }
    let next_at = now.saturating_add(if returned_cargo { 10000 } else { 0 });
    support.loot = Some(PetLootRuntime { capacity, next_at, return_requested: false, target: None, claim_queued: false });
    if returned_cargo { support.can_act_at = support.can_act_at.max(next_at); }
}

fn receipt_matches(receipt: &PetLootDropReceipt, result: &PetLootDropResult) -> bool {
    receipt.claim_id == result.claim_id && receipt.map == *result.map_key.map_name() && receipt.map_instance == result.map_key.map_instance()
}

fn claim_matches(claim: &PendingPetLootClaim, result: &PetLootClaimResult) -> bool {
    claim.claim_id == result.claim_id && claim.pet_id == result.pet_id && claim.target_id == result.target_id && claim.map_key == result.map_key
}

fn floor_record(item: &DroppedItem) -> Result<InventoryRecord, String> {
    let amount = i16::try_from(item.amount).map_err(|_| "Pet floor item amount is out of range")?;
    if amount <= 0 { return Err("Pet floor item amount must be positive".into()); }
    Ok(InventoryRecord { item_id: item.item_id, unique_id: item.attributes.unique_id, amount, refine: item.attributes.refine,
        is_identified: item.is_identified, is_damaged: item.attributes.damaged, card0: item.attributes.cards[0],
        card1: item.attributes.cards[1], card2: item.attributes.cards[2], card3: item.attributes.cards[3], ..Default::default() })
}

impl ScriptWorldService {
    pub(crate) fn call_pet_loot(&self, server: &Server, character: &mut Character, args: &[Value], now: u64) -> Result<Value, String> {
        let plan = plan_persistent_effects(character, &[(Function::PetLoot, args.to_vec())], now)?;
        let change = ScriptInventoryTransaction { char_id: character.char_id, account_id: character.account_id,
            consumption: None, exact_removals: vec![], removals: vec![], identifications: vec![], grants: plan.additional_grants.clone(),
            variables: vec![], zeny: None, hp: None, sp: None, max_weight: server.character_service().max_weight(character),
            max_slots: usize::from(self.configuration.config().game.max_inventory), world: Some(plan.clone()), reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![] };
        let committed = self.repository.script_inventory_transaction(&change).map_err(|error| error.to_string())?;
        if !change.grants.is_empty() {
            server.item_service().install_inventory(server, character, committed.inventory, None);
        }
        self.apply_committed_effects(server, character, &plan, committed.world.ok_or("Committed pet cargo result is missing")?, now)?;
        Ok(Value::default())
    }

    pub(crate) fn pet_loot_target(&self, state: &mut ServerState, character: &mut Character, target_id: u32, now: u64) -> Result<(), String> {
        let Some(support) = character.game_systems.pet_support.as_mut() else { return Ok(()); };
        let Some(loot) = support.loot.as_mut() else { return Ok(()); };
        loot.claim_queued = false;
        let capacity = usize::from(loot.capacity);
        if character.game_systems.pending_pet_loot.is_some() || now < loot.next_at || loot.return_requested { return Ok(()); }
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating && pet.intimacy > 0) else { return Ok(()); };
        if !character.loaded_from_client_side || character.status.hp == 0 || support.casting.is_some()
            || support.pet_id != pet.id || state.contains_locked_map_item(target_id) { return Ok(()); }
        if character.game_systems.pet_loot.as_ref().is_some_and(|cargo| cargo.pet_id != pet.id || cargo.items.len() >= capacity || cargo.pending_drop.is_some()) {
            return Ok(());
        }
        let Some(map) = state.get_map_instance_from_character(character) else { return Ok(()); };
        let Some(item) = map.state().get_dropped_item(target_id).copied() else { return Ok(()); };
        let (x, y) = character.game_systems.rendered_companions.get(&pet_world_id(pet.id)).map_or((character.x, character.y), |position| (position.x, position.y));
        if item.owner_id.is_some_and(|owner| owner != character.char_id) || x.abs_diff(item.x()).max(y.abs_diff(item.y())) > 1 {
            return Ok(());
        }
        let claim_id = self.next_pet_loot_id.fetch_add(1, Ordering::Relaxed);
        let expires_at = now.saturating_add(5000);
        let pet_id = pet.id;
        character.game_systems.pending_pet_loot = Some(PendingPetLootClaim { claim_id, pet_id, target_id,
            map_key: character.map_instance_key.clone(), expires_at, committed: false });
        state.insert_locked_map_item(target_id);
        map.add_to_next_tick(MapEvent::ClaimPetLoot(PetLootClaimRequest { claim_id, char_id: character.char_id, pet_id, target_id, x, y, expires_at }));
        Ok(())
    }

    pub fn cancel_pet_loot_in_state(&self, state: &ServerState, character: &mut Character) {
        if let Some(pending) = character.game_systems.pending_pet_loot.take() {
            if let Some(map) = state.get_map_instance(pending.map_key.map_name(), pending.map_key.map_instance()) {
                map.add_to_next_tick(MapEvent::FinalizePetLoot(PetLootFinalize { claim_id: pending.claim_id, target_id: pending.target_id, commit: false }));
            }
        }
        if let Some(loot) = character.game_systems.pet_support.as_mut().and_then(|support| support.loot.as_mut()) {
            loot.claim_queued = false;
            loot.target = None;
        }
    }

    pub fn complete_pet_loot(&self, server: &Server, state: &mut ServerState, result: PetLootClaimResult, now: u64) -> Result<(), String> {
        let map = state.get_map_instance(result.map_key.map_name(), result.map_key.map_instance());
        let mut commit = false;
        let outcome = if let Some(mut character) = state.characters_mut().remove(&result.char_id) {
            let outcome = self.commit_pet_loot(server, state, &mut character, &result, now);
            if matches!(outcome, Ok(true)) { commit = true; }
            state.characters_mut().insert(result.char_id, character);
            outcome.map(|_| ())
        } else { Ok(()) };
        if let Some(map) = map {
            map.add_to_next_tick(MapEvent::FinalizePetLoot(PetLootFinalize { claim_id: result.claim_id, target_id: result.target_id, commit }));
        } else { state.remove_locked_map_item(result.target_id); }
        outcome
    }

    fn commit_pet_loot(&self, server: &Server, state: &ServerState, character: &mut Character, result: &PetLootClaimResult, now: u64) -> Result<bool, String> {
        if !character.game_systems.pending_pet_loot.as_ref().is_some_and(|claim| claim_matches(claim, result)) {
            return Ok(character.game_systems.completed_pet_loot.as_ref().is_some_and(|claim| claim.committed && claim_matches(claim, result)));
        }
        let mut pending = character.game_systems.pending_pet_loot.take().unwrap();
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating && pet.intimacy > 0 && pet.id == result.pet_id) else { return Ok(false); };
        let Some(capacity) = character.game_systems.pet_support.as_ref().filter(|support| support.pet_id == pet.id)
            .and_then(|support| support.loot.as_ref()).map(|loot| usize::from(loot.capacity)) else { return Ok(false); };
        if character.map_instance_key != result.map_key || now >= pending.expires_at || character.status.hp == 0 || !character.loaded_from_client_side { return Ok(false); }
        let Some(item) = result.item.as_ref().filter(|item| item.map_item_id == result.target_id && item.owner_id.is_none_or(|owner| owner == character.char_id)) else { return Ok(false); };
        let record = floor_record(item)?;
        let definition = self.configuration.find_item(record.item_id).ok_or("Pet floor item has no definition")?;
        if (definition.item_type.is_stackable() && record.unique_id != 0)
            || (!definition.item_type.is_stackable() && record.unique_id != 0 && record.amount != 1) {
            return Err("Invalid item identity in pet loot".into());
        }
        let mut staged = character.game_systems.clone();
        let cargo = staged.pet_loot.get_or_insert_with(|| PetLootCargo { pet_id: result.pet_id, ..Default::default() });
        if cargo.pet_id != result.pet_id || cargo.pending_drop.is_some() || cargo.items.len() >= capacity { return Ok(false); }
        if record.unique_id != 0 && cargo.items.iter().any(|item| item.unique_id == record.unique_id) { return Err("Duplicate equipment in pet cargo".into()); }
        cargo.items.push(record);
        let saved = self.repository.save_character_game_systems(character.char_id, &staged).map_err(|error| error.to_string())?;
        install_state(character, saved);
        pending.committed = true;
        character.game_systems.completed_pet_loot = Some(pending);
        if let Some(command) = character.game_systems.companion_commands.get_mut(&pet_world_id(result.pet_id)) { command.destination = None; }
        if let Some(loot) = character.game_systems.pet_support.as_mut().and_then(|support| support.loot.as_mut()) {
            loot.target = None;
            loot.claim_queued = false;
            if character.game_systems.pet_loot.as_ref().is_some_and(|cargo| cargo.items.len() >= capacity) { loot.return_requested = true; }
        }
        if character.game_systems.pet_loot.as_ref().is_some_and(|cargo| cargo.items.len() >= capacity) {
            if let Err(error) = self.return_pet_loot_in_state(server, state, character, now) { warn!("Pet cargo return deferred: {}", error); }
        }
        Ok(true)
    }

    pub fn return_pet_loot_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<bool, String> {
        self.return_pet_cargo(server, state, character, now, true)
    }

    pub(crate) fn drop_pet_loot_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<bool, String> {
        self.return_pet_cargo(server, state, character, now, false)
    }

    fn return_pet_cargo(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64, return_to_inventory: bool) -> Result<bool, String> {
        self.cancel_pet_loot_in_state(state, character);
        let Some(cargo) = character.game_systems.pet_loot.as_ref().filter(|cargo| !cargo.items.is_empty()) else { return Ok(true); };
        if cargo.pending_drop.is_some() { return Ok(false); }
        character.game_systems.pet_loot_retry_at = now.saturating_add(10000);
        if let Some(support) = character.game_systems.pet_support.as_mut() {
            support.can_act_at = support.can_act_at.max(now.saturating_add(10000));
            if let Some(loot) = &mut support.loot {
                loot.return_requested = true;
                loot.next_at = now.saturating_add(10000);
            }
        }
        if return_to_inventory {
            let returned = self.repository.return_pet_loot(character.char_id, character.game_systems.revision,
                server.character_service().max_weight(character), usize::from(self.configuration.config().game.max_inventory)).map_err(|error| error.to_string())?;
            let changed = returned.returned > 0;
            install_state(character, returned.systems);
            if changed {
                server.item_service().install_inventory(server, character, returned.inventory, None);
                character.refresh_script_context();
            }
        }
        if character.game_systems.pet_loot.is_none() {
            if let Some(loot) = character.game_systems.pet_support.as_mut().and_then(|support| support.loot.as_mut()) { loot.return_requested = false; }
            return Ok(true);
        }
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoDrop) { return Err("Pet cargo cannot be dropped on this map".into()); }
        let map = state.get_map_instance_from_character(character).ok_or("Pet cargo return map is unavailable")?;
        let cargo = character.game_systems.pet_loot.as_ref().unwrap();
        let (x, y) = character.game_systems.rendered_companions.get(&pet_world_id(cargo.pet_id)).map_or((character.x, character.y), |position| (position.x, position.y));
        let claim_id = self.next_pet_loot_id.fetch_add(1, Ordering::Relaxed);
        let receipt = PetLootDropReceipt { claim_id, map: character.current_map_name().clone(), map_instance: character.current_map_instance() };
        let mut staged = character.game_systems.clone();
        staged.pet_loot.as_mut().unwrap().pending_drop = Some(PetLootDropReservation { receipt, x, y, expires_at: now.saturating_add(5500) });
        let saved = self.repository.save_character_game_systems(character.char_id, &staged).map_err(|error| error.to_string())?;
        install_state(character, saved);
        map.add_to_delayed_tick(MapEvent::PreparePetLootDrop(PetLootDropRequest { claim_id, char_id: character.char_id, x, y,
            items: character.game_systems.pet_loot.as_ref().unwrap().items.clone() }), 500);
        Ok(false)
    }

    pub fn complete_pet_loot_drop(&self, _server: &Server, state: &mut ServerState, result: PetLootDropResult, now: u64) -> Result<(), String> {
        let map = state.get_map_instance(result.map_key.map_name(), result.map_key.map_instance());
        let allowed = map.is_some() && !state.map_flags(&result.map_key).enabled(MapFlag::NoDrop);
        let mut commit = false;
        let outcome = if let Some(mut character) = state.characters_mut().remove(&result.char_id) {
            let saved = self.commit_pet_loot_drop(result.char_id, &character.game_systems, &result, allowed, now);
            let outcome = match saved {
                Ok((saved, accepted)) => {
                    commit = accepted;
                    if let Some(saved) = saved { install_state(&mut character, saved); }
                    character.game_systems.pet_loot_retry_at = now.saturating_add(10000);
                    if let Some(loot) = character.game_systems.pet_support.as_mut().and_then(|support| support.loot.as_mut()) {
                        loot.return_requested = !accepted;
                        loot.next_at = now.saturating_add(10000);
                    }
                    Ok(())
                }
                Err(error) => Err(error),
            };
            state.characters_mut().insert(result.char_id, character);
            outcome
        } else {
            match self.repository.character_game_systems(result.char_id).map_err(|error| error.to_string())
                .and_then(|systems| self.commit_pet_loot_drop(result.char_id, &systems, &result, allowed, now)) {
                Ok((_, accepted)) => { commit = accepted; Ok(()) }
                Err(error) => Err(error),
            }
        };
        if let Some(map) = map { map.add_to_next_tick(MapEvent::FinalizePetLootDrop(PetLootDropFinalize { claim_id: result.claim_id, commit })); }
        outcome
    }

    fn commit_pet_loot_drop(&self, char_id: u32, current: &CharacterGameSystems, result: &PetLootDropResult, allowed: bool, now: u64) -> Result<(Option<CharacterGameSystems>, bool), String> {
        let Some(pending) = current.pet_loot.as_ref().and_then(|cargo| cargo.pending_drop.as_ref()).filter(|pending| receipt_matches(&pending.receipt, result)) else {
            return Ok((None, current.last_pet_loot_drop.as_ref().is_some_and(|receipt| receipt_matches(receipt, result))));
        };
        let accepted = allowed && result.accepted && now < pending.expires_at;
        let mut staged = current.clone();
        if accepted {
            staged.last_pet_loot_drop = Some(pending.receipt.clone());
            staged.pet_loot = None;
        } else {
            staged.pet_loot.as_mut().unwrap().pending_drop = None;
        }
        let saved = self.repository.save_character_game_systems(char_id, &staged).map_err(|error| error.to_string())?;
        Ok((Some(saved), accepted))
    }

    pub(crate) fn tick_pet_loot(&self, server: &Server, state: &ServerState, character: &mut Character, now: u64) -> Result<(), String> {
        if character.game_systems.pending_pet_loot.as_ref().is_some_and(|claim| now >= claim.expires_at || character.status.hp == 0
            || claim.map_key != character.map_instance_key || character.game_systems.pet.as_ref().is_none_or(|pet| pet.id != claim.pet_id || pet.incubating || pet.intimacy <= 0)) {
            self.cancel_pet_loot_in_state(state, character);
        }
        if let Some(pending) = character.game_systems.pet_loot.as_ref().and_then(|cargo| cargo.pending_drop.as_ref()).cloned() {
            if now < pending.expires_at { return Ok(()); }
            if let Some(map) = state.get_map_instance(&pending.receipt.map, pending.receipt.map_instance) {
                map.add_to_next_tick(MapEvent::FinalizePetLootDrop(PetLootDropFinalize { claim_id: pending.receipt.claim_id, commit: false }));
            }
            let mut staged = character.game_systems.clone();
            staged.pet_loot.as_mut().unwrap().pending_drop = None;
            let saved = self.repository.save_character_game_systems(character.char_id, &staged).map_err(|error| error.to_string())?;
            install_state(character, saved);
        }
        let return_ready = now >= character.game_systems.pet_loot_retry_at && character.game_systems.pet_support.as_ref().and_then(|support| support.loot.as_ref())
            .is_none_or(|loot| now >= loot.next_at);
        let return_requested = character.game_systems.pet_loot.as_ref().is_some_and(|cargo| !cargo.items.is_empty()
            && (character.game_systems.pet.as_ref().is_none_or(|pet| pet.id != cargo.pet_id || pet.incubating || pet.intimacy <= 0)
                || character.game_systems.pet_support.as_ref().and_then(|support| support.loot.as_ref()).is_none_or(|loot| loot.return_requested || cargo.items.len() >= usize::from(loot.capacity))));
        if return_ready && return_requested {
            if let Err(error) = self.return_pet_loot_in_state(server, state, character, now) { warn!("Pet cargo return deferred: {}", error); }
            return Ok(());
        }
        let Some(pet) = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating && pet.intimacy > 0) else { return Ok(()); };
        let id = pet_world_id(pet.id);
        let Some(support) = character.game_systems.pet_support.as_ref().filter(|support| support.pet_id == pet.id) else { return Ok(()); };
        let Some(loot) = support.loot.as_ref() else { return Ok(()); };
        if now < loot.next_at || now < support.can_act_at || support.casting.is_some() || loot.claim_queued || loot.return_requested
            || character.status.hp == 0 || !character.loaded_from_client_side || character.game_systems.pending_pet_loot.is_some()
            || character.game_systems.companion_commands.get(&id).is_some_and(|command| command.target.is_some()) { return Ok(()); }
        let Some(map) = state.get_map_instance_from_character(character) else { return Ok(()); };
        let (x, y) = character.game_systems.rendered_companions.get(&id).map_or((character.x, character.y), |position| (position.x, position.y));
        let range = self.configuration.get_mob(i32::from(pet.class_id)).range2.max(0) as u16 / 2;
        let map_state = map.state();
        let mut candidates = Vec::with_capacity(8);
        for item in map_state.map_items().values().filter(|item| *item.object_type() == crate::server::model::map_item::MapItemType::DroppedItem)
            .filter(|item| !state.contains_locked_map_item(item.id())).filter_map(|item| map_state.get_dropped_item(item.id()))
            .filter(|item| item.owner_id.is_none_or(|owner| owner == character.char_id) && x.abs_diff(item.x()).max(y.abs_diff(item.y())) <= range) {
            let distance = x.abs_diff(item.x()).max(y.abs_diff(item.y()));
            let position = candidates.iter().position(|(other_distance, other): &(u16, DroppedItem)|
                (distance, item.map_item_id) < (*other_distance, other.map_item_id)).unwrap_or(candidates.len());
            if position < 8 { candidates.insert(position, (distance, *item)); candidates.truncate(8); }
        }
        let target = candidates.into_iter().find_map(|(distance, item)| {
            (distance <= 1 || !movement::path::path_search_client_side_algorithm(map_state.x_size(), map_state.y_size(), map_state.cells(), x, y, item.x(), item.y()).is_empty()).then_some(item)
        });
        let Some(target) = target else {
            if let Some(loot) = character.game_systems.pet_support.as_mut().and_then(|support| support.loot.as_mut()) {
                if loot.target.take().is_some() {
                    if let Some(command) = character.game_systems.companion_commands.get_mut(&id) { command.destination = None; }
                }
                loot.next_at = now.saturating_add(100);
            }
            return Ok(());
        };
        let loot = character.game_systems.pet_support.as_mut().unwrap().loot.as_mut().unwrap();
        loot.target = Some(target.map_item_id);
        if x.abs_diff(target.x()).max(y.abs_diff(target.y())) > 1 {
            character.game_systems.companion_commands.entry(id).or_default().destination = Some((target.x(), target.y()));
        } else {
            loot.claim_queued = true;
            server.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld { char_id: character.char_id, request: ScriptWorldRequest::Pet(PetRequest::PetLootTarget(target.map_item_id)) }));
        }
        Ok(())
    }
}
