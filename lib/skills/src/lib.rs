use std::any::Any;

use models::enums::EnumWithNumberValue;
use models::enums::element::Element;
use models::enums::skill::{SkillTargetType, SkillType, UseSkillFailure};
use models::enums::status::StatusEffect;
use models::enums::weapon::AmmoType;
use models::item::NormalInventoryItem;
use models::status::StatusSnapshot;
use models::status_bonus::TemporaryStatusBonuses;

pub mod npc;
pub mod skill_enums;
pub mod skills;


type SkillRequirementResult<T> = std::result::Result<T, ()>;

/// One skill. Every hook has a default, so a skill overrides only what differs from the generic behaviour.
pub trait Skill: Send + Sync {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn skill_type(&self) -> SkillType;
    fn level(&self) -> u8;
    fn id(&self) -> u32;
    fn range(&self) -> i8;
    fn max_level(&self) -> u8;
    fn sp_cost(&self) -> u16;
    fn target_type(&self) -> SkillTargetType;
    fn is_ranged(&self) -> bool;
    fn is_magic(&self) -> bool;
    fn is_physical(&self) -> bool;

    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        self.skill_type() == SkillType::Offensive
    }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        self.skill_type() == SkillType::Support
    }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        self.skill_type() == SkillType::Interactive
    }
    #[inline(always)]
    fn is_performance_skill(&self) -> bool {
        self.skill_type() == SkillType::Performance
    }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        self.skill_type() == SkillType::Passive
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        self.target_type() == SkillTargetType::Ground
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        0
    }
    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        None
    }
    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        None
    }
    #[inline(always)]
    fn element(&self) -> Element {
        Element::Neutral
    }
    #[inline(always)]
    fn inflict_status_effect_to_target(
        &self,
        _status: &StatusSnapshot,
        _target_status: &StatusSnapshot,
        mut _rng: fastrand::Rng,
    ) -> Vec<StatusEffect> {
        vec![]
    }
    #[inline(always)]
    fn inflict_status_effect_to_self(&self, _status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Option<StatusEffect> {
        None
    }
    #[inline(always)]
    fn damage_if_failed(&self) -> f64 {
        0.0
    }

    #[inline(always)]
    fn mitigate_skills(&self) -> Vec<u32> {
        vec![]
    }

    #[inline(always)]
    fn validate_sp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn validate_hp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn validate_state(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn validate_zeny(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn validate_spirit_sphere(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    #[inline(always)]
    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn validate_weapon(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn validate_range(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn update_cast_time(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, _new_value: u32) {}

    #[inline(always)]
    fn cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }

    // Support skills can provide bonus to self, like Improve concentration
    // Passive skills can provide bonus to self, like demon bane
    // Offensive skills can provide bonus to self, like spiral spear
    #[inline(always)]
    fn bonuses_to_self(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Support skills can provide bonus to self, like Increase Agi
    #[inline(always)]
    fn bonuses_to_target(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Support skills can provide bonus to self, like Maximize power
    #[inline(always)]
    fn bonuses_to_party(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Type in packet to send to client. it is use by the client to determine if
    // target is valid or if it should display specific cursor
    #[inline(always)]
    fn client_type(&self) -> usize {
        self.target_type().value()
    }

    // Increase chance of success of skills, like FullStripe
    #[inline(always)]
    fn success_chance(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot) -> f32 {
        100.0
    }

    #[inline(always)]
    fn dispell_skills(&self) -> Vec<u32> {
        vec![]
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        false
    }
    #[inline(always)]
    fn has_bonuses_to_target(&self) -> bool {
        false
    }
    #[inline(always)]
    fn has_bonuses_to_party(&self) -> bool {
        false
    }
}

/// Category views for call sites that need an `Option`. They follow `skill_type()` and `target_type()`, so no skill
/// can disagree with its own category.
impl dyn Skill + '_ {
    pub fn as_offensive_skill(&self) -> Option<&dyn Skill> {
        self.is_offensive_skill().then_some(self)
    }
    pub fn as_supportive_skill(&self) -> Option<&dyn Skill> {
        self.is_supportive_skill().then_some(self)
    }
    pub fn as_interactive_skill(&self) -> Option<&dyn Skill> {
        self.is_interactive_skill().then_some(self)
    }
    pub fn as_performance_skill(&self) -> Option<&dyn Skill> {
        self.is_performance_skill().then_some(self)
    }
    pub fn as_passive_skill(&self) -> Option<&dyn Skill> {
        self.is_passive_skill().then_some(self)
    }
    pub fn as_ground_skill(&self) -> Option<&dyn Skill> {
        self.is_ground_skill().then_some(self)
    }
}
