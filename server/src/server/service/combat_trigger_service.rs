use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::mob::{MobClass, MobGroup, MobRace};
use models::enums::skill::SkillTargetType;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusEffect;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::{
    AutoBonus, AutoEffectFlag, AutoSpellFlag, BattleFlag, CombatProc, CombatProcKind, CombatTargetFilter, CombatTrigger, StatusBonus,
    ZenyProcFlag,
};
use models::status_change::StatusChangeKind;
use script_sdk::Value;

use crate::server::service::global_config_service::GlobalConfigService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEffectTarget {
    Owner,
    Other,
    Automatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicReflectionKind {
    Equipment,
    Mirror,
    Kaite,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComaBonuses {
    bonuses: Vec<BonusType>,
}

impl ComaBonuses {
    pub fn from_bonuses(bonuses: &[StatusBonus]) -> Self {
        Self {
            bonuses: bonuses
                .iter()
                .map(|bonus| *bonus.bonus())
                .filter(|bonus| {
                    matches!(
                        bonus,
                        BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(..)
                            | BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(..)
                            | BonusType::WeaponComaAgainstElement(..)
                            | BonusType::WeaponComaAgainstClass(..)
                            | BonusType::WeaponComaAgainstRace(..)
                    )
                })
                .collect(),
        }
    }

    pub fn chance(&self, target: &StatusSnapshot, flags: u32) -> u16 {
        let immune = target.has_mob_capability(models::enums::mob::MobCapability::StatusImmune);
        let class = *target.mob_class();
        self.chance_against(target, class, immune, flags)
    }

    pub fn chance_against(&self, target: &StatusSnapshot, class: MobClass, status_immune: bool, flags: u32) -> u16 {
        if class == MobClass::Battlefield
            || target
                .mob_groups()
                .iter()
                .any(|group| matches!(group, MobGroup::GVG | MobGroup::Battlefield))
        {
            return 0;
        }
        let weapon = flags & BattleFlag::Weapon.as_flag() != 0;
        self.bonuses
            .iter()
            .filter_map(|bonus| match bonus {
                BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(filter, rate) if matches_class(*filter, class) => {
                    Some((*rate * 100.0).round() as i64)
                }
                BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(filter, rate)
                    if !status_immune && matches_race(*filter, *target.race()) =>
                {
                    Some((*rate * 100.0).round() as i64)
                }
                BonusType::WeaponComaAgainstClass(filter, rate) if weapon && matches_class(*filter, class) => Some(i64::from(*rate)),
                BonusType::WeaponComaAgainstRace(filter, rate) if weapon && !status_immune && matches_race(*filter, *target.race()) => {
                    Some(i64::from(*rate))
                }
                BonusType::WeaponComaAgainstElement(filter, rate)
                    if weapon && !status_immune && (*filter == Element::AllElement || filter == target.element()) =>
                {
                    Some(i64::from(*rate))
                }
                _ => None,
            })
            .sum::<i64>()
            .clamp(0, 10000) as u16
    }
}

pub fn coma_chance(source: &StatusSnapshot, target: &StatusSnapshot, flags: u32) -> u16 {
    ComaBonuses::from_bonuses(source.bonuses()).chance(target, flags)
}

const NPC_MAXPAIN_ATK: u32 = 717;

pub fn physical_reflection(owner: &StatusSnapshot, battle_flags: u32, skill_id: u32, damage: u32) -> u32 {
    let weapon = battle_flags & BattleFlag::Weapon.as_flag() != 0;
    if damage == 0 || !weapon && battle_flags & BattleFlag::Magic.as_flag() == 0 {
        return 0;
    }
    let max_pain = owner
        .status_change(StatusChangeKind::MaxPain)
        .filter(|_| skill_id != NPC_MAXPAIN_ATK)
        .map_or(0, |change| (damage as u64).saturating_mul(change.values[0].max(0) as u64) / 10)
        .min(u32::MAX as u64) as u32;
    if !weapon || battle_flags & BattleFlag::Short.as_flag() == 0 {
        return max_pain;
    }
    let card_rate = owner
        .bonuses()
        .iter()
        .filter_map(|bonus| match bonus.bonus() {
            BonusType::PhysicalAttackReflectChancePercentage(percent) => Some(*percent as i64),
            _ => None,
        })
        .sum::<i64>()
        .max(0);
    let shield_rate = if skill_id == SkillEnum::WsCarttermination.id() {
        0
    } else {
        owner
            .status_change(StatusChangeKind::ReflectShield)
            .filter(|change| skill_id != 0 || change.inherited_from.is_none())
            .map_or(0, |change| change.values[1].max(0) as i64)
    };
    (((damage as u64).saturating_mul(card_rate as u64) / 100 + (damage as u64).saturating_mul(shield_rate as u64) / 100).min(u32::MAX as u64) as u32).saturating_add(max_pain)
}

pub fn magic_reflection(owner: &StatusSnapshot, battle_flags: u32, skill_id: u32, rng: &mut fastrand::Rng) -> Option<MagicReflectionKind> {
    if battle_flags & BattleFlag::Magic.as_flag() == 0 {
        return None;
    }
    let direct = GlobalConfigService::instance()
        .find_skill_config(&Value::Number(skill_id as i32))
        .is_some_and(|skill| *skill.target_type() == SkillTargetType::Target);
    let equipment_rate = owner
        .bonuses()
        .iter()
        .filter_map(|bonus| match bonus.bonus() {
            BonusType::MagicAttackReflectChancePercentage(percent) => Some(*percent as i32),
            _ => None,
        })
        .sum::<i32>();
    if direct && chance(equipment_rate.saturating_mul(100), rng) {
        return Some(MagicReflectionKind::Equipment);
    }
    if owner
        .status_change(StatusChangeKind::MagicMirror)
        .is_some_and(|change| chance(change.values[1].saturating_mul(100), rng))
    {
        return Some(MagicReflectionKind::Mirror);
    }
    if owner.status_change(StatusChangeKind::Kaite).is_some_and(|change| change.values[1] > 0) {
        return Some(MagicReflectionKind::Kaite);
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombatEffect {
    CastSkill {
        skill_id: u32,
        level: u16,
        target: CombatEffectTarget,
    },
    /// A cast of the Auto Spell skill, paid with SP.
    AutoSpell {
        skill_id: u32,
        level: u16,
    },
    ApplyStatus {
        effect: StatusEffect,
        rate: u16,
        duration: u32,
        target: CombatEffectTarget,
    },
    RunBonus(AutoBonus),
    Heal {
        hp: i32,
        sp: i32,
    },
    Vanish {
        hp: u32,
        sp: u32,
    },
    Reflect {
        damage: u32,
        magic: bool,
    },
    DropItem {
        item_id: u32,
    },
    DropGroup {
        group_id: u32,
    },
    Zeny(u32),
    NoRecovery {
        duration: u32,
    },
    SetDefense {
        value: i16,
        duration: u32,
        magic: bool,
    },
    BreakEquipment {
        weapon: bool,
    },
    ClassChange,
    Splash {
        radius: u16,
    },
}

pub struct CombatEvent<'a> {
    pub trigger: CombatTrigger,
    pub battle_flags: u32,
    pub skill_id: u32,
    pub damage: u32,
    pub right_hand_damage: Option<u32>,
    pub uses_ammo: bool,
    pub other: &'a StatusSnapshot,
    pub monster_class: MobClass,
    pub monster_level: u32,
}

impl CombatEvent<'_> {
    fn drain_damage(&self) -> u32 {
        self.right_hand_damage.unwrap_or(self.damage).min(self.damage)
    }
}

pub fn resolve(bonuses: &[StatusBonus], event: &CombatEvent<'_>, rng: &mut fastrand::Rng) -> Vec<CombatEffect> {
    let mut effects = Vec::new();
    let physical = event.battle_flags & BattleFlag::Weapon.as_flag() != 0;
    let magic = event.battle_flags & BattleFlag::Magic.as_flag() != 0;
    let melee = event.battle_flags & BattleFlag::Short.as_flag() != 0;
    let normal = event.battle_flags & BattleFlag::Normal.as_flag() != 0;
    if event.trigger == CombatTrigger::Attack
        && event.damage > 0
        && chance(
            i32::from(ComaBonuses::from_bonuses(bonuses).chance_against(
                event.other,
                event.monster_class,
                event.other.has_mob_capability(models::enums::mob::MobCapability::StatusImmune),
                event.battle_flags,
            )),
            rng,
        )
    {
        effects.push(CombatEffect::ApplyStatus {
            effect: StatusEffect::Coma,
            rate: 10000,
            duration: 0,
            target: CombatEffectTarget::Other,
        });
    }
    if event.trigger == CombatTrigger::Attack && physical && normal && event.damage > 0 {
        let radius = bonuses
            .iter()
            .filter_map(|bonus| match bonus.bonus() {
                BonusType::SplashRadius(radius) => Some(i32::from(*radius)),
                _ => None,
            })
            .sum::<i32>();
        if radius > 0 {
            effects.push(CombatEffect::Splash {
                radius: radius.min(u16::MAX as i32) as u16,
            });
        }
    }
    let mut procs = Vec::<CombatProc>::new();
    for bonus in bonuses {
        match *bonus.bonus() {
            BonusType::CombatProc(mut proc, count) if proc.kind != CombatProcKind::Zeny => {
                proc.rate = proc.rate.saturating_mul(count as i32);
                let mut identity = proc;
                identity.rate = 0;
                let existing = procs.iter_mut().find(|existing| {
                    let mut candidate = **existing;
                    candidate.rate = 0;
                    candidate == identity
                        && (!matches!(proc.kind, CombatProcKind::ItemDrop | CombatProcKind::GroupDrop)
                            || existing.rate.signum() == proc.rate.signum())
                });
                if let Some(existing) = existing {
                    existing.rate = existing.rate.saturating_add(proc.rate);
                } else {
                    procs.push(proc);
                }
            }
            BonusType::AutoBonus(bonus, _) if bonus.trigger == event.trigger => {
                if (bonus.trigger != CombatTrigger::Skill || bonus.trigger_skill == event.skill_id)
                    && BattleFlag::matches(bonus.battle_flags, event.battle_flags)
                    && chance(bonus.rate, rng)
                {
                    effects.push(CombatEffect::RunBonus(bonus));
                }
            }
            BonusType::GainHpWhenKillingEnemy(hp) if event.trigger == CombatTrigger::Kill && physical && melee => {
                effects.push(CombatEffect::Heal { hp: hp as i32, sp: 0 })
            }
            BonusType::GainSpWhenKillingEnemy(sp) if event.trigger == CombatTrigger::Kill && physical && melee => {
                effects.push(CombatEffect::Heal { hp: 0, sp: sp as i32 })
            }
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(hp) if event.trigger == CombatTrigger::Kill && magic => {
                effects.push(CombatEffect::Heal { hp: hp as i32, sp: 0 })
            }
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(sp) if event.trigger == CombatTrigger::Kill && magic => {
                effects.push(CombatEffect::Heal { hp: 0, sp: sp as i32 })
            }
            BonusType::SpDrainWhenKillingRace(race, sp)
                if event.trigger == CombatTrigger::Kill && physical && melee && matches_race(race, *event.other.race()) =>
            {
                effects.push(CombatEffect::Heal { hp: 0, sp: sp as i32 })
            }
            BonusType::GainSpWhenHittingEnemy(sp) | BonusType::SpDrainPerHit(sp)
                if event.trigger == CombatTrigger::Attack && physical && event.drain_damage() > 0 =>
            {
                effects.push(CombatEffect::Heal { hp: 0, sp: sp as i32 })
            }
            BonusType::GainHpWhenHittingEnemy(hp) if event.trigger == CombatTrigger::Attack && physical && event.drain_damage() > 0 => {
                effects.push(CombatEffect::Heal { hp: hp as i32, sp: 0 })
            }
            BonusType::SpDrainWhenAttackingRace(race, sp)
                if event.trigger == CombatTrigger::Attack
                    && physical
                    && event.drain_damage() > 0
                    && matches_race(race, *event.other.race()) =>
            {
                effects.push(CombatEffect::Heal { hp: 0, sp: sp as i32 })
            }
            BonusType::HpDrainWhenAttackingPercentage(percent, rate)
                if event.trigger == CombatTrigger::Attack && physical && event.drain_damage() > 0 && chance(rate as i32 * 100, rng) =>
            {
                effects.push(CombatEffect::Heal {
                    hp: percent_of(event.drain_damage(), percent as i16),
                    sp: 0,
                })
            }
            BonusType::SpDrainWhenAttackingPercentage(percent, rate)
                if event.trigger == CombatTrigger::Attack && physical && event.drain_damage() > 0 && chance(rate as i32 * 100, rng) =>
            {
                effects.push(CombatEffect::Heal {
                    hp: 0,
                    sp: percent_of(event.drain_damage(), percent as i16),
                })
            }
            BonusType::ChanceToInflictStatusOnAttackPercentage(effect, percent)
                if event.trigger == CombatTrigger::Attack && physical && chance((percent * 100.0) as i32, rng) =>
            {
                effects.push(CombatEffect::ApplyStatus {
                    effect,
                    rate: 10000,
                    duration: 0,
                    target: CombatEffectTarget::Other,
                })
            }
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(effect, percent)
                if event.trigger == CombatTrigger::Attack && physical && chance((percent * 100.0) as i32, rng) =>
            {
                effects.push(CombatEffect::ApplyStatus {
                    effect,
                    rate: 10000,
                    duration: 0,
                    target: CombatEffectTarget::Owner,
                })
            }
            BonusType::BreakArmorPercentage(percent)
                if event.trigger == CombatTrigger::Attack && physical && chance((percent * 100.0) as i32, rng) =>
            {
                effects.push(CombatEffect::BreakEquipment { weapon: false })
            }
            BonusType::BreakWeaponPercentage(percent)
                if event.trigger == CombatTrigger::Attack && physical && normal && chance(percent as i32 * 100, rng) =>
            {
                effects.push(CombatEffect::BreakEquipment { weapon: true })
            }
            BonusType::ClassChangePercentageOnHit(percent)
                if event.trigger == CombatTrigger::Attack && physical && chance(percent as i32 * 100, rng) =>
            {
                effects.push(CombatEffect::ClassChange)
            }
            BonusType::DropChanceItemIdPercentage(item_id, rate) if event.trigger == CombatTrigger::Kill && chance(rate as i32, rng) => {
                effects.push(CombatEffect::DropItem { item_id })
            }
            _ => {}
        }
    }
    for proc in procs {
        resolve_proc(&mut effects, proc, 1, event, rng);
    }
    resolve_zeny(&mut effects, bonuses, event, rng);
    effects
}

fn resolve_zeny(effects: &mut Vec<CombatEffect>, bonuses: &[StatusBonus], event: &CombatEvent<'_>, rng: &mut fastrand::Rng) {
    if event.trigger != CombatTrigger::Kill {
        return;
    }
    let mut primary = (0_i64, 0_i32);
    let mut additions = (0_i64, 0_i32);
    for bonus in bonuses {
        match *bonus.bonus() {
            BonusType::CombatProc(proc, count) if proc.kind == CombatProcKind::Zeny => {
                let amount = proc.value as i64 * proc.level.signum() as i64;
                if proc.flags & ZenyProcFlag::Additive.as_flag() != 0 {
                    additions.0 = additions.0.saturating_add(amount.saturating_mul(count as i64));
                    additions.1 = additions.1.saturating_add(proc.rate.saturating_mul(count as i32));
                } else if proc.rate > primary.1 {
                    primary = (amount, proc.rate);
                }
            }
            BonusType::GainZenyWhenKillingMonster(amount, rate) if rate as i32 * 100 > primary.1 => {
                primary = (amount as i64, rate as i32 * 100);
            }
            _ => {}
        }
    }
    let amount = primary.0.saturating_add(additions.0);
    let maximum = if amount < 0 {
        amount.saturating_neg().saturating_mul(event.monster_level as i64)
    } else {
        amount
    };
    if maximum > 0 && chance(primary.1.saturating_add(additions.1), rng) {
        effects.push(CombatEffect::Zeny(rng.u32(1..=maximum.min(u32::MAX as i64) as u32)));
    }
}

fn resolve_proc(effects: &mut Vec<CombatEffect>, proc: CombatProc, count: u16, event: &CombatEvent<'_>, rng: &mut fastrand::Rng) {
    if proc.kind == CombatProcKind::Zeny {
        return;
    }
    if proc.trigger != event.trigger
        || (proc.trigger_skill != 0 && proc.trigger_skill != event.skill_id)
        || !BattleFlag::matches(proc.battle_flags, event.battle_flags)
        || !matches_filter(proc.target_filter, event)
    {
        return;
    }
    if event.damage == 0 && event.trigger != CombatTrigger::Skill && event.trigger != CombatTrigger::Kill {
        return;
    }
    if matches!(proc.kind, CombatProcKind::HpDrain | CombatProcKind::SpDrain) && event.drain_damage() == 0 {
        return;
    }
    let rate = if proc.rate < 0 && event.trigger == CombatTrigger::Kill {
        ((-(proc.rate as i64)) * count as i64 * event.monster_level as i64 / 10 + 1).min(i32::MAX as i64) as i32
    } else {
        proc.rate.saturating_mul(count as i32)
    };
    let rate = if proc.kind == CombatProcKind::Spell
        && (event.trigger == CombatTrigger::Attack && event.uses_ammo
            || event.trigger == CombatTrigger::Hit
                && event.battle_flags & BattleFlag::Weapon.as_flag() != 0
                && event.battle_flags & BattleFlag::Long.as_flag() != 0)
    {
        rate / 2
    } else {
        rate
    };
    if proc.kind == CombatProcKind::Status && rate <= 0 || proc.kind != CombatProcKind::Status && !chance(rate, rng) {
        return;
    }
    match proc.kind {
        CombatProcKind::Spell => {
            if proc.flags != u32::MAX && proc.flags & AutoSpellFlag::SkillSelected.as_flag() != 0 {
                let mut level = proc.level.max(1) as u16;
                if level > 1 {
                    let roll = rng.u16(0..100);
                    if roll >= 50 {
                        level /= 2;
                    } else if roll >= 15 {
                        level -= 1;
                    }
                }
                effects.push(CombatEffect::AutoSpell { skill_id: proc.value, level: level.max(1) });
                return;
            }
            let level = if proc.flags != u32::MAX && proc.flags & AutoSpellFlag::RandomLevel.as_flag() != 0 {
                rng.u16(1..=proc.level.max(1) as u16)
            } else {
                proc.level.max(1) as u16
            };
            let target = if proc.flags == u32::MAX {
                CombatEffectTarget::Automatic
            } else if proc.trigger == CombatTrigger::Skill {
                if proc.flags & AutoSpellFlag::OtherTarget.as_flag() != 0 {
                    CombatEffectTarget::Owner
                } else {
                    CombatEffectTarget::Other
                }
            } else if proc.flags & AutoSpellFlag::OtherTarget.as_flag() != 0 {
                CombatEffectTarget::Other
            } else {
                CombatEffectTarget::Owner
            };
            effects.push(CombatEffect::CastSkill {
                skill_id: proc.value,
                level,
                target,
            });
        }
        CombatProcKind::Status => {
            let Ok(effect) = StatusEffect::try_from_value(proc.value as usize) else {
                return;
            };
            if proc.flags & AutoEffectFlag::SelfTarget.as_flag() != 0 {
                effects.push(CombatEffect::ApplyStatus {
                    effect,
                    rate: rate.clamp(0, u16::MAX as i32) as u16,
                    duration: proc.duration,
                    target: CombatEffectTarget::Owner,
                });
            }
            if proc.flags & AutoEffectFlag::OtherTarget.as_flag() != 0 || proc.flags == 0 {
                effects.push(CombatEffect::ApplyStatus {
                    effect,
                    rate: rate.clamp(0, u16::MAX as i32) as u16,
                    duration: proc.duration,
                    target: CombatEffectTarget::Other,
                });
            }
        }
        CombatProcKind::HpDrain => effects.push(CombatEffect::Heal {
            hp: percent_of(event.drain_damage(), proc.level),
            sp: 0,
        }),
        CombatProcKind::SpDrain => effects.push(CombatEffect::Heal {
            hp: 0,
            sp: percent_of(event.drain_damage(), proc.level),
        }),
        CombatProcKind::HpVanish => effects.push(CombatEffect::Vanish {
            hp: percent_of(event.other.max_hp(), proc.level).max(0) as u32,
            sp: 0,
        }),
        CombatProcKind::SpVanish => effects.push(CombatEffect::Vanish {
            hp: 0,
            sp: percent_of(event.other.max_sp(), proc.level).max(0) as u32,
        }),
        CombatProcKind::ItemDrop => effects.push(CombatEffect::DropItem { item_id: proc.value }),
        CombatProcKind::GroupDrop => effects.push(CombatEffect::DropGroup { group_id: proc.value }),
        CombatProcKind::NoRecovery => effects.push(CombatEffect::NoRecovery { duration: proc.duration }),
        CombatProcKind::SetDef | CombatProcKind::SetMdef => effects.push(CombatEffect::SetDefense {
            value: proc.level,
            duration: proc.duration,
            magic: proc.kind == CombatProcKind::SetMdef,
        }),
        CombatProcKind::BreakWeapon | CombatProcKind::BreakArmor => effects.push(CombatEffect::BreakEquipment {
            weapon: proc.kind == CombatProcKind::BreakWeapon,
        }),
        CombatProcKind::ClassChange if event.monster_class != MobClass::Boss => effects.push(CombatEffect::ClassChange),
        CombatProcKind::ClassChange => {}
        CombatProcKind::Zeny => {}
    }
}

fn chance(rate: i32, rng: &mut fastrand::Rng) -> bool {
    rate > 0 && (rate >= 10000 || rng.i32(0..10000) < rate)
}
fn percent_of(amount: u32, percent: i16) -> i32 {
    ((amount as i64 * percent as i64) / 100).clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
fn matches_race(filter: MobRace, race: MobRace) -> bool {
    filter == MobRace::All || filter == race
}
fn matches_class(filter: MobClass, class: MobClass) -> bool {
    filter == MobClass::All || filter == class
}
fn matches_filter(filter: CombatTargetFilter, event: &CombatEvent<'_>) -> bool {
    match filter {
        CombatTargetFilter::Any => true,
        CombatTargetFilter::Race(race) => MobRace::try_from_value(race as usize).is_ok_and(|race| matches_race(race, *event.other.race())),
        CombatTargetFilter::Class(class) => {
            MobClass::try_from_value(class as usize).is_ok_and(|class| matches_class(class, event.monster_class))
        }
        CombatTargetFilter::Monster(id) => id == event.other.job(),
    }
}

#[cfg(test)]
mod tests {
    use models::enums::element::Element;
    use models::enums::size::Size;

    use super::*;

    #[test]
    fn actual_guardian_class_coma_is_distinct_from_race_immunity_and_battlefield_rejection() {
        crate::tests::common::before_all();
        let mut target = target();
        target.set_mob_class(MobClass::Guardian);
        target.set_mob_capabilities(models::enums::mob::MobCapability::StatusImmune.as_flag());
        let coma = ComaBonuses::from_bonuses(&[
            StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                MobClass::Guardian,
                10.0,
            )),
            StatusBonus::new(BonusType::WeaponComaAgainstClass(MobClass::Guardian, 1000)),
            StatusBonus::new(BonusType::WeaponComaAgainstRace(MobRace::All, 10000)),
        ]);
        assert_eq!(coma.chance(&target, BattleFlag::Weapon.as_flag()), 2000);
        assert_eq!(coma.chance(&target, BattleFlag::Misc.as_flag()), 1000);
        target.set_mob_class(MobClass::Battlefield);
        assert_eq!(coma.chance(&target, BattleFlag::Weapon.as_flag()), 0);
        target.set_mob_class(MobClass::Normal);
        target.set_mob_capabilities(0);
        assert_eq!(coma.chance(&target, BattleFlag::Weapon.as_flag()), 10000);
    }

    #[test]
    fn coma_combines_matching_rates_once_and_respects_weapon_and_target_immunities() {
        let mut target = target();
        let bonuses = [
            StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                MobClass::All,
                15.0,
            )),
            StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(
                MobRace::Plant,
                25.0,
            )),
            StatusBonus::new(BonusType::WeaponComaAgainstClass(MobClass::Normal, 1000)),
            StatusBonus::new(BonusType::WeaponComaAgainstRace(MobRace::Plant, 2000)),
            StatusBonus::new(BonusType::WeaponComaAgainstElement(Element::Neutral, 3000)),
        ];
        let coma = ComaBonuses::from_bonuses(&bonuses);
        assert_eq!(
            coma.chance_against(&target, MobClass::Normal, false, BattleFlag::Misc.as_flag()),
            4000
        );
        assert_eq!(
            coma.chance_against(&target, MobClass::Normal, false, BattleFlag::Magic.as_flag()),
            4000
        );
        assert_eq!(
            coma.chance_against(&target, MobClass::Normal, false, BattleFlag::Weapon.as_flag()),
            10000
        );
        assert_eq!(
            coma.chance_against(&target, MobClass::Boss, true, BattleFlag::Weapon.as_flag()),
            1500
        );
        for group in [MobGroup::GVG, MobGroup::Battlefield] {
            target.set_mob_groups(vec![group]);
            assert_eq!(
                coma.chance_against(&target, MobClass::Normal, false, BattleFlag::Weapon.as_flag()),
                0
            );
        }
        target.set_mob_groups(vec![]);
        let combined = [
            StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                MobClass::Normal,
                60.0,
            )),
            StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(
                MobRace::Plant,
                40.0,
            )),
        ];
        let event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: 1,
            damage: 1,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        for seed in 0..32 {
            assert!(matches!(
                resolve(&combined, &event, &mut fastrand::Rng::with_seed(seed)).as_slice(),
                [CombatEffect::ApplyStatus {
                    effect: StatusEffect::Coma,
                    ..
                }]
            ));
        }
        for bonus in bonuses {
            let data = bonus.bonus().serialize_to_sc_data();
            assert_eq!(
                BonusType::deserialize_from_sc_data(data.0, data.1, data.2),
                Some(*bonus.bonus())
            );
        }
    }

    fn target() -> StatusSnapshot {
        StatusSnapshot::new_for_mob(
            1002,
            200,
            20,
            200,
            20,
            1,
            1,
            1,
            1,
            1,
            1,
            10,
            10,
            10,
            10,
            150,
            0,
            0,
            Size::Medium,
            Element::Neutral,
            MobRace::Plant,
            1,
        )
    }

    #[test]
    fn a_timed_magic_mirror_copies_damage_instead_of_using_equipment_reflection() {
        crate::tests::common::before_all();
        let mut status = models::status::Status::default();
        status.active_statuses.push(models::status_change::StatusChange {
            kind: StatusChangeKind::MagicMirror,
            values: [5, 100, 0, 0],
            started_at: 0,
            expires_at: Some(5000),
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        });
        let service = crate::server::service::status_service::StatusService::new(
            GlobalConfigService::instance(),
            crate::tests::common::test_script_vm(),
        );
        let snapshot = service.to_snapshot(&status);
        let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        assert_eq!(
            magic_reflection(&snapshot, flags, SkillEnum::MgFirebolt.id(), &mut fastrand::Rng::with_seed(1)),
            Some(MagicReflectionKind::Mirror)
        );
    }

    #[test]
    fn automatic_spell_rates_distinguish_ammo_attacks_ranged_retaliation_and_skill_triggers() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::Spell, 10000);
        proc.value = SkillEnum::MgFirebolt.id();
        proc.level = 1;
        let mut event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Normal.as_flag(),
            skill_id: 0,
            damage: 100,
            right_hand_damage: None,
            uses_ammo: true,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        for seed in 0..128 {
            let expected = fastrand::Rng::with_seed(seed).i32(0..10000) < 5000;
            let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
            assert_eq!(
                !resolve(&bonuses, &event, &mut fastrand::Rng::with_seed(seed)).is_empty(),
                expected
            );
        }
        event.uses_ammo = false;
        assert_eq!(
            resolve(
                &[StatusBonus::new(BonusType::CombatProc(proc, 1))],
                &event,
                &mut fastrand::Rng::with_seed(1)
            )
            .len(),
            1
        );
        proc.trigger = CombatTrigger::Hit;
        event.trigger = CombatTrigger::Hit;
        for seed in 0..128 {
            let expected = fastrand::Rng::with_seed(seed).i32(0..10000) < 5000;
            assert_eq!(
                !resolve(
                    &[StatusBonus::new(BonusType::CombatProc(proc, 1))],
                    &event,
                    &mut fastrand::Rng::with_seed(seed)
                )
                .is_empty(),
                expected
            );
        }
        proc.trigger = CombatTrigger::Skill;
        proc.battle_flags = BattleFlag::normalize(BattleFlag::Skill.as_flag(), false);
        event.trigger = CombatTrigger::Skill;
        event.battle_flags = BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        event.uses_ammo = true;
        assert_eq!(
            resolve(
                &[StatusBonus::new(BonusType::CombatProc(proc, 1))],
                &event,
                &mut fastrand::Rng::with_seed(1)
            )
            .len(),
            1
        );
    }

    #[test]
    fn status_proc_checks_attack_range_and_applies_both_targets() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::Status, 10000);
        proc.value = StatusEffect::Stun.value() as u32;
        proc.duration = 1500;
        proc.flags = AutoEffectFlag::SelfTarget.as_flag()
            | AutoEffectFlag::OtherTarget.as_flag()
            | AutoEffectFlag::Short.as_flag()
            | AutoEffectFlag::Weapon.as_flag();
        proc.battle_flags = AutoEffectFlag::battle_flags(proc.flags);
        let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
        let mut event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Normal.as_flag(),
            skill_id: 0,
            damage: 100,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        let mut rng = fastrand::Rng::with_seed(1);
        assert!(resolve(&bonuses, &event, &mut rng).is_empty());
        event.battle_flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        assert_eq!(resolve(&bonuses, &event, &mut rng), vec![
            CombatEffect::ApplyStatus {
                effect: StatusEffect::Stun,
                rate: 10000,
                duration: 1500,
                target: CombatEffectTarget::Owner
            },
            CombatEffect::ApplyStatus {
                effect: StatusEffect::Stun,
                rate: 10000,
                duration: 1500,
                target: CombatEffectTarget::Other
            }
        ]);
    }

    #[test]
    fn on_skill_autospell_requires_matching_skill_and_selects_random_level() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Skill, CombatProcKind::Spell, 10000);
        proc.trigger_skill = 19;
        proc.value = 20;
        proc.level = 10;
        proc.flags = AutoSpellFlag::OtherTarget.as_flag() | AutoSpellFlag::RandomLevel.as_flag();
        let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
        let mut event = CombatEvent {
            trigger: CombatTrigger::Skill,
            battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: 18,
            damage: 0,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        let mut rng = fastrand::Rng::with_seed(1);
        assert!(resolve(&bonuses, &event, &mut rng).is_empty());
        event.skill_id = 19;
        assert!(matches!(resolve(&bonuses, &event, &mut rng).as_slice(), [
            CombatEffect::CastSkill {
                skill_id: 20,
                level: 1..=10,
                target: CombatEffectTarget::Owner
            }
        ]));
    }

    #[test]
    fn extra_drops_respect_monster_filter_and_negative_level_rate() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Kill, CombatProcKind::ItemDrop, -1000);
        proc.value = 501;
        proc.target_filter = CombatTargetFilter::Monster(1002);
        let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
        let mut event = CombatEvent {
            trigger: CombatTrigger::Kill,
            battle_flags: 0,
            skill_id: 0,
            damage: 0,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 100,
        };
        let mut rng = fastrand::Rng::with_seed(1);
        assert_eq!(resolve(&bonuses, &event, &mut rng), vec![CombatEffect::DropItem {
            item_id: 501
        }]);
        proc.target_filter = CombatTargetFilter::Race(MobRace::Demon.value() as u16);
        let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
        event.monster_level = 200;
        assert!(resolve(&bonuses, &event, &mut rng).is_empty());
    }

    #[test]
    fn drain_and_vanish_use_damage_and_target_maximum_without_truncating_chance() {
        let target = target();
        let mut hp = CombatProc::new(CombatTrigger::Attack, CombatProcKind::HpDrain, 10000);
        hp.level = 15;
        let mut sp = CombatProc::new(CombatTrigger::Attack, CombatProcKind::SpVanish, 10000);
        sp.level = 30;
        let bonuses = [
            StatusBonus::new(BonusType::CombatProc(hp, 1)),
            StatusBonus::new(BonusType::CombatProc(sp, 1)),
        ];
        let event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: 0,
            skill_id: 0,
            damage: 300,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        assert_eq!(resolve(&bonuses, &event, &mut fastrand::Rng::with_seed(1)), vec![
            CombatEffect::Heal { hp: 45, sp: 0 },
            CombatEffect::Vanish { hp: 0, sp: 6 }
        ]);
    }

    #[test]
    fn duplicate_proc_rates_combine_and_negative_rates_cancel_without_a_second_roll() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::HpDrain, 6000);
        proc.level = 20;
        let mut cancel = proc;
        cancel.rate = -2000;
        let event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: 0,
            skill_id: 0,
            damage: 100,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        assert_eq!(
            resolve(
                &[
                    StatusBonus::new(BonusType::CombatProc(proc, 1)),
                    StatusBonus::new(BonusType::CombatProc(proc, 1)),
                    StatusBonus::new(BonusType::CombatProc(cancel, 1))
                ],
                &event,
                &mut fastrand::Rng::with_seed(2)
            ),
            vec![CombatEffect::Heal { hp: 20, sp: 0 }]
        );
        cancel.rate = -12000;
        assert!(
            resolve(
                &[
                    StatusBonus::new(BonusType::CombatProc(proc, 2)),
                    StatusBonus::new(BonusType::CombatProc(cancel, 1))
                ],
                &event,
                &mut fastrand::Rng::with_seed(2)
            )
            .is_empty()
        );
    }

    #[test]
    fn classic_drain_uses_the_admitted_right_hand_share_and_weapon_skills_are_eligible() {
        let target = target();
        let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::HpDrain, 10000);
        proc.level = 15;
        proc.battle_flags = BattleFlag::normalize(0, false);
        let bonuses = [StatusBonus::new(BonusType::CombatProc(proc, 1))];
        let mut event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
            skill_id: SkillEnum::SmBash.id(),
            damage: 300,
            right_hand_damage: Some(100),
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        let mut rng = fastrand::Rng::with_seed(1);
        assert_eq!(resolve(&bonuses, &event, &mut rng), vec![CombatEffect::Heal { hp: 15, sp: 0 }]);
        event.right_hand_damage = Some(0);
        assert!(resolve(&bonuses, &event, &mut rng).is_empty());
        event.right_hand_damage = None;
        assert_eq!(resolve(&bonuses, &event, &mut rng), vec![CombatEffect::Heal { hp: 45, sp: 0 }]);
    }

    #[test]
    fn normal_splash_is_admitted_once_only_after_positive_weapon_hits() {
        let target = target();
        let bonuses = [StatusBonus::new(BonusType::SplashRadius(2))];
        let mut event = CombatEvent {
            trigger: CombatTrigger::Attack,
            battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag(),
            skill_id: 0,
            damage: 100,
            right_hand_damage: None,
            uses_ammo: false,
            other: &target,
            monster_class: MobClass::Normal,
            monster_level: 1,
        };
        assert_eq!(resolve(&bonuses, &event, &mut fastrand::Rng::with_seed(2)), vec![
            CombatEffect::Splash { radius: 2 }
        ]);
        event.damage = 0;
        assert!(resolve(&bonuses, &event, &mut fastrand::Rng::with_seed(2)).is_empty());
        event.damage = 100;
        event.battle_flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        assert!(resolve(&bonuses, &event, &mut fastrand::Rng::with_seed(2)).is_empty());
    }

    #[test]
    fn physical_reflection_keeps_card_and_skill_rounding_separate_and_limits_devotion_inheritance() {
        let mut owner = target();
        owner.set_bonuses(vec![StatusBonus::new(BonusType::PhysicalAttackReflectChancePercentage(30))]);
        owner.set_active_statuses(vec![models::status_change::StatusChange {
            kind: StatusChangeKind::ReflectShield,
            values: [10, 40, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        assert_eq!(physical_reflection(&owner, flags, 0, 9), 5);
        assert_eq!(physical_reflection(&owner, flags, SkillEnum::WsCarttermination.id(), 100), 30);
        let mut inherited = owner.active_statuses()[0].clone();
        inherited.inherited_from = Some((100, 0));
        owner.set_active_statuses(vec![inherited]);
        assert_eq!(physical_reflection(&owner, flags, 0, 100), 30);
        assert_eq!(physical_reflection(&owner, flags, SkillEnum::SmBash.id(), 100), 70);
        assert_eq!(
            physical_reflection(&owner, BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag(), 0, 100),
            0
        );
        assert_eq!(physical_reflection(&owner, 0, 0, 100), 0);
    }

    #[test]
    fn max_pain_reflects_a_level_scaled_share_of_any_range_physical_hit() {
        let mut owner = target();
        owner.set_active_statuses(vec![models::status_change::StatusChange {
            kind: StatusChangeKind::MaxPain,
            values: [5, 0, 0, 0],
            started_at: 0,
            expires_at: None,
            next_periodic_at: 0,
            flags: 0,
            inherited_from: None,
        }]);
        let ranged = BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag();
        assert_eq!(physical_reflection(&owner, ranged, 0, 200), 100);
        assert_eq!(physical_reflection(&owner, ranged, NPC_MAXPAIN_ATK, 200), 0);
        assert_eq!(physical_reflection(&owner, BattleFlag::Magic.as_flag(), 0, 200), 100);
    }
}
