//! Readable API for NPC, item and event scripts.
//!
//! A script is a plain function that takes a [`Ctx`] and returns a [`Script`]:
//!
//! ```
//! use script_sdk_2::{Ctx, Function, MockTransport, Request, Script, Stop, Value};
//!
//! fn prontera_guard(ctx: &Ctx) -> Script {
//!     ctx.mes_as("Prontera Guard", "Outside the walls, stay alert.\nWant a tip?")?;
//!     ctx.next()?;
//!     if ctx.menu(&["Yes", "No"])? == 0 {
//!         ctx.mes("Watch the south gate.")?;
//!     }
//!     if ctx.items().count(501)? < 1 {
//!         ctx.items().give(501, 1)?;
//!     }
//!     ctx.close()
//! }
//!
//! let transport = MockTransport::new(|request| match request {
//!     Request::Call { function: Function::Select, .. } => Ok(Value::Number(1)),
//!     _ => Ok(Value::default()),
//! });
//! assert_eq!(prontera_guard(&Ctx::new(&transport)), Err(Stop::End));
//! ```
//!
//! Every `ctx` call returns `Result<_, Stop>`, so `?` stops the script early. Dialogue lines are sent as
//! [`Ctx::mes`] calls, and [`Ctx::menu`] returns the 0-based index of the choice. Variables use their rathena
//! names through [`Ctx::var`], and game actions are grouped by domain: [`Ctx::items`], [`Ctx::quests`],
//! [`Ctx::timers`], [`Ctx::warp`]. [`Ctx::call`] reaches any other host function.
//!
//! Tests run the same scripts against [`MockTransport`], so no game host is needed.

pub mod constants;
mod delay;
mod battleground;
mod bonus;
mod bonus_names;
mod ctx;
mod dialogue;
mod flow;
mod guild;
mod fx;
mod input;
mod instance;
mod item;
mod items;
mod module;
mod npc;
mod party;
mod pet;
mod player;
mod quests;
mod random;
pub mod runtime;
#[doc(hidden)]
pub mod registry;
mod timers;
mod transport;
mod value;
mod vars;
mod waiting_room;
mod world;

pub use ctx::Ctx;
pub use flow::{Script, Stop, finish};
pub use input::{Bound, Input};
pub use battleground::{Battleground, BattlegroundEvents, Spot};
pub use bonus::{AutoBonus, AutoSkillBonus, AutoSpell, AutoSpellOnSkill, Bonus, Bonus2, Bonus3, Bonuses};
pub use fx::Fx;
pub use guild::Guild;
pub use instance::{EnterOptions, Instance};
pub use item::{ItemBonus, ItemUse};
pub use items::Items;
pub use party::Party;
pub use npc::Npc;
pub use pet::{PetCatch, PetInfo, Pets};
pub use player::{Player, SkillGrant};
pub use quests::Quests;
pub use script_sdk::{Entry, Function, Request, Value, VariableScope};
pub use timers::Timers;
pub use transport::{MockTransport, Transport, WasmTransport};
pub use value::Val;
pub use vars::Var;
pub use waiting_room::{Rules, WaitingRoom};
pub use world::Area;
