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
        "Chivalry Captain#knt" => chivalry_captain_knt,
        "Knight Windsor#knt" => knight_windsor_knt,
        "Knight1" => knight1,
        "Knight2" => knight2,
        "Knight3" => knight3,
        "Lady Amy#knt" => lady_amy_knt,
        "Sir Andrew#knt" => sir_andrew_knt,
        "Sir Edmond#knt" => sir_edmond_knt,
        "Sir Gray#knt" => sir_gray_knt,
        "Sir Siracuse#knt" => sir_siracuse_knt,
        "Sir Windsor#knt" => sir_windsor_knt,
        "Timer#knt" => timer_knt,
        "Warp#knt" => warp_knt,
        "Windsor Benedict#knt" => windsor_benedict_knt,
    }
    events {
        "Knight1::OnDisable" => knight1_ondisable,
        "Knight1::OnEnable" => knight1_onenable,
        "Knight1::OnInit" => knight1_oninit,
        "Knight1::OnMyMobDead" => knight1_onmymobdead,
        "Knight1::OnTimer180000" => knight1_ontimer180000,
        "Knight1::OnTimer181000" => knight1_ontimer181000,
        "Knight1::OnTimer182000" => knight1_ontimer182000,
        "Knight2::OnDisable" => knight2_ondisable,
        "Knight2::OnEnable" => knight2_onenable,
        "Knight2::OnInit" => knight2_oninit,
        "Knight2::OnMyMobDead" => knight2_onmymobdead,
        "Knight2::OnTimer180000" => knight2_ontimer180000,
        "Knight2::OnTimer181000" => knight2_ontimer181000,
        "Knight2::OnTimer182000" => knight2_ontimer182000,
        "Knight3::OnDisable" => knight3_ondisable,
        "Knight3::OnEnable" => knight3_onenable,
        "Knight3::OnInit" => knight3_oninit,
        "Knight3::OnMyMobDead" => knight3_onmymobdead,
        "Knight3::OnTimer180000" => knight3_ontimer180000,
        "Knight3::OnTimer181000" => knight3_ontimer181000,
        "Knight3::OnTimer182000" => knight3_ontimer182000,
        "Timer#knt::OnDisable" => timer_knt_ondisable,
        "Timer#knt::OnEnable" => timer_knt_onenable,
        "Timer#knt::OnInit" => timer_knt_oninit,
        "Timer#knt::OnMyMobDead" => timer_knt_onmymobdead,
        "Timer#knt::OnTimer300000" => timer_knt_ontimer300000,
        "Timer#knt::OnTimer300500" => timer_knt_ontimer300500,
        "Timer#knt::OnTimer301500" => timer_knt_ontimer301500,
        "Warp#knt::OnInit" => warp_knt_oninit,
        "Warp#knt::OnTouch_" => warp_knt_ontouch,
        "Windsor Benedict#knt::OnInit" => windsor_benedict_knt_oninit,
        "Windsor Benedict#knt::OnStart" => windsor_benedict_knt_onstart,
        "Windsor Benedict#knt::OnStartArena" => windsor_benedict_knt_onstartarena,
    }
}
