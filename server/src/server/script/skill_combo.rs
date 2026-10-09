use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::Status;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::ScriptSkillService;
use crate::server::Server;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// Latency allowance added to a combo window, the client sends the follow-up after the previous skill's delay ends.
const COMBO_GRACE_MS: u128 = 300;

/// `skill_get_time(SG_FRIEND, 1)`: the Friend bonus waits this long for the next roll.
const FRIEND_DURATION_MS: i32 = 10_000;

/// `skill_get_time2(ASC_EDP)`: the Deadly Poison an Enchant Deadly Poison hit leaves behind.
const EDP_POISON_DURATION_MS: i32 = 60_000;

/// Rathena rate of the Triple Attack proc is `30 - level` percent.
const TRIPLE_ATTACK_BASE_RATE: u32 = 30;

const TUMBLING_RANGED_DODGE_RATE: u32 = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct SkillCombo {
    pub skill_id: u32,
    pub target_id: u32,
    pub expires_at: u128,
    /// Stance combos hold back the next normal attack so the kick can be sent in time.
    pub blocks_attack: bool,
    /// Rankers can chain kicks inside one window, this is the last one used.
    pub last_kick: u32,
    pub window_ms: u128,
}

/// The Taekwon kicks a stance can open, in the order a normal attack checks their stances.
const STANCE_KICKS: [SkillEnum; 4] = [SkillEnum::TkStormkick, SkillEnum::TkDownkick, SkillEnum::TkTurnkick, SkillEnum::TkCounter];

fn learned(character: &Character, skill_id: u32) -> bool {
    learned_level(&character.status, skill_id) > 0
}

fn spheres(character: &Character, tick: u128) -> usize {
    character.script_skill_state.spirit_spheres.iter().filter(|expiry| **expiry > tick).count()
}

/// `50 + 50 * level` percent, Friend of the Sun, Moon and Stars.
fn friend_bonus(level: u8) -> i32 {
    50 + 50 * i32::from(level)
}

/// `rate += rate * bonus / 100`
fn boosted_rate(rate: u32, bonus: u32) -> u32 {
    rate + rate * bonus / 100
}

fn has_job(status: &Status, jobs: &[JobName]) -> bool {
    jobs.iter().any(|job| status.job == job.value() as u32)
}

/// The Friend bonus of the skill is used up by the next roll of that skill, whether it hits or not.
fn take_friend_bonus(character: &mut Character, skill_id: u32) -> u32 {
    let Some(change) = character
        .status
        .status_change(StatusChangeKind::SkillRateUp)
        .filter(|change| change.values[0] == skill_id as i32)
    else {
        return 0;
    };
    let bonus = change.values[1].max(0) as u32;
    StatusEffectService::end_status(&mut character.status, Some(StatusChangeKind::SkillRateUp));
    bonus
}

impl ScriptSkillService {
    /// Enchant Deadly Poison: a basic attack may leave a Deadly Poison on the monster it hits, the chance in percent is the EDP status value.
    pub fn edp_poison_request(status: &Status) -> Option<StatusChangeRequest> {
        let edp = status.status_change(StatusChangeKind::Edp)?;
        (edp.values[1] > 0).then(|| StatusChangeRequest {
            kind: StatusChangeKind::DeadlyPoison,
            duration_ms: EDP_POISON_DURATION_MS,
            values: [0; 4],
            rate: edp.values[1].saturating_mul(100).min(10_000) as u16,
            flags: 0,
        })
    }

    pub fn triggers_friend_share(skill_id: u32) -> bool {
        Self::friend_share(skill_id).is_some()
    }

    fn friend_share(skill_id: u32) -> Option<skills::FriendShare> {
        Self::skill_object_by_id(skill_id).and_then(|skill| skill.friend_share())
    }

    pub fn is_combo_skill(skill_id: u32) -> bool {
        Self::skill_object_by_id(skill_id).is_some_and(|skill| skill.combo_chain_ready()) || Self::is_kick(skill_id)
    }

    fn is_kick(skill_id: u32) -> bool {
        Self::stance_of(skill_id).is_some()
    }

    fn stance_of(skill_id: u32) -> Option<skills::Stance> {
        Self::skill_object_by_id(skill_id).and_then(|skill| skill.stance())
    }

    fn blocks_kick_chain(skill_id: u32) -> bool {
        Self::skill_object_by_id(skill_id).is_some_and(|skill| skill.blocks_kick_chain())
    }

    fn active_combo(character: &Character, tick: u128) -> Option<&SkillCombo> {
        character.script_skill_state.combo.as_ref().filter(|combo| combo.expires_at > tick)
    }

    fn previous_combo(character: &Character, tick: u128) -> Option<u32> {
        character.script_skill_state.combo.as_ref().filter(|combo| combo.expires_at > tick).map(|combo| combo.skill_id)
    }

    pub fn combo_ready(&self, character: &Character, skill_id: u32, tick: u128) -> bool {
        Self::is_combo_skill(skill_id) && self.validate_combo(character, skill_id, tick).is_ok()
    }

