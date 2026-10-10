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
        "Charles Orleans#cook_" => charles_orleans_cook,
        "Child with Cat#cook_" => child_with_cat_cook,
        "Madeleine Chu#cook_" => madeleine_chu_cook,
        "Servant" => servant,
        "Wickebine#cook_" => wickebine_cook,
    }
    events {
        "Wickebine#cook_::OnDisable" => wickebine_cook_ondisable,
        "Wickebine#cook_::OnEnable" => wickebine_cook_onenable,
        "Wickebine#cook_::OnInit" => wickebine_cook_oninit,
    }
}
