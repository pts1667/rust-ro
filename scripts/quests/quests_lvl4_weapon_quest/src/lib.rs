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
        "Bazo#lv4" => bazo_lv4,
        "Bill Thayer#lv4" => bill_thayer_lv4,
        "Citizen#lv4-1" => citizen_lv4_1,
        "Citizen#lv4-2" => citizen_lv4_2,
        "Hein#lv4" => hein_lv4,
        "Hibilaithan#lv4" => hibilaithan_lv4,
        "Kayron#lv4" => kayron_lv4,
        "Reyghema#lv4" => reyghema_lv4,
        "Tabezthan#lv4" => tabezthan_lv4,
        "Waltboughst#lv4" => waltboughst_lv4,
    }
    events {
    }
}
