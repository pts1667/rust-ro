//! Item scripts. See docs/adr/5-item-script-api.md for the split between passive and use scripts.

use script_sdk::{Function, Request};

use crate::args;
use crate::bonus::Bonuses;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::input::Input;
use crate::player::Player;
use crate::value::Val;

/// A passive item script, run when the item is worn or refined. It can read the wearer and describe bonuses, and
/// nothing else.
pub struct ItemBonus<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl<'c, 'a> ItemBonus<'c, 'a> {
    pub fn new(ctx: &'c Ctx<'a>) -> Self {
        Self { ctx }
    }

    pub fn bonus(&self) -> Bonuses<'c, 'a> {
        self.ctx.bonus()
    }

    pub fn wearer(&self) -> Player<'c, 'a> {
        self.ctx.player()
    }

    /// The refine level of the worn equipment this item applies to, or `0`, as rathena's `getrefine`.
    pub fn refine(&self) -> Result<i32, Stop> {
        self.ctx.call(Function::GetRefine, args![])?.number()
    }

    /// Calls a host function for a read. The host refuses effects in a passive run, as it did for the legacy script.
    pub fn call(&self, function: Function, arguments: Vec<Val>) -> Result<Val, Stop> {
        self.ctx.call(function, arguments)
    }

    pub fn constant(&self, name: &str) -> Result<Val, Stop> {
        self.ctx.constant(name)
    }

    pub fn binary(&self, left: Val, operation: &str, right: Val) -> Result<Val, Stop> {
        binary(left, operation, right)
    }

    pub fn truthy(&self, value: Val) -> bool {
        value.is_true()
    }
}

/// An item script run when the item is consumed. Its effects are queued and applied only when the script returns
/// `Ok`. A conversation that is cancelled applies none of them.
pub struct ItemUse<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl<'c, 'a> ItemUse<'c, 'a> {
    pub fn new(ctx: &'c Ctx<'a>) -> Self {
        Self { ctx }
    }

    /// The attached player, read the same way as in a passive script.
    pub fn wearer(&self) -> Player<'c, 'a> {
        self.ctx.player()
    }

    /// The refine level of the equipment this item applies to, or `0`. See [`ItemBonus::refine`].
    pub fn refine(&self) -> Result<i32, Stop> {
        self.ctx.call(Function::GetRefine, args![])?.number()
    }

    /// Reads a character variable such as `Zeny`, or a plain name. Scoped names (`#`, `$`, `@`, `'`, `.`) are refused.
    pub fn read(&self, name: &str) -> Result<Val, Stop> {
        character_name(name)?;
        self.ctx.var(name).get()
    }

    /// Writes a character variable when the item is consumed. Scoped names are refused.
    pub fn write(&self, name: &str, value: impl Into<Val>) -> Script {
        character_name(name)?;
        self.ctx.var(name).set(value)
    }

    /// Reads a variable the way the script spells it, scope prefix included. The host answers scoped names from the
    /// values it loaded before the script ran, so the generator lists every such name the script reads.
    pub fn read_any(&self, name: &str) -> Result<Val, Stop> {
        self.ctx.request(Request::Read(name.into()))
    }

    /// Writes a variable the way the script spells it, scope prefix included. The write is queued like any other effect.
    pub fn write_any(&self, name: &str, value: impl Into<Val>) -> Script {
        self.ctx.request(Request::Write { name: name.into(), value: value.into().into_value() }).map(|_| ())
    }

    /// See [`ItemBonus::call`]. Effects are queued here.
    pub fn call(&self, function: Function, arguments: Vec<Val>) -> Result<Val, Stop> {
        self.ctx.call(function, arguments)
    }

    pub fn constant(&self, name: &str) -> Result<Val, Stop> {
        self.ctx.constant(name)
    }

    pub fn binary(&self, left: Val, operation: &str, right: Val) -> Result<Val, Stop> {
        binary(left, operation, right)
    }

    pub fn truthy(&self, value: Val) -> bool {
        value.is_true()
    }

    /// How many of `item` the attached player carries, including the changes this script has already queued.
    pub fn count(&self, item: i32) -> Result<i32, Stop> {
        self.ctx.items().count(item)
    }

    /// Queues the gain of `amount` of `item`.
    pub fn grant(&self, item: i32, amount: i32) -> Script {
        self.ctx.items().give(item, amount)
    }

    /// Queues the loss of `amount` of `item`. The script fails if the player does not carry enough.
    pub fn take(&self, item: i32, amount: i32) -> Script {
        self.ctx.items().take(item, amount)
    }

    /// Queues a heal of `hp` and `sp` points.
    pub fn heal(&self, hp: i32, sp: i32) -> Script {
        self.ctx.call(Function::Heal, args![hp, sp]).map(|_| ())
    }

    /// Queues a heal of `hp` and `sp` percent of the maximum.
    pub fn heal_percent(&self, hp: i32, sp: i32) -> Script {
        self.ctx.call(Function::PercentHeal, args![hp, sp]).map(|_| ())
    }

