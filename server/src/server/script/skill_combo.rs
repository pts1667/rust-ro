use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status_bonus::BattleFlag;
use models::status_change::StatusChangeKind;

use super::ScriptSkillService;
use crate::server::Server;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// Latency allowance added to a combo window, the client sends the follow-up after the previous skill's delay ends.
const COMBO_GRACE_MS: u128 = 300;

/// Rathena rate of the Triple Attack proc is `30 - level` percent.
const TRIPLE_ATTACK_BASE_RATE: u32 = 30;

const STANCE_KICK_RATE: u32 = 15;
const STANCE_COUNTER_RATE: u32 = 20;
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

/// Taekwon stance with the kick it unlocks and the percent chance a normal attack opens the window.
const STANCES: [(StatusChangeKind, SkillEnum, u32); 4] = [
    (StatusChangeKind::ReadyStorm, SkillEnum::TkStormkick, STANCE_KICK_RATE),
    (StatusChangeKind::ReadyDown, SkillEnum::TkDownkick, STANCE_KICK_RATE),
    (StatusChangeKind::ReadyTurn, SkillEnum::TkTurnkick, STANCE_KICK_RATE),
    (StatusChangeKind::ReadyCounter, SkillEnum::TkCounter, STANCE_COUNTER_RATE),
];

fn learned(character: &Character, skill: SkillEnum) -> bool {
    learned_level(&character.status, skill.id()) > 0
}

fn spheres(character: &Character, tick: u128) -> usize {
    character.script_skill_state.spirit_spheres.iter().filter(|expiry| **expiry > tick).count()
}

impl ScriptSkillService {
    pub fn is_combo_skill(skill_id: u32) -> bool {
        [SkillEnum::MoChaincombo, SkillEnum::MoCombofinish, SkillEnum::ChTigerfist, SkillEnum::ChChaincrush]
            .iter()
            .any(|skill| skill.id() == skill_id)
            || Self::is_kick(skill_id)
    }

    fn is_kick(skill_id: u32) -> bool {
        STANCES.iter().any(|(_, kick, _)| kick.id() == skill_id)
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
        let after = |skills: &[SkillEnum]| previous.is_some_and(|previous| skills.iter().any(|skill| skill.id() == previous));
        let allowed = if skill_id == SkillEnum::MoChaincombo.id() {
            after(&[SkillEnum::MoTripleattack])
        } else if skill_id == SkillEnum::MoCombofinish.id() {
            after(&[SkillEnum::MoChaincombo])
        } else if skill_id == SkillEnum::ChTigerfist.id() {
            after(&[SkillEnum::MoCombofinish, SkillEnum::ChChaincrush])
        } else if skill_id == SkillEnum::ChChaincrush.id() {
            after(&[SkillEnum::MoCombofinish, SkillEnum::ChTigerfist])
        } else if Self::is_kick(skill_id) {
            character.status.job != JobName::SoulLinker.value() as u32
                && match Self::active_combo(character, tick) {
                    None => false,
                    Some(combo) if combo.skill_id == SkillEnum::TkJumpkick.id() => false,
                    Some(combo) if combo.last_kick != 0 => combo.last_kick != skill_id,
                    Some(combo) => combo.skill_id == skill_id || character.status.taekwon_ranked,
                }
        } else if skill_id == SkillEnum::MoExtremityfist.id() {
            // Outside a combo Asura Strike needs the caster to be able to move, inside one it follows the Champion chain.
            match previous {
                Some(_) => after(&[SkillEnum::MoCombofinish, SkillEnum::ChChaincrush]),
                None => !character.status.blocks_movement(),
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
        let chains = if skill_id == SkillEnum::MoTripleattack.id() {
            learned(character, SkillEnum::MoChaincombo)
        } else if skill_id == SkillEnum::MoChaincombo.id() {
            learned(character, SkillEnum::MoCombofinish)
        } else if skill_id == SkillEnum::MoCombofinish.id() {
            (learned(character, SkillEnum::ChTigerfist) && spheres >= 1)
                || (learned(character, SkillEnum::ChChaincrush) && spheres >= 2)
                || (learned(character, SkillEnum::MoExtremityfist) && spheres >= 4 && explosion)
        } else if skill_id == SkillEnum::ChTigerfist.id() {
            learned(character, SkillEnum::ChChaincrush) && spheres >= 2
        } else if skill_id == SkillEnum::ChChaincrush.id() {
            (learned(character, SkillEnum::MoExtremityfist) && spheres >= 1 && explosion)
                || (learned(character, SkillEnum::ChTigerfist) && spheres >= 1)
        } else {
            false
        };
        if chains {
            let window_ms = u128::from(delay) + COMBO_GRACE_MS;
            character.script_skill_state.combo = Some(SkillCombo { skill_id, target_id, expires_at: tick + window_ms, blocks_attack: false, last_kick: 0, window_ms });
        }
    }

    /// A normal attack of a Taekwon holding a stance may open the window of its kick, the window shrinks with AGI and DEX.
    pub fn try_stance_combo(&self, character: &mut Character, target_id: u32, tick: u128) {
        let Some((_, kick, _)) = STANCES
            .iter()
            .find(|(stance, _, rate)| character.status.has_status_change(*stance) && fastrand::u32(0..100) < *rate)
        else {
            return;
        };
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let window_ms = (2000_i64 - 4 * i64::from(snapshot.agi()) - 2 * i64::from(snapshot.dex())).max(0) as u128 + COMBO_GRACE_MS;
        character.script_skill_state.combo =
            Some(SkillCombo { skill_id: kick.id(), target_id, expires_at: tick + window_ms, blocks_attack: true, last_kick: 0, window_ms });
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
        if level == 0 || fastrand::u32(0..100) >= TRIPLE_ATTACK_BASE_RATE.saturating_sub(u32::from(level)) {
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
}
