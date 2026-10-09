use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Script;

impl Ctx<'_> {
    /// Moves the player to `x`, `y` on `map`.
    pub fn warp(&self, map: &str, x: i32, y: i32) -> Script {
        self.call(Function::Warp, args![map, x, y]).map(|_| ())
    }

    /// Shows or hides a named NPC on the map.
    pub fn set_npc_visible(&self, npc: &str, visible: bool) -> Script {
        let function = if visible { Function::EnableNpc } else { Function::DisableNpc };
        self.call(function, args![npc]).map(|_| ())
    }
}
