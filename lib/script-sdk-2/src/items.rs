use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

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
}
