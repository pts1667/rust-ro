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
mod ground;
pub use ground::{GroundKind, GroundPlacement};


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
    /// Crusader's Devotion on the target.
    Devotion,
    /// Venom Splasher's status on the target; boss monsters are immune.
    VenomSplasher,
    /// Confusion, then the caster's Wink of Charm on the target.
    WinkCharm,
    /// Draws a random Tarot Card of Fate effect for the target.
    Tarot,
    /// Cancels the target's cast, or drains SP when the target has Magic Rod.
    SpellBreaker,
    /// Shows a monster's stats to the caster.
    Estimation,
    /// Opens the identify list for the caster.
    Identify,
    /// Casts a random skill from the Abracadabra table.
    HocusPocus,
    /// Gives the caster a stone item.
    FindStone,
    /// Enchant Arms: the weapon element status on the target.
    EnchantArms,
    /// Orcish status on the target.
    ReverseOrcish,
    /// Ninja Shadow Jump: moves the caster to a ground point.
    ShadowLeap,
    /// Creator Condensed Potion: splits a potion into a ground heal.
    CondensedPotion,
    /// Creator Plant Cultivation: grows a plant on the ground.
    Cultivate,
    /// Greed: collects the items around the caster.
    CollectItems,
    /// Wedding Bond: links the caster to the family baby.
    BondBaby,
    /// Adds spirit spheres for the caster, following the grant rule.
    Spheres(SphereGrant),
    /// Taekwon High Jump: moves the caster forward.
    HighJump,
    /// Taekwon Mission: starts a Taekwon mission on the caster.
    Mission,
    /// Taekwon Run: toggles running for the caster.
    Run,
    /// A request handled by the script world: vending, cart, homunculus or family calls.
    ServiceCall(ServiceCall),
    /// Removes a trap the caster controls, or springs it when `spring` is set.
    TrapControl { spring: bool },
    /// Martyr's Reckoning: a hit that the caster takes for an ally.
    Martyr,
    /// Snatch: a monster hit may warp the target away.
    Intimidate,
    /// Gloria Domini: drains a share of the target's SP.
    Pressure,
    /// Benedictio: cures the target's status effects around the caster.
    Benedictio,
    /// Envenom: spends the poison reaction on a hit.
    PoisonReact,
    /// Bowling Bash: hits the target and the monsters in the line it knocks back into.
    Bowling,
    /// Water Ball: a ball that flies from the caster to the target cell.
    WaterBall,
    /// Timed status the skill puts on its target, with the rules for its rate and weapon.
    Support(SupportProfile),
    /// Blessing and Increase Agi damage an undead target instead of buffing it.
    UndeadBuffDamage,
    /// Party-wide status: every party member in range receives it, the caster included.
    PartyBuff,
    /// Song or dance: hands its status out on each pulse, or harasses enemies with damage or drain.
    Performance(PerformanceProfile),
    /// Amp: ends the dance once it has lasted long enough to adapt.
    Adaptation,
    /// Longing for Freedom: lets a dancer keep dancing out of an ensemble.
    LongingFreedom,
    /// Encore: repeats the last performance for half its SP.
    Encore,
    /// Cast Cancel: ends the caster's own cast.
    CastCancel,
}

/// The formula a skill's damage uses when it is neither the weapon nor the magic one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscDamage {
    /// Blitz Beat: one falcon strike per hit.
    FalconStrike,
    /// Falcon Assault: a five-strike volley that scales with its level.
    FalconAssault,
    Pressure,
    /// Counts as a weapon hit.
    Throwstone,
}

/// The rate bonus a Friend of the Sun, Moon and Stars gives to the next roll of this skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendShare {
    TripleAttack,
    Counter,
}

/// A check the server runs before a skill effect is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectValidation {
    Splasher,
    Devotion,
}

/// The unit a summon skill creates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummonKind {
    MarineSphere,
    Flora,
}

/// The part of a skill's requirements that is paid when the cast completes, not when it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementDeferral {
    Nothing,
    Sp,
    Removals,
    Everything,
}

/// What a weapon hit does after it lands, beyond its damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponAftermath {
    Backstab,
    FinalStrike,
    DelayedHit,
}

