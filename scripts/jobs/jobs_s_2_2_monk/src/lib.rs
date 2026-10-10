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

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;

script_sdk_2::script_module! {
    npcs {
        "Apprentice Monk#mk" => apprentice_monk_mk,
        "Bashu#mk" => bashu_mk,
        "Boohae#mk" => boohae_mk,
        "Door Keeper#mk" => door_keeper_mk,
        "Guarding Monk#mk" => guarding_monk_mk,
        "Hyunmoo#mk" => hyunmoo_mk,
        "Hyunmoo#mk2" => hyunmoo_mk2,
        "Proctor#btl#3" => proctor_btl_3,
        "Proctor#mk" => proctor_mk,
        "Proctor#mk2" => proctor_mk2,
        "Sensei Moohae#mk" => sensei_moohae_mk,
        "Supervisor#race_monk" => supervisor_race_monk,
        "Tomoon#mk" => tomoon_mk,
        "Touha#mk" => touha_mk,
        "Trap#t_monk1_1" => trap_t_monk1_1,
        "exit_monk#1" => exit_monk_1,
        "exit_monk#2" => exit_monk_2,
        "exit_monk#3" => exit_monk_3,
        "mob_monk#1_1" => mob_monk_1_1,
        "mob_monk#1_2" => mob_monk_1_2,
        "mob_monk#1_3" => mob_monk_1_3,
        "mob_monk#1_4" => mob_monk_1_4,
        "mob_monk#1_5" => mob_monk_1_5,
        "mob_monk#2_1" => mob_monk_2_1,
        "mob_monk#2_2" => mob_monk_2_2,
        "mob_monk#2_3" => mob_monk_2_3,
        "mob_monk#2_4" => mob_monk_2_4,
        "mob_monk#2_5" => mob_monk_2_5,
        "mob_monk#3_1" => mob_monk_3_1,
        "mob_monk#3_2" => mob_monk_3_2,
        "mob_monk#3_3" => mob_monk_3_3,
        "mob_monk#3_4" => mob_monk_3_4,
        "mob_monk#3_5" => mob_monk_3_5,
        "resetter#monk" => resetter_monk,
        "switchreset#monkmonk" => switchreset_monkmonk,
    }
    events {
        "Guarding Monk#mk::OnTouch_" => guarding_monk_mk_ontouch,
        "Supervisor#race_monk::OnTouch_" => supervisor_race_monk_ontouch,
        "Trap#t_monk1_1::OnTouch_" => trap_t_monk1_1_ontouch,
        "exit_monk#1::OnTouch_" => exit_monk_1_ontouch,
        "exit_monk#2::OnTouch_" => exit_monk_2_ontouch,
        "exit_monk#3::OnTouch_" => exit_monk_3_ontouch,
        "mob_monk#1_1::OnDisable" => mob_monk_1_1_ondisable,
        "mob_monk#1_1::OnTouch_" => mob_monk_1_1_ontouch,
        "mob_monk#1_2::OnDisable" => mob_monk_1_2_ondisable,
        "mob_monk#1_2::OnTouch_" => mob_monk_1_2_ontouch,
        "mob_monk#1_3::OnDisable" => mob_monk_1_3_ondisable,
        "mob_monk#1_3::OnTouch_" => mob_monk_1_3_ontouch,
        "mob_monk#1_4::OnDisable" => mob_monk_1_4_ondisable,
        "mob_monk#1_4::OnTouch_" => mob_monk_1_4_ontouch,
        "mob_monk#1_5::OnDisable" => mob_monk_1_5_ondisable,
        "mob_monk#1_5::OnTouch_" => mob_monk_1_5_ontouch,
        "mob_monk#2_1::OnDisable" => mob_monk_2_1_ondisable,
        "mob_monk#2_1::OnTouch_" => mob_monk_2_1_ontouch,
        "mob_monk#2_2::OnDisable" => mob_monk_2_2_ondisable,
        "mob_monk#2_2::OnTouch_" => mob_monk_2_2_ontouch,
        "mob_monk#2_3::OnDisable" => mob_monk_2_3_ondisable,
        "mob_monk#2_3::OnTouch_" => mob_monk_2_3_ontouch,
        "mob_monk#2_4::OnDisable" => mob_monk_2_4_ondisable,
        "mob_monk#2_4::OnTouch_" => mob_monk_2_4_ontouch,
        "mob_monk#2_5::OnDisable" => mob_monk_2_5_ondisable,
        "mob_monk#2_5::OnTouch_" => mob_monk_2_5_ontouch,
        "mob_monk#3_1::OnDisable" => mob_monk_3_1_ondisable,
        "mob_monk#3_1::OnTouch_" => mob_monk_3_1_ontouch,
        "mob_monk#3_2::OnDisable" => mob_monk_3_2_ondisable,
        "mob_monk#3_2::OnTouch_" => mob_monk_3_2_ontouch,
        "mob_monk#3_3::OnDisable" => mob_monk_3_3_ondisable,
        "mob_monk#3_3::OnTouch_" => mob_monk_3_3_ontouch,
        "mob_monk#3_4::OnDisable" => mob_monk_3_4_ondisable,
        "mob_monk#3_4::OnTouch_" => mob_monk_3_4_ontouch,
        "mob_monk#3_5::OnDisable" => mob_monk_3_5_ondisable,
        "mob_monk#3_5::OnTouch_" => mob_monk_3_5_ontouch,
        "resetter#monk::OnEnable" => resetter_monk_onenable,
        "resetter#monk::OnInit" => resetter_monk_oninit,
        "resetter#monk::OnTimer500000" => resetter_monk_ontimer500000,
    }
}
