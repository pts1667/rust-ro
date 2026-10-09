use std::any::Any;

use models::enums::EnumWithNumberValue;
use models::enums::element::Element;
use models::enums::skill::{SkillTargetType, SkillType, UseSkillFailure};
use models::enums::status::StatusEffect;
use models::enums::weapon::AmmoType;
use models::item::NormalInventoryItem;
use models::status::StatusSnapshot;
use models::status_bonus::TemporaryStatusBonuses;
use models::status_change::StatusChangeKind;

pub mod npc;
pub mod skill_enums;
pub mod skills;


type SkillRequirementResult<T> = std::result::Result<T, ()>;

/// When a status infliction starts, relative to the hit that caused it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusDelay {
    Immediate,
    /// Milliseconds after the hit.
    Ms(u32),
    /// Milliseconds after the target's attack motion, computed from the attacker's status.
    AfterAttackMotion(u32),
}

/// A status a hit can inflict. The server rolls `chance` against the target's resistances.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusInfliction {
    pub kind: StatusChangeKind,
    /// Hundredths of a percent, so `10_000` always applies.
    pub chance: u16,
    /// Takes the skill's secondary duration instead of its primary one.
    pub secondary_duration: bool,
    pub values: [i32; 4],
    pub delay: StatusDelay,
    /// Skips character targets.
    pub monsters_only: bool,
}

impl StatusInfliction {
    pub fn primary(kind: StatusChangeKind, chance: i32, level: u8) -> Self {
        Self {
            kind,
            chance: chance.clamp(0, i32::from(u16::MAX)) as u16,
            secondary_duration: false,
            values: [i32::from(level), 0, 0, 0],
            delay: StatusDelay::Immediate,
            monsters_only: false,
        }
    }

    pub fn secondary(kind: StatusChangeKind, chance: i32, level: u8) -> Self {
        Self { secondary_duration: true, ..Self::primary(kind, chance, level) }
    }

    pub fn delayed(self, delay: StatusDelay) -> Self {
        Self { delay, ..self }
    }

    pub fn monsters_only(self) -> Self {
        Self { monsters_only: true, ..self }
    }
}

/// What a status hook sees about one damaging hit.
pub struct HitContext<'a> {
    pub source: &'a StatusSnapshot,
    pub target: &'a StatusSnapshot,
    /// Hits this skill has landed on the target in the current Storm Gust window. Only Storm Gust counts them.
    pub hits_on_target: u8,
    /// False for monsters and companions, which Stone Fling treats differently.
    pub caster_is_player: bool,
}

/// What an actor (monster or NPC) does with a skill beyond its damage and inflicted statuses. The server runs the
/// variant with the parameters the skill sets, so it never matches on the skill name.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActorBehaviour {
    /// Damage and statuses from the other hooks.
    Default,
    /// Applies the skill's statuses to every enemy around a centre: the actor, or the skill's ground point.
    AreaStatus { at_target_point: bool },
    /// Clears the ground graffiti around the skill's ground point.
    EraseGraffiti,
    /// Clears ground units around the actor with a fixed success chance.
    Ganbantein,
    /// Moves a monster to the skill's point.
    BodyRelocation,
    /// Restores the target's HP by the skill's heal amount; undead targets take Holy damage instead.
    Heal,
    /// Restores a fixed amount of HP.
    FixedHeal(u32),
    /// Revives a dead player.
    Resurrect,
    /// Ends the listed statuses on the target. `undead_follow_up`: an undead monster target gets Blind, any other
    /// monster target loses its aggro.
    Cure { kinds: &'static [StatusChangeKind], undead_follow_up: bool },
    /// Ends all the target's status effects, with `chance` percent.
    Dispel { chance: u8 },
    /// Ends the status if the target has it, otherwise applies it for the skill's duration.
    Toggle(StatusChangeKind),
    /// Stone Curse, with the caster's id in the status.
    StoneCurse,
    /// Strips the listed equipment slots from the target. `full` uses the Full Strip success rate.
    Strip { kinds: &'static [StatusChangeKind], full: bool },
    /// Random warp for the actor.
    Teleport,
    /// Pushes a monster back by the metadata's knockback distance.
    BackSlide,
    /// Weapon or magic damage around a centre.
    Splash(SplashProfile),
    /// Damage that lands after `delay_ms` instead of at the attack motion.
    Delayed { delay_ms: u32, always_lands: bool },
    /// Flat weapon damage, with no attack formula.
    FixedWeapon { amount: u32 },
    /// Magic damage from the metadata formula, with Soul Linker's Sma side effects.
    Magic(MagicProfile),
}

/// Parameters of [`ActorBehaviour::Splash`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SplashProfile {
    /// Element the actor's weapon hits with instead of its own.
    pub element: Option<models::enums::element::Element>,
    /// Each recipient takes one blow instead of the skill's hit count.
    pub single_hit: bool,
    /// Weapon damage scales with the distance from the centre.
    pub distance_ratio: bool,
    /// Cells each landed hit pushes the recipient back.
    pub knockback_cells: u16,
    /// Gives the actor a weapon attack element buff before the area hits.
    pub self_element_buff: bool,
    /// Magic damage multiplier for recipients two cells from the centre.
    pub far_magic_scale: Option<f32>,
}

