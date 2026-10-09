use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::world::Area;

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

impl Fx<'_, '_> {
    /// Plays the sound file `file` for the player. `kind` is the sound type number.
    pub fn sound_effect(&self, file: &str, kind: i32) -> Script {
        self.ctx.call(Function::SoundEffect, args![file, kind]).map(|_| ())
    }

    /// Plays the sound file `file` for everyone. With `map`, only the players on it hear it, and with `area` only the
    /// ones inside it. Without `map`, only the players who see the current NPC hear it. Errors when `area` is given
    /// without `map`.
    pub fn sound_effect_all(&self, file: &str, kind: i32, map: Option<&str>, area: Option<Area>) -> Script {
        let mut arguments = args![file, kind];
        match (map, area) {
            (Some(map), Some(area)) => arguments.extend(args![map, area.x1, area.y1, area.x2, area.y2]),
            (Some(map), None) => arguments.extend(args![map]),
            (None, None) => {}
            (None, Some(_)) => return Err(Stop::Error("An area needs a map".into())),
        }
        self.ctx.call(Function::SoundEffectAll, arguments).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::world::Area;
    use crate::Ctx;

    #[test]
    fn cutin_sends_the_image_then_the_position() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).fx().cutin("ep15_bg", 2).unwrap();
        assert_eq!(transport.calls(Function::Cutin), vec![vec![Value::new_string("ep15_bg".into()), Value::new_number(2)]]);
    }

    #[test]
    fn sound_effect_all_sends_the_area_after_the_map() {
        let transport = MockTransport::silent();
        let area = Area { x1: 1, y1: 2, x2: 3, y2: 4 };
        Ctx::new(&transport).fx().sound_effect_all("bgm.mp3", 0, Some("prontera"), Some(area)).unwrap();
        assert_eq!(transport.calls(Function::SoundEffectAll)[0].len(), 7);
    }

    #[test]
    fn sound_effect_all_rejects_an_area_without_a_map() {
        let transport = MockTransport::silent();
        let area = Area { x1: 1, y1: 2, x2: 3, y2: 4 };
        assert!(Ctx::new(&transport).fx().sound_effect_all("bgm.mp3", 0, None, Some(area)).is_err());
        assert!(transport.calls(Function::SoundEffectAll).is_empty());
    }
}
