use models::enums::skill::SkillType;
use models::enums::status::StatusEffect;
use models::status_bonus::TemporaryStatusBonuses;
use skills::Skill;

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub target: u32,
    pub repeat: bool,
    pub last_attack_tick: u128,
    pub last_attack_motion: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct PendingSkill {
    pub target_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Damage {
    pub target_id: u32,
    pub attacker_id: u32,
    pub damage: u32,
    pub healing: u32,
    pub right_hand_damage: Option<u32>,
    pub attacked_at: u128,
    /// Duration in ms the target cannot move after being hit (flinch/hit stun)
    pub damage_motion: u32,
    pub battle_flags: u32,
    pub skill_id: u32,
    pub skill_level: u8,
    pub landed: bool,
    pub proc_depth: u8,
    pub credit_id: u32,
    pub defenses_applied: bool,
    pub magic_context: Option<crate::server::service::map_combat_service::MagicAttackContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageContribution {
    pub actor_id: u32,
    pub owner_id: u32,
    pub damage: u32,
}

impl Damage {
    pub fn set_signed_damage(&mut self, amount: i32) {
        self.damage = amount.max(0) as u32;
        self.healing = if amount < 0 { amount.unsigned_abs() } else { 0 };
    }

    pub fn admitted_right_hand_damage(&self, admitted: u32) -> Option<u32> {
        self.right_hand_damage.map(|right| {
            if self.damage == 0 {
                0
            } else {
                (u64::from(right.min(self.damage)) * u64::from(admitted.min(self.damage)) / u64::from(self.damage)) as u32
            }
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddBonuses {
    pub target_id: u32,
    pub effects: Vec<StatusEffect>,
    pub bonuses: TemporaryStatusBonuses,
}

pub struct SkillInUse {
    pub target: Option<u32>,
    pub start_skill_tick: u128,
    pub skill: Box<dyn Skill>,
    pub used_at_tick: Option<u128>, // when the skill was actually used
}

pub struct SkillCasted {
    requirements_valid: bool,
    no_delay: bool,
}

impl SkillCasted {
    pub fn invalid() -> Self {
        Self {
            requirements_valid: false,
            no_delay: false,
        }
    }

    pub fn valid() -> Self {
        Self {
            requirements_valid: true,
            no_delay: false,
        }
    }

    pub fn no_delay() -> Self {
        Self {
            requirements_valid: true,
            no_delay: true,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.requirements_valid
    }

    pub fn has_no_delay(&self) -> bool {
        self.no_delay
    }
}

pub struct SkillUsed {
    pub skill_type: SkillType,
    pub source_id: u32,
    pub target_id: u32,
    // When negative it heals target
    pub damage_to_target: i32,
    // When negative it heals self
    pub damage_to_self: i32,
    pub effects: Vec<StatusEffect>,
    pub bonuses: TemporaryStatusBonuses,
    // For offensive skills
    pub attacked_at: u128,
    /// Duration in ms the target cannot move after being hit
    pub damage_motion: u32,
    pub battle_flags: u32,
    pub skill_id: u32,
    pub skill_level: u8,
    pub landed: bool,
    pub proc_depth: u8,
    pub credit_id: u32,
    pub magic_context: Option<crate::server::service::map_combat_service::MagicAttackContext>,
}

impl SkillUsed {
    pub fn to_damage(&self) -> Damage {
        Damage {
            target_id: self.target_id,
            attacker_id: self.source_id,
            damage: self.damage_to_target.max(0) as u32,
            healing: if self.damage_to_target < 0 { self.damage_to_target.unsigned_abs() } else { 0 },
            right_hand_damage: None,
            attacked_at: self.attacked_at,
            damage_motion: self.damage_motion,
            battle_flags: self.battle_flags,
            skill_id: self.skill_id,
            skill_level: self.skill_level,
            landed: self.landed,
            proc_depth: self.proc_depth,
            credit_id: self.credit_id,
            defenses_applied: true,
            magic_context: self.magic_context,
        }
    }
}