/// Parameters of [`ActorBehaviour::Magic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MagicProfile {
    /// Ends the actor's Sma before the damage.
    pub consumes_sma: bool,
    /// From this skill level, the actor gains Sma after the hit unless it already has it.
    pub grants_sma_from_level: Option<u8>,
}

/// One skill: its identity, requirements, timings, damage and effects.
///
/// Every player skill lives in `skills/<job>/` and every monster skill in `npc/`, one type per skill. The trait gives a
/// default for every hook, so a skill overrides only what differs from the generic behaviour.
///
/// The required methods describe the skill itself. The `is_*_skill` helpers are derived from
/// [`skill_type`](Skill::skill_type) and [`target_type`](Skill::target_type), so a skill never sets them directly.
///
/// # Use of a skill
///
/// 1. The `validate_*` hooks check the caster's SP, HP, ammo, weapon, zeny and items. A failed check stops the cast.
/// 2. The server stores the status-adjusted timings with the `update_*` hooks and reads them back through
///    [`cast_time`](Skill::cast_time) and the `after_cast_*` getters.
/// 3. The damage hooks and [`bonuses_to_target`](Skill::bonuses_to_target) decide the outcome.
///
/// Hooks marked "Not read by the server yet" have no caller outside this crate today.
pub trait Skill: Send + Sync {
    /// Creates the skill at `level`, or `None` if `level` is above [`max_level`](Skill::max_level).
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized;
    /// Returns `self` as [`Any`], so the concrete skill type can be recovered.
    fn as_any(&self) -> &dyn Any;
    /// Category that decides how the server resolves a cast. Drives the `is_*_skill` helpers.
    fn skill_type(&self) -> SkillType;
    /// The level this instance was created with.
    fn level(&self) -> u8;
    /// Skill id in the client's numbering, the same value as the `SkillEnum` variant.
    fn id(&self) -> u32;
    /// Cast range in cells.
    fn range(&self) -> i8;
    /// Highest level the skill can be learned to.
    fn max_level(&self) -> u8;
    /// SP spent on a cast at this level.
    fn sp_cost(&self) -> u16;
    /// Who the skill targets: a single target, the caster, a ground position and so on.
    fn target_type(&self) -> SkillTargetType;
    /// Whether the skill counts as long-range in battle calculations.
    fn is_ranged(&self) -> bool;
    /// Whether the damage is magical.
    fn is_magic(&self) -> bool;
    /// Whether the damage is physical.
    fn is_physical(&self) -> bool;

