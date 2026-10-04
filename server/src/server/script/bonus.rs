use std::sync::RwLock;

use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::item::ItemGroup;
use models::enums::mob::{MobClass, MobGroup, MobRace};
use models::enums::size::Size;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusEffect;
use models::enums::weapon::WeaponType;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status_bonus::{
    AutoBonus, AutoEffectFlag, BattleFlag, CombatProc, CombatProcKind, CombatTargetFilter, CombatTrigger, ZenyProcFlag,
};
use script_sdk::{Function, Value};

pub struct BonusScriptHandler {
    pub(crate) bonuses: RwLock<Vec<BonusType>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(name: &str, values: &[i32]) -> Vec<Value> {
        std::iter::once(Value::String(name.into()))
            .chain(values.iter().copied().map(Value::Number))
            .collect()
    }

    #[test]
    fn classic_flee_skill_weapon_and_misc_bonuses_keep_signed_persistent_parameters() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus, args("bFleeRate", &[-150])).unwrap();
        host.apply(Function::Bonus, args("bMiscAtkDef", &[35])).unwrap();
        host.apply(Function::Bonus, args("bMiscDef", &[-25])).unwrap();
        host.apply(Function::Bonus2, vec![
            Value::String("bSubSkill".into()),
            Value::String("MG_FIREBOLT".into()),
            Value::Number(55),
        ])
        .unwrap();
        host.apply(Function::Bonus2, args("bSubSkill", &[SkillEnum::SmBash.id() as i32, -250]))
            .unwrap();
        host.apply(
            Function::Bonus2,
            args("bWeaponAtk", &[WeaponType::Sword1H.value() as i32, -70000]),
        )
        .unwrap();
        host.apply(
            Function::Bonus2,
            args("bWeaponDamageRate", &[WeaponType::Dagger.value() as i32, 300]),
        )
        .unwrap();
        let bonuses = host.drain();
        assert_eq!(bonuses, vec![
            BonusType::FleePercentage(-150),
            BonusType::ResistanceMiscAttackPercentage(35),
            BonusType::ResistanceMiscAttackPercentage(-25),
            BonusType::ResistanceSkillIdPercentage(SkillEnum::MgFirebolt.id(), 55),
            BonusType::ResistanceSkillIdPercentage(SkillEnum::SmBash.id(), -250),
            BonusType::ConditionalWeaponAtk(WeaponType::Sword1H, -70000),
            BonusType::ConditionalWeaponDamagePercentage(WeaponType::Dagger, 300),
        ]);
        assert_eq!(bonuses.iter().map(BonusType::serialize_to_sc_data).collect::<Vec<_>>(), vec![
            (169, -150, 0),
            (173, 35, 0),
            (173, -25, 0),
            (170, SkillEnum::MgFirebolt.id() as i32, 55),
            (170, SkillEnum::SmBash.id() as i32, -250),
            (171, WeaponType::Sword1H.value() as i32, -70000),
            (172, WeaponType::Dagger.value() as i32, 300),
        ]);
        for bonus in bonuses {
            let (id, value, parameter) = bonus.serialize_to_sc_data();
            let restored = BonusType::deserialize_from_sc_data(id, value, parameter).unwrap();
            assert_eq!(restored.serialize_to_sc_data(), (id, value, parameter));
        }
        assert!(host.apply(Function::Bonus2, args("bWeaponAtk", &[10000, 5])).is_err());
    }

    #[test]
    fn coma_weapon_filters_and_always_protected_casting_keep_distinct_persistent_values() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus2, args("bWeaponComaEle", &[Element::Fire.value() as i32, 125]))
            .unwrap();
        host.apply(
            Function::Bonus2,
            args("bWeaponComaClass", &[MobClass::Boss.value() as i32, 10000]),
        )
        .unwrap();
        host.apply(Function::Bonus2, args("bWeaponComaRace", &[MobRace::Plant.value() as i32, -25]))
            .unwrap();
        host.apply(Function::Bonus, args("bNoCastCancel", &[1])).unwrap();
        host.apply(Function::Bonus, args("bNoCastCancel2", &[1])).unwrap();
        host.apply(Function::Bonus, args("bDoubleAddRate", &[5])).unwrap();
        let bonuses = host.drain();
        assert_eq!(bonuses, vec![
            BonusType::WeaponComaAgainstElement(Element::Fire, 125),
            BonusType::WeaponComaAgainstClass(MobClass::Boss, 10000),
            BonusType::WeaponComaAgainstRace(MobRace::Plant, -25),
            BonusType::EnableNoCancelCast,
            BonusType::EnableNoCancelCast2,
            BonusType::DoubleAttackAdditionalChancePercentage(5)
        ]);
        for bonus in bonuses {
            let data = bonus.serialize_to_sc_data();
            assert_eq!(BonusType::deserialize_from_sc_data(data.0, data.1, data.2), Some(bonus));
        }
    }

    #[test]
    fn weapon_attack_bonuses_preserve_signed_values_and_remain_distinct_from_base_attack() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus, args("bAtk", &[3])).unwrap();
        host.apply(Function::Bonus, args("bAtk2", &[-70000])).unwrap();
        host.apply(Function::Bonus, args("bBaseAtk", &[5])).unwrap();
        assert_eq!(host.drain(), vec![
            BonusType::WeaponAtk(3),
            BonusType::WeaponRefineAtk(-70000),
            BonusType::Atk(5)
        ]);
        for bonus in [BonusType::WeaponAtk(i32::MAX), BonusType::WeaponRefineAtk(-70000)] {
            let (kind, value, extra) = bonus.serialize_to_sc_data();
            assert_eq!(BonusType::deserialize_from_sc_data(kind, value, extra), Some(bonus));
        }
    }

    #[test]
    fn automatic_spell_decodes_skill_level_chance_and_battle_flags() {
        let host = BonusScriptHandler::new();
        let flags = BattleFlag::Magic.as_flag() | BattleFlag::Skill.as_flag();
        host.apply(Function::Bonus5, args("bAutoSpell", &[19, 5, 7, flags as i32, 3]))
            .unwrap();
        let bonuses = host.drain();
        let [BonusType::CombatProc(proc, 1)] = bonuses.as_slice() else {
            panic!("Expected a typed spell trigger");
        };
        assert_eq!(proc.kind, CombatProcKind::Spell);
        assert_eq!(proc.value, 19);
        assert_eq!(proc.level, 5);
        assert_eq!(proc.rate, 70);
        assert_eq!(proc.flags, 3);
        assert!(BattleFlag::matches(proc.battle_flags, flags | BattleFlag::Long.as_flag()));
        assert!(!BattleFlag::matches(
            proc.battle_flags,
            BattleFlag::Weapon.as_flag() | BattleFlag::Normal.as_flag() | BattleFlag::Long.as_flag()
        ));
    }

    #[test]
    fn attacked_status_bonus_is_not_an_outgoing_attack_trigger() {
        let host = BonusScriptHandler::new();
        host.apply(
            Function::Bonus2,
            args("bAddEffWhenHit", &[StatusEffect::Stun.value() as i32, 500]),
        )
        .unwrap();
        let bonuses = host.drain();
        let BonusType::CombatProc(proc, 1) = bonuses[0] else {
            panic!("Expected an incoming status trigger");
        };
        assert_eq!(proc.trigger, CombatTrigger::Hit);
        assert_eq!(proc.rate, 500);
    }

    #[test]
    fn drain_chance_keeps_tenths_of_a_percent_and_drop_rate_keeps_large_values() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus2, args("bHPDrainRate", &[3, 15])).unwrap();
        host.apply(Function::Bonus2, args("bAddMonsterDropItem", &[501, 10000])).unwrap();
        let bonuses = host.drain();
        let BonusType::CombatProc(drain, 1) = bonuses[0] else {
            panic!("Expected a drain trigger");
        };
        let BonusType::CombatProc(drop, 1) = bonuses[1] else {
            panic!("Expected a drop trigger");
        };
        assert_eq!(drain.rate, 30);
        assert_eq!(drain.level, 15);
        assert_eq!(drop.rate, 10000);
    }

    #[test]
    fn autobonus_registration_uses_compiled_program_ids_and_skill_trigger() {
        let host = BonusScriptHandler::new();
        host.register_auto_bonus(
            Function::AutoBonus3,
            &[10.into(), 1000.into(), 3000.into(), Value::String("MG_FIREBOLT".into()), 11.into()],
            13414,
        )
        .unwrap();
        let bonuses = host.drain();
        let BonusType::AutoBonus(bonus, 1) = bonuses[0] else {
            panic!("Expected compiled automatic bonus");
        };
        assert_eq!(bonus.trigger, CombatTrigger::Skill);
        assert_eq!(bonus.trigger_skill, SkillEnum::MgFirebolt.id());
        assert_eq!(bonus.rate, 10000);
        assert_eq!(bonus.program_id, 10);
        assert_eq!(bonus.visual_program_id, 11);
        assert_eq!(bonus.source_item_id, 13414);
    }

    #[test]
    fn unknown_bonus_reports_an_error_instead_of_silently_disappearing() {
        let host = BonusScriptHandler::new();
        assert!(
            host.apply(Function::Bonus3, args("bTypo", &[1, 2, 3]))
                .unwrap_err()
                .contains("bTypo")
        );
        assert!(host.apply(Function::Bonus, args("bTypo", &[1])).unwrap_err().contains("bTypo"));
        assert!(host.drain().is_empty());
    }

    #[test]
    fn explicit_boolean_bonus_arguments_enable_and_disable_immunity() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus, args("bUnbreakableHelm", &[1])).unwrap();
        host.apply(Function::Bonus, args("bUnbreakableWeapon", &[0])).unwrap();
        assert_eq!(host.drain(), vec![BonusType::UnbreakableHelm]);
    }

    #[test]
    fn break_rates_keep_hundredths_of_a_percent_and_recovery_keeps_large_values() {
        let host = BonusScriptHandler::new();
        host.apply(Function::Bonus, args("bBreakWeaponRate", &[30])).unwrap();
        host.apply(Function::Bonus, args("bAddItemSPHealRate", &[300])).unwrap();
        let bonuses = host.drain();
        let BonusType::CombatProc(proc, 1) = bonuses[0] else {
            panic!("Expected typed break chance");
        };
        assert_eq!(proc.rate, 30);
        assert_eq!(proc.kind, CombatProcKind::BreakWeapon);
        assert_eq!(bonuses[1], BonusType::SpRegenFromItemPercentage(300));
    }
}

