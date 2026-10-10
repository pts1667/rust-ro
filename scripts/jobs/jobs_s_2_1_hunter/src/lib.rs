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

pub use part_01::*;
pub use part_02::*;

script_sdk_2::script_module! {
    npcs {
        "1-1" => s_1_1,
        "57-4" => s_57_4,
        "Guide#hnt" => guide_hnt,
        "Guild Receptionist#hnt" => guild_receptionist_hnt,
        "Hunter Guildsman#hnt" => hunter_guildsman_hnt,
        "Hunter Info#hnt" => hunter_info_hnt,
        "Hunter#htnGM" => hunter_htngm,
        "Hunter#htnGM2" => hunter_htngm2,
        "Manager#hnt" => manager_hnt,
        "Switch#hnt" => switch_hnt,
        "Waiting Room#hnt" => waiting_room_hnt,
        "exit#hnttest" => exit_hnttest,
    }
    events {
        "1-1::OnTouch_" => s_1_1_ontouch,
        "57-4::OnTouch_" => s_57_4_ontouch,
        "Guide#hnt::OnTouch_" => guide_hnt_ontouch,
        "Manager#hnt::OnDisable" => manager_hnt_ondisable,
        "Manager#hnt::OnEnable" => manager_hnt_onenable,
        "Manager#hnt::OnInit" => manager_hnt_oninit,
        "Manager#hnt::OnMyMobDead" => manager_hnt_onmymobdead,
        "Manager#hnt::OnMyMobDead2" => manager_hnt_onmymobdead2,
        "Manager#hnt::OnReset" => manager_hnt_onreset,
        "Manager#hnt::OnTimer1000" => manager_hnt_ontimer1000,
        "Manager#hnt::OnTimer11000" => manager_hnt_ontimer11000,
        "Manager#hnt::OnTimer13000" => manager_hnt_ontimer13000,
        "Manager#hnt::OnTimer134000" => manager_hnt_ontimer134000,
        "Manager#hnt::OnTimer14000" => manager_hnt_ontimer14000,
        "Manager#hnt::OnTimer164000" => manager_hnt_ontimer164000,
        "Manager#hnt::OnTimer187000" => manager_hnt_ontimer187000,
        "Manager#hnt::OnTimer188000" => manager_hnt_ontimer188000,
        "Manager#hnt::OnTimer189000" => manager_hnt_ontimer189000,
        "Manager#hnt::OnTimer191000" => manager_hnt_ontimer191000,
        "Manager#hnt::OnTimer192000" => manager_hnt_ontimer192000,
        "Manager#hnt::OnTimer193000" => manager_hnt_ontimer193000,
        "Manager#hnt::OnTimer194000" => manager_hnt_ontimer194000,
        "Manager#hnt::OnTimer195000" => manager_hnt_ontimer195000,
        "Manager#hnt::OnTimer197000" => manager_hnt_ontimer197000,
        "Manager#hnt::OnTimer3000" => manager_hnt_ontimer3000,
        "Manager#hnt::OnTimer5000" => manager_hnt_ontimer5000,
        "Manager#hnt::OnTimer7000" => manager_hnt_ontimer7000,
        "Manager#hnt::OnTimer74000" => manager_hnt_ontimer74000,
        "Manager#hnt::OnTimer9000" => manager_hnt_ontimer9000,
        "Switch#hnt::OnDisable" => switch_hnt_ondisable,
        "Switch#hnt::OnEnable" => switch_hnt_onenable,
        "Switch#hnt::OnTouch_" => switch_hnt_ontouch,
        "Waiting Room#hnt::OnInit" => waiting_room_hnt_oninit,
        "Waiting Room#hnt::OnStart" => waiting_room_hnt_onstart,
        "Waiting Room#hnt::OnStartArena" => waiting_room_hnt_onstartarena,
        "exit#hnttest::OnInit" => exit_hnttest_oninit,
        "exit#hnttest::OnTouch_" => exit_hnttest_ontouch,
    }
}
