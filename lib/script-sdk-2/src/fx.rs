use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Script;

impl<'a> Ctx<'a> {
    /// Visual effects for the player, such as `ctx.fx().cutin(..)`.
    pub fn fx<'c>(&'c self) -> Fx<'c, 'a> {
        Fx { ctx: self }
    }
}

pub struct Fx<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Fx<'_, '_> {
    /// Shows the full-screen image `image` (a file name in the client's `cutin` folder) at `position`.
    pub fn cutin(&self, image: &str, position: i32) -> Script {
        self.ctx.call(Function::Cutin, args![image, position]).map(|_| ())
    }

    /// Plays special effect `effect` on the player.
    pub fn special_effect(&self, effect: i32) -> Script {
        self.ctx.call(Function::SpecialEffect, args![effect]).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn cutin_sends_the_image_then_the_position() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).fx().cutin("ep15_bg", 2).unwrap();
        assert_eq!(transport.calls(Function::Cutin), vec![vec![Value::new_string("ep15_bg".into()), Value::new_number(2)]]);
    }
}
