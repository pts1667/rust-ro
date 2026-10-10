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
        "Blacksmith Guide#pront" => blacksmith_guide_pront,
        "Monster Encyclopedia#2pr" => monster_encyclopedia_2pr,
        "Monster Encyclopedia#3pr" => monster_encyclopedia_3pr,
        "Monster Encyclopedia#4pr" => monster_encyclopedia_4pr,
        "Monster Encyclopedia#5pr" => monster_encyclopedia_5pr,
        "Monster Encyclopedia#6pr" => monster_encyclopedia_6pr,
        "Monster Encyclopedia#7pr" => monster_encyclopedia_7pr,
        "Monster Encyclopedia#prt" => monster_encyclopedia_prt,
        "Vending Guide#pront" => vending_guide_pront,
    }
    events {
    }
}