/// How a Magic Rod treats a skill's magic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicRodRule {
    Standard,
    /// Water Ball is a unit, so Magic Rod absorbs it, and the gain falls off with its level.
    WaterBall,
}

/// The Taekwon stance a kick readies: the status the Taekwon waits in, and the percent chance a normal attack opens the kick's window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stance {
    pub ready: StatusChangeKind,
    pub rate: u32,
    /// The Friend bonus of this kick raises its rate.
    pub friend_boosted: bool,
}

/// The combo skills a skill may follow. `standing` allows it with no combo open, unless the caster is blocked from moving.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboFollows {
    pub after: Vec<u32>,
    pub standing: bool,
}

/// A combo skill this one opens the window for, when the caster has learned it and has enough spirit spheres.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComboLink {
    pub next: u32,
    pub min_spheres: u8,
    pub needs_explosion: bool,
}

/// The effects a Soul Linker spirit gives the skills of its owner class. `values[1]` of the Spirit status holds the owner's skill id.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpiritRules {
    /// Multiplies the SP that autocast skills take.
    pub autocast_sp_multiplier: Option<u32>,
    /// Adds the caster's base level to the bonus of Aid Potion.
    pub alchemy_base_level_bonus: bool,
    /// Lets Estin family buffs reach any Soul Linker.
    pub links_soul_linker_targets: bool,
    /// Keeps the spirit's own statuses from being dispelled.
    pub dispel_immune: bool,
    /// Lowers the Chase Walk value by this much.
    pub chase_walk_penalty: i32,
    /// Makes Chase Walk last ten times longer.
    pub long_chase_walk: bool,
    /// Raises the base stats of the High spirit.
    pub high_stat_bonus: bool,
    /// Keeps SP regeneration while Explosion Spirits is active.
    pub explosion_sp_regen: bool,
    /// Adds the caster's STR to Beast Bane.
    pub beast_bane_strength: bool,
}

/// The success rule a recipe skill crafts by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftingRule {
    /// Weapon-material tempering, with its own formula per material.
    Forge,
    /// Create Deadly Poison, from the caster's DEX and LUK.
    DeadlyPoison,
    /// Always succeeds.
    FixedSuccess,
    /// Pharmacy, from its own learned skills and the potion.
    Pharmacy,
}

/// The window or instant craft a menu skill opens for its caster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKind {
    ElementalConverter,
    MakingArrow,
    WeaponRefine,
    AutoSpell,
    StarPlace,
    RepairWeapon,
    Pharmacy,
    HolyWater,
    DeadlyPoison,
}

/// How a skill's costs differ from its metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CostRules {
    /// Pays no HP (Magnum Break).
    pub no_hp: bool,
    /// Pays no zeny (Throw Zeny).
    pub no_zeny: bool,
    /// Soul Linker's Kaina lowers the SP cost.
    pub kaina_sp_reduction: bool,
    /// Unfair Trick lowers the zeny cost by a tenth (Mammonite).
    pub unfair_trick_zeny: bool,
    /// Not blocked by Into Abyss, which requires gemstones (Ganbantein).
    pub abyss_exempt: bool,
    /// Takes no item costs at all (Call Homunculus).
    pub no_item_costs: bool,
    /// Consumes one item per skill level (pitchers and Cultivation).
    pub item_per_level: bool,
    /// Only the autocast cost pays SP (Holy Light).
    pub autocast_sp: bool,
}

/// The status an area skill applies: its chance (hundredths of a percent) and when it lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AreaStatusProfile {
    pub kind: StatusChangeKind,
    pub chance: i32,
    pub delay_ms: u128,
    pub duration: AreaDuration,
    /// Party members resist with a quarter of the chance and take the plain duration.
    pub party_quarter_chance: bool,
    /// Only undead and demon monsters are affected.
    pub undead_only: bool,
    /// The chance follows the caster's and the target's base levels.
    pub level_rate: bool,
}

/// Which duration the area status lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaDuration {
    /// The skill's duration for its level, with the status's own adjustment.
    Adjusted,
    /// The skill's plain duration for its level.
    Plain,
    /// Until it is removed.
    Permanent,
}

/// How a skill adds spirit spheres.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphereGrant {
    /// One sphere valued at the skill level.
    Level,
    /// Five spheres valued at 5.
    Five,
    /// One sphere valued at 10 on a level-based chance; otherwise one sphere is removed.
    Glitter,
}

