use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;
use crate::world::Area;

impl<'a> Ctx<'a> {
    /// Battleground teams, queues and monsters, such as `ctx.battleground().join(bg, 0, None)`.
    pub fn battleground<'c>(&'c self) -> Battleground<'c, 'a> {
        Battleground { ctx: self }
    }
}

/// A map position: `map`, `x`, `y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spot<'s> {
    pub map: &'s str,
    pub x: i32,
    pub y: i32,
}

/// The NPC labels a battleground team runs when a player quits, dies, or the battle starts. Empty means none.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BattlegroundEvents<'s> {
    pub quit: &'s str,
    pub die: &'s str,
    pub active: &'s str,
}

pub struct Battleground<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Battleground<'_, '_> {
    /// Creates a team with its respawn point `cemetery` (`None` for no respawn point). Returns the team id, or `0` when
    /// the cemetery map does not exist.
    pub fn create(&self, cemetery: Option<Spot>, events: BattlegroundEvents) -> Result<i32, Stop> {
        let (map, x, y) = match cemetery {
            Some(spot) => (spot.map, spot.x, spot.y),
            None => ("-", 0, 0),
        };
        self.ctx.call(Function::BgCreate, args![map, x, y, events.quit, events.die, events.active])?.number()
    }

    /// Puts character `char_id` (`0` for the attached player) into team `bg`. `destination` overrides the team's
    /// respawn point. Returns whether the character joined and was moved.
    pub fn join(&self, bg: i32, char_id: i32, destination: Option<Spot>) -> Result<bool, Stop> {
        let (map, x, y) = match destination {
            Some(spot) => (spot.map, spot.x, spot.y),
            None => ("", 0, 0),
        };
        Ok(self.ctx.call(Function::BgJoin, args![bg, map, x, y, char_id])?.number()? != 0)
    }

    /// Removes character `char_id` (the attached player when `None`) from their team.
    pub fn leave(&self, char_id: Option<i32>) -> Script {
        self.ctx.call(Function::BgLeave, char_id.map(|id| args![id]).unwrap_or_default()).map(|_| ())
    }

    /// Like [`leave`](Self::leave), but counts as a desertion and starts the deserter penalty.
    pub fn desert(&self, char_id: Option<i32>) -> Script {
        self.ctx.call(Function::BgDesert, char_id.map(|id| args![id]).unwrap_or_default()).map(|_| ())
    }

    pub fn destroy(&self, bg: i32) -> Script {
        self.ctx.call(Function::BgDestroy, args![bg]).map(|_| ())
    }

    /// Warps every member of team `bg` to `spot`.
    pub fn warp(&self, bg: i32, spot: Spot) -> Script {
        self.ctx.call(Function::BgWarp, args![bg, spot.map, spot.x, spot.y]).map(|_| ())
    }

    /// Moves the respawn point of team `bg` to `x`, `y`.
    pub fn set_cemetery(&self, bg: i32, x: i32, y: i32) -> Script {
        self.ctx.call(Function::BgTeamSetXy, args![bg, x, y]).map(|_| ())
    }

    /// The character ids in team `bg`.
    pub fn members(&self, bg: i32) -> Result<Vec<i32>, Stop> {
        let members = self.ctx.call(Function::BgGetData, args![bg, 1])?.into_array().unwrap_or_default();
        members.into_iter().map(|member| member.number()).collect()
    }

    pub fn member_count(&self, bg: i32) -> Result<i32, Stop> {
        self.ctx.call(Function::BgGetData, args![bg, 0])?.number()
    }

    /// How many members of team `bg` stand inside `area` on `map`.
    pub fn count_in_area(&self, bg: i32, map: &str, area: Area) -> Result<i32, Stop> {
        self.ctx.call(Function::BgGetAreaUsers, args![bg, map, area.x1, area.y1, area.x2, area.y2])?.number()
    }

