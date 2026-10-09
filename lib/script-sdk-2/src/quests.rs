use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

impl<'a> Ctx<'a> {
    /// The quest log of the player, such as `ctx.quests().complete(3056)`.
    pub fn quests<'c>(&'c self) -> Quests<'c, 'a> {
        Quests { ctx: self }
    }
}

pub struct Quests<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Quests<'_, '_> {
    pub fn start(&self, quest: i32) -> Script {
        self.ctx.call(Function::SetQuest, args![quest]).map(|_| ())
    }

    pub fn complete(&self, quest: i32) -> Script {
        self.ctx.call(Function::CompleteQuest, args![quest]).map(|_| ())
    }

    pub fn erase(&self, quest: i32) -> Script {
        self.ctx.call(Function::EraseQuest, args![quest]).map(|_| ())
    }

    /// Replaces `old` with `new` in the quest log.
    pub fn change(&self, old: i32, new: i32) -> Script {
        self.ctx.call(Function::ChangeQuest, args![old, new]).map(|_| ())
    }

    /// `-1` when the player does not have the quest, otherwise the quest's state.
    pub fn check(&self, quest: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::CheckQuest, args![quest])?.number()
    }

    /// `0` not started, `1` in progress, `2` completed.
    pub fn progress(&self, quest: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::IsBeginQuest, args![quest])?.number()
    }
}
