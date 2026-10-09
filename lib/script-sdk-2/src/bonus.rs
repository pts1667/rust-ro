use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

pub use crate::bonus_names::{Bonus, Bonus2, Bonus3};

impl<'a> Ctx<'a> {
    /// Stat, flag and proc bonuses, such as `ctx.bonus().apply(Bonus::Str(5))`.
    pub fn bonus<'c>(&'c self) -> Bonuses<'c, 'a> {
        Bonuses { ctx: self }
    }
}

/// An auto-spell cast on an attack (`bAutoSpell`) or when hit (`bAutoSpellWhenHit`).
#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpell {
    /// Skill id or rathena name, such as `"MC_LOUD"`.
    pub skill: Val,
    pub level: i32,
    pub rate: i32,
    /// Trigger flags. Needed to set `battle_flags`.
    pub flags: Option<i32>,
    /// Battle flags of the trigger. Requires `flags`.
    pub battle_flags: Option<i32>,
}

impl AutoSpell {
    pub fn new(skill: impl Into<Val>, level: i32, rate: i32) -> Self {
        Self { skill: skill.into(), level, rate, flags: None, battle_flags: None }
    }
}

/// An auto-spell cast when `trigger_skill` is used (`bAutoSpellOnSkill`).
#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpellOnSkill {
    pub trigger_skill: Val,
    pub skill: Val,
    pub level: i32,
    pub rate: i32,
    pub flags: Option<i32>,
}

/// An automatic bonus that runs `program` on attacks or when hit (`autobonus`, `autobonus2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutoBonus {
    pub program: i32,
    pub rate: i32,
    pub duration: i32,
    /// Battle flags of the trigger. Set to `0` automatically when `visual_program` is given.
    pub battle_flags: Option<i32>,
    pub visual_program: Option<i32>,
}

/// An automatic bonus that runs `program` when `trigger_skill` is used (`autobonus3`).
#[derive(Debug, Clone, PartialEq)]
pub struct AutoSkillBonus {
    pub program: i32,
    pub rate: i32,
    pub duration: i32,
    pub trigger_skill: Val,
    pub visual_program: Option<i32>,
}

pub struct Bonuses<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Bonuses<'_, '_> {
    /// Applies a stat or flag bonus, as rathena's `bonus`.
    pub fn apply(&self, bonus: Bonus) -> Script {
        let (name, value) = bonus.parts();
        let mut arguments = args![name];
        arguments.extend(value.map(Val::from));
        self.ctx.call(Function::Bonus, arguments).map(|_| ())
    }

    /// Applies a keyed bonus, as rathena's `bonus2`: a race, element, effect or other key, and its value.
    pub fn apply2(&self, bonus: Bonus2) -> Script {
        let (name, [key, value]) = bonus.parts();
        self.ctx.call(Function::Bonus2, args![name, key, value]).map(|_| ())
    }

    /// Applies a bonus of three values, as rathena's `bonus3`.
    pub fn apply3(&self, bonus: Bonus3) -> Script {
        let (name, [first, second, third]) = bonus.parts();
        self.ctx.call(Function::Bonus3, args![name, first, second, third]).map(|_| ())
    }

    /// An auto-spell on attacks: `bonus3`, `bonus4` or `bonus5` depending on the optional values set.
    pub fn auto_spell(&self, spell: AutoSpell) -> Script {
        self.auto_spell_named("bAutoSpell", spell)
    }

    /// An auto-spell when hit: the same forms as [`auto_spell`](Self::auto_spell).
    pub fn auto_spell_when_hit(&self, spell: AutoSpell) -> Script {
        self.auto_spell_named("bAutoSpellWhenHit", spell)
    }

    /// An auto-spell when `trigger_skill` is used: `bonus4`, or `bonus5` when `flags` is set.
    pub fn auto_spell_on_skill(&self, spell: AutoSpellOnSkill) -> Script {
        let mut arguments = args!["bAutoSpellOnSkill", spell.trigger_skill, spell.skill, spell.level, spell.rate];
        let function = match spell.flags {
            Some(flags) => {
                arguments.push(Val::from(flags));
                Function::Bonus5
            }
            None => Function::Bonus4,
        };
        self.ctx.call(function, arguments).map(|_| ())
    }

    /// An automatic bonus on attacks, as rathena's `autobonus`.
    pub fn auto_bonus(&self, bonus: AutoBonus) -> Script {
        self.auto_bonus_with(Function::AutoBonus, bonus)
    }

    /// An automatic bonus when hit, as rathena's `autobonus2`.
    pub fn auto_bonus_when_hit(&self, bonus: AutoBonus) -> Script {
        self.auto_bonus_with(Function::AutoBonus2, bonus)
    }

    /// An automatic bonus when a skill is used, as rathena's `autobonus3`.
    pub fn auto_bonus_on_skill(&self, bonus: AutoSkillBonus) -> Script {
        let mut arguments = args![bonus.program, bonus.rate, bonus.duration, bonus.trigger_skill];
        if let Some(visual) = bonus.visual_program {
            arguments.push(Val::from(visual));
        }
        self.ctx.call(Function::AutoBonus3, arguments).map(|_| ())
    }

