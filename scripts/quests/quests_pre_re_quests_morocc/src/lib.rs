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
        "#arm" => arm,
        "#arm1" => arm1,
        "#eisen" => eisen,
        "#erich" => erich,
        "#ern" => ern,
        "#helmut" => helmut,
        "#peter" => peter,
        "#poe" => poe,
        "#twonoble" => twonoble,
        "#urgen" => urgen,
        "Aged Noble#rihart" => aged_noble_rihart,
        "Calbern" => calbern,
        "Girl#prince" => girl_prince,
        "Guard#princein" => guard_princein,
        "Inspector#prince" => inspector_prince,
        "Messenger#prince1" => messenger_prince1,
        "Prince" => prince,
        "Prince#another_ern" => prince_another_ern,
        "Prince#another_ern1" => prince_another_ern1,
        "Prince#eisen" => prince_eisen,
        "Prince#eisen1" => prince_eisen1,
        "Prince#eisen2" => prince_eisen2,
        "Prince#eisen3" => prince_eisen3,
        "Prince#eisen4" => prince_eisen4,
        "Prince#eisen5" => prince_eisen5,
        "Prince#eisen6" => prince_eisen6,
        "Prince#ern" => prince_ern,
        "Prince#helmut" => prince_helmut,
        "Prince#peter" => prince_peter,
        "Prince#poe" => prince_poe,
        "Prince#urgen" => prince_urgen,
        "Servant#hans" => servant_hans,
        "Young Noble#valter" => young_noble_valter,
    }
    events {
        "#arm1::OnTouch" => arm1_ontouch,
        "#arm::OnTouch" => arm_ontouch,
        "#eisen::OnTouch_" => eisen_ontouch,
        "#erich::OnTouch_" => erich_ontouch,
        "#ern::OnTouch_" => ern_ontouch,
        "#helmut::OnTouch_" => helmut_ontouch,
        "#peter::OnTouch_" => peter_ontouch,
        "#poe::OnTouch_" => poe_ontouch,
        "#twonoble::OnTouch" => twonoble_ontouch,
        "#urgen::OnTouch_" => urgen_ontouch,
        "Aged Noble#rihart::OnDisable" => aged_noble_rihart_ondisable,
        "Aged Noble#rihart::OnEnable" => aged_noble_rihart_onenable,
        "Aged Noble#rihart::OnInit" => aged_noble_rihart_oninit,
        "Prince#another_ern1::OnDisable" => prince_another_ern1_ondisable,
        "Prince#another_ern1::OnEnable" => prince_another_ern1_onenable,
        "Prince#another_ern1::OnInit" => prince_another_ern1_oninit,
        "Prince#another_ern::OnDisable" => prince_another_ern_ondisable,
        "Prince#another_ern::OnEnable" => prince_another_ern_onenable,
        "Prince#another_ern::OnInit" => prince_another_ern_oninit,
        "Prince#eisen1::OnDisable" => prince_eisen1_ondisable,
        "Prince#eisen1::OnEnable" => prince_eisen1_onenable,
        "Prince#eisen1::OnInit" => prince_eisen1_oninit,
        "Prince#eisen2::OnDisable" => prince_eisen2_ondisable,
        "Prince#eisen2::OnEnable" => prince_eisen2_onenable,
        "Prince#eisen2::OnInit" => prince_eisen2_oninit,
        "Prince#eisen3::OnDisable" => prince_eisen3_ondisable,
        "Prince#eisen3::OnEnable" => prince_eisen3_onenable,
        "Prince#eisen3::OnInit" => prince_eisen3_oninit,
        "Prince#eisen4::OnDisable" => prince_eisen4_ondisable,
        "Prince#eisen4::OnEnable" => prince_eisen4_onenable,
        "Prince#eisen4::OnInit" => prince_eisen4_oninit,
        "Prince#eisen5::OnDisable" => prince_eisen5_ondisable,
        "Prince#eisen5::OnEnable" => prince_eisen5_onenable,
        "Prince#eisen5::OnInit" => prince_eisen5_oninit,
        "Prince#eisen6::OnDisable" => prince_eisen6_ondisable,
        "Prince#eisen6::OnEnable" => prince_eisen6_onenable,
        "Prince#eisen6::OnInit" => prince_eisen6_oninit,
        "Young Noble#valter::OnDisable" => young_noble_valter_ondisable,
        "Young Noble#valter::OnEnable" => young_noble_valter_onenable,
        "Young Noble#valter::OnInit" => young_noble_valter_oninit,
    }
}
