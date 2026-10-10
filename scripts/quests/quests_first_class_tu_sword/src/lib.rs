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
        "Bankley" => bankley,
        "Dequ'ee" => dequ_ee,
        "Geil" => geil,
        "Hans" => hans,
        "Muetro" => muetro,
        "Shurank" => shurank,
    }
    events {
    }
}
