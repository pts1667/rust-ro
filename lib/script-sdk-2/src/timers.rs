use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Script;
use crate::value::Val;

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

    /// Sets the elapsed time of the timer of `npc` (the current NPC when `None`), in milliseconds.
    pub fn set_elapsed(&self, milliseconds: i32, npc: Option<&str>) -> Script {
        let mut arguments = args![milliseconds];
        arguments.extend(npc.map(Val::from));
        self.ctx.call(Function::SetNpcTimer, arguments).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn set_elapsed_names_the_npc_only_when_given() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.timers().set_elapsed(5000, None).unwrap();
        ctx.timers().set_elapsed(5000, Some("Guard")).unwrap();
        let calls = transport.calls(Function::SetNpcTimer);
        assert_eq!(calls[0], vec![Value::new_number(5000)]);
        assert_eq!(calls[1].len(), 2);
    }
}
