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
        "Ascetic" => ascetic,
        "Fortune Teller" => fortune_teller,
        "Poring Fortune Teller" => poring_fortune_teller,
    }
    events {
        "Ascetic::OnTouch_" => ascetic_ontouch,
    }
}
