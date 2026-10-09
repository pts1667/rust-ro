use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Script;
use crate::Val;

/// A rectangle of map cells, from `(x1, y1)` to `(x2, y2)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Ctx<'_> {
    /// Moves the player to `x`, `y` on `map`.
    pub fn warp(&self, map: &str, x: i32, y: i32) -> Script {
        self.call(Function::Warp, args![map, x, y]).map(|_| ())
    }

    /// Spawns `amount` monsters of `class` called `name` at `x`, `y` on `map`. `event` is the NPC label run when one dies.
    pub fn monster(&self, map: &str, x: i32, y: i32, name: &str, class: i32, amount: i32, event: Option<&str>) -> Script {
        let mut arguments = args![map, x, y, name, class, amount];
        arguments.extend(event.map(Val::from));
        self.call(Function::Monster, arguments).map(|_| ())
    }

    /// Spawns `amount` monsters at random points inside `area` on `map`.
    pub fn area_monster(&self, map: &str, area: Area, name: &str, class: i32, amount: i32, event: Option<&str>) -> Script {
        let mut arguments = args![map, area.x1, area.y1, area.x2, area.y2, name, class, amount];
        arguments.extend(event.map(Val::from));
        self.call(Function::AreaMonster, arguments).map(|_| ())
    }

    /// Broadcasts `message`. `flag` picks the audience, such as `constants::BC_ALL`.
    pub fn announce(&self, message: &str, flag: i32) -> Script {
        self.call(Function::Announce, args![message, flag]).map(|_| ())
    }

    /// Shows or hides a named NPC on the map.
    pub fn set_npc_visible(&self, npc: &str, visible: bool) -> Script {
        let function = if visible { Function::EnableNpc } else { Function::DisableNpc };
        self.call(function, args![npc]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::world::Area;
    use crate::Ctx;

    #[test]
    fn monster_sends_the_optional_event_only_when_given() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.monster("prontera", 150, 150, "Poring", 1002, 3, None).unwrap();
        ctx.monster("prontera", 150, 150, "Poring", 1002, 3, Some("Ev::OnKill")).unwrap();
        let calls = transport.calls(Function::Monster);
        assert_eq!(calls[0].len(), 6);
        assert_eq!(calls[1].last(), Some(&Value::new_string("Ev::OnKill".into())));
    }

    #[test]
    fn area_monster_sends_the_corners_in_order() {
        let transport = MockTransport::silent();
        let area = Area { x1: 1, y1: 2, x2: 3, y2: 4 };
        Ctx::new(&transport).area_monster("prontera", area, "Poring", 1002, 5, None).unwrap();
        let call = &transport.calls(Function::AreaMonster)[0];
        assert_eq!(call[..5], [Value::new_string("prontera".into()), Value::new_number(1), Value::new_number(2), Value::new_number(3), Value::new_number(4)]);
        assert_eq!(call[5], Value::new_string("Poring".into()));
    }
}
