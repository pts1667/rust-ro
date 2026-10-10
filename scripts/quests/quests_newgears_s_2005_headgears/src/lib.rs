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
        "Freight Manager#toast" => freight_manager_toast,
        "Kasis#LhzHat" => kasis_lhzhat,
        "Kid#LhzHat" => kid_lhzhat,
        "Margaret Mary#LhzHat" => margaret_mary_lhzhat,
        "Metelle#LhzHat" => metelle_lhzhat,
        "Phendark#LhzHat" => phendark_lhzhat,
        "Relaxed-Looking Lady" => relaxed_looking_lady,
        "Rybio#LhzHat" => rybio_lhzhat,
        "Strange Guy#LhzHat" => strange_guy_lhzhat,
        "Zealotus#LhzHat" => zealotus_lhzhat,
    }
    events {
    }
}
