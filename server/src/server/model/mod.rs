pub mod action;
pub mod damage_notification;
pub mod duel;
pub mod events;
pub mod game_systems;
pub(crate) mod ground_unit;
pub mod hotkey;
pub mod item;
pub mod map;
pub mod map_flags;
pub mod map_instance;
pub mod map_item;
pub mod mob_spawn;
pub use movement;
pub use movement::path;
pub use movement::position;
pub mod request;
pub mod response;
pub mod script;
pub(crate) mod character_lifecycle;
pub(crate) mod script_timer;
pub mod session;
pub mod status;
pub mod tasks_queue;
pub mod warp;

pub trait Npc {
    fn get_map_name(&self) -> String;
}
