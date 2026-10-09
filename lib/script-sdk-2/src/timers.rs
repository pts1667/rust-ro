use script_sdk::Function;

use crate::ctx::Ctx;
use crate::flow::Script;

impl<'a> Ctx<'a> {
    /// The timer of the current NPC, such as `ctx.timers().start()`.
    pub fn timers<'c>(&'c self) -> Timers<'c, 'a> {
        Timers { ctx: self }
    }
}

pub struct Timers<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Timers<'_, '_> {
    pub fn init(&self) -> Script {
        self.ctx.call(Function::InitNpcTimer, vec![]).map(|_| ())
    }

    pub fn start(&self) -> Script {
        self.ctx.call(Function::StartNpcTimer, vec![]).map(|_| ())
    }

    pub fn stop(&self) -> Script {
        self.ctx.call(Function::StopNpcTimer, vec![]).map(|_| ())
    }
}