/// The script world request an actor makes for the caster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceCall {
    Vending,
    Pushcart,
    CallHomunculus,
    RestHomunculus,
    ResurrectHomunculus,
    CallPartner,
    CallBaby,
    CallParents,
}

/// Parameters of [`ActorBehaviour::Support`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SupportProfile {
    /// How the rate of the status is decided.
    pub chance: SupportChance,
    /// The target must have an equipped weapon (elemental endows).
    pub needs_weapon: bool,
}

/// The rate rule of a timed support skill, out of 10000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SupportChance {
    #[default]
    Certain,
    Fixed(u16),
    /// Depends on the caster's and the target's base levels.
    Provoke,
    /// Depends on the caster's base level and intelligence.
    DecreaseAgi,
    /// Grows with the skill level.
    Endow,
}

/// A map flag that blocks a skill on that map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapRestriction {
    NoVending,
    NoTeleport,
    NoWarp,
    NoIceWall,
}

/// Parameters of [`ActorBehaviour::Performance`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformanceProfile {
    /// The status the performance hands out on each pulse.
    pub status: StatusChangeKind,
    pub reach: PerformanceReach,
    /// Needs a partner standing next to the caster.
    pub ensemble: bool,
    pub effect: PerformanceEffect,
    /// The lesson skill whose level adds to the performance values.
    pub lesson: models::enums::skill_enums::SkillEnum,
}

/// Who a performance reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceReach {
    /// Everybody on a normal map, only the party where players fight each other.
    Everyone,
    Anyone,
    Party,
    Enemies,
}

/// What a performance does on each pulse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceEffect {
    /// Hands the status to everybody in reach.
    Aura,
    /// Damage to every enemy in the area (Unchained Serenade).
    Damage,
    /// SP drain on enemy players (Hip Shaker).
    Drain,
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
    /// Es magic: a player target is stunned instead, unless the server allows Es magic on players.
    pub es_magic: bool,
}

/// What a class-specific skill does to the player or monster it lands on. The server's class dispatch runs one arm per variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassEffect {
    /// Drains the target's spirit spheres into SP.
    AbsorbSpirits,
    /// Moves the caster's spirit spheres to a party member.
    KiTranslation,
    FullProtection,
    Redemptio,
    Marionette,
    /// `slot` indexes the remembered map for the comfort's day.
    StarComfort { slot: usize, kind: StatusChangeKind },
    /// Hatred of the Sun, Moon and Stars, monsters only.
    StarHate,
    /// Shares the partner's max HP when `hp`, otherwise max SP.
    ConjugalShare { hp: bool },
    AidPotion,
    AidBerserkPotion,
    Twilight(TwilightStage),
    Question,
    Gravity,
    LevelUp,
    InstantDeath,
    FullRecovery,
    Coma,
    Fortune,
    SummonMonster,
    /// Kills a monster outright.
    Death,
    ElementChange(Element),
    /// Turns a monster into a random monster of its class, or Poring when `monocell`.
    ClassChange { monocell: bool },
    HpConversion,
    SoulChange,
    MindBreaker,
    StealItem,
    StealCoin,
    /// The Soul Linker's class soul link, held on the target as `Spirit`.
    SoulLink,
    /// Soul Linker's party buffs.
    SoulLinkBuff(StatusChangeKind),
    /// Estin's debuffs on monsters; casting one on a player punishes the caster.
    Estin(StatusChangeKind),
    /// Taekwon stance or Tumbling, held until cast again.
    Stance(StatusChangeKind),
    SevenWind,
}

/// The materials of one Twilight Alchemy stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwilightStage {
    WhitePotion,
    WhiteSlimPotion,
    /// Needs 200 Empty Bottles in the inventory, then makes the Alcohol, Acid and Fire Bottles.
    Bottles,
}

