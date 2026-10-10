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
        "Dwarf Blacksmith#east" => dwarf_blacksmith_east,
        "Dwarf Blacksmith#north" => dwarf_blacksmith_north,
        "Dwarf Blacksmith#south" => dwarf_blacksmith_south,
        "Dwarf Blacksmith#west" => dwarf_blacksmith_west,
        "Roskva" => roskva,
        "Tialfi" => tialfi,
    }
    events {
    }
}
