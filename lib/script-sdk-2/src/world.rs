use crate::battleground::Spot;
use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
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

    /// [`announce`](Self::announce) in text colour `color`, written as rathena does, such as `"0xFFCE00"`.
    pub fn announce_colored(&self, message: &str, flag: i32, color: &str) -> Script {
        self.call(Function::Announce, args![message, flag, color]).map(|_| ())
    }

    /// Shows or hides a named NPC on the map.
    pub fn set_npc_visible(&self, npc: &str, visible: bool) -> Script {
        let function = if visible { Function::EnableNpc } else { Function::DisableNpc };
        self.call(function, args![npc]).map(|_| ())
    }
}

impl Ctx<'_> {
    /// Kills the monsters on `map` spawned by the script event `label`. `"all"` kills every script monster on the map.
    pub fn kill_monster(&self, map: &str, label: &str) -> Script {
        self.call(Function::KillMonster, args![map, label]).map(|_| ())
    }
}

impl Ctx<'_> {
    /// Broadcasts `message` to everyone on `map`, or to the current NPC's map when `map` is `"this"`. `flag` picks the
    /// audience, and `color` is an optional text colour.
    pub fn map_announce(&self, map: &str, message: &str, flag: i32, color: Option<i32>) -> Script {
        let mut arguments = args![map, message, flag];
        arguments.extend(color.map(Val::from));
        self.call(Function::MapAnnounce, arguments).map(|_| ())
    }

    /// Points the attached player's view at `x`, `y`, as rathena's `viewpoint`. `action` `1` adds a marker, `2` updates
    /// it and `0` removes it.
    pub fn view_point(&self, action: i32, x: i32, y: i32, number: i32, color: i32) -> Script {
        self.call(Function::ViewPoint, args![action, x, y, number, color]).map(|_| ())
    }

    /// How many players are on `map`.
    pub fn map_users(&self, map: &str) -> Result<i32, Stop> {
        self.call(Function::GetMapUsers, args![map])?.number()
    }

    /// How many script monsters on `map` belong to the event `label`. `"all"` counts every script monster there.
    pub fn mob_count(&self, map: &str, label: &str) -> Result<i32, Stop> {
        self.call(Function::MobCount, args![map, label])?.number()
    }

    /// The current local time, formatted with a chrono-style `format` and cut to `limit` characters.
    pub fn time_string(&self, format: &str, limit: i32) -> Result<String, Stop> {
        self.call(Function::GetTimeStr, args![format, limit]).map(|value| value.text())
    }
}

impl Ctx<'_> {
    /// Moves every player on `source` to `destination`, as rathena's `mapwarp`.
    pub fn map_warp(&self, source: &str, destination: Spot) -> Script {
        self.call(Function::MapWarp, args![source, destination.map, destination.x, destination.y]).map(|_| ())
    }

    /// Moves every player inside `area` on `source` to `destination`, as rathena's `areawarp`.
    pub fn area_warp(&self, source: &str, area: Area, destination: Spot) -> Script {
        self.call(Function::AreaWarp, args![source, area.x1, area.y1, area.x2, area.y2, destination.map, destination.x, destination.y])
            .map(|_| ())
    }

    /// Heals every player inside `area` on `map` by `hp` and `sp` percent, as rathena's `areapercentheal`.
    pub fn area_heal(&self, map: &str, area: Area, hp: i32, sp: i32) -> Script {
        self.call(Function::AreaPercentHeal, args![map, area.x1, area.y1, area.x2, area.y2, hp, sp]).map(|_| ())
    }

    /// One field of the current local time, such as `constants::DT_YEAR`, as rathena's `gettime`.
    pub fn time_field(&self, field: impl Into<Val>) -> Result<i32, Stop> {
        self.call(Function::GetTime, args![field.into()])?.number()
    }
}

