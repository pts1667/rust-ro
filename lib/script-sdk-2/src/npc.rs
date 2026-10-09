use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Script;

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
}