    /// True when [`skill_type`](Skill::skill_type) is `Offensive`.
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        self.skill_type() == SkillType::Offensive
    }
    /// True when [`skill_type`](Skill::skill_type) is `Support`.
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool {
        self.skill_type() == SkillType::Support
    }
    /// True when [`skill_type`](Skill::skill_type) is `Interactive`.
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool {
        self.skill_type() == SkillType::Interactive
    }
    /// True when [`skill_type`](Skill::skill_type) is `Performance`.
    #[inline(always)]
    fn is_performance_skill(&self) -> bool {
        self.skill_type() == SkillType::Performance
    }
    /// True when [`skill_type`](Skill::skill_type) is `Passive`.
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        self.skill_type() == SkillType::Passive
    }
    /// True when [`target_type`](Skill::target_type) is `Ground`.
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        self.target_type() == SkillTargetType::Ground
    }

    /// Hits per cast. A negative value means the animation shows `|n|` hits while the damage is applied as a single hit
    /// (Vulcan Arrow, Sonic Blow).
    #[inline(always)]
    fn hit_count(&self) -> i8 {
        0
    }
    /// Damage as a multiple of ATK, where `1.0` is 100%. `None` for skills without ATK-based damage.
    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        None
    }
    /// Damage as a multiple of MATK, where `1.0` is 100%. `None` for skills without MATK-based damage.
    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        None
    }
    /// Element of the damage. `Weapon` and `Ammo` take the element of the equipped weapon or ammunition.
    #[inline(always)]
    fn element(&self) -> Element {
        Element::Neutral
    }
    /// Statuses this skill inflicts on a living target when a damaging hit lands. Read for every such hit, from the
    /// player and the monster caster paths alike.
    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
    /// Whether the statuses from [`inflict_status_effect_to_target`](Skill::inflict_status_effect_to_target) are
    /// alternatives: the first one that lands is the only one applied.
    #[inline(always)]
    fn status_alternatives(&self) -> bool {
        false
    }
    /// Whether the server counts consecutive hits on each target for this skill, passed as `hits_on_target`.
    #[inline(always)]
    fn counts_hits_on_target(&self) -> bool {
        false
    }
    /// Whether a landed hit pushes the target back by the metadata's knockback distance.
    #[inline(always)]
    fn knocks_back_on_hit(&self) -> bool {
        true
    }
    /// How an actor (monster or NPC) executes this skill beyond damage and statuses. Read only by the actor path.
    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Default
    }
    /// The status a self or support skill applies, when its metadata does not name one.
    #[inline(always)]
    fn status_kind(&self) -> Option<StatusChangeKind> {
        None
    }
    /// Status effect this skill inflicts on the caster, if any. Not read by the server yet.
    #[inline(always)]
    fn inflict_status_effect_to_self(&self, _status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Option<StatusEffect> {
        None
    }
    /// Damage dealt to the caster when the skill fails. Not read by the server yet.
    #[inline(always)]
    fn damage_if_failed(&self) -> f64 {
        0.0
    }

    /// Ids of the skills whose effect this skill mitigates. Not read by the server yet.
    #[inline(always)]
    fn mitigate_skills(&self) -> Vec<u32> {
        vec![]
    }

    /// The SP to spend, or `Err` when the caster has too little.
    #[inline(always)]
    fn validate_sp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    /// The HP to spend, or `Err` when the caster cannot pay it.
    #[inline(always)]
    fn validate_hp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    /// The ammunition to consume, or `Err` when it is missing or of the wrong type.
    #[inline(always)]
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    /// `Err` when the caster is not in a state the skill needs. Not read by the server yet.
    #[inline(always)]
    fn validate_state(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    /// The zeny to spend, or `Err` when the caster cannot pay it.
    #[inline(always)]
    fn validate_zeny(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    /// The spirit spheres to spend, or `Err` when there are too few. Not read by the server yet.
    #[inline(always)]
    fn validate_spirit_sphere(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    /// The items to consume (`Ok(Some)`), nothing (`Ok(None)`), or the reason the cast fails (`Err`).
    #[inline(always)]
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }
    /// `Err` when `target_type` is not one the skill accepts. Not read by the server yet.
    #[inline(always)]
    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    /// `Err` when the equipped weapon is not one the skill accepts.
    #[inline(always)]
    fn validate_weapon(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    /// `Err` when the caster is out of range for a check beyond the generic one. Not read by the server yet.
    #[inline(always)]
    fn validate_range(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    /// Whether the item requirement is waived for this `state`. Not read by the server yet.
    #[inline(always)]
    fn skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    /// Level-dependent cast time in milliseconds, before status modifiers.
    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        0
    }
    /// Level-dependent after-cast action delay in milliseconds, before status modifiers.
    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    /// Level-dependent after-cast walk delay in milliseconds, before status modifiers.
    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    /// Stores the cast time that [`cast_time`](Skill::cast_time) returns for the current cast.
    #[inline(always)]
    fn update_cast_time(&mut self, _new_value: u32) {}
    /// Stores the delay that [`after_cast_act_delay`](Skill::after_cast_act_delay) returns for the current cast.
    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {}
    /// Stores the delay that [`after_cast_walk_delay`](Skill::after_cast_walk_delay) returns for the current cast.
    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, _new_value: u32) {}

    /// Cast time in milliseconds for the current cast, as set by [`update_cast_time`](Skill::update_cast_time).
    #[inline(always)]
    fn cast_time(&self) -> u32 {
        0
    }
    /// After-cast action delay in milliseconds. The server locks further casting for this long.
    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    /// After-cast walk delay in milliseconds. Stored, but the server does not read it from here yet.
    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }

    /// Status bonuses applied to the caster while the skill's effect lasts, such as Improve Concentration for supportive
    /// skills or Demon Bane for passive skills. Not read by the server yet.
    #[inline(always)]
    fn bonuses_to_self(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    /// Status bonuses applied to the target of a supportive skill, such as Increase AGI. `tick` is the current time.
    #[inline(always)]
    fn bonuses_to_target(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    /// Status bonuses applied to party members, such as Maximize Power. Not read by the server yet.
    #[inline(always)]
    fn bonuses_to_party(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    /// Target-type value sent to the client, which uses it to pick valid targets and cursors. Defaults to the value of
    /// [`target_type`](Skill::target_type).
    #[inline(always)]
    fn client_type(&self) -> usize {
        self.target_type().value()
    }

    /// Success chance in percent against `target_status`, such as Full Strip's chance. Not read by the server yet.
    #[inline(always)]
    fn success_chance(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot) -> f32 {
        100.0
    }

    /// Skill ids this skill removes from the target. Not read by the server yet.
    #[inline(always)]
    fn dispell_skills(&self) -> Vec<u32> {
        vec![]
    }

    /// Whether [`bonuses_to_self`](Skill::bonuses_to_self) returns anything. Not read by the server yet.
    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        false
    }
    /// Whether [`bonuses_to_target`](Skill::bonuses_to_target) returns anything. Not read by the server yet.
    #[inline(always)]
    fn has_bonuses_to_target(&self) -> bool {
        false
    }
    /// Whether [`bonuses_to_party`](Skill::bonuses_to_party) returns anything. Not read by the server yet.
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
