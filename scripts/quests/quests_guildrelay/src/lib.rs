#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

mod part_01;
mod part_02;
mod part_03;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;

script_sdk_2::script_module! {
    npcs {
        "GuildDummy4" => guilddummy4,
        "RelayDummy1" => relaydummy1,
        "RelayDummy2" => relaydummy2,
        "RelayDummy3" => relaydummy3,
    }
    events {
    }
}