    /// Chain Combo, Combo Finish, Tiger Fist and Chain Crush are only castable inside the window of the skill they follow.
    pub fn validate_combo(&self, character: &Character, skill_id: u32, tick: u128) -> Result<(), String> {
        let previous = Self::previous_combo(character, tick);
        let allowed = if Self::is_kick(skill_id) {
            character.status.job != JobName::SoulLinker.value() as u32
                && match Self::active_combo(character, tick) {
                    None => false,
                    Some(combo) if Self::blocks_kick_chain(combo.skill_id) => false,
                    Some(combo) if combo.last_kick != 0 => combo.last_kick != skill_id,
                    Some(combo) => combo.skill_id == skill_id || character.status.taekwon_ranked,
                }
        } else if let Some(follows) = Self::skill_object_by_id(skill_id).and_then(|skill| skill.combo_follows()) {
            // Outside a combo some follow-ups need the caster to be able to move, inside one they follow their chain only.
            match previous {
                Some(previous) => follows.after.contains(&previous),
                None => follows.standing && !character.status.blocks_movement(),
            }
        } else {
            true
        };
        allowed.then_some(()).ok_or_else(|| "This skill can only be used as part of a combo".into())
    }

    /// `skill_combo`: ends the previous combo and opens the window of the next skill the character could chain.
    pub fn advance_combo(&self, character: &mut Character, skill_id: u32, target_id: u32, delay: u32, tick: u128) {
        let previous = character.script_skill_state.combo.take().filter(|combo| combo.expires_at > tick);
        if Self::is_kick(skill_id) {
            if let Some(previous) = previous.filter(|_| character.status.taekwon_ranked) {
                character.script_skill_state.combo = Some(SkillCombo {
                    skill_id,
                    target_id,
                    expires_at: tick + previous.window_ms,
                    blocks_attack: false,
                    last_kick: skill_id,
                    window_ms: previous.window_ms,
                });
            }
            return;
        }
        let spheres = spheres(character, tick);
        let explosion = character.status.has_status_change(StatusChangeKind::ExplosionSpirits);
        let chains = Self::skill_object_by_id(skill_id).is_some_and(|skill| {
            skill
                .combo_links()
                .iter()
                .any(|link| learned(character, link.next) && spheres >= usize::from(link.min_spheres) && (!link.needs_explosion || explosion))
        });
        if chains {
            let window_ms = u128::from(delay) + COMBO_GRACE_MS;
            character.script_skill_state.combo = Some(SkillCombo { skill_id, target_id, expires_at: tick + window_ms, blocks_attack: false, last_kick: 0, window_ms });
        }
    }

    /// A normal attack of a Taekwon holding a stance may open the window of its kick, the window shrinks with AGI and DEX.
    pub fn try_stance_combo(&self, character: &mut Character, target_id: u32, tick: u128) {
        let mut chosen = None;
        for kick in STANCE_KICKS.iter().map(|kick| kick.id()) {
            let Some(stance) = Self::stance_of(kick) else {
                continue;
            };
            if !character.status.has_status_change(stance.ready) {
                continue;
            }
            let rate = if stance.friend_boosted {
                boosted_rate(stance.rate, take_friend_bonus(character, kick))
            } else {
                stance.rate
            };
            if fastrand::u32(0..100) < rate {
                chosen = Some(kick);
                break;
            }
        }
        let Some(kick) = chosen else {
            return;
        };
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let window_ms = (2000_i64 - 4 * i64::from(snapshot.agi()) - 2 * i64::from(snapshot.dex())).max(0) as u128 + COMBO_GRACE_MS;
        character.script_skill_state.combo =
            Some(SkillCombo { skill_id: kick, target_id, expires_at: tick + window_ms, blocks_attack: true, last_kick: 0, window_ms });
    }

    pub fn attack_blocked_by_combo(character: &Character, tick: u128) -> bool {
        Self::active_combo(character, tick).is_some_and(|combo| combo.blocks_attack)
    }

    /// Tumbling avoids ranged damage, running (Spurt) also avoids melee.
    pub fn try_dodge(character: &Character, flags: u32) -> bool {
        flags != 0
            && character.status.has_status_change(StatusChangeKind::Dodge)
            && (flags & BattleFlag::Long.as_flag() != 0 || character.status.has_status_change(StatusChangeKind::Spurt))
            && fastrand::u32(0..100) < TUMBLING_RANGED_DODGE_RATE
    }