    /// Sets the score shown to the players on `map`.
    pub fn update_score(&self, map: &str, first: i32, second: i32) -> Script {
        self.ctx.call(Function::BgUpdateScore, args![map, first, second]).map(|_| ())
    }

    /// Books `map` for the battleground queue. `ended` marks the battle on `map` as finished.
    pub fn reserve(&self, map: &str, ended: bool) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::BgReserve, args![map, i32::from(ended)])?.number()? != 0)
    }

    /// Releases the booking of `map`.
    pub fn unbook(&self, map: &str) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::BgUnbook, args![map])?.number()? != 0)
    }

    /// A battleground type's data, looked up by its rathena name, such as `"Bossnia"`. `kind` picks the field:
    /// `0` id, `1` required players, `2` max players, `3` min level, `4` max level, `5` maps, `6` deserter seconds.
    /// Returned as a dynamic value, because the fields differ in type.
    pub fn info(&self, name: &str, kind: i32) -> Result<Val, Stop> {
        self.ctx.call(Function::BgInfo, args![name, kind])
    }

    /// Spawns a monster for team `bg` at `spot` and returns its id. `event` is the NPC label run when it dies.
    /// The server spawns exactly one monster per call.
    pub fn monster(&self, bg: i32, spot: Spot, name: &str, class: i32, event: Option<&str>) -> Result<i32, Stop> {
        let mut arguments = args![bg, spot.map, spot.x, spot.y, name, class];
        arguments.extend(event.map(Val::from));
        self.ctx.call(Function::BgMonster, arguments)?.number()
    }

    /// Moves the monster `mob_id` (from [`monster`](Self::monster)) into team `bg`.
    pub fn set_monster_team(&self, mob_id: i32, bg: i32) -> Script {
        self.ctx.call(Function::BgMonsterSetTeam, args![mob_id, bg]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::battleground::{BattlegroundEvents, Spot};
    use crate::transport::MockTransport;
    use crate::world::Area;
    use crate::Ctx;

    #[test]
    fn create_without_a_cemetery_sends_a_dash() {
        let transport = MockTransport::new(|_| Ok(Value::Number(4)));
        let id = Ctx::new(&transport).battleground().create(None, BattlegroundEvents { quit: "Q", ..Default::default() });
        assert_eq!(id, Ok(4));
        let call = &transport.calls(Function::BgCreate)[0];
        assert_eq!(call[0], Value::new_string("-".into()));
        assert_eq!(call[3], Value::new_string("Q".into()));
    }

    #[test]
    fn join_without_a_destination_sends_an_empty_map() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        assert_eq!(Ctx::new(&transport).battleground().join(2, 0, None), Ok(true));
        assert_eq!(transport.calls(Function::BgJoin)[0], vec![Value::new_number(2), Value::new_string(String::new()), Value::new_number(0), Value::new_number(0), Value::new_number(0)]);
    }

    #[test]
    fn members_reads_the_member_list() {
        let transport = MockTransport::new(|_| Ok(Value::Array(vec![Value::Number(10), Value::Number(11)])));
        assert_eq!(Ctx::new(&transport).battleground().members(1), Ok(vec![10, 11]));
    }

    #[test]
    fn monster_sends_the_event_only_when_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(9)));
        let spot = Spot { map: "bat_a01", x: 5, y: 6 };
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.battleground().monster(1, spot, "Stone", 1288, None), Ok(9));
        ctx.battleground().monster(1, spot, "Stone", 1288, Some("Ev::OnDead")).unwrap();
        let calls = transport.calls(Function::BgMonster);
        assert_eq!(calls[0].len(), 6);
        assert_eq!(calls[1].len(), 7);
    }

    #[test]
    fn count_in_area_sends_the_corners() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        Ctx::new(&transport).battleground().count_in_area(1, "bat_a01", Area { x1: 1, y1: 2, x2: 3, y2: 4 }).unwrap();
        assert_eq!(transport.calls(Function::BgGetAreaUsers)[0].len(), 6);
    }
}
