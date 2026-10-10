use script_sdk::{Function, Request, Value};

use crate::flow::Stop;
use crate::transport::Transport;
use crate::value::Val;

/// The conversation a script runs in: the player, the NPC and the host behind them.
///
/// Scripts take `&Ctx` and return [`Script`](crate::Script). Each method is a request to the host, and its
/// failure is a [`Stop`] that the script propagates with `?`.
pub struct Ctx<'a> {
    transport: &'a dyn Transport,
}

impl<'a> Ctx<'a> {
    pub fn new(transport: &'a dyn Transport) -> Self {
        Self { transport }
    }

    pub fn request(&self, request: Request) -> Result<Val, Stop> {
        self.transport.request(request).map(Val::from).map_err(Stop::from)
    }

    /// Calls a host function by its enum variant. Typed methods on `Ctx` cover the common ones.
    pub fn call(&self, function: Function, arguments: Vec<Val>) -> Result<Val, Stop> {
        let arguments = arguments.into_iter().map(Value::from).collect();
        self.request(Request::Call { function, arguments })
    }

    /// Looks up a server constant by its rathena name, such as `"JOB_NOVICE"`.
    pub fn constant(&self, name: &str) -> Result<Val, Stop> {
        self.request(Request::Constant(name.into()))
    }

    /// The values the NPC's placement passes to its script, or the ones of the event that started it.
    pub fn arguments(&self) -> Result<Vec<Val>, Stop> {
        self.request(Request::Arguments)?.into_array().ok_or_else(|| Stop::from("Script arguments are invalid"))
    }
}