impl Ctx<'_> {
    /// Sets or clears a cell type (`constants::CELL_*`) over `area` of `map`, as rathena's `setcell`.
    pub fn set_cell(&self, map: &str, area: Area, cell: i32, enabled: bool) -> Script {
        self.call(Function::SetCell, args![map, area.x1, area.y1, area.x2, area.y2, cell, i32::from(enabled)]).map(|_| ())
    }

    /// Sets a map flag (`constants::MF_*`) on `map`. `values` are the flag's own arguments, such as a skill id for the
    /// skill duration flag.
    pub fn set_map_flag(&self, map: &str, flag: i32, values: &[i32]) -> Script {
        self.map_flag(Function::SetMapFlag, map, flag, values)
    }

    /// Clears a map flag set with [`set_map_flag`](Self::set_map_flag).
    pub fn remove_map_flag(&self, map: &str, flag: i32, values: &[i32]) -> Script {
        self.map_flag(Function::RemoveMapFlag, map, flag, values)
    }

    /// Spawns a castle guardian `name` of monster `class` at `x`, `y` of `map`, running `event` when it dies. rathena's
    /// guardian index is not sent, because the server does not support it yet (see script-server-gaps.md).
    pub fn guardian(&self, map: &str, x: i32, y: i32, name: &str, class: i32, event: Option<&str>) -> Script {
        let mut arguments = args![map, x, y, name, class];
        arguments.extend(event.map(Val::from));
        self.call(Function::Guardian, arguments).map(|_| ())
    }

    /// Makes the script monsters of event `label` on `map` immune to damage, or lifts it.
    pub fn set_mob_immunity(&self, map: &str, label: &str, immune: bool) -> Script {
        self.call(Function::SetMobImmunity, args![map, label, immune]).map(|_| ())
    }

    /// The value of server battle setting `name`, such as `"max_hair_style"`, as rathena's `getbattleflag`.
    pub fn battle_flag(&self, name: &str) -> Result<i32, Stop> {
        self.call(Function::GetBattleFlag, args![name])?.number()
    }

    fn map_flag(&self, function: Function, map: &str, flag: i32, values: &[i32]) -> Script {
        let mut arguments = args![map, flag];
        arguments.extend(values.iter().copied().map(Val::from));
        self.call(function, arguments).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::world::Area;
    use crate::battleground::Spot;
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

    #[test]
    fn kill_monster_sends_the_map_and_label() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).kill_monster("in_moc_16", "all").unwrap();
        assert_eq!(transport.calls(Function::KillMonster), vec![vec![Value::new_string("in_moc_16".into()), Value::new_string("all".into())]]);
    }

    #[test]
    fn map_announce_sends_the_colour_only_when_given() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.map_announce("prontera", "Hello", 0, None).unwrap();
        ctx.map_announce("prontera", "Hello", 0, Some(16)).unwrap();
        let calls = transport.calls(Function::MapAnnounce);
        assert_eq!((calls[0].len(), calls[1].len()), (3, 4));
    }

    #[test]
    fn view_point_sends_five_values_in_order() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).view_point(1, 73, 22, 1, 16724821).unwrap();
        assert_eq!(transport.calls(Function::ViewPoint)[0][1], Value::new_number(73));
    }

    #[test]
    fn map_warp_sends_source_then_destination() {
        let transport = MockTransport::silent();
        let destination = Spot { map: "prontera", x: 150, y: 150 };
        Ctx::new(&transport).map_warp("payon", destination).unwrap();
        assert_eq!(transport.calls(Function::MapWarp)[0].len(), 4);
    }

    #[test]
    fn area_warp_sends_the_area_before_the_destination() {
        let transport = MockTransport::silent();
        let area = Area { x1: 1, y1: 2, x2: 3, y2: 4 };
        Ctx::new(&transport).area_warp("payon", area, Spot { map: "prontera", x: 5, y: 6 }).unwrap();
        let call = &transport.calls(Function::AreaWarp)[0];
        assert_eq!((call.len(), call[5].clone()), (8, Value::new_string("prontera".into())));
    }

    #[test]
    fn set_cell_sends_the_cell_type_then_the_flag() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).set_cell("prontera", Area { x1: 1, y1: 2, x2: 3, y2: 4 }, 5, true).unwrap();
        let call = &transport.calls(Function::SetCell)[0];
        assert_eq!(call.len(), 7);
        assert_eq!(call[5], Value::new_number(5));
        assert_eq!(call[6], Value::new_number(1));
    }

    #[test]
    fn guardian_sends_the_event_only_when_given() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.guardian("gef_fild10", 10, 20, "Guardian", 1285, None).unwrap();
        ctx.guardian("gef_fild10", 10, 20, "Guardian", 1285, Some("Guardian::OnDeath")).unwrap();
        let calls = transport.calls(Function::Guardian);
        assert_eq!(calls[0].len(), 5);
        assert_eq!(calls[1].len(), 6);
    }

    #[test]
    fn remove_map_flag_sends_the_values_after_the_flag() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).remove_map_flag("prontera", 6, &[]).unwrap();
        Ctx::new(&transport).set_map_flag("prontera", 6, &[9]).unwrap();
        assert_eq!(transport.calls(Function::RemoveMapFlag)[0].len(), 2);
        assert_eq!(transport.calls(Function::SetMapFlag)[0].last(), Some(&Value::new_number(9)));
    }
}
