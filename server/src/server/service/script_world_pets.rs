use models::script_context::ScriptPetState;
use script_sdk::Value;

use super::{PetDefinition, ScriptWorldService, pet_world_id, world_data};
use crate::server::Server;
use crate::server::model::events::game_event::PetCaptureClaimResult;
use crate::server::model::events::map_event::{MapEvent, PetCaptureFinalize};
use crate::server::model::game_systems::PetRecord;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub fn pet_script_state(pet: &PetRecord) -> ScriptPetState {
    ScriptPetState {
        id: pet.id,
        class_id: pet.class_id,
        name: pet.name.clone(),
        intimacy: pet.intimacy,
        hunger: pet.hunger,
        renamed: pet.renamed,
        level: pet.level,
        actor_id: pet_world_id(pet.id),
        egg_id: pet.egg_item_id,
        food_id: world_data().pets.iter().find(|definition| definition.class_id == pet.class_id).map_or(0, |definition| definition.food_item),
        equipped_item: pet.equipped_item,
        support_bonuses: Vec::new(),
    }
}

pub fn pet_information(pet: Option<&ScriptPetState>, field: i32) -> Value {
    if field == 2 {
        return Value::String(pet.map_or("null", |pet| pet.name.as_str()).into());
    }
    let Some(pet) = pet else { return Value::default(); };
    Value::Number(match field {
        0 => pet.id as i32,
        1 => i32::from(pet.class_id),
        3 => pet.intimacy,
        4 => pet.hunger,
        5 => i32::from(pet.renamed),
        6 => i32::from(pet.level),
        7 => pet.actor_id as i32,
        8 => pet.egg_id,
        9 => pet.food_id,
        _ => 0,
    })
}

pub fn advance_pet_hunger(pet: &mut PetRecord, definition: &PetDefinition, now: u64) -> bool {
    if pet.incubating || definition.hungry_delay == 0 || now < pet.next_hunger_at { return false; }
    let hunger = pet.hunger - definition.fullness;
    let starving = hunger < 0;
    pet.hunger = hunger.clamp(0, 100);
    if starving { pet.intimacy = (pet.intimacy + definition.intimacy_hungry).clamp(0, 1000); }
    pet.next_hunger_at = now.saturating_add(if starving { 20000 } else { definition.hungry_delay });
    true
}

pub fn pet_bonus_state(character: &Character) -> Option<ScriptPetState> {
    character.game_systems.pet.as_ref().filter(|pet| !pet.incubating).map(|pet| {
        let mut state = pet_script_state(pet);
        if let Some(support) = character.game_systems.pet_support.as_ref().filter(|support|
            support.pet_id == pet.id && pet.intimacy > 0 && (!support.requires_accessory || pet.equipped_item > 0)) {
            state.support_bonuses.extend(support.base_bonuses.iter().copied());
            if let Some(bonus) = support.bonus.as_ref().filter(|bonus| bonus.active) {
                state.support_bonuses.extend(bonus.bonuses.iter().copied());
            }
        }
        state
    })
}

pub fn pet_constant(name: &str) -> Option<Value> {
    let value = match name {
        "PETINFO_ID" | "PET_ID" | "PET_INTIMATE_NONE" | "PET_HUNGRY_NONE" => 0,
        "PETINFO_CLASS" | "PET_CLASS" | "PET_INTIMATE_AWKWARD" => 1,
        "PETINFO_NAME" | "PET_NAME" => 2,
        "PETINFO_INTIMATE" | "PET_INTIMATE" => 3,
        "PETINFO_HUNGRY" | "PET_HUNGRY" => 4,
        "PETINFO_RENAMED" | "PET_RENAMED" => 5,
        "PETINFO_LEVEL" | "PET_LEVEL" => 6,
        "PETINFO_BLOCKID" => 7,
        "PETINFO_EGGID" => 8,
        "PETINFO_FOODID" => 9,
        "PET_HUNGRY_VERY_HUNGRY" => 10,
        "PET_HUNGRY_HUNGRY" => 25,
        "PET_HUNGRY_NEUTRAL" => 75,
        "PET_HUNGRY_SATISFIED" => 90,
        "PET_INTIMATE_SHY" | "PET_HUNGRY_STUFFED" => 100,
        "PET_INTIMATE_NEUTRAL" => 250,
        "PET_INTIMATE_CORDIAL" => 750,
        "PET_INTIMATE_LOYAL" => 910,
        "PET_INTIMATE_MAX" => 1000,
        "PET_CATCH_UNIVERSAL_ALL" | "PET_CATCH_UNIVERSAL_ITEM" => 2,
        _ => return None,
    };
    Some(Value::Number(value))
}

