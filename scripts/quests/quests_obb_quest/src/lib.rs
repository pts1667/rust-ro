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

pub use part_01::*;
pub use part_02::*;

script_sdk_2::script_module! {
    npcs {
        "Jacob#thai" => jacob_thai,
        "Old Man#thai" => old_man_thai,
        "Tommy#thai" => tommy_thai,
        "Tourist#thai" => tourist_thai,
    }
    events {
    }
}