    fn auto_spell_named(&self, name: &str, spell: AutoSpell) -> Script {
        let mut arguments = args![name, spell.skill, spell.level, spell.rate];
        let function = match (spell.battle_flags, spell.flags) {
            (None, None) => Function::Bonus3,
            (None, Some(flags)) => {
                arguments.push(Val::from(flags));
                Function::Bonus4
            }
            (Some(battle), Some(flags)) => {
                arguments.push(Val::from(battle));
                arguments.push(Val::from(flags));
                Function::Bonus5
            }
            (Some(_), None) => return Err(Stop::Error("battle_flags needs flags".into())),
        };
        self.ctx.call(function, arguments).map(|_| ())
    }

    fn auto_bonus_with(&self, function: Function, bonus: AutoBonus) -> Script {
        let mut arguments = args![bonus.program, bonus.rate, bonus.duration];
        if bonus.battle_flags.is_some() || bonus.visual_program.is_some() {
            arguments.push(Val::from(bonus.battle_flags.unwrap_or(0)));
        }
        arguments.extend(bonus.visual_program.map(Val::from));
        self.ctx.call(function, arguments).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::bonus::{AutoBonus, AutoSkillBonus, AutoSpell, AutoSpellOnSkill, Bonus, Bonus2, Bonus3};
    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn apply_sends_the_name_and_the_value() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).bonus().apply(Bonus::Str(5)).unwrap();
        assert_eq!(transport.calls(Function::Bonus)[0], vec![Value::new_string("bStr".into()), Value::new_number(5)]);
    }

    #[test]
    fn flag_bonuses_send_only_the_name() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).bonus().apply(Bonus::NoCastCancel).unwrap();
        assert_eq!(transport.calls(Function::Bonus)[0], vec![Value::new_string("bNoCastCancel".into())]);
    }

    #[test]
    fn apply2_sends_key_then_value() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).bonus().apply2(Bonus2::AddRace(7, 5)).unwrap();
        assert_eq!(transport.calls(Function::Bonus2)[0], vec![Value::new_string("bAddRace".into()), Value::new_number(7), Value::new_number(5)]);
    }

    #[test]
    fn apply3_sends_three_values() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).bonus().apply3(Bonus3::SubEle(1, 2, 3)).unwrap();
        assert_eq!(transport.calls(Function::Bonus3)[0].len(), 4);
    }

    #[test]
    fn auto_spell_picks_the_form_from_the_optional_values() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.bonus().auto_spell(AutoSpell::new("MC_LOUD", 1, 10)).unwrap();
        let mut with_flags = AutoSpell::new("MC_LOUD", 1, 10);
        with_flags.flags = Some(2);
        ctx.bonus().auto_spell(with_flags.clone()).unwrap();
        let mut with_battle = with_flags;
        with_battle.battle_flags = Some(4);
        ctx.bonus().auto_spell(with_battle).unwrap();
        assert_eq!(transport.calls(Function::Bonus3)[0].len(), 4);
        assert_eq!(transport.calls(Function::Bonus4)[0].len(), 5);
        assert_eq!(transport.calls(Function::Bonus5)[0].len(), 6);
    }

    #[test]
    fn battle_flags_without_flags_is_an_error() {
        let transport = MockTransport::silent();
        let mut spell = AutoSpell::new("MC_LOUD", 1, 10);
        spell.battle_flags = Some(4);
        assert!(Ctx::new(&transport).bonus().auto_spell(spell).is_err());
        assert!(transport.requests().is_empty());
    }

    #[test]
    fn auto_spell_on_skill_uses_bonus4_without_flags() {
        let transport = MockTransport::silent();
        let spell = AutoSpellOnSkill { trigger_skill: "SM_BASH".into(), skill: "MC_LOUD".into(), level: 1, rate: 10, flags: None };
        Ctx::new(&transport).bonus().auto_spell_on_skill(spell).unwrap();
        assert_eq!(transport.calls(Function::Bonus4)[0].len(), 5);
    }

    #[test]
    fn auto_bonus_with_a_visual_program_sends_zero_battle_flags() {
        let transport = MockTransport::silent();
        let bonus = AutoBonus { program: 1, rate: 10, duration: 5000, battle_flags: None, visual_program: Some(4) };
        Ctx::new(&transport).bonus().auto_bonus(bonus).unwrap();
        assert_eq!(transport.calls(Function::AutoBonus)[0], vec![Value::new_number(1), Value::new_number(10), Value::new_number(5000), Value::new_number(0), Value::new_number(4)]);
    }

    #[test]
    fn auto_bonus_on_skill_sends_the_trigger_skill_fourth() {
        let transport = MockTransport::silent();
        let bonus = AutoSkillBonus { program: 1, rate: 10, duration: 5000, trigger_skill: "SM_BASH".into(), visual_program: None };
        Ctx::new(&transport).bonus().auto_bonus_on_skill(bonus).unwrap();
        assert_eq!(transport.calls(Function::AutoBonus3)[0].len(), 4);
    }
}
