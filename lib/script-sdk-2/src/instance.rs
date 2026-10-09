use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// Instance lookups and control, such as `ctx.instance().npc_name("Guard", None)`.
    pub fn instance<'c>(&'c self) -> Instance<'c, 'a> {
        Instance { ctx: self }
    }
}

/// The optional parts of entering an instance. Unset parts take the server's defaults: no position, the attached
/// player, and the player's own instance of the definition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EnterOptions {
    pub position: Option<(i32, i32)>,
    pub char_id: Option<i32>,
    pub instance: Option<i32>,
}

pub struct Instance<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Instance<'_, '_> {
    /// Creates an instance from the definition `name`. `mode` is the instance mode number; the server's default is party.
    /// Returns a negative number when the instance cannot be created.
    pub fn create(&self, name: &str, mode: Option<i32>) -> Result<i32, Stop> {
        let mut arguments = args![name];
        arguments.extend(mode.map(Val::from));
        self.ctx.call(Function::InstanceCreate, arguments)?.number()
    }

    /// Enters the player into the instance `name`, optionally at `position`. Returns the `IE_*` result code.
    pub fn enter(&self, name: &str, position: Option<(i32, i32)>) -> Result<i32, Stop> {
        self.enter_with(name, EnterOptions { position, ..EnterOptions::default() })
    }

    /// [`enter`](Self::enter) with the options that pick another character or an explicit instance.
    pub fn enter_with(&self, name: &str, options: EnterOptions) -> Result<i32, Stop> {
        let mut arguments = args![name];
        if options.position.is_some() || options.char_id.is_some() || options.instance.is_some() {
            let (x, y) = options.position.unwrap_or((-1, -1));
            arguments.extend(args![x, y]);
        }
        if options.char_id.is_some() || options.instance.is_some() {
            arguments.push(Val::from(options.char_id.unwrap_or(0)));
        }
        arguments.extend(options.instance.map(Val::from));
        self.ctx.call(Function::InstanceEnter, arguments)?.number()
    }

    /// Warps every member of instance `id` (or the player's own instance) to `map`, `x`, `y`. A `flags` value of `1`
    /// leaves out the members who are dead.
    pub fn warp_all(&self, map: &str, x: i32, y: i32, id: Option<i32>, flags: Option<i32>) -> Script {
        let mut arguments = args![map, x, y];
        if id.is_some() || flags.is_some() {
            arguments.push(Val::from(id.unwrap_or(0)));
        }
        arguments.extend(flags.map(Val::from));
        self.ctx.call(Function::InstanceWarpAll, arguments).map(|_| ())
    }

    /// Destroys instance `id`, or the player's own instance when `None`.
    pub fn destroy(&self, id: Option<i32>) -> Script {
        self.ctx.call(Function::InstanceDestroy, args![id.unwrap_or(0)]).map(|_| ())
    }

    /// Broadcasts `message` to everyone inside instance `id`, or the player's own instance when `None`.
    pub fn announce(&self, id: Option<i32>, message: &str) -> Script {
        self.ctx.call(Function::InstanceAnnounce, args![id.unwrap_or(0), message]).map(|_| ())
    }

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

    /// Whether party `party_id` has at least `amount` members (default 1), and every member is within the level range
    /// the instance allows.
    pub fn check_party(&self, party_id: i32, amount: Option<i32>) -> Result<bool, Stop> {
        self.check(Function::InstanceCheckParty, party_id, amount)
    }

    /// The guild counterpart of [`check_party`](Self::check_party).
    pub fn check_guild(&self, guild_id: i32, amount: Option<i32>) -> Result<bool, Stop> {
        self.check(Function::InstanceCheckGuild, guild_id, amount)
    }

    fn check(&self, function: Function, owner: i32, amount: Option<i32>) -> Result<bool, Stop> {
        let mut arguments = args![owner];
        arguments.extend(amount.map(Val::from));
        Ok(self.ctx.call(function, arguments)?.number()? != 0)
    }

    /// A field of the instance definition `name`. `kind` picks it: `0` id, `1` time limit, `2` idle timeout,
    /// `3` entry map, `4` and `5` entry x and y, `6` map count. Returned as a dynamic value, because the fields differ
    /// in type. `-1` when there is no such definition. Kind `7` needs an index: use [`info_at`](Self::info_at).
    pub fn info(&self, name: &str, kind: i32) -> Result<Val, Stop> {
        self.ctx.call(Function::InstanceInfo, args![name, kind])
    }

    /// The map at `index` of the instance definition `name`.
    pub fn info_at(&self, name: &str, index: i32) -> Result<Val, Stop> {
        self.ctx.call(Function::InstanceInfo, args![name, 7, index])
    }

    /// A field of the live instance `id` (or the player's own when `None`). `kind` picks it: `0` definition name,
    /// `1` mode number, `2` owner id. Returned as a dynamic value, as [`info`](Self::info) is.
    pub fn live_info(&self, kind: i32, id: Option<i32>) -> Result<Val, Stop> {
        let mut arguments = args![kind];
        arguments.extend(id.map(Val::from));
        self.ctx.call(Function::InstanceLiveInfo, arguments)
    }

    /// The ids of the live instances on `map`, optionally only those of `mode`.
    pub fn list(&self, map: &str, mode: Option<i32>) -> Result<Vec<i32>, Stop> {
        let mut arguments = args![map];
        arguments.extend(mode.map(Val::from));
        let ids = self.ctx.call(Function::InstanceList, arguments)?.into_array().unwrap_or_default();
        ids.into_iter().map(|id| id.number()).collect()
    }

    /// A variable of instance `id` (or the player's own). The name may start with `'`, as in rathena scripts.
    pub fn var(&self, name: &str, id: Option<i32>) -> Result<Val, Stop> {
        let mut arguments = args![name];
        arguments.extend(id.map(Val::from));
        self.ctx.call(Function::GetInstanceVar, arguments)
    }

    /// Sets a variable of instance `id` (or the player's own). Names ending in `$` take text, the rest take numbers.
    pub fn set_var(&self, name: &str, value: impl Into<Val>, id: Option<i32>) -> Script {
        let mut arguments = args![name, value.into()];
        arguments.extend(id.map(Val::from));
        self.ctx.call(Function::SetInstanceVar, arguments).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::instance::EnterOptions;
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

    #[test]
    fn warp_all_fills_the_id_before_flags() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).instance().warp_all("mid_camp", 310, 42, None, Some(1)).unwrap();
        let call = &transport.calls(Function::InstanceWarpAll)[0];
        assert_eq!(call.len(), 5);
        assert_eq!(call[3], Value::new_number(0));
        assert_eq!(call[4], Value::new_number(1));
    }

    #[test]
    fn enter_with_an_explicit_instance_fills_the_earlier_slots() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        let options = EnterOptions { instance: Some(4), ..EnterOptions::default() };
        Ctx::new(&transport).instance().enter_with("Endless Tower", options).unwrap();
        let call = &transport.calls(Function::InstanceEnter)[0];
        assert_eq!(call.len(), 5);
        assert_eq!(call[1], Value::new_number(-1));
        assert_eq!(call[3], Value::new_number(0));
        assert_eq!(call[4], Value::new_number(4));
    }

    #[test]
    fn set_var_sends_the_id_last() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).instance().set_var("'stage", 2, Some(7)).unwrap();
        assert_eq!(transport.calls(Function::SetInstanceVar)[0].last(), Some(&Value::new_number(7)));
    }
}
