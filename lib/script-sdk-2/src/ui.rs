//! Plugin windows, overhead notifications and what a plugin shows to bots (`docs/plugin-ui.md`).

use script_sdk::{BotAction, Function};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

pub use script_sdk::plugin_ui::{
    Align, Audience, Easing, EventKind, Frame, HudSlot, ImageRef, InputValue, Motion, MotionKind, Node, Overhead, Placement, Table, UiEvent,
    Window, format_countdown, validate_overhead, validate_window,
};

/// Handles the events of one window the plugin declared: `fn(&Ctx, &UiEvent) -> Script`, with the player attached.
pub type WindowFn = fn(&Ctx<'_>, &UiEvent) -> Script;

/// Runs a bot action the plugin declared, with the bot's character attached. What it passes to
/// [`Bot::reply`] is the answer the bot receives.
pub type BotActionFn = fn(&Ctx<'_>, &BotAction) -> Script;

/// Who an overhead notification is drawn above.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Above {
    /// The attached player.
    Player,
    /// Any unit by its id (a character id, an NPC or monster id).
    Unit(u32),
}

impl<'a> Ctx<'a> {
    /// Windows and overhead notifications for the attached player; needs the `Interface` capability.
    pub fn ui<'c>(&'c self) -> Ui<'c, 'a> {
        Ui { ctx: self }
    }

    /// What bots see of this plugin, and the answer to a bot action.
    pub fn bot<'c>(&'c self) -> Bot<'c, 'a> {
        Bot { ctx: self }
    }
}

pub struct Ui<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Ui<'_, '_> {
    /// Opens `window`, or replaces the window with the same id. It stays open after the script ends, until the
    /// player or the plugin closes it; its events go to the handler declared for its id in `windows { }`.
    pub fn open(&self, window: &Window) -> Script {
        self.ctx.call(Function::UiOpen, vec![json(window)?.into()]).map(|_| ())
    }

    pub fn close(&self, id: &str) -> Script {
        self.ctx.call(Function::UiClose, vec![id.into()]).map(|_| ())
    }

    pub fn overhead(&self, above: Above, overhead: &Overhead) -> Script {
        let unit = match above {
            Above::Player => 0,
            Above::Unit(id) => i32::try_from(id).map_err(|_| Stop::from("Unit id is out of range"))?,
        };
        self.ctx.call(Function::UiOverhead, vec![unit.into(), json(overhead)?.into()]).map(|_| ())
    }
}

pub struct Bot<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Bot<'_, '_> {
    /// Sets what bots observe of this plugin under `key` for the attached player (a bot's own character). It stays
    /// until replaced, removed or the character logs out.
    pub fn observe(&self, key: &str, value: &impl Serialize) -> Script {
        self.ctx.call(Function::BotObserve, vec![key.into(), json(value)?.into()]).map(|_| ())
    }

    pub fn forget(&self, key: &str) -> Script {
        self.ctx.call(Function::BotObserve, vec![key.into(), "null".into()]).map(|_| ())
    }

    /// The answer to the bot action being run. The last reply wins; an action that ends without one answers `null`.
    pub fn reply(&self, value: &impl Serialize) -> Script {
        self.ctx.call(Function::BotReply, vec![json(value)?.into()]).map(|_| ())
    }
}

/// The `args` of a bot action, decoded.
pub fn bot_args<T: DeserializeOwned>(action: &BotAction) -> Result<T, Stop> {
    serde_json::from_str(&action.args).map_err(|error| Stop::from(format!("Invalid arguments for {}: {error}", action.name)))
}

fn json(value: &impl Serialize) -> Result<String, Stop> {
    serde_json::to_string(value).map_err(|error| Stop::from(error.to_string()))
}

#[cfg(test)]
mod tests {
    use script_sdk::{Request, Value};

    use super::*;
    use crate::MockTransport;

    #[test]
    fn windows_and_overheads_go_to_the_host_as_json() {
        let transport = MockTransport::new(|_| Ok(Value::default()));
        let ctx = Ctx::new(&transport);
        let window = Window::new("shop", "Shop", 200, Node::column([Node::text("Hello"), Node::button("buy", "Buy")]));
        ctx.ui().open(&window).unwrap();
        ctx.ui().close("shop").unwrap();
        ctx.ui().overhead(Above::Player, &Overhead::text("Hi", None, 1000).rise(20.0, 500)).unwrap();
        let requests = transport.requests();
        let Request::Call { function: Function::UiOpen, arguments } = &requests[0] else { panic!("{:?}", requests[0]) };
        let Value::String(sent) = &arguments[0] else { panic!() };
        assert_eq!(serde_json::from_str::<Window>(sent).unwrap(), window);
        assert_eq!(requests[1], Request::Call { function: Function::UiClose, arguments: vec!["shop".into()] });
        assert!(matches!(&requests[2], Request::Call { function: Function::UiOverhead, arguments } if arguments[0] == Value::Number(0)));
    }

    #[test]
    fn bots_get_observations_replies_and_arguments() {
        #[derive(serde::Deserialize)]
        struct Buy {
            item_id: u32,
        }
        let transport = MockTransport::new(|_| Ok(Value::default()));
        let ctx = Ctx::new(&transport);
        ctx.bot().observe("stock", &[501, 502]).unwrap();
        ctx.bot().forget("stock").unwrap();
        ctx.bot().reply(&serde_json::json!({ "bought": 2 })).unwrap();
        assert_eq!(transport.requests()[0], Request::Call { function: Function::BotObserve, arguments: vec!["stock".into(), "[501,502]".into()] });
        assert_eq!(transport.requests()[1], Request::Call { function: Function::BotObserve, arguments: vec!["stock".into(), "null".into()] });
        assert_eq!(transport.requests()[2], Request::Call { function: Function::BotReply, arguments: vec![r#"{"bought":2}"#.into()] });
        let action = BotAction { name: "buy".into(), args: r#"{"item_id":501}"#.into() };
        assert_eq!(bot_args::<Buy>(&action).unwrap().item_id, 501);
        assert!(bot_args::<Buy>(&BotAction { name: "buy".into(), args: "{}".into() }).is_err());
    }
}