impl ScriptWorldService {
    pub(crate) fn refresh_pet_bonuses(&self, server: &Server, character: &mut Character, previous: Option<ScriptPetState>) {
        if pet_bonus_state(character) != previous {
            character.refresh_script_context();
            let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
            if character.status.hp > snapshot.max_hp() || character.status.sp > snapshot.max_sp() {
                server.character_service().update_hp_sp(character, character.status.hp.min(snapshot.max_hp()), character.status.sp.min(snapshot.max_sp()));
            }
            server.character_service().reload_client_side_status(character);
        }
    }

    pub fn complete_pet_capture(&self, server: &Server, state: &mut ServerState, result: PetCaptureClaimResult, now: u64) -> Result<(), String> {
        let map = state.get_map_instance(result.map_key.map_name(), result.map_key.map_instance()).ok_or("Pet capture map disappeared")?;
        let mut commit = false;
        let allowed_map = !state.map_flags(&result.map_key).enabled(crate::server::model::map_flags::MapFlag::NoPetCapture);
        let completion = if let Some(mut character) = state.characters_mut().remove(&result.char_id) {
            let outcome = self.commit_pet_capture(server, &mut character, &result, allowed_map, now);
            if matches!(outcome, Ok(true)) { commit = true; }
            state.characters_mut().insert(result.char_id, character);
            outcome.map(|_| ())
        } else {
            Ok(())
        };
        map.add_to_next_tick(MapEvent::FinalizePetCapture(PetCaptureFinalize { claim_id: result.claim_id, target_id: result.target_id, commit }));
        completion
    }

    fn commit_pet_capture(&self, server: &Server, character: &mut Character, result: &PetCaptureClaimResult, allowed_map: bool, now: u64) -> Result<bool, String> {
        let matches = character.game_systems.pending_pet_capture.as_ref().is_some_and(|pending|
            pending.claim_id == result.claim_id && pending.target_id == result.target_id && pending.map_key == result.map_key);
        if !matches {
            return Ok(character.game_systems.completed_pet_capture.as_ref().is_some_and(|completed|
                completed.claim_id == result.claim_id && completed.target_id == result.target_id && completed.map_key == result.map_key && completed.committed));
        }
        let mut pending = character.game_systems.pending_pet_capture.take().unwrap();
        let allowed = allowed_map && now < pending.expires_at && character.status.hp > 0 && character.loaded_from_client_side && character.map_instance_key == result.map_key;
        let Some(definition) = result.class_id.filter(|_| allowed).and_then(|class| world_data().pets.iter().find(|definition| definition.class_id == class)) else {
            self.send(character.char_id, vec![0xA0, 0x01, 0])?;
            return Ok(false);
        };
        let pet = PetRecord {
            id: 0,
            owner_char_id: character.char_id,
            class_id: definition.class_id,
            name: definition.name.clone(),
            level: definition.level,
            egg_item_id: definition.egg_item,
            egg_inventory_id: 0,
            intimacy: definition.intimacy_start,
            hunger: 100,
            equipped_item: 0,
            next_hunger_at: now + definition.hungry_delay,
            incubating: true,
            renamed: false,
        };
        if let Err(error) = self.repository.create_pet_egg(character.char_id, &pet, server.character_service().max_weight(character) * 9 / 10) {
            if let Err(notification_error) = self.send(character.char_id, vec![0xA0, 0x01, 0]) { warn!("Failed to notify rejected pet capture: {}", notification_error); }
            return Err(error.to_string());
        }
        server.inventory_service().reload_inventory(server.runtime(), character.char_id, character);
        pending.committed = true;
        character.game_systems.completed_pet_capture = Some(pending);
        if let Err(error) = self.send(character.char_id, vec![0xA0, 0x01, 1]) { warn!("Failed to notify committed pet capture: {}", error); }
        Ok(true)
    }

    pub fn cancel_pet_capture(&self, server: &Server, character: &mut Character) {
        self.cancel_pet_capture_in_state(server, server.state(), character);
    }

    pub fn cancel_pet_capture_in_state(&self, _server: &Server, state: &ServerState, character: &mut Character) {
        if let Some(pending) = character.game_systems.pending_pet_capture.take() {
            if let Some(map) = state.get_map_instance(pending.map_key.map_name(), pending.map_key.map_instance()) {
                map.add_to_next_tick(MapEvent::FinalizePetCapture(PetCaptureFinalize { claim_id: pending.claim_id, target_id: pending.target_id, commit: false }));
            }
        }
    }
}