    /// Part of a normal attack of a Monk who learned Triple Attack: some hits become the skill, which opens the combo.
    pub fn try_triple_attack(&self, server: &Server, state: &ServerState, character: &mut Character, target_id: u32, tick: u128) -> bool {
        let level = learned_level(&character.status, SkillEnum::MoTripleattack.id());
        if level == 0 {
            return false;
        }
        let bonus = take_friend_bonus(character, SkillEnum::MoTripleattack.id());
        if fastrand::u32(0..100) >= boosted_rate(TRIPLE_ATTACK_BASE_RATE.saturating_sub(u32::from(level)), bonus) {
            return false;
        }
        if self.cast_skill(server, state, character, SkillEnum::MoTripleattack.id(), level, target_id, false, tick, true).is_err() {
            return false;
        }
        let motion = StatusService::instance().attack_motion(&StatusService::instance().to_snapshot(&character.status));
        let next = tick + u128::from(motion);
        character.timing.set_canmove_tick(next);
        character.timing.set_canact_tick(next);
        self.advance_combo(character, SkillEnum::MoTripleattack.id(), target_id, motion, tick);
        true
    }

    /// Friend of the Sun, Moon and Stars (`party_skill_check`): a Star Gladiator's Counter raises the Triple Attack rate of party Monks on the same map,
    /// and a Monk's Combo Finish raises the Counter rate of party Star Gladiators that have a ready counter.
    pub fn share_friend_rate(&self, server: &Server, state: &mut ServerState, caster_id: u32, skill_id: u32, tick: u128) -> Result<(), String> {
        let Some(caster) = state.get_character(caster_id) else {
            return Ok(());
        };
        let party = caster.game_systems.party_id;
        if party == 0 {
            return Ok(());
        }
        let instance = caster.map_instance_key.clone();
        let caster_level = learned_level(&caster.status, SkillEnum::SgFriend.id());
        let in_party = |member: &Character| member.char_id != caster_id && member.game_systems.party_id == party && member.map_instance_key == instance;
        let grants: Vec<(u32, u32, i32)> = if Self::friend_share(skill_id) == Some(skills::FriendShare::TripleAttack) {
            if caster_level == 0 {
                return Ok(());
            }
            state
                .characters()
                .values()
                .filter(|member| in_party(member) && has_job(&member.status, &[JobName::Monk, JobName::Champion]))
                .filter(|member| learned_level(&member.status, SkillEnum::MoTripleattack.id()) > 0)
                .map(|member| (member.char_id, SkillEnum::MoTripleattack.id(), friend_bonus(caster_level)))
                .collect()
        } else if Self::friend_share(skill_id) == Some(skills::FriendShare::Counter) {
            state
                .characters()
                .values()
                .filter(|member| in_party(member) && has_job(&member.status, &[JobName::StarGladiator]))
                .filter(|member| member.status.has_status_change(StatusChangeKind::ReadyCounter))
                .filter(|member| learned_level(&member.status, SkillEnum::SgFriend.id()) > 0)
                .map(|member| {
                    let level = learned_level(&member.status, SkillEnum::SgFriend.id());
                    (member.char_id, SkillEnum::TkCounter.id(), friend_bonus(level))
                })
                .collect()
        } else {
            return Ok(());
        };
        for (member_id, skill, bonus) in grants {
            let Some(member) = state.characters_mut().get_mut(&member_id) else {
                continue;
            };
            let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::SkillRateUp, FRIEND_DURATION_MS, skill as i32);
            request.values[1] = bonus;
            StatusEffectService::start(server, member, request, tick, &self.client_notification_sender)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_status(kind: StatusChangeKind, values: [i32; 4]) -> Status {
        let mut request = StatusChangeRequest::guaranteed(kind, -1, 0);
        request.values = values;
        let mut status = Status::default();
        StatusEffectService::apply_status(&mut status, request, 0, 0).unwrap();
        status
    }

    #[test]
    fn friend_bonus_grows_by_half_per_level_and_boosts_the_rate() {
        assert_eq!(friend_bonus(1), 100);
        assert_eq!(friend_bonus(3), 200);
        assert_eq!(boosted_rate(20, 100), 40);
        assert_eq!(boosted_rate(29, 0), 29);
    }

    #[test]
    fn enchant_deadly_poison_poisons_monsters_with_its_chance_for_sixty_seconds() {
        assert!(ScriptSkillService::edp_poison_request(&Status::default()).is_none());
        let request = ScriptSkillService::edp_poison_request(&with_status(StatusChangeKind::Edp, [5, 0, 0, 0])).unwrap();
        assert_eq!(request.kind, StatusChangeKind::DeadlyPoison);
        assert_eq!((request.rate, request.duration_ms), (500, 60_000));
    }

    #[test]
    fn friend_bonus_is_used_up_by_the_roll_of_its_own_skill_only() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.status = with_status(StatusChangeKind::SkillRateUp, [SkillEnum::MoTripleattack.id() as i32, 150, 0, 0]);
        assert_eq!(take_friend_bonus(&mut character, SkillEnum::TkCounter.id()), 0);
        assert!(character.status.has_status_change(StatusChangeKind::SkillRateUp));
        assert_eq!(take_friend_bonus(&mut character, SkillEnum::MoTripleattack.id()), 150);
        assert!(!character.status.has_status_change(StatusChangeKind::SkillRateUp));
    }
}