    /// Queues one reward drawn from item group `group`. `amount` of `None` gives the entry's own amount, and
    /// `subgroup` of `None` draws from the first subgroup.
    pub fn random_group(&self, group: impl Into<Val>, amount: Option<i32>, subgroup: Option<i32>, identified: bool) -> Script {
        let arguments = args![group.into(), amount.unwrap_or(0), subgroup.unwrap_or(1), i32::from(identified)];
        self.ctx.call(Function::RandomGroupItem, arguments).map(|_| ())
    }

    pub fn mes(&self, text: &str) -> Result<(), Stop> {
        self.ctx.mes(text)
    }

    pub fn mes_as(&self, speaker: &str, text: &str) -> Result<(), Stop> {
        self.ctx.mes_as(speaker, text)
    }

    pub fn next(&self) -> Result<(), Stop> {
        self.ctx.next()
    }

    pub fn menu(&self, options: &[&str]) -> Result<usize, Stop> {
        self.ctx.menu(options)
    }

    pub fn close(&self) -> Script {
        self.ctx.close()
    }

    pub fn close_window(&self) -> Result<(), Stop> {
        self.ctx.close_window()
    }

    pub fn input_number(&self, min: i32, max: i32) -> Result<Input<i32>, Stop> {
        self.ctx.input_number(min, max)
    }

    pub fn input_text(&self, min: usize, max: usize) -> Result<Input<String>, Stop> {
        self.ctx.input_text(min, max)
    }
}

/// rathena's binary operators, with the rules of the legacy item runtime.
fn binary(left: Val, operation: &str, right: Val) -> Result<Val, Stop> {
    left.into_value().binary(operation, right.into_value()).map(Val::from).map_err(Stop::Error)
}

fn character_name(name: &str) -> Result<(), Stop> {
    if name.starts_with(['#', '$', '@', '\'', '.']) {
        return Err(Stop::Error(format!("{name} is a scoped variable, which item scripts cannot use yet")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Request, Value};

    use crate::item::{ItemBonus, ItemUse};
    use crate::transport::MockTransport;
    use crate::{Ctx, Stop};

    #[test]
    fn heal_sends_hp_then_sp() {
        let transport = MockTransport::silent();
        ItemUse::new(&Ctx::new(&transport)).heal(45, 0).unwrap();
        assert_eq!(transport.calls(Function::Heal)[0], vec![Value::new_number(45), Value::new_number(0)]);
    }

    #[test]
    fn heal_percent_uses_its_own_function() {
        let transport = MockTransport::silent();
        ItemUse::new(&Ctx::new(&transport)).heal_percent(10, 5).unwrap();
        assert_eq!(transport.calls(Function::PercentHeal)[0].len(), 2);
        assert!(transport.calls(Function::Heal).is_empty());
    }

    #[test]
    fn random_group_fills_the_defaults_the_server_expects() {
        let transport = MockTransport::silent();
        ItemUse::new(&Ctx::new(&transport)).random_group("rwc_box", None, None, false).unwrap();
        assert_eq!(transport.calls(Function::RandomGroupItem)[0], vec![
            Value::new_string("rwc_box".into()),
            Value::new_number(0),
            Value::new_number(1),
            Value::new_number(0),
        ]);
    }

    #[test]
    fn scoped_variables_are_refused_without_a_request() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        let item = ItemUse::new(&ctx);
        assert!(matches!(item.read("#gold"), Err(Stop::Error(_))));
        assert!(matches!(item.write("@megaphone$", "hi"), Err(Stop::Error(_))));
        assert!(transport.requests().is_empty());
    }

    #[test]
    fn character_variables_are_read_and_written_by_name() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        let item = ItemUse::new(&ctx);
        item.read("Zeny").unwrap();
        item.write("Zeny", 0).unwrap();
        assert!(matches!(&transport.requests()[0], Request::Read(name) if name == "Zeny"));
    }

    #[test]
    fn menu_returns_the_choice_made_in_the_conversation() {
        let transport = MockTransport::new(|_| Ok(Value::Number(2)));
        assert_eq!(ItemUse::new(&Ctx::new(&transport)).menu(&["Open", "Keep"]), Ok(1));
    }

    #[test]
    fn refine_asks_for_the_refine_of_the_applied_equipment() {
        let transport = MockTransport::new(|_| Ok(Value::Number(7)));
        assert_eq!(ItemBonus::new(&Ctx::new(&transport)).refine(), Ok(7));
        assert_eq!(transport.calls(Function::GetRefine)[0].len(), 0);
    }

    #[test]
    fn read_any_and_write_any_send_the_name_as_written() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        let item = ItemUse::new(&ctx);
        item.read_any("@megaphone$").unwrap();
        item.write_any("@megaphone$", "hi").unwrap();
        assert!(matches!(&transport.requests()[0], Request::Read(name) if name == "@megaphone$"));
        assert!(matches!(&transport.requests()[1], Request::Write { name, .. } if name == "@megaphone$"));
    }
}
