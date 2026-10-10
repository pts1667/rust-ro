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
        "#brisindwarf1" => brisindwarf1,
        "#brisindwarf2" => brisindwarf2,
        "#brisindwarf3" => brisindwarf3,
        "#brisindwarf4" => brisindwarf4,
        "#brisinsold2" => brisinsold2,
        "#brisinsummon" => brisinsummon,
        "#doppelganger1" => doppelganger1,
        "#doppelganger2" => doppelganger2,
        "#hermite" => hermite,
        "#knight1" => knight1,
        "#knight2" => knight2,
        "#knight3" => knight3,
        "#lowen" => lowen,
        "#lowenone" => lowenone,
        "#lowentrace" => lowentrace,
        "#lowentrace1" => lowentrace1,
        "#monologue" => monologue,
        "Alfrik#1" => alfrik_1,
        "Bard#brising" => bard_brising,
        "Berling#1" => berling_1,
        "Dvalin#1" => dvalin_1,
        "Gravestone" => gravestone,
        "Grer#1" => grer_1,
        "Librarian#2_" => librarian_2,
        "Lowen Ellenen" => lowen_ellenen,
        "Lowen Ellenen#2" => lowen_ellenen_2,
        "Praying Man" => praying_man,
        "Soldier#1_brising" => soldier_1_brising,
        "Soldier#2_brising" => soldier_2_brising,
        "Studying Scholar#1" => studying_scholar_1,
        "Valkyrie#1" => valkyrie_1,
        "Woman#Rosa Ellenen" => woman_rosa_ellenen,
    }
    events {
        "#brisindwarf1::OnTouch_" => brisindwarf1_ontouch,
        "#brisindwarf2::OnTouch_" => brisindwarf2_ontouch,
        "#brisindwarf3::OnTouch_" => brisindwarf3_ontouch,
        "#brisindwarf4::OnTimer1000" => brisindwarf4_ontimer1000,
        "#brisindwarf4::OnTimer240000" => brisindwarf4_ontimer240000,
        "#brisindwarf4::OnTimer298000" => brisindwarf4_ontimer298000,
        "#brisindwarf4::OnTimer300000" => brisindwarf4_ontimer300000,
        "#brisindwarf4::OnTimer301000" => brisindwarf4_ontimer301000,
        "#brisindwarf4::OnTouch_" => brisindwarf4_ontouch,
        "#brisinsold2::OnInit" => brisinsold2_oninit,
        "#brisinsold2::OnSold2On" => brisinsold2_onsold2on,
        "#brisinsold2::OnTimer420000" => brisinsold2_ontimer420000,
        "#brisinsold2::OnTimer480000" => brisinsold2_ontimer480000,
        "#brisinsold2::OnTimer540000" => brisinsold2_ontimer540000,
        "#brisinsold2::OnTimer542000" => brisinsold2_ontimer542000,
        "#brisinsold2::OnTimer550000" => brisinsold2_ontimer550000,
        "#brisinsold2::OnTimer550500" => brisinsold2_ontimer550500,
        "#brisinsummon::OnDoppel1Off" => brisinsummon_ondoppel1off,
        "#brisinsummon::OnDoppel1On" => brisinsummon_ondoppel1on,
        "#brisinsummon::OnDoppel2Off" => brisinsummon_ondoppel2off,
        "#brisinsummon::OnDoppel2On" => brisinsummon_ondoppel2on,
        "#brisinsummon::OnHermiteOff" => brisinsummon_onhermiteoff,
        "#brisinsummon::OnInit" => brisinsummon_oninit,
        "#brisinsummon::OnKnight1Off" => brisinsummon_onknight1off,
        "#brisinsummon::OnKnight1On" => brisinsummon_onknight1on,
        "#brisinsummon::OnKnight2Off" => brisinsummon_onknight2off,
        "#brisinsummon::OnKnight2On" => brisinsummon_onknight2on,
        "#brisinsummon::OnKnight3Off" => brisinsummon_onknight3off,
        "#brisinsummon::OnKnight3On" => brisinsummon_onknight3on,
        "#brisinsummon::OnLowen2Off" => brisinsummon_onlowen2off,
        "#brisinsummon::OnLowenOff" => brisinsummon_onlowenoff,
        "#brisinsummon::OnLowenOn" => brisinsummon_onlowenon,
        "#brisinsummon::OnMobDeath" => brisinsummon_onmobdeath,
        "#brisinsummon::OnReset" => brisinsummon_onreset,
        "#brisinsummon::OnSummon" => brisinsummon_onsummon,
        "#lowenone::OnTouch_" => lowenone_ontouch,
        "#lowentrace1::OnTouch_" => lowentrace1_ontouch,
        "#lowentrace::OnTouch_" => lowentrace_ontouch,
        "#monologue::OnTouch_" => monologue_ontouch,
        "Alfrik#1::OnInit" => alfrik_1_oninit,
        "Berling#1::OnInit" => berling_1_oninit,
        "Dvalin#1::OnInit" => dvalin_1_oninit,
        "Grer#1::OnInit" => grer_1_oninit,
        "Soldier#1_brising::OnTouch_" => soldier_1_brising_ontouch,
    }
}
