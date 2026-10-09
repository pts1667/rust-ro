use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Stop;

impl<'a> Ctx<'a> {
    /// Instance lookups, such as `ctx.instance().npc_name("Guard", None)`.
    pub fn instance<'c>(&'c self) -> Instance<'c, 'a> {
        Instance { ctx: self }
    }
}

pub struct Instance<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Instance<'_, '_> {
    /// The name of NPC `npc` inside instance `id`, or the player's own instance when `id` is `None`.
    pub fn npc_name(&self, npc: &str, id: Option<i32>) -> Result<String, Stop> {
        let mut arguments = args![npc];
        arguments.extend(id.map(Into::into));
        self.ctx.call(Function::InstanceNpcName, arguments).map(|value| value.text())
    }

    /// The name of `map` inside instance `id`, or empty when that instance has no such map.
    pub fn map_name(&self, map: &str, id: Option<i32>) -> Result<String, Stop> {
        let mut arguments = args![map];
        arguments.extend(id.map(Into::into));
        self.ctx.call(Function::InstanceMapName, arguments).map(|value| value.text())
    }

    /// The id of the player's own instance, or `0` when there is none.
    pub fn id(&self) -> Result<i32, Stop> {
        self.ctx.call(Function::InstanceId, args![])?.number()
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn npc_name_passes_the_instance_only_when_given() {
        let transport = MockTransport::new(|_| Ok(Value::new_string("Guard_3".into())));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.instance().npc_name("Guard", Some(3)).as_deref(), Ok("Guard_3"));
        ctx.instance().npc_name("Guard", None).unwrap();
        let calls = transport.calls(Function::InstanceNpcName);
        assert_eq!(calls[0].len(), 2);
        assert_eq!(calls[1].len(), 1);
    }
}
