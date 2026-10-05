use models::enums::vanish::VanishType;
use models::enums::EnumWithNumberValue;
use models::status_change::StatusChangeKind;
use packets::packets::{Packet, PacketZcNotifyVanish};

use super::MapInstanceService;
use crate::server::model::events::game_event::{GameEvent, PetCaptureClaimResult, ReleaseScriptCapture};
use crate::server::model::events::map_event::{PetCaptureClaimRequest, PetCaptureFinalize};
use crate::server::service::script_world_service::world_data;
use crate::server::state::map_instance::{MapInstanceState, PendingPetCapture};

pub fn pet_capture_rate(base: i32, hp: u32, max_hp: u32) -> u32 {
    let hp_percent = u64::from(hp.min(max_hp)) * 100 / u64::from(max_hp.max(1));
    (i64::from(base) + (100 - hp_percent as i64) * i64::from(base) / 100).clamp(1, 10000) as u32
}

impl MapInstanceService {
    pub fn claim_pet_capture(&self, state: &mut MapInstanceState, request: PetCaptureClaimRequest, tick: u128) {
        self.claim_pet_capture_with_roll(state, request, tick, fastrand::u32(0..10000));
    }

    pub(crate) fn claim_pet_capture_with_roll(&self, state: &mut MapInstanceState, request: PetCaptureClaimRequest, tick: u128, roll: u32) {
        let class_id = if let Some(claim) = state.pending_pet_captures.get(&request.claim_id) {
            (claim.char_id == request.char_id && claim.mob.id == request.target_id).then_some(claim.mob.mob_id as u16)
        } else {
            let eligible = state.get_mob(request.target_id).filter(|mob| {
                request.claim_id != 0
                    && request.char_id != 0
                    && tick <= u128::from(request.expires_at)
                    && !state.flags.enabled(crate::server::model::map_flags::MapFlag::NoPetCapture)
                    && mob.is_present()
                    && mob.hp() > 0
                    && mob.x.abs_diff(request.x).max(mob.y.abs_diff(request.y)) <= 5
                    && !mob.status_effects.has_status_change(StatusChangeKind::Hiding)
                    && !mob.status_effects.has_status_change(StatusChangeKind::Cloaking)
            });
            let captured = eligible.and_then(|mob| {
                let pet = world_data().pets.iter().find(|pet| pet.class_id == mob.mob_id as u16)?;
                let allowed = match request.flag {
                    0 => request.lure_item_id == pet.tame_item,
                    1 => !mob.status.has_mob_capability(models::enums::mob::MobCapability::StatusImmune),
                    2 => true,
                    _ => false,
                };
                (allowed && roll < pet_capture_rate(pet.capture_rate, mob.hp(), mob.status.max_hp())).then_some(pet.class_id)
            });
            if captured.is_some() {
                if let Some(mob) = state.mobs_mut().remove(&request.target_id) {
                    state.pending_pet_captures.insert(request.claim_id, PendingPetCapture {
                        char_id: request.char_id,
                        mob,
                    });
                }
            }
            captured
        };
        self.server_task_queue
            .add_to_first_index(GameEvent::PetCaptureClaimResult(PetCaptureClaimResult {
                claim_id: request.claim_id,
                char_id: request.char_id,
                target_id: request.target_id,
                map_key: state.key().clone(),
                class_id,
            }));
    }

    pub fn finalize_pet_capture(&self, state: &mut MapInstanceState, request: PetCaptureFinalize) {
        if !state
            .pending_pet_captures
            .get(&request.claim_id)
            .is_some_and(|claim| claim.mob.id == request.target_id)
        {
            if !state.pending_pet_captures.values().any(|claim| claim.mob.id == request.target_id) {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: request.target_id }));
            }
            return;
        }
        let claim = state.pending_pet_captures.remove(&request.claim_id).unwrap();
        if request.commit {
            state.remove_item_with_id(request.target_id);
            if let Some(track) = state
                .mob_spawns_tracks_mut()
                .get_mut(&claim.mob.spawn_id)
                .filter(|_| !claim.mob.summoned)
            {
                track.decrement_spawn();
            }
            let mut packet = PacketZcNotifyVanish::new(self.configuration_service.packetver());
            packet.set_gid(claim.mob.id);
            packet.set_atype(VanishType::OutOfSight.value() as u8);
            packet.fill_raw();
            self.notify_area(state, claim.mob.x, claim.mob.y, packet.raw);
        } else {
            state.mobs_mut().insert(claim.mob.id, claim.mob);
        }
        self.server_task_queue
            .add_to_first_index(GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: request.target_id }));
    }
}

#[cfg(test)]
mod tests {
    use super::pet_capture_rate;

    #[test]
    fn primary_capture_rate_uses_integer_current_hp_percent_and_clamps_the_result() {
        assert_eq!(pet_capture_rate(2000, 1000, 1000), 2000);
        assert_eq!(pet_capture_rate(2000, 500, 1000), 3000);
        assert_eq!(pet_capture_rate(2000, 1, 1000), 4000);
        assert_eq!(pet_capture_rate(6000, 1, 1000), 10000);
        assert_eq!(pet_capture_rate(0, 1, 10), 1);
    }
}
