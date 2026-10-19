use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::Val;

impl Ctx<'_> {
    /// `unitwalk`: walks `unit` (a player's account or character id, a monster id or an NPC id) to `(x, y)` on its map and
    /// returns at once. `event` (`"Npc::OnLabel"`) runs when it stops there, attached to the player when the unit is one.
    /// False when no path leads there, the unit cannot walk, or a `unit_walk` already moves it.
    pub fn unit_walk(&self, unit: i32, x: i32, y: i32, event: Option<&str>) -> Result<bool, Stop> {
        let mut arguments = args![unit, x, y];
        arguments.extend(event.map(Val::from));
        Ok(self.call(Function::UnitWalk, arguments)?.number()? != 0)
    }

    /// `unitwalkto`: [`unit_walk`](Self::unit_walk) to the cell `target` stands on now, on the same map.
    pub fn unit_walk_to(&self, unit: i32, target: i32, event: Option<&str>) -> Result<bool, Stop> {
        let mut arguments = args![unit, target];
        arguments.extend(event.map(Val::from));
        Ok(self.call(Function::UnitWalkTo, arguments)?.number()? != 0)
    }

    /// `unitstopwalk`: stops `unit` and drops the event of its walk.
    pub fn unit_stop_walk(&self, unit: i32) -> Script {
        self.call(Function::UnitStopWalk, args![unit]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn walks_send_the_unit_first_and_the_event_only_when_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.unit_walk(2000001, 150, 150, None), Ok(true));
        ctx.unit_walk_to(2000001, 110001, Some("Guide::OnThere")).unwrap();
        assert_eq!(transport.calls(Function::UnitWalk), vec![vec![Value::new_number(2000001), Value::new_number(150), Value::new_number(150)]]);
        assert_eq!(transport.calls(Function::UnitWalkTo)[0][2], Value::new_string("Guide::OnThere".into()));
    }

    #[test]
    fn unit_walk_is_false_when_the_unit_does_not_start() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        assert_eq!(Ctx::new(&transport).unit_walk(5, 1, 1, None), Ok(false));
    }

    #[test]
    fn stop_sends_the_unit() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).unit_stop_walk(7).unwrap();
        assert_eq!(transport.calls(Function::UnitStopWalk), vec![vec![Value::new_number(7)]]);
    }
}
