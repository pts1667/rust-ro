use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// Waiting rooms attached to NPCs, such as `ctx.waiting_room().open("Dungeon", 12, Rules::default())`.
    pub fn waiting_room<'c>(&'c self) -> WaitingRoom<'c, 'a> {
        WaitingRoom { ctx: self }
    }
}

/// The optional rules of a waiting room. Unset ones take the server's defaults: the trigger is the room limit,
/// no fee, levels `1` to `175`, and no event.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rules<'s> {
    pub event: Option<&'s str>,
    pub trigger: Option<i32>,
    pub zeny: Option<i32>,
    pub min_level: Option<i32>,
    pub max_level: Option<i32>,
}

/// The highest base level the server accepts for a waiting room.
const DEFAULT_MAX_LEVEL: i32 = 175;

pub struct WaitingRoom<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl WaitingRoom<'_, '_> {
    /// Opens a waiting room on the current NPC: `limit` players at most, shown under `title`.
    pub fn open(&self, title: &str, limit: i32, rules: Rules) -> Script {
        let mut arguments = args![title, limit];
        let optional = [
            rules.event.map(Val::from),
            Some(Val::from(rules.trigger.unwrap_or(limit))),
            Some(Val::from(rules.zeny.unwrap_or(0))),
            Some(Val::from(rules.min_level.unwrap_or(1))),
            Some(Val::from(rules.max_level.unwrap_or(DEFAULT_MAX_LEVEL))),
        ];
        let given = [rules.event.is_some(), rules.trigger.is_some(), rules.zeny.is_some(), rules.min_level.is_some(), rules.max_level.is_some()];
        let Some(last) = given.iter().rposition(|given| *given) else {
            return self.ctx.call(Function::WaitingRoom, arguments).map(|_| ());
        };
        for value in optional.into_iter().take(last + 1) {
            arguments.push(value.unwrap_or_else(|| Val::from("")));
        }
        self.ctx.call(Function::WaitingRoom, arguments).map(|_| ())
    }

    /// Closes the waiting room of `npc`, or of the current NPC when `None`.
    pub fn delete(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::DelWaitingRoom, npc_arguments(npc)).map(|_| ())
    }

    /// Removes every player from the waiting room of `npc`, or of the current NPC when `None`.
    pub fn kick_all(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::WaitingRoomKickAll, npc_arguments(npc)).map(|_| ())
    }

    /// Removes the player named `name` from the waiting room of `npc`, or of the current NPC when `None`.
    pub fn kick(&self, npc: Option<&str>, name: &str) -> Script {
        self.ctx.call(Function::WaitingRoomKick, args![npc.unwrap_or(""), name]).map(|_| ())
    }

    /// Lets the room's event run again, and runs it now.
    pub fn enable_event(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::EnableWaitingRoomEvent, npc_arguments(npc)).map(|_| ())
    }

    /// Stops the room's event from running when the trigger is met.
    pub fn disable_event(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::DisableWaitingRoomEvent, npc_arguments(npc)).map(|_| ())
    }

    /// A number about the waiting room of `npc`. `kind` picks the field: `0` player count, `1` limit, `2` trigger,
    /// `3` event disabled, `32` full, `33` at trigger. Returns `-1` without a room or for an unknown kind. Text
    /// fields (`4` title, `5`, `16` event) are not covered here; use `ctx.call` for them.
    pub fn state(&self, kind: i32, npc: Option<&str>) -> Result<i32, Stop> {
        let mut arguments = args![kind];
        arguments.extend(npc.map(Val::from));
        self.ctx.call(Function::GetWaitingRoomState, arguments)?.number()
    }

    /// The title of the waiting room of `npc`, or `None` without a room.
    pub fn title(&self, npc: Option<&str>) -> Result<Option<String>, Stop> {
        self.text_state(4, npc)
    }

    /// The event label of the waiting room of `npc`, or `None` without a room.
    pub fn event(&self, npc: Option<&str>) -> Result<Option<String>, Stop> {
        self.text_state(16, npc)
    }

    fn text_state(&self, kind: i32, npc: Option<&str>) -> Result<Option<String>, Stop> {
        let mut arguments = args![kind];
        arguments.extend(npc.map(Val::from));
        let value = self.ctx.call(Function::GetWaitingRoomState, arguments)?;
        Ok((!value.is_number()).then(|| value.text()))
    }

    /// Warps up to `count` players from the waiting room to `map`, `x`, `y`. `count` defaults to the trigger.
    pub fn warp(&self, map: &str, x: i32, y: i32, count: Option<i32>) -> Script {
        let mut arguments = args![map, x, y];
        arguments.extend(count.map(Val::from));
        self.ctx.call(Function::WarpWaitingPc, arguments).map(|_| ())
    }
}

fn npc_arguments(npc: Option<&str>) -> Vec<Val> {
    npc.map(|npc| args![npc]).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::waiting_room::Rules;
    use crate::Ctx;

    #[test]
    fn open_without_rules_sends_only_title_and_limit() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).waiting_room().open("Room", 12, Rules::default()).unwrap();
        assert_eq!(transport.calls(Function::WaitingRoom)[0].len(), 2);
    }

    #[test]
    fn open_fills_the_gap_before_a_later_rule() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).waiting_room().open("Room", 12, Rules { zeny: Some(500), ..Default::default() }).unwrap();
        let call = &transport.calls(Function::WaitingRoom)[0];
        assert_eq!(call.len(), 5);
        assert_eq!(call[2], Value::new_string(String::new()));
        assert_eq!(call[3], Value::new_number(12));
        assert_eq!(call[4], Value::new_number(500));
    }

    #[test]
    fn title_is_none_without_a_room() {
        let transport = MockTransport::new(|_| Ok(Value::Number(-1)));
        assert_eq!(Ctx::new(&transport).waiting_room().title(None), Ok(None));
    }

    #[test]
    fn title_reads_the_text_field() {
        let transport = MockTransport::new(|_| Ok(Value::new_string("Dungeon".into())));
        assert_eq!(Ctx::new(&transport).waiting_room().title(Some("Guard")), Ok(Some("Dungeon".into())));
        assert_eq!(transport.calls(Function::GetWaitingRoomState)[0], vec![Value::new_number(4), Value::new_string("Guard".into())]);
    }

    #[test]
    fn state_reads_the_player_count() {
        let transport = MockTransport::new(|_| Ok(Value::Number(3)));
        assert_eq!(Ctx::new(&transport).waiting_room().state(0, None), Ok(3));
    }
}