fn positive(value: &Value) -> Result<u32, String> {
    let value = value.number_value()?;
    u32::try_from(value).map_err(|_| "Expected a nonnegative number".into())
}

fn skill_id(value: &Value) -> Result<u32, String> {
    if let Value::Number(id) = value {
        let id = u32::try_from(*id).map_err(|_| "Expected a valid skill ID")?;
        return std::panic::catch_unwind(|| SkillEnum::from_id(id).id()).map_err(|_| format!("Unknown skill {id}"));
    }
    let name = value.string_value()?;
    std::panic::catch_unwind(|| SkillEnum::from_name(name).id()).map_err(|_| format!("Unknown skill {name}"))
}

fn effect_id(value: &Value) -> Result<u16, String> {
    let id = positive(value)?;
    StatusEffect::try_from_value(id as usize)?;
    Ok(id as u16)
}

#[macro_export]
macro_rules! bonus {
    ($self:ident, $bonus:expr) => {
        $self.bonuses.write().unwrap().push($bonus)
    };
}

impl BonusScriptHandler {
    pub fn apply(&self, function: Function, params: Vec<Value>) -> Result<(), String> {
        let minimum = match function {
            Function::Bonus => 1,
            Function::Bonus2 | Function::Skill => 3,
            Function::Bonus3 => 4,
            Function::Bonus4 => 5,
            Function::Bonus5 => 6,
            _ => return Err("Expected a bonus function".into()),
        };
        let minimum = if function == Function::Skill { 2 } else { minimum };
        if params.len() < minimum {
            return Err("Not enough bonus arguments".into());
        }
        if function == Function::Skill {
            let skill = skill_id(&params[0])?;
            let level = u8::try_from(params[1].number_value()?).map_err(|_| "Skill grant level is out of range")?;
            bonus!(self, BonusType::EnableSkillId(skill, level));
            return Ok(());
        }
        let bonus_name = params[0].string_value()?.to_owned();
        if self.apply_combat_bonus(function, &params)? {
            return Ok(());
        }
        if matches!(function, Function::Bonus3 | Function::Bonus4 | Function::Bonus5) {
            return Err(format!("Unknown {:?} operation {}", function, params[0].string_value()?));
        }
        let before = self.bonuses.read().unwrap().len();
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match function {
            Function::Bonus => self.handle_bonus(params),
            Function::Bonus2 => self.handle_bonus2(params),
            Function::Bonus3 | Function::Bonus4 | Function::Bonus5 => unreachable!(),
            Function::Skill => unreachable!(),
            _ => unreachable!(),
        }))
        .map_err(|_| "Invalid bonus arguments".to_string())?;
        if self.bonuses.read().unwrap().len() == before {
            return Err(format!("Unknown or invalid {:?} operation {}", function, bonus_name));
        }
        Ok(())
    }

    pub fn register_auto_bonus(&self, function: Function, params: &[Value], source_item_id: u32) -> Result<(), String> {
        if params.len() < 3 {
            return Err("An automatic bonus needs program, rate and duration".into());
        }
        let trigger = match function {
            Function::AutoBonus => CombatTrigger::Attack,
            Function::AutoBonus2 => CombatTrigger::Hit,
            Function::AutoBonus3 => CombatTrigger::Skill,
            _ => return Err("Expected an automatic bonus operation".into()),
        };
        let program_id = positive(&params[0])?;
        let duration = positive(&params[2])?;
        let trigger_skill = if trigger == CombatTrigger::Skill {
            skill_id(params.get(3).ok_or("Skill-triggered bonuses need a triggering skill")?)?
        } else {
            0
        };
        let battle_flags = if trigger == CombatTrigger::Skill {
            0
        } else {
            BattleFlag::normalize(params.get(3).map(Value::number_value).transpose()?.unwrap_or(0) as u32, false)
        };
        let visual_program_id = params.get(4).map(positive).transpose()?.unwrap_or(0);
        let bonus = AutoBonus {
            trigger,
            rate: params[1].number_value()?.saturating_mul(10).clamp(-10000, 10000),
            duration,
            battle_flags,
            trigger_skill,
            program_id,
            visual_program_id,
            source_item_id,
            source_location: 0,
        };
        bonus!(self, BonusType::AutoBonus(bonus, 1));
        Ok(())
    }

    fn apply_combat_bonus(&self, function: Function, params: &[Value]) -> Result<bool, String> {
        let name = params[0].string_value()?.to_ascii_lowercase();
        let arity = params.len() - 1;
        let number = |index: usize| params.get(index).ok_or("Missing bonus argument")?.number_value();
        let put = |proc: CombatProc| {
            self.bonuses.write().unwrap().push(BonusType::CombatProc(proc, 1));
        };
        match name.as_str() {
            "bfleerate" if function == Function::Bonus => bonus!(self, BonusType::FleePercentage(number(1)?)),
            "bmiscatkdef" | "bmiscdef" if function == Function::Bonus => {
                bonus!(self, BonusType::ResistanceMiscAttackPercentage(number(1)?))
            }
            "bsubskill" if function == Function::Bonus2 => {
                bonus!(self, BonusType::ResistanceSkillIdPercentage(skill_id(&params[1])?, number(2)?))
            }
            "bweaponatk" | "bweapondamagerate" if function == Function::Bonus2 => {
                let weapon = WeaponType::try_from_value(positive(&params[1])? as usize)?;
                let value = number(2)?;
                bonus!(
                    self,
                    if name == "bweaponatk" {
                        BonusType::ConditionalWeaponAtk(weapon, value)
                    } else {
                        BonusType::ConditionalWeaponDamagePercentage(weapon, value)
                    }
                );
            }
            "bdoubleaddrate" if function == Function::Bonus => {
                bonus!(
                    self,
                    BonusType::DoubleAttackAdditionalChancePercentage(
                        i16::try_from(number(1)?).map_err(|_| "Additional Double Attack chance is out of bounds")?
                    )
                );
            }
            "bmagicatkele" if function == Function::Bonus2 => {
                let element = Element::try_from_value(positive(&params[1])? as usize)?;
                let value = i16::try_from(number(2)?).map_err(|_| "Magic element damage modifier is out of bounds")?;
                bonus!(self, BonusType::MagicalDamageUsingElementPercentage(element, value));
            }
            "bintravision"
            | "bnocastcancel"
            | "bnocastcancel2"
            | "bnogemstone"
            | "bnoknockback"
            | "bnosizefix"
            | "bnowalkdelay"
            | "brestartfullrecover"
            | "bunbreakablearmor"
            | "bunbreakablegarment"
            | "bunbreakablehelm"
            | "bunbreakableshield"
            | "bunbreakableshoes"
            | "bunbreakableweapon"
            | "bunstripableweapon"
            | "bunstripablearmor"
            | "bunstripablehelm"
            | "bunstripableshield"
            | "bunstripable"
                if function == Function::Bonus && arity <= 1 =>
            {
                let enabled = if arity == 0 { true } else { number(1)? != 0 };
                if enabled {
                    bonus!(self, match name.as_str() {
                        "bintravision" => BonusType::EnableSeeHidden,
                        "bnocastcancel" => BonusType::EnableNoCancelCast,
                        "bnocastcancel2" => BonusType::EnableNoCancelCast2,
                        "bnogemstone" => BonusType::EnableNoGemstoneRequired,
                        "bnoknockback" => BonusType::EnableNoKnockback,
                        "bnosizefix" => BonusType::EnableIgnoreSizeModifier,
                        "bnowalkdelay" => BonusType::EnableNoWalkDelay,
                        "brestartfullrecover" => BonusType::EnableFullHpSpRecoverOnResurrect,
                        "bunbreakablearmor" => BonusType::UnbreakableArmor,
                        "bunbreakablegarment" => BonusType::UnbreakableShoulder,
                        "bunbreakablehelm" => BonusType::UnbreakableHelm,
                        "bunbreakableshield" => BonusType::UnbreakableShield,
                        "bunbreakableshoes" => BonusType::UnbreakableShoes,
                        "bunstripableweapon" => BonusType::UnstripableWeapon,
                        "bunstripablearmor" => BonusType::UnstripableArmor,
                        "bunstripablehelm" => BonusType::UnstripableHelm,
                        "bunstripableshield" => BonusType::UnstripableShield,
                        "bunstripable" => BonusType::Unstripable,
                        _ => BonusType::UnbreakableWeapon,
                    });
                }
            }
            "bbreakweaponrate" | "bbreakarmorrate" | "bclasschange" if function == Function::Bonus => {
                let kind = match name.as_str() {
                    "bbreakweaponrate" => CombatProcKind::BreakWeapon,
                    "bbreakarmorrate" => CombatProcKind::BreakArmor,
                    _ => CombatProcKind::ClassChange,
                };
                let mut proc = CombatProc::new(CombatTrigger::Attack, kind, number(1)?);
                proc.battle_flags = BattleFlag::normalize(0, kind == CombatProcKind::ClassChange);
                put(proc);
            }
            "bgetzenynum" | "baddgetzenynum" if function == Function::Bonus2 => {
                let maximum = number(1)?;
                let mut proc = CombatProc::new(CombatTrigger::Kill, CombatProcKind::Zeny, number(2)?.saturating_mul(100));
                proc.value = maximum.unsigned_abs();
                proc.level = if maximum < 0 { -1 } else { 1 };
                proc.flags = if name == "baddgetzenynum" {
                    ZenyProcFlag::Additive.as_flag()
                } else {
                    0
                };
                put(proc);
            }
            "badditemhealrate" | "badditemsphealrate" if function == Function::Bonus => {
                let value = i16::try_from(number(1)?).map_err(|_| "Item recovery bonus is out of range")?;
                bonus!(
                    self,
                    if name == "badditemhealrate" {
                        BonusType::HpRegenFromItemPercentage(value)
                    } else {
                        BonusType::SpRegenFromItemPercentage(value)
                    }
                );
            }
            "badditemhealrate" | "badditemsphealrate" if function == Function::Bonus2 => {
                let value = i16::try_from(number(2)?).map_err(|_| "Item recovery bonus is out of range")?;
                let id = positive(&params[1])?;
                bonus!(
                    self,
                    if name == "badditemhealrate" {
                        BonusType::HpRegenFromItemIDPercentage(id, value)
                    } else {
                        BonusType::SpRegenFromItemIDPercentage(id, value)
                    }
                );
            }
            "badditemgrouphealrate" | "badditemgroupsphealrate" if function == Function::Bonus2 => {
                let value = i16::try_from(number(2)?).map_err(|_| "Item recovery bonus is out of range")?;
                let group = crate::server::script::game_data::group(&params[1])
                    .ok_or("Unknown recovery item group")?
                    .id;
                bonus!(
                    self,
                    if name == "badditemgrouphealrate" {
                        BonusType::HpRegenFromItemGroupPercentage(group, value)
                    } else {
                        BonusType::SpRegenFromItemGroupPercentage(group, value)
                    }
                );
            }
            "bautospell" | "bautospellwhenhit" if matches!(function, Function::Bonus3 | Function::Bonus4 | Function::Bonus5) => {
                let trigger = if name == "bautospell" {
                    CombatTrigger::Attack
                } else {
                    CombatTrigger::Hit
                };
                let mut proc = CombatProc::new(
                    trigger,
                    CombatProcKind::Spell,
                    number(3)?.saturating_mul(10).clamp(-10000, 10000),
                );
                proc.value = skill_id(&params[1])?;
                proc.level = number(2)?.clamp(1, i16::MAX as i32) as i16;
                proc.flags = if arity == 3 {
                    u32::MAX
                } else {
                    number(if arity == 5 { 5 } else { 4 })? as u32
                };
                let flags = if arity == 5 {
                    number(4)? as u32
                } else if trigger == CombatTrigger::Hit {
                    BattleFlag::Normal.as_flag() | BattleFlag::Skill.as_flag()
                } else {
                    0
                };
                proc.battle_flags = BattleFlag::normalize(flags, true);
                put(proc);
            }
            "bautospellonskill" if matches!(function, Function::Bonus4 | Function::Bonus5) => {
                let mut proc = CombatProc::new(
                    CombatTrigger::Skill,
                    CombatProcKind::Spell,
                    number(4)?.saturating_mul(10).clamp(-10000, 10000),
                );
                proc.trigger_skill = skill_id(&params[1])?;
                proc.value = skill_id(&params[2])?;
                proc.level = number(3)?.clamp(1, i16::MAX as i32) as i16;
                proc.flags = if arity == 5 { number(5)? as u32 } else { u32::MAX };
                put(proc);
            }
            "baddeff" | "baddeff2" | "baddeffwhenhit" if matches!(function, Function::Bonus2 | Function::Bonus3 | Function::Bonus4) => {
                let mut proc = CombatProc::new(
                    if name == "baddeffwhenhit" {
                        CombatTrigger::Hit
                    } else {
                        CombatTrigger::Attack
                    },
                    CombatProcKind::Status,
                    number(2)?.clamp(0, u16::MAX as i32),
                );
                proc.value = effect_id(&params[1])? as u32;
                proc.flags = AutoEffectFlag::normalize(if name == "baddeff2" {
                    AutoEffectFlag::SelfTarget.as_flag()
                } else if arity >= 3 {
                    number(3)? as u32
                } else {
                    0
                });
                proc.battle_flags = AutoEffectFlag::battle_flags(proc.flags);
                proc.duration = if arity >= 4 { positive(&params[4])? } else { 0 };
                put(proc);
            }
            "baddeffonskill" if matches!(function, Function::Bonus3 | Function::Bonus4 | Function::Bonus5) => {
                let mut proc = CombatProc::new(CombatTrigger::Skill, CombatProcKind::Status, number(3)?.clamp(0, 10000));
                proc.trigger_skill = skill_id(&params[1])?;
                proc.value = effect_id(&params[2])? as u32;
                proc.flags = if arity >= 4 {
                    number(4)? as u32
                } else {
                    AutoEffectFlag::OtherTarget.as_flag()
                };
                proc.duration = if arity >= 5 { positive(&params[5])? } else { 0 };
                put(proc);
            }
            "baddmonsterdropitem"
            | "baddmonsterdropitemgroup"
            | "baddmonsteriddropitem"
            | "baddclassdropitem"
            | "baddclassdropitemgroup"
                if matches!(function, Function::Bonus2 | Function::Bonus3) =>
            {
                let group = name.ends_with("group");
                let mut proc = CombatProc::new(
                    CombatTrigger::Kill,
                    if group {
                        CombatProcKind::GroupDrop
                    } else {
                        CombatProcKind::ItemDrop
                    },
                    number(arity)?,
                );
                proc.value = positive(&params[1])?;
                if arity == 3 {
                    proc.target_filter = if name == "baddmonsteriddropitem" {
                        CombatTargetFilter::Monster(positive(&params[2])?)
                    } else if name.starts_with("baddclass") {
                        CombatTargetFilter::Class(number(2)? as u16)
                    } else {
                        CombatTargetFilter::Race(number(2)? as u16)
                    };
                }
                put(proc);
            }
            "bhpdrainrate" | "bspdrainrate" if matches!(function, Function::Bonus | Function::Bonus2) => {
                let (chance, percent) = if arity == 1 {
                    (10000, number(1)?)
                } else {
                    (number(1)?.saturating_mul(10), number(2)?)
                };
                let mut proc = CombatProc::new(
                    CombatTrigger::Attack,
                    if name == "bhpdrainrate" {
                        CombatProcKind::HpDrain
                    } else {
                        CombatProcKind::SpDrain
                    },
                    chance.clamp(0, 10000),
                );
                proc.level = percent.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                proc.battle_flags = BattleFlag::normalize(0, false);
                put(proc);
            }
            "bhpvanishrate" | "bspvanishrate" if matches!(function, Function::Bonus2 | Function::Bonus3) => {
                let mut proc = CombatProc::new(
                    CombatTrigger::Attack,
                    if name == "bhpvanishrate" {
                        CombatProcKind::HpVanish
                    } else {
                        CombatProcKind::SpVanish
                    },
                    number(1)?.saturating_mul(10).clamp(0, 10000),
                );
                proc.level = number(2)?.clamp(0, 100) as i16;
                proc.battle_flags = BattleFlag::normalize(if arity == 3 { number(3)? as u32 } else { 0 }, true);
                put(proc);
            }
            "bexpaddclass" if function == Function::Bonus2 => bonus!(
                self,
                BonusType::GainExpWhenKillingClassPercentage(MobClass::try_from_value(number(1)? as usize)?, number(2)? as i8)
            ),
            "bsubrace2" if function == Function::Bonus2 => bonus!(
                self,
                BonusType::ResistanceDamageFromMobGroupPercentage(MobGroup::try_from_value(number(1)? as usize)?, number(2)? as i8)
            ),
            "bsubele" if function == Function::Bonus3 => bonus!(
                self,
                BonusType::ResistanceDamageFromElementWithFlags(
                    (
                        Element::try_from_value(number(1)? as usize)?,
                        BattleFlag::normalize(number(3)? as u32, false)
                    ),
                    number(2)? as i16
                )
            ),
            "baddele" if function == Function::Bonus3 => bonus!(
                self,
                BonusType::PhysicalDamageAgainstElementWithFlags(
                    (
                        Element::try_from_value(number(1)? as usize)?,
                        BattleFlag::normalize(number(3)? as u32, false)
                    ),
                    number(2)? as i16
                )
            ),
            "bsubrace" if function == Function::Bonus3 => bonus!(
                self,
                BonusType::ResistanceDamageFromRaceWithFlags(
                    (
                        MobRace::try_from_value(number(1)? as usize)?,
                        BattleFlag::normalize(number(3)? as u32, false)
                    ),
                    number(2)? as i16
                )
            ),
            "bhpdrainvalue" if function == Function::Bonus => bonus!(self, BonusType::GainHpWhenHittingEnemy(number(1)? as i16)),
            "bbreakweaponrate" | "bbreakarmorrate" | "bclasschange" if function == Function::Bonus => {
                let kind = match name.as_str() {
                    "bbreakweaponrate" => CombatProcKind::BreakWeapon,
                    "bbreakarmorrate" => CombatProcKind::BreakArmor,
                    _ => CombatProcKind::ClassChange,
                };
                let mut proc = CombatProc::new(CombatTrigger::Attack, kind, number(1)?.clamp(0, 10000));
                proc.battle_flags = BattleFlag::normalize(0, false);
                put(proc);
            }
            "bstatenorecoverrace" if function == Function::Bonus3 => {
                let mut proc = CombatProc::new(CombatTrigger::Attack, CombatProcKind::NoRecovery, number(2)?);
                proc.target_filter = CombatTargetFilter::Race(number(1)? as u16);
                proc.duration = positive(&params[3])?;
                proc.battle_flags = BattleFlag::normalize(0, true);
                put(proc);
            }
            "bsetdefrace" | "bsetmdefrace" if function == Function::Bonus4 => {
                let mut proc = CombatProc::new(
                    CombatTrigger::Attack,
                    if name == "bsetdefrace" {
                        CombatProcKind::SetDef
                    } else {
                        CombatProcKind::SetMdef
                    },
                    number(2)?,
                );
                proc.target_filter = CombatTargetFilter::Race(number(1)? as u16);
                proc.duration = positive(&params[3])?;
                proc.level = number(4)? as i16;
                proc.battle_flags = BattleFlag::normalize(0, true);
                put(proc);
            }
            "bcastrate" | "bcastrate2" if function == Function::Bonus2 => bonus!(
                self,
                BonusType::CastTimeWhenUsingSkillIdPercentage(skill_id(&params[1])?, number(2)? as i8)
            ),
            "bskillatk" | "bskillheal" | "baddskillblow" if function == Function::Bonus2 && params[1].is_number() => {
                let skill = skill_id(&params[1])?;
                let val = number(2)? as i8;
                bonus!(self, match name.as_str() {
                    "bskillatk" => BonusType::SkillIdDamagePercentage(skill, val),
                    "bskillheal" => BonusType::HealSkillIdPercentage(skill, val),
                    _ => BonusType::KnockbackWhenUsingSkillId(skill, val),
                });
            }
            "badddamageclass" if function == Function::Bonus2 => bonus!(
                self,
                BonusType::PhysicalDamageAgainstMobIdPercentage(positive(&params[1])?, number(2)? as i8)
            ),
            "badddefmonster" if function == Function::Bonus2 => bonus!(
                self,
                BonusType::ResistancePhysicalAttackFromMobIdPercentage(positive(&params[1])?, number(2)? as i8)
            ),
            _ => return Ok(false),
        }
        Ok(true)
    }

    pub fn new() -> Self {
        Self {
            bonuses: Default::default(),
        }
    }

    pub fn drain(&self) -> Vec<BonusType> {
        let mut write_guard = self.bonuses.write().unwrap();
        write_guard.drain(0..).collect()
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        let mut write_guard = self.bonuses.write().unwrap();
        write_guard.clear();
    }

    pub fn handle_bonus(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        if params.len() == 1 {
            match bonus.to_lowercase().as_str() {
                "bintravision" => {
                    bonus!(self, BonusType::EnableSeeHidden);
                }
                "bnocastcancel" => {
                    bonus!(self, BonusType::EnableNoCancelCast);
                }
                "bnogemstone" => {
                    bonus!(self, BonusType::EnableNoGemstoneRequired);
                }
                "bnoknockback" => {
                    bonus!(self, BonusType::EnableNoKnockback);
                }
                "bnosizefix" => {
                    bonus!(self, BonusType::EnableIgnoreSizeModifier);
                }
                "bnowalkdelay" => {
                    bonus!(self, BonusType::EnableNoWalkDelay);
                }
                "brestartfullrecover" => {
                    bonus!(self, BonusType::EnableFullHpSpRecoverOnResurrect);
                }
                "bunbreakablearmor" => {
                    bonus!(self, BonusType::UnbreakableArmor);
                }
                "bunbreakablegarment" => {
                    bonus!(self, BonusType::UnbreakableShoulder);
                }
                "bunbreakablehelm" => {
                    bonus!(self, BonusType::UnbreakableHelm);
                }
                "bunbreakableshield" => {
                    bonus!(self, BonusType::UnbreakableShield);
                }
                "bunbreakableshoes" => {
                    bonus!(self, BonusType::UnbreakableShoes);
                }
                "bunbreakableweapon" => {
                    bonus!(self, BonusType::UnbreakableWeapon);
                }
                _ => {}
            }
        } else {
            let value = params[1].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "bagi" => {
                    bonus!(self, BonusType::Agi(value as i8));
                }
                "badditemhealrate" => {
                    bonus!(self, BonusType::HpRegenFromItemPercentage(value as i16));
                }
                "ballstats" => {
                    bonus!(self, BonusType::AllStats(value as i8));
                }
                "baspd" => {
                    bonus!(self, BonusType::Aspd(value as i8));
                }
                "baspdrate" => {
                    bonus!(self, BonusType::AspdPercentage(value as f32));
                }
                "batkele" => {
                    bonus!(self, BonusType::ElementWeapon(Element::from_value(value as usize)));
                }
                "batkrate" => {
                    bonus!(self, BonusType::AtkPercentage(value as i8));
                }
                "bbaseatk" => {
                    bonus!(self, BonusType::Atk(value as i16));
                }
                "batk" => {
                    bonus!(self, BonusType::WeaponAtk(value));
                }
                "batk2" => {
                    bonus!(self, BonusType::WeaponRefineAtk(value));
                }
                "bbreakarmorrate" => {
                    bonus!(self, BonusType::BreakArmorPercentage((value / 100) as f32));
                }
                "bbreakweaponrate" => {
                    bonus!(self, BonusType::BreakWeaponPercentage((value / 100) as i8));
                }
                "bcastrate" => {
                    bonus!(self, BonusType::CastTimePercentage(value as i8));
                }
                "bclasschange" => {
                    bonus!(self, BonusType::ClassChangePercentageOnHit((value / 100) as i8));
                }
                "bcritatkrate" => {
                    bonus!(self, BonusType::CriticalDamagePercentage(value as i8));
                }
                "bcritical" => {
                    bonus!(self, BonusType::Crit(value as f32));
                }
                "bcriticallong" => {
                    bonus!(self, BonusType::LongRangeCriticalChance(value as i8));
                }
                "bdef" => {
                    bonus!(self, BonusType::Def(value as i16));
                }
                "bdef2rate" => {
                    bonus!(self, BonusType::VitDefPercentage(value as i8));
                }
                "bdefele" => {
                    bonus!(self, BonusType::ElementDefense(Element::from_value(value as usize)));
                }
                "bdefrate" => {
                    bonus!(self, BonusType::DefPercentage(value as i8));
                }
                "bdefratioatkclass" => {
                    bonus!(
                        self,
                        BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::from_value(value as usize))
                    )
                }
                "bdelayrate" => {
                    bonus!(self, BonusType::SkillDelayIncDecPercentage(value as i8));
                }
                "bdex" => {
                    bonus!(self, BonusType::Dex(value as i8));
                }
                "bdoublerate" => {
                    bonus!(self, BonusType::DoubleAttackChancePercentage(value as i8));
                }
                "bflee" => {
                    bonus!(self, BonusType::Flee(value as i16));
                }
                "bflee2" => {
                    bonus!(self, BonusType::PerfectDodge(value as i8));
                }
                "bhpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemy(value as i8));
                }
                "bhprecovrate" => {
                    bonus!(self, BonusType::NaturalHpRecoveryPercentage(value as i8));
                }
                "bhealpower" => {
                    bonus!(self, BonusType::HealSkillPercentage(value as i8));
                }
                "bhealpower2" => {
                    bonus!(self, BonusType::HpRegenFromSkillPercentage(value as i8));
                }
                "bhit" => {
                    bonus!(self, BonusType::Hit(value as i16));
                }
                "bhitrate" => {
                    bonus!(self, BonusType::HitPercentage(value as i8));
                }
                "bignoredefclass" => {
                    bonus!(self, BonusType::IgnoreDefClass(MobClass::from_value(value as usize)))
                }
                "bignoredefrace" => {
                    bonus!(self, BonusType::IgnoreDefRace(MobRace::from_value(value as usize)))
                }
                "bint" => {
                    bonus!(self, BonusType::Int(value as i8));
                }
                "blongatkdef" => {
                    bonus!(self, BonusType::ResistanceRangeAttackPercentage(value as i8));
                }
                "blongatkrate" => {
                    bonus!(self, BonusType::DamageRangedAtkPercentage(value as i8));
                }
                "bluk" => {
                    bonus!(self, BonusType::Luk(value as i8));
                }
                "bmatkrate" => {
                    bonus!(self, BonusType::MatkPercentage(value as i8));
                }
                "bmagicdamagereturn" => {
                    bonus!(self, BonusType::MagicAttackReflectChancePercentage(value as i8));
                }
                "bmagichpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemyWithMagicAttack(value as i8));
                }
                "bmagicspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemyWithMagicAttack(value as i8));
                }
                "bmatk" => {
                    bonus!(self, BonusType::Matk(value as i16));
                }
                "bmaxhp" => {
                    bonus!(self, BonusType::Maxhp(value));
                }
                "bmaxhprate" => {
                    bonus!(self, BonusType::MaxhpPercentage(value as i8));
                }
                "bmaxsp" => {
                    bonus!(self, BonusType::Maxsp(value));
                }
                "bmaxsprate" => {
                    bonus!(self, BonusType::MaxspPercentage(value as i8));
                }
                "bmdef" => {
                    bonus!(self, BonusType::Mdef(value as i16));
                }
                "bnomagicdamage" => {
                    bonus!(self, BonusType::ResistanceMagicAttackPercentage(value as i8));
                }
                "bnoregen" => {
                    if value == 1 {
                        bonus!(self, BonusType::DisableHpRegen);
                    } else if value == 2 {
                        bonus!(self, BonusType::DisableSpRegen);
                    }
                }
                "bperfecthitrate" => {
                    bonus!(self, BonusType::PerfectHitPercentage(value as i8));
                }
                "bspdrainvalue" => {
                    bonus!(self, BonusType::GainSpWhenHittingEnemy(value as i8));
                }
                "bspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemy(value as i8));
                }
                "bsprecovrate" => {
                    bonus!(self, BonusType::NaturalSpRecoveryPercentage(value as i8));
                }
                "bshortweapondamagereturn" => {
                    bonus!(self, BonusType::PhysicalAttackReflectChancePercentage(value as i8));
                }
                "bspeedaddrate" | "bspeedrate" => {
                    bonus!(self, BonusType::SpeedPercentage(value as i8));
                }
                "bsplashrange" => {
                    bonus!(self, BonusType::SplashRadius(value as i8));
                }
                "bstr" => {
                    bonus!(self, BonusType::Str(value as i8));
                }
                "busesprate" => {
                    bonus!(self, BonusType::SpConsumption(value as i8));
                }
                "bvit" => {
                    bonus!(self, BonusType::Vit(value as i8));
                }
                _ => {}
            }
        }
    }

    pub fn handle_bonus2(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        if params[1].is_string() {
            let value1 = params[1].string_value().unwrap();
            let value2 = params[2].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "bskillatk" => {
                    bonus!(
                        self,
                        BonusType::SkillIdDamagePercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8)
                    );
                }
                "bskillheal" => {
                    bonus!(
                        self,
                        BonusType::HealSkillIdPercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8)
                    );
                }
                "baddskillblow" => {
                    bonus!(
                        self,
                        BonusType::KnockbackWhenUsingSkillId(SkillEnum::from_name(value1.as_str()).id(), value2 as i8)
                    );
                }
                "bcastrate2" => {
                    bonus!(
                        self,
                        BonusType::CastTimeWhenUsingSkillIdPercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8)
                    )
                }
                _ => {}
            }
        } else {
            let value1 = params[1].number_value().unwrap();
            let value2 = params[2].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "baddclass" => {
                    bonus!(
                        self,
                        BonusType::PhysicalDamageAgainstClassPercentage(MobClass::from_value(value1 as usize), value2 as i8)
                    )
                }
                "badddamageclass" => {
                    BonusType::PhysicalDamageAgainstMobIdPercentage(value1 as u32, value2 as i8);
                }
                "badddefmonster" => {
                    BonusType::ResistancePhysicalAttackFromMobIdPercentage(value1 as u32, value2 as i8);
                }
                "baddeff" => {
                    bonus!(
                        self,
                        BonusType::ChanceToInflictStatusOnAttackPercentage(
                            StatusEffect::from_value(value1 as usize),
                            value2 as f32 / 100.0
                        )
                    )
                }
                "baddeff2" => {
                    bonus!(
                        self,
                        BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(
                            StatusEffect::from_value(value1 as usize),
                            value2 as f32 / 100.0
                        )
                    )
                }
                "baddeffwhenhit" => {
                    bonus!(
                        self,
                        BonusType::ChanceToInflictStatusOnAttackPercentage(
                            StatusEffect::from_value(value1 as usize),
                            value2 as f32 / 100.0
                        )
                    )
                }
                "baddele" => {
                    bonus!(
                        self,
                        BonusType::PhysicalDamageAgainstElementPercentage(Element::from_value(value1 as usize), value2 as i8)
                    )
                }
                "badditemgrouphealrate" => match ItemGroup::from_value(value1 as usize) {
                    ItemGroup::Herb => bonus!(self, BonusType::HpRegenFromHerbPercentage(value2 as i16)),
                    ItemGroup::Fruit => bonus!(self, BonusType::HpRegenFromFruitPercentage(value2 as i16)),
                    ItemGroup::Meat => bonus!(self, BonusType::HpRegenFromMeatPercentage(value2 as i16)),
                    ItemGroup::Candy => bonus!(self, BonusType::HpRegenFromCandyPercentage(value2 as i16)),
                    ItemGroup::Juice => bonus!(self, BonusType::HpRegenFromJuicePercentage(value2 as i16)),
                    ItemGroup::Fish => bonus!(self, BonusType::HpRegenFromFishPercentage(value2 as i16)),
                    ItemGroup::Food => bonus!(self, BonusType::HpRegenFromFoodPercentage(value2 as i16)),
                    ItemGroup::Potion => bonus!(self, BonusType::HpRegenFromPotionPercentage(value2 as i16)),
                    _ => {}
                },
                "badditemhealrate" => {
                    bonus!(self, BonusType::HpRegenFromItemIDPercentage(value1 as u32, value2 as i16));
                }
                "baddmonsterdropitem" => {
                    bonus!(self, BonusType::DropChanceItemIdPercentage(value1 as u32, value2 as i8));
                }
                "baddmonsterdropitemgroup" => match ItemGroup::from_value(value1 as usize) {
                    ItemGroup::Ore => bonus!(self, BonusType::DropChanceOrePercentage(value2 as i8)),
                    ItemGroup::Jewel => bonus!(self, BonusType::DropChanceJewelPercentage(value2 as i8)),
                    ItemGroup::Recovery => bonus!(self, BonusType::DropChanceRecoveryPercentage(value2 as i8)),
                    ItemGroup::Food => bonus!(self, BonusType::DropChanceFoodPercentage(value2 as i8)),
                    _ => {}
                },
                "baddrace" => {
                    bonus!(
                        self,
                        BonusType::PhysicalDamageAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "baddrace2" => {
                    bonus!(
                        self,
                        BonusType::DamageAgainstMobGroupPercentage(MobGroup::from_value(value1 as usize), value2 as i8)
                    )
                }
                "baddsize" => {
                    bonus!(
                        self,
                        BonusType::PhysicalDamageAgainstSizePercentage(Size::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bcomaclass" => {
                    bonus!(
                        self,
                        BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                            MobClass::from_value(value1 as usize),
                            value2 as f32 / 100.0
                        )
                    )
                }
                "bweaponcomaele" => bonus!(
                    self,
                    BonusType::WeaponComaAgainstElement(Element::from_value(value1 as usize), value2)
                ),
                "bweaponcomaclass" => bonus!(
                    self,
                    BonusType::WeaponComaAgainstClass(MobClass::from_value(value1 as usize), value2)
                ),
                "bweaponcomarace" => bonus!(
                    self,
                    BonusType::WeaponComaAgainstRace(MobRace::from_value(value1 as usize), value2)
                ),
                "bcomarace" => {
                    bonus!(
                        self,
                        BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(
                            MobRace::from_value(value1 as usize),
                            value2 as f32 / 100.0
                        )
                    )
                }
                "bcriticaladdrace" => {
                    bonus!(
                        self,
                        BonusType::CriticalAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bexpaddrace" => {
                    bonus!(
                        self,
                        BonusType::GainExpWhenKillingRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bgetzenynum" => {
                    bonus!(self, BonusType::GainZenyWhenKillingMonster(value1 as u16, value2 as i8));
                }
                "bhpdrainrate" => {
                    bonus!(
                        self,
                        BonusType::HpDrainWhenAttackingPercentage(value2 as i8, (value1 / 10_i32) as i8)
                    );
                }
                "bhplossrate" => {
                    bonus!(self, BonusType::HpLossEveryMs(value1 as u16, value2 as u16));
                }
                "bhpregenrate" => {
                    bonus!(self, BonusType::HpRegenEveryMs(value1 as u16, value2 as u16));
                }
                "bignoredefracerate" => {
                    bonus!(
                        self,
                        BonusType::IgnoreDefRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bignoremdefracerate" => {
                    bonus!(
                        self,
                        BonusType::IgnoreMDefRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bignoremdefclassrate" => {
                    bonus!(
                        self,
                        BonusType::IgnoreMDefClassPercentage(MobClass::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bmagicaddrace" => {
                    bonus!(
                        self,
                        BonusType::MagicalDamageAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "breseff" => {
                    bonus!(
                        self,
                        BonusType::ResistanceToStatusPercentage(StatusEffect::from_value(value1 as usize), (value2 / 100) as f32)
                    )
                }
                "bspdrainrate" => {
                    bonus!(
                        self,
                        BonusType::SpDrainWhenAttackingPercentage(value2 as i8, (value1 / 10_i32) as i8)
                    );
                }
                "bspdrainvaluerace" => {
                    bonus!(
                        self,
                        BonusType::SpDrainWhenAttackingRace(MobRace::from_value(value1 as usize), value2 as u16)
                    )
                }
                "bspgainrace" => {
                    bonus!(
                        self,
                        BonusType::SpDrainWhenKillingRace(MobRace::from_value(value1 as usize), value2 as u16)
                    )
                }
                "bsplossrate" => {
                    bonus!(self, BonusType::SpLossEveryMs(value1 as u16, value2 as u16));
                }
                "bspregenrate" => {
                    bonus!(self, BonusType::SpRegenEveryMs(value1 as u16, value2 as u16));
                }
                "bspvanishrate" => {
                    bonus!(self, BonusType::SpRegenEveryMs(value2 as u16, (value1 / 10_i32) as u16));
                }
                "bsubclass" => {
                    bonus!(
                        self,
                        BonusType::ResistanceDamageFromClassPercentage(MobClass::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bsubele" => {
                    bonus!(
                        self,
                        BonusType::ResistanceDamageFromElementPercentage(Element::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bsubrace" => {
                    bonus!(
                        self,
                        BonusType::ResistanceDamageFromRacePercentage(MobRace::from_value(value1 as usize), value2 as i8)
                    )
                }
                "bsubsize" => {
                    bonus!(
                        self,
                        BonusType::ResistanceDamageFromSizePercentage(Size::from_value(value1 as usize), value2 as i8)
                    )
                }
                _ => {}
            }
        }
    }
}
