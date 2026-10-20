//! What a plugin can show to a player besides the stock dialogue: windows built from a tree of [`Node`]s, and
//! [`Overhead`] notifications drawn above a unit. Shared by the plugin SDK, the server and the client so the three
//! agree on one vocabulary; the wire format is in [`wire`]. See `docs/plugin-ui.md`.

pub mod catalog;
mod event;
mod overhead;
mod validate;
mod window;
pub mod wire;

pub use event::{EventKind, InputValue, UiEvent};
pub use overhead::{Audience, Easing, Motion, MotionKind, Overhead, OverheadText, Transform};
pub use validate::{validate_overhead, validate_window, Limits, LIMITS};
pub use window::{Align, Column, ImageRef, Node, Row, Table, TableColumn, TableRow, Window};

/// Where the assets a plugin ships in its `ui/` folder are found by the client: `plugin/<name>/<file>`.
pub fn plugin_asset_path(plugin: &str, file: &str) -> String {
    format!("plugin/{plugin}/{file}")
}
