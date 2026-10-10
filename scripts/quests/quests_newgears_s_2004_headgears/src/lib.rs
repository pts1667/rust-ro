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
        "#Alarm Mask Man1" => alarm_mask_man1,
        "#Alarm Mask Man2" => alarm_mask_man2,
        "#Alarm Mask Man3" => alarm_mask_man3,
        "#Alarm Mask Man4" => alarm_mask_man4,
        "Argen#1" => argen_1,
        "Educated Traveller" => educated_traveller,
        "Fuzzy Fuzz#1" => fuzzy_fuzz_1,
        "Hat Merchant#zero" => hat_merchant_zero,
        "Ipore#1" => ipore_1,
        "Meruntei#1" => meruntei_1,
        "Muscle Man#Alarm Mask" => muscle_man_alarm_mask,
        "Nanhyang#1" => nanhyang_1,
        "Nehris#1" => nehris_1,
        "Neko Neko#1" => neko_neko_1,
        "Nephia#1" => nephia_1,
        "Nine Tails#Kitsune Man" => nine_tails_kitsune_man,
        "Nine Tails#Kitsune Mask" => nine_tails_kitsune_mask,
        "Old Blacksmith#hgear" => old_blacksmith_hgear,
        "Orc Hero#1" => orc_hero_1,
        "Orc Warrior#1" => orc_warrior_1,
        "Pretty Lindsay#1" => pretty_lindsay_1,
        "Seth#1" => seth_1,
        "SpawnManager#Kitsune" => spawnmanager_kitsune,
        "Zhenbolt#1" => zhenbolt_1,
    }
    events {
        "#Alarm Mask Man1::OnInit" => alarm_mask_man1_oninit,
        "#Alarm Mask Man2::OnInit" => alarm_mask_man2_oninit,
        "#Alarm Mask Man3::OnInit" => alarm_mask_man3_oninit,
        "#Alarm Mask Man4::OnInit" => alarm_mask_man4_oninit,
        "Muscle Man#Alarm Mask::OnTimer4000" => muscle_man_alarm_mask_ontimer4000,
        "Nine Tails#Kitsune Man::OnInit" => nine_tails_kitsune_man_oninit,
        "Nine Tails#Kitsune Mask::OnInit" => nine_tails_kitsune_mask_oninit,
        "Nine Tails#Kitsune Mask::OnTouch_" => nine_tails_kitsune_mask_ontouch,
        "Orc Hero#1::OnTouch_" => orc_hero_1_ontouch,
        "Orc Warrior#1::OnTouch_" => orc_warrior_1_ontouch,
        "SpawnManager#Kitsune::OnInit" => spawnmanager_kitsune_oninit,
        "SpawnManager#Kitsune::OnMyMobDead" => spawnmanager_kitsune_onmymobdead,
        "SpawnManager#Kitsune::OnMyMobDead2" => spawnmanager_kitsune_onmymobdead2,
    }
}
