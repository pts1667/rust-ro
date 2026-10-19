use crate::value::Val;
use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

impl<'a> Ctx<'a> {
    /// Effects on the current NPC, such as `ctx.npc().emotion(0)`.
    pub fn npc<'c>(&'c self) -> Npc<'c, 'a> {
        Npc { ctx: self }
    }
}

pub struct Npc<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Npc<'_, '_> {
    /// Shows an emotion bubble over the NPC.
    pub fn emotion(&self, emotion: i32) -> Script {
        self.ctx.call(Function::Emotion, args![emotion]).map(|_| ())
    }

    /// Runs the label `event` of another NPC, such as `"Guard::OnTalk"`.
    pub fn do_event(&self, event: &str) -> Script {
        self.ctx.call(Function::DoNpcEvent, args![event]).map(|_| ())
    }

    /// Plays a special effect on the NPC.
    pub fn special_effect(&self, effect: i32) -> Script {
        self.ctx.call(Function::NpcSpecialEffect, args![effect]).map(|_| ())
    }

    /// Opens the buy and sell window of the NPC's shop, stocked from its placement.
    pub fn open_shop(&self) -> Script {
        self.ctx.call(Function::Shop, args![]).map(|_| ())
    }
}

impl Npc<'_, '_> {
    /// Changes the sprite of the NPC named `npc` to `sprite`, a number or a sprite constant.
    pub fn set_display(&self, npc: &str, sprite: impl Into<Val>) -> Script {
        self.ctx.call(Function::SetNpcDisplay, vec![Val::from(npc), sprite.into()]).map(|_| ())
    }

    /// The id of NPC `npc`, or of the current NPC when `None`. `0` when no such NPC is on the map.
    pub fn id(&self, npc: Option<&str>) -> Result<i32, Stop> {
        let mut arguments = args![0];
        arguments.extend(npc.map(Val::from));
        self.ctx.call(Function::GetNpcId, arguments)?.number()
    }

    /// The name of the current NPC, including any `#` prefix.
    pub fn name(&self) -> Result<String, Stop> {
        self.info(0)
    }

    /// The part of the current NPC's name before `#`, the one the player sees.
    pub fn visible_name(&self) -> Result<String, Stop> {
        self.info(1)
    }

    /// The part of the current NPC's name after `#`.
    pub fn hidden_name(&self) -> Result<String, Stop> {
        self.info(2)
    }

    /// The map the current NPC is on.
    pub fn map_name(&self) -> Result<String, Stop> {
        self.info(4)
    }

    /// Starts `npc` (the current NPC when `None`) walking to `(x, y)` on its map and returns at once. False when no path
    /// leads there: paths are short, like a click on the ground, so a long route is a chain of walks.
    pub fn walk_to(&self, npc: Option<&str>, x: i32, y: i32) -> Result<bool, Stop> {
        let mut arguments = args![x, y];
        arguments.extend(npc.map(Val::from));
        Ok(self.ctx.call(Function::NpcWalkTo, arguments)?.number()? != 0)
    }

    /// [`walk_to`](Self::walk_to), then runs `event` (`"Npc::OnLabel"`, without a player) when the NPC arrives. A stop,
    /// another walk or a jump (`movenpc`, knockback) drops the event.
    pub fn walk_to_then(&self, npc: Option<&str>, x: i32, y: i32, event: &str) -> Result<bool, Stop> {
        Ok(self.ctx.call(Function::NpcWalkTo, args![x, y, npc.unwrap_or(""), event])?.number()? != 0)
    }

    /// Stops `npc` (the current NPC when `None`) on the cell it is entering.
    pub fn stop_walking(&self, npc: Option<&str>) -> Script {
        self.ctx.call(Function::NpcStop, npc.map(|npc| args![npc]).unwrap_or_default()).map(|_| ())
    }

