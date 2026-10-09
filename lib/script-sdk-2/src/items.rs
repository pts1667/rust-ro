use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// The inventory helpers, such as `ctx.items().count(7311)`.
    pub fn items<'c>(&'c self) -> Items<'c, 'a> {
        Items { ctx: self }
    }
}

pub struct Items<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Items<'_, '_> {
    /// How many of `item` the player carries.
    pub fn count(&self, item: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::CountItem, args![item])?.number()
    }

    pub fn give(&self, item: i32, amount: i32) -> Script {
        self.ctx.call(Function::GetItem, args![item, amount]).map(|_| ())
    }

    /// Whether the player wears `item`.
    pub fn is_equipped(&self, item: i32) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::IsEquipped, args![item])?.number()? != 0)
    }

    pub fn take(&self, item: i32, amount: i32) -> Script {
        self.ctx.call(Function::DelItem, args![item, amount]).map(|_| ())
    }

    /// Places `amount` of `item` on the floor at `x`, `y` of `map`, which may be `"this"` for the current map.
    /// This is rathena's `makeitem`: everyone on the map can pick it up.
    pub fn place(&self, item: impl Into<Val>, amount: i32, map: &str, x: i32, y: i32) -> Script {
        self.ctx.call(Function::MakeItem, args![item.into(), amount, map, x, y]).map(|_| ())
    }
}

impl Items<'_, '_> {
    /// Whether the player could carry every `(item, amount)` pair in `items` without becoming overweight.
    pub fn check_weight(&self, items: &[(i32, i32)]) -> Result<bool, Stop> {
        let arguments = items.iter().flat_map(|(item, amount)| args![*item, *amount]).collect();
        Ok(self.ctx.call(Function::CheckWeight, arguments)?.number()? != 0)
    }
}

impl Items<'_, '_> {
    /// The English name of `item`, or `"null"` when the item does not exist.
    pub fn name(&self, item: i32) -> Result<String, Stop> {
        self.ctx.call(Function::GetItemName, args![item]).map(|value| value.text())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn is_equipped_reads_the_host_flag() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        assert_eq!(Ctx::new(&transport).items().is_equipped(2101), Ok(true));
        assert_eq!(transport.calls(Function::IsEquipped), vec![vec![Value::new_number(2101)]]);
    }

    #[test]
    fn check_weight_sends_item_and_amount_pairs() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        assert_eq!(Ctx::new(&transport).items().check_weight(&[(501, 2), (502, 1)]), Ok(true));
        assert_eq!(transport.calls(Function::CheckWeight)[0].len(), 4);
    }

    #[test]
    fn name_sends_the_item_id() {
        let transport = MockTransport::new(|_| Ok(Value::new_string("Jellopy".into())));
        assert_eq!(Ctx::new(&transport).items().name(909).as_deref(), Ok("Jellopy"));
        assert_eq!(transport.calls(Function::GetItemName)[0], vec![Value::new_number(909)]);
    }
}
