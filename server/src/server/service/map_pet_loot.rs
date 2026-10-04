use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use models::item::ItemInstanceAttributes;
use packets::packets::{Packet, PacketZcItemDisappear};

use super::MapInstanceService;
use crate::server::model::events::game_event::{GameEvent, PetLootClaimResult, PetLootDropResult};
use crate::server::model::events::map_event::{PetLootClaimRequest, PetLootDropFinalize, PetLootDropRequest, PetLootFinalize};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_item::MapItemType;
use crate::server::service::script_world_service::pet_world_id;
use crate::server::state::map_instance::{CompletedPetLoot, CompletedPetLootDrop, MapInstanceState, PendingPetLootDrop};

const MAX_PET_LOOT_RECEIPTS: usize = 256;

fn drop_fingerprint(request: &PetLootDropRequest) -> u64 {
    let mut hash = DefaultHasher::new();
    (request.char_id, request.x, request.y, request.items.len()).hash(&mut hash);
    for item in &request.items {
        (
            item.id,
            item.unique_id,
            item.item_id,
            item.amount,
            item.refine,
            item.is_identified,
            item.equip,
            item.is_damaged,
            item.card0,
            item.card1,
            item.card2,
            item.card3,
        )
            .hash(&mut hash);
    }
    hash.finish()
}

impl MapInstanceService {
    pub fn claim_pet_loot(&self, state: &mut MapInstanceState, request: PetLootClaimRequest, tick: u128) {
        let item = if let Some(completed) = state.completed_pet_loot.get(&request.claim_id) {
            (completed.char_id == request.char_id && completed.pet_id == request.pet_id && completed.target_id == request.target_id)
                .then_some(completed.item)
                .flatten()
        } else if let Some(pending) = state.pending_pet_loot.get(&request.claim_id) {
            (pending.char_id == request.char_id && pending.pet_id == request.pet_id && pending.item.map_item_id == request.target_id)
                .then_some(pending.item)
        } else {
            let eligible = request.claim_id != 0
                && request.char_id != 0
                && request.pet_id != 0
                && tick <= u128::from(request.expires_at)
                && state
                    .get_map_item(request.char_id)
                    .is_some_and(|item| *item.object_type() == MapItemType::Character)
                && (state
                    .get_map_item(pet_world_id(request.pet_id))
                    .is_some_and(|item| *item.object_type() == MapItemType::Pet)
                    || state.characters().iter().any(|snapshot| {
                        snapshot.map_item().id() == pet_world_id(request.pet_id) && *snapshot.map_item().object_type() == MapItemType::Pet
                    }))
                && state.get_dropped_item(request.target_id).is_some_and(|item| {
                    item.amount > 0
                        && item.owner_id.is_none_or(|owner| owner == request.char_id)
                        && item.x().abs_diff(request.x).max(item.y().abs_diff(request.y)) <= 1
                });
            if eligible {
                state.reserve_pet_loot(request.claim_id, request.char_id, request.pet_id, request.target_id)
            } else {
                None
            }
        };
        self.server_task_queue
            .add_to_first_index(GameEvent::PetLootClaimResult(PetLootClaimResult {
                claim_id: request.claim_id,
                char_id: request.char_id,
                pet_id: request.pet_id,
                target_id: request.target_id,
                map_key: state.key().clone(),
                item,
            }));
    }