    /// Walking speed of `npc` (the current NPC when `None`) in milliseconds per cell, from 20 to 1000; NPCs start at 200.
    pub fn set_walk_speed(&self, npc: Option<&str>, speed: i32) -> Script {
        let mut arguments = args![speed];
        arguments.extend(npc.map(Val::from));
        self.ctx.call(Function::NpcSpeed, arguments).map(|_| ())
    }

    fn info(&self, kind: i32) -> Result<String, Stop> {
        self.ctx.call(Function::StrNpcInfo, args![kind]).map(|value| value.text())
    }

    /// A variable of NPC `npc`, named with its leading `.` (such as `".flag"`). `index` picks the array element.
    pub fn variable(&self, name: &str, npc: &str, index: i32) -> Result<Val, Stop> {
        self.ctx.call(Function::GetVariableOfNpc, args![name, npc, index])
    }

    /// Sets a variable of NPC `npc`. Names ending in `$` take text, the rest take numbers.
    pub fn set_variable(&self, name: &str, npc: &str, index: i32, value: impl Into<Val>) -> Script {
        self.ctx.call(Function::SetVariableOfNpc, args![name, npc, index, value.into()]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn emotion_and_effect_send_one_call_each() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.npc().emotion(3).unwrap();
        ctx.npc().special_effect(12).unwrap();
        assert_eq!(transport.calls(Function::Emotion), vec![vec![Value::new_number(3)]]);
        assert_eq!(transport.calls(Function::NpcSpecialEffect), vec![vec![Value::new_number(12)]]);
    }

    #[test]
    fn set_display_sends_the_npc_then_the_sprite() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).npc().set_display("Guard", 45).unwrap();
        assert_eq!(transport.calls(Function::SetNpcDisplay)[0], vec![Value::new_string("Guard".into()), Value::new_number(45)]);
    }

    #[test]
    fn id_names_the_npc_only_when_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(7)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.npc().id(None), Ok(7));
        ctx.npc().id(Some("Guard")).unwrap();
        let calls = transport.calls(Function::GetNpcId);
        assert_eq!(calls[0], vec![Value::new_number(0)]);
        assert_eq!(calls[1], vec![Value::new_number(0), Value::new_string("Guard".into())]);
    }

    #[test]
    fn walking_names_the_npc_only_when_given_and_keeps_its_place_for_the_event() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.npc().walk_to(None, 10, 12), Ok(true));
        ctx.npc().walk_to(Some("Baker"), 10, 12).unwrap();
        ctx.npc().walk_to_then(None, 10, 12, "Baker::OnOven").unwrap();
        let calls = transport.calls(Function::NpcWalkTo);
        assert_eq!(calls[0], vec![Value::new_number(10), Value::new_number(12)]);
        assert_eq!(calls[1][2], Value::new_string("Baker".into()));
        assert_eq!(calls[2][2..], [Value::new_string(String::new()), Value::new_string("Baker::OnOven".into())]);
    }

    #[test]
    fn walk_to_is_false_without_a_path() {
        let transport = MockTransport::new(|_| Ok(Value::Number(0)));
        assert_eq!(Ctx::new(&transport).npc().walk_to(None, 10, 12), Ok(false));
    }

    #[test]
    fn stop_and_speed_name_the_npc_only_when_given() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.npc().stop_walking(None).unwrap();
        ctx.npc().set_walk_speed(Some("Baker"), 400).unwrap();
        assert_eq!(transport.calls(Function::NpcStop), vec![vec![]]);
        assert_eq!(transport.calls(Function::NpcSpeed), vec![vec![Value::new_number(400), Value::new_string("Baker".into())]]);
    }

    #[test]
    fn set_variable_sends_the_name_npc_index_then_value() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).npc().set_variable(".flag", "Guard", 2, 5).unwrap();
        assert_eq!(transport.calls(Function::SetVariableOfNpc)[0], vec![
            Value::new_string(".flag".into()),
            Value::new_string("Guard".into()),
            Value::new_number(2),
            Value::new_number(5),
        ]);
    }
}
