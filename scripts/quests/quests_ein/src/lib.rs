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
mod part_05;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;
pub use part_04::*;
pub use part_05::*;

script_sdk_2::script_module! {
    npcs {
        "#kenka" => kenka,
        "2nd Control Panel#ins" => s_2nd_control_panel_ins,
        "3rd Pressure Governor#1" => s_3rd_pressure_governor_1,
        "Buender Hikeman#ein" => buender_hikeman_ein,
        "Calla#ein" => calla_ein,
        "Catzllanpu#ein" => catzllanpu_ein,
        "Cavitar" => cavitar,
        "Conveyor#ins" => conveyor_ins,
        "Conveyor#ins2" => conveyor_ins2,
        "Decii#ein" => decii_ein,
        "Drunken Man#ein" => drunken_man_ein,
        "Einbroch Smog Alert" => einbroch_smog_alert,
        "Ellhenje#ein" => ellhenje_ein,
        "Factory Quest Test" => factory_quest_test,
        "Kaijeta#ein" => kaijeta_ein,
        "Keneshiotz#ein" => keneshiotz_ein,
        "Kesunboss#ein" => kesunboss_ein,
        "Klitzer#ein" => klitzer_ein,
        "Laboratory Soldier#ein-1" => laboratory_soldier_ein_1,
        "Laboratory Soldier#ein-2" => laboratory_soldier_ein_2,
        "Liotzburg#ein" => liotzburg_ein,
        "Maid#ein" => maid_ein,
        "Main Control Panel#ins" => main_control_panel_ins,
        "Megass#EIN" => megass_ein,
        "Pipe#ins" => pipe_ins,
        "Satra#ein" => satra_ein,
        "Scientist#ein" => scientist_ein,
        "Security#ein" => security_ein,
        "Sick Old Man#ein" => sick_old_man_ein,
        "Supineque#ein" => supineque_ein,
        "Unknown Stuff#ein" => unknown_stuff_ein,
        "Uwe Kleine#ein" => uwe_kleine_ein,
        "Young Man#Shinokas_Quest" => young_man_shinokas_quest,
        "Zelmeto" => zelmeto,
    }
    events {
        "#kenka::OnTouch_" => kenka_ontouch,
        "Einbroch Smog Alert::OnEnable" => einbroch_smog_alert_onenable,
        "Einbroch Smog Alert::OnMyMobDead" => einbroch_smog_alert_onmymobdead,
        "Einbroch Smog Alert::OnTimer600000" => einbroch_smog_alert_ontimer600000,
        "Liotzburg#ein::OnTouch_" => liotzburg_ein_ontouch,
        "Megass#EIN::OnTouch_" => megass_ein_ontouch,
        "Security#ein::OnTouch_" => security_ein_ontouch,
    }
}