    pub fn finalize_pet_loot(&self, state: &mut MapInstanceState, request: PetLootFinalize) {
        let matches = state
            .pending_pet_loot
            .get(&request.claim_id)
            .is_some_and(|pending| pending.item.map_item_id == request.target_id);
        if !matches {
            if !state.is_pet_loot_reserved(request.target_id) {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ReleaseScriptCapture(request.target_id));
            }
            return;
        }
        let pending = state.pending_pet_loot.remove(&request.claim_id).unwrap();
        if request.commit {
            state.remove_item_with_id(request.target_id);
            let mut packet = PacketZcItemDisappear::new(self.configuration_service.packetver());
            packet.set_itaid(request.target_id);
            packet.fill_raw();
            self.notify_area(state, pending.item.x(), pending.item.y(), packet.raw);
            self.server_task_queue
                .add_to_first_index(GameEvent::MapNotifyItemRemoved(request.target_id));
        } else {
            state.restore_pet_loot(pending.item);
        }
        state.completed_pet_loot.insert(request.claim_id, CompletedPetLoot {
            char_id: pending.char_id,
            pet_id: pending.pet_id,
            target_id: request.target_id,
            item: request.commit.then_some(pending.item),
        });
        state.completed_pet_loot_order.push_back(request.claim_id);
        while state.completed_pet_loot_order.len() > MAX_PET_LOOT_RECEIPTS {
            if let Some(oldest) = state.completed_pet_loot_order.pop_front() {
                state.completed_pet_loot.remove(&oldest);
            }
        }
        self.server_task_queue
            .add_to_first_index(GameEvent::ReleaseScriptCapture(request.target_id));
    }

    pub fn prepare_pet_loot_drop(&self, state: &mut MapInstanceState, request: PetLootDropRequest) {
        let fingerprint = drop_fingerprint(&request);
        let accepted = if let Some(completed) = state.completed_pet_loot_drops.get(&request.claim_id) {
            completed.char_id == request.char_id && completed.fingerprint == fingerprint && completed.accepted
        } else if let Some(pending) = state.pending_pet_loot_drops.get(&request.claim_id) {
            pending.char_id == request.char_id && pending.fingerprint == fingerprint
        } else {
            let valid = request.claim_id != 0
                && request.char_id != 0
                && !request.items.is_empty()
                && !state.flags.enabled(MapFlag::NoDrop)
                && request
                    .items
                    .iter()
                    .all(|item| item.amount > 0 && self.configuration_service.find_item(item.item_id).is_some());
            if valid {
                let mut rng = fastrand::Rng::new();
                let mut drops = Vec::with_capacity(request.items.len());
                for item in &request.items {
                    let Some(drop) = self.prepare_dropped_item(
                        state,
                        &mut rng,
                        request.x,
                        request.y,
                        item.item_id,
                        item.is_identified,
                        item.amount as u16,
                        None,
                        ItemInstanceAttributes {
                            unique_id: item.unique_id,
                            refine: item.refine,
                            damaged: item.is_damaged,
                            cards: [item.card0, item.card1, item.card2, item.card3],
                        },
                        false,
                    ) else {
                        break;
                    };
                    drops.push(drop);
                }
                if drops.len() == request.items.len() {
                    state.pending_pet_loot_drops.insert(request.claim_id, PendingPetLootDrop {
                        char_id: request.char_id,
                        x: request.x,
                        y: request.y,
                        fingerprint,
                        items: drops,
                    });
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        self.server_task_queue
            .add_to_first_index(GameEvent::PetLootDropResult(PetLootDropResult {
                claim_id: request.claim_id,
                char_id: request.char_id,
                map_key: state.key().clone(),
                accepted,
            }));
    }

    pub fn finalize_pet_loot_drop(&self, state: &mut MapInstanceState, request: PetLootDropFinalize) {
        let Some(pending) = state.pending_pet_loot_drops.remove(&request.claim_id) else {
            return;
        };
        if request.commit {
            for item in &pending.items {
                state.insert_dropped_item(*item);
            }
            self.notify_drop_items(state, pending.x, pending.y, pending.items);
        }
        state.completed_pet_loot_drops.insert(request.claim_id, CompletedPetLootDrop {
            char_id: pending.char_id,
            fingerprint: pending.fingerprint,
            accepted: request.commit,
        });
        state.completed_pet_loot_drop_order.push_back(request.claim_id);
        while state.completed_pet_loot_drop_order.len() > MAX_PET_LOOT_RECEIPTS {
            if let Some(oldest) = state.completed_pet_loot_drop_order.pop_front() {
                state.completed_pet_loot_drops.remove(&oldest);
            }
        }
    }
}
