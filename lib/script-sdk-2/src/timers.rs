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
    /// Resets the timer of `npc` (the current NPC when `None`) to zero and starts it.
    pub fn init(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::InitNpcTimer, npc_arguments(npc)).map(|_| ())
    }

    pub fn start(&self) -> Script {
        self.ctx.call(Function::StartNpcTimer, vec![]).map(|_| ())
    }

    /// Stops the timer of `npc` (the current NPC when `None`).
    pub fn stop(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::StopNpcTimer, npc_arguments(npc)).map(|_| ())
    }

    /// Stops the timer of `npc` and starts it again from zero.
    pub fn restart(&self, npc: Option<&str>) -> Script {
        self.stop(npc)?;
        self.init(npc)
    }

    /// Sets the elapsed time of the timer of `npc` (the current NPC when `None`), in milliseconds.
    pub fn set_elapsed(&self, milliseconds: i32, npc: Option<&str>) -> Script {
        let mut arguments = args![milliseconds];
        arguments.extend(npc.map(Val::from));
        self.ctx.call(Function::SetNpcTimer, arguments).map(|_| ())
    }
}

fn npc_arguments(npc: Option<&str>) -> Vec<Val> {
    npc.map(|npc| args![npc]).unwrap_or_default()
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

    #[test]
    fn restart_stops_then_inits_the_same_npc() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).timers().restart(Some("Guard")).unwrap();
        let guard = vec![Value::new_string("Guard".into())];
        assert_eq!(transport.calls(Function::StopNpcTimer), vec![guard.clone()]);
        assert_eq!(transport.calls(Function::InitNpcTimer), vec![guard]);
    }
}
