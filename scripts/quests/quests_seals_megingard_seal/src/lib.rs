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
        "A File#megin1" => a_file_megin1,
        "A File#megin2" => a_file_megin2,
        "A File#megin3" => a_file_megin3,
        "A File#megin4" => a_file_megin4,
        "A File#megin5" => a_file_megin5,
        "Crusader#God1" => crusader_god1,
        "Crusader#God_" => crusader_god,
        "Crusader#megin2" => crusader_megin2,
        "Egnigem" => egnigem,
        "Employee#megin1" => employee_megin1,
        "Lady#megin" => lady_megin,
        "Librarian#megin" => librarian_megin,
        "Man#megin" => man_megin,
        "Rebarev Doug_" => rebarev_doug,
        "Security Officer#megin" => security_officer_megin,
        "Suspicious Man#megin" => suspicious_man_megin,
    }
    events {
    }
}
