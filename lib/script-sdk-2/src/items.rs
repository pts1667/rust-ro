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

    pub fn take(&self, item: i32, amount: i32) -> Script {
        self.ctx.call(Function::DelItem, args![item, amount]).map(|_| ())
    }
}