/// What a homunculus or mercenary skill does when it resolves. Read only by the companion dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionEffect {
    /// Sets a trap on the target cell.
    Trap,
    /// Homunculus passive; casting does nothing.
    Passive,
    /// Gives the companion itself a status.
    SelfStatus(StatusChangeKind),
    /// Gives the companion and its owner the same status.
    SelfAndOwnerStatus(StatusChangeKind),
    /// Fully heals the companion and gives it the change status.
    Transform,
    /// Heals the owner, boosted by the companion's Brain Surgery.
    OwnerHeal,
    /// Chance to swap places with the owner.
    Castle,
    /// Heals a random recipient among the companion, its owner and the enemy it is fighting.
    ChaoticHeal,
    /// Ends these statuses on the target.
    TargetCure(&'static [StatusChangeKind]),
    /// Ends these statuses on the companion.
    SourceCure(&'static [StatusChangeKind]),
    /// Gives the owner the companion's HP, then the companion self-destructs.
    Scapegoat,
    /// Silences the target, or ends its silence when already silenced.
    LexDivina,
    MercenaryDecreaseAgi,
    MercenaryProvoke,
    /// Devotion on the owner, the owner must be within ten levels.
    Devotion,
    Moonlight,
    /// Intimacy-scaled damage that resets the intimacy.
    IntimacyStrike,
    /// Self-destructive explosion that needs high intimacy.
    BioExplosion,
    /// A random homunculus bolt spell.
    Caprice,
    /// Mercenary weapon skill with its own formula.
    Weapon(MercenaryWeapon),
}

/// The mercenary weapon skills whose damage formula the companion dispatch computes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MercenaryWeapon {
    Bash,
    Magnum,
    BowlingBash,
    Double,
    ChargeArrow,
    SharpShooting,
    Pierce,
    Brandish,
    SpiralPierce,
    Crash,
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
    /// How this skill's costs differ from its metadata. Read by requirements and autocast.
    #[inline(always)]
    fn cost_rules(&self) -> CostRules {
        CostRules::default()
    }
    /// The map flag that blocks this skill, if any.
    #[inline(always)]
    fn map_restriction(&self) -> Option<MapRestriction> {
        None
    }
    /// The ground unit this skill leaves on the map, if any.
    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        None
    }
    /// How this skill places itself on the ground, when its unit alone does not say.
    #[inline(always)]
    fn ground_placement(&self) -> Option<GroundPlacement> {
        None
    }
    /// True when the skill needs a player-side callback that actors do not run, so actors cannot cast it.
    #[inline(always)]
    fn player_only_callback(&self) -> bool {
        false
    }
    /// The damage a monster weapon attack deals, as a fraction of its weapon attack, when the skill is one.
    #[inline(always)]
    fn weapon_ratio(&self, _level: u8) -> Option<f32> {
        None
    }
    /// The success rule of the recipe this skill crafts, if it has one.
    #[inline(always)]
    fn crafting_rule(&self) -> Option<CraftingRule> {
        None
    }
    /// The multiplier of the skill's metadata magic damage, when the skill has its own formula.
    #[inline(always)]
    fn magic_modifier(&self, _target_small: bool, _source_base_level: u32) -> Option<f32> {
        None
    }
    /// The menu this skill opens for its caster, if any.
    #[inline(always)]
    fn menu(&self) -> Option<MenuKind> {
        None
    }
    /// True when a pet can use the skill on a ground point.
    #[inline(always)]
    fn pet_ground_attack(&self) -> bool {
        false
    }
    /// Added to the hit rate of the skill, in percent.
    #[inline(always)]
    fn hit_rate_bonus(&self) -> i32 {
        0
    }
    /// The Soul Linker spirit whose owner shortens this skill's delay.
    #[inline(always)]
    fn spirit_owner(&self) -> Option<u32> {
        None
    }
    /// Always hits while the caster's spirit is owned by the spirit owner.
    #[inline(always)]
    fn spirit_auto_hit(&self) -> bool {
        false
    }
    /// The skill's own delayed handler applies the damage, not its cast.
    #[inline(always)]
    fn deferred_damage(&self) -> bool {
        false
    }
    /// Counts as a weapon skill for hit and damage rules.
    #[inline(always)]
    fn counts_as_weapon(&self) -> bool {
        self.is_physical()
    }
    /// Adds the weapon battle flag to the damage it deals.
    #[inline(always)]
    fn adds_weapon_flag(&self) -> bool {
        false
    }
    /// The formula the damage uses instead of the weapon or magic one.
    #[inline(always)]
    fn misc_damage(&self) -> Option<MiscDamage> {
        None
    }
    /// Uses the Grand Cross formula.
    #[inline(always)]
    fn grand_cross_damage(&self) -> bool {
        false
    }
    /// A caster hitting itself with this skill takes the weapon battle flag.
    #[inline(always)]
    fn self_hit_counts_as_weapon(&self) -> bool {
        false
    }
    /// The attack ratio the skill uses instead of its own, for a bow or any other weapon.
    #[inline(always)]
    fn attack_ratio(&self, _bow: bool) -> Option<f32> {
        None
    }
    /// The raw attack is taken from the caster's HP and STR.
    #[inline(always)]
    fn attack_from_hp(&self) -> bool {
        false
    }
    /// Flat attack added per skill level before the element applies.
    #[inline(always)]
    fn flat_attack_per_level(&self) -> u16 {
        0
    }
    /// Takes the mastery bonus of the weapon.
    #[inline(always)]
    fn stacks_mastery(&self) -> bool {
        true
    }
    /// Takes the weapon refine bonus.
    #[inline(always)]
    fn stacks_refine(&self) -> bool {
        self.stacks_mastery()
    }
    /// Takes the forged star damage of the weapon.
    #[inline(always)]
    fn stacks_forged_stars(&self) -> bool {
        true
    }
    /// Takes the damage of the caster's spirit spheres.
    #[inline(always)]
    fn stacks_spirit_spheres(&self) -> bool {
        true
    }
    /// Each hit adds one spirit sphere worth of damage.
    #[inline(always)]
    fn adds_hits_to_spheres(&self) -> bool {
        false
    }
    /// Leaves out the Berserk and Overthrust bonuses of the weapon ratio.
    #[inline(always)]
    fn skips_weapon_ratio_bonuses(&self) -> bool {
        false
    }
    /// Consumes ammo even when the weapon is not a ranged one.
    #[inline(always)]
    fn uses_ammo(&self) -> bool {
        false
    }
    /// Adds three times the Tobidougu level to the attack.
    #[inline(always)]
    fn scales_with_tobidougu(&self) -> bool {
        false
    }
    /// Leaves out the Hilt Binding attack bonus.
    #[inline(always)]
    fn excludes_hilt_binding(&self) -> bool {
        false
    }
    /// The skill gives the caster a Double Attack chance from its level.
    #[inline(always)]
    fn grants_double_attack(&self) -> bool {
        false
    }
    /// Reflect Shield does not apply to damage from this skill.
    #[inline(always)]
    fn bypasses_reflect_shield(&self) -> bool {
        false
    }
    /// Devotion does not pass this skill's damage to its protector.
    #[inline(always)]
    fn skips_devotion_protection(&self) -> bool {
        false
    }
    /// The damage lands at the time the attack was made, not when it is applied.
    #[inline(always)]
    fn lands_at_attack_time(&self) -> bool {
        false
    }
    /// Kyrie Eleison absorbs this skill's damage even when it is not physical.
    #[inline(always)]
    fn kyrie_absorbs(&self) -> bool {
        false
    }
    /// Ends Kyrie Eleison when the skill lands.
    #[inline(always)]
    fn breaks_kyrie(&self) -> bool {
        false
    }
    /// How Magic Rod treats this skill.
    #[inline(always)]
    fn magic_rod_rule(&self) -> MagicRodRule {
        MagicRodRule::Standard
    }
    /// What the hit does after it lands.
    #[inline(always)]
    fn weapon_aftermath(&self) -> Option<WeaponAftermath> {
        None
    }
    /// The caster must stand behind the target.
    #[inline(always)]
    fn requires_behind_target(&self) -> bool {
        false
    }
    /// Casting this skill keeps the Magic Power status.
    #[inline(always)]
    fn keeps_magic_power(&self) -> bool {
        false
    }
    /// The part of the requirements that waits until the cast completes, at this level.
    #[inline(always)]
    fn deferred_requirement(&self, _level: u8) -> RequirementDeferral {
        RequirementDeferral::Nothing
    }
    /// The effect of this skill depends on a condition that is checked when it completes.
    #[inline(always)]
    fn conditional_completion(&self) -> bool {
        false
    }
    /// The pending item skill of this skill identifies an item.
    #[inline(always)]
    fn identifies_items(&self) -> bool {
        false
    }
    /// Casting this skill needs an active Estin or Estun readiness effect.
    #[inline(always)]
    fn requires_sma_readiness(&self) -> bool {
        false
    }
    /// A failed cast sends its own failure packet.
    #[inline(always)]
    fn sends_failure_packet(&self) -> bool {
        false
    }
    /// Castable while the caster is under Chase Walk.
    #[inline(always)]
    fn allowed_while_chase_walking(&self) -> bool {
        false
    }
    /// Casting another skill does not end Cloaking.
    #[inline(always)]
    fn keeps_cloaking(&self) -> bool {
        false
    }
    /// Casting below this learned level needs an adjacent wall.
    #[inline(always)]
    fn wall_needed_below_level(&self) -> Option<u8> {
        None
    }
    /// The unit a summon skill creates.
    #[inline(always)]
    fn summon_kind(&self) -> Option<SummonKind> {
        None
    }
    /// The check the server runs before this skill's effect is applied.
    #[inline(always)]
    fn validates_effect(&self) -> Option<EffectValidation> {
        None
    }
    /// Controls a Marionette rather than being under one.
    #[inline(always)]
    fn controls_marionette(&self) -> bool {
        false
    }
    /// An ensemble dance in progress counts for Longing for Freedom.
    #[inline(always)]
    fn ensemble_counts_for_longing(&self) -> bool {
        true
    }
    /// The combo skills this one may follow.
    #[inline(always)]
    fn combo_follows(&self) -> Option<ComboFollows> {
        None
    }
    /// The combo skills this one opens the window for.
    #[inline(always)]
    fn combo_links(&self) -> Vec<ComboLink> {
        Vec::new()
    }
    /// A combo skill that the caster can chain from a combo window.
    #[inline(always)]
    fn combo_chain_ready(&self) -> bool {
        false
    }
    /// A Taekwon kick that cannot follow this combo skill.
    #[inline(always)]
    fn blocks_kick_chain(&self) -> bool {
        false
    }
    /// The Taekwon stance this kick readies.
    #[inline(always)]
    fn stance(&self) -> Option<Stance> {
        None
    }
    /// The skill this one shares its Friend bonus with.
    #[inline(always)]
    fn friend_share(&self) -> Option<FriendShare> {
        None
    }
    /// The effects this skill gives while it is a Soul Linker spirit's owner.
    #[inline(always)]
    fn spirit_rules(&self) -> SpiritRules {
        SpiritRules::default()
    }
    /// The skill needs its caster to stand in water or Deluge.
    #[inline(always)]
    fn needs_water_or_deluge(&self) -> bool {
        false
    }
    /// Hostility checks do not apply to the caster's party members.
    #[inline(always)]
    fn party_targets_skip_hostility(&self) -> bool {
        false
    }
    /// Undead targets take this skill's healing as damage.
    #[inline(always)]
    fn heals_undead_as_damage(&self) -> bool {
        false
    }
    /// Damage from this skill passes the guards and barriers of its target.
    #[inline(always)]
    fn ignores_damage_guards(&self) -> bool {
        false
    }
    /// The Armor Change this skill applies is a debuff.
    #[inline(always)]
    fn inverts_armor_change(&self) -> bool {
        false
    }
    /// The two numbers a song hands its status, `values[1]` and `values[2]`.
    #[inline(always)]
    fn performance_values(&self, _level: i32, _stats: &StatusSnapshot, _lesson: i32) -> (i32, i32) {
        (0, 0)
    }
    /// The damage or drain a harassing performance applies on each pulse.
    #[inline(always)]
    fn performance_amount(&self, _level: i32, _stats: &StatusSnapshot, _lesson: i32) -> u32 {
        0
    }
    /// The HP a performance heals its listeners on each pulse.
    #[inline(always)]
    fn performance_heal(&self, _level: i32, _stats: &StatusSnapshot, _lesson: i32) -> u32 {
        0
    }
    /// The status an area skill applies to every enemy around the caster, or `None` if it applies none.
    #[inline(always)]
    fn area_status(&self) -> Option<AreaStatusProfile> {
        None
    }
    /// What a homunculus or mercenary skill does when it resolves. Read only by the companion dispatch.
    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        None
    }
    /// The class-specific effect this skill has on the player or monster it lands on. Read only by the class dispatch.
    #[inline(always)]
    fn class_effect(&self) -> Option<ClassEffect> {
        None
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
