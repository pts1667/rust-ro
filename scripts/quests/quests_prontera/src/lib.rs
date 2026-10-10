#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

mod common;
mod part_01;
mod part_02;
mod part_03;
mod part_04;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;
pub use part_04::*;

script_sdk_2::script_module! {
    npcs {
        "#gototomb" => gototomb,
        "#prince1" => prince1,
        "#prince2" => prince2,
        "#prince3" => prince3,
        "#prt_poem01" => prt_poem01,
        "#prt_poem02" => prt_poem02,
        "#prt_poem03" => prt_poem03,
        "#prt_poem04" => prt_poem04,
        "#prt_poem05" => prt_poem05,
        "#prtcurse" => prtcurse,
        "Assassin Guildsman#poiso" => assassin_guildsman_poiso,
        "Busy Boy#prt" => busy_boy_prt,
        "Culvert Guardian" => culvert_guardian,
        "Dazed Boy#prt" => dazed_boy_prt,
        "Dog#prt" => dog_prt,
        "Exhausted-Looking Woman" => exhausted_looking_woman,
        "Father Bamph" => father_bamph,
        "Father Bamph#tomb" => father_bamph_tomb,
        "Father Biscuss" => father_biscuss,
        "Father Biscuss#tomb" => father_biscuss_tomb,
        "Historian#prt01" => historian_prt01,
        "Historian#prt02" => historian_prt02,
        "Historian#prt03" => historian_prt03,
        "Librarian#curse" => librarian_curse,
        "Marjana#poison" => marjana_poison,
        "Recruiter" => recruiter,
        "Teacher" => teacher,
    }
    events {
        "#prt_poem01::OnTouch" => prt_poem01_ontouch,
        "#prt_poem02::OnTouch" => prt_poem02_ontouch,
        "#prt_poem03::OnTouch" => prt_poem03_ontouch,
        "#prt_poem04::OnTouch" => prt_poem04_ontouch,
        "#prt_poem05::OnTouch" => prt_poem05_ontouch,
        "#prtcurse::OnTouch" => prtcurse_ontouch,
        "Busy Boy#prt::OnTouch" => busy_boy_prt_ontouch,
        "Marjana#poison::OnEnable" => marjana_poison_onenable,
        "Marjana#poison::OnInit" => marjana_poison_oninit,
    }
}
