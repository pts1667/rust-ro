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
        "A pile of turtle crystal" => a_pile_of_turtle_crystal,
        "Cherokee" => cherokee,
        "Elin" => elin,
        "Grampa" => grampa,
        "Grandpa Turtle#tur" => grandpa_turtle_tur,
        "Hat store girl#new30" => hat_store_girl_new30,
        "Iromo#ep3_2" => iromo_ep3_2,
        "Iromo's Mother#ep3_2" => iromo_s_mother_ep3_2,
        "Kinsey#tur" => kinsey_tur,
        "Knight Leader#tur" => knight_leader_tur,
        "Knight#tur" => knight_tur,
        "Knight#tur2" => knight_tur2,
        "Knight#tur3" => knight_tur3,
        "Knight#tur4" => knight_tur4,
        "Letter#tur" => letter_tur,
        "Little Boy#ep3_2" => little_boy_ep3_2,
        "Mudasamu#tur" => mudasamu_tur,
        "Sailor#tur2" => sailor_tur2,
        "Sailor_alberta" => sailor_alberta,
        "Skull Stone#tur" => skull_stone_tur,
        "Stylish Merchant#new30" => stylish_merchant_new30,
        "Turtle Pillar#tur" => turtle_pillar_tur,
        "Turtle Statue#tur" => turtle_statue_tur,
        "Turtle Stone#tur3" => turtle_stone_tur3,
        "Turtle Tree Roots#tur" => turtle_tree_roots_tur,
        "Turtle stone#tur" => turtle_stone_tur,
        "Turtle stone#tur2" => turtle_stone_tur2,
        "Turtle_Scholar_alberta" => turtle_scholar_alberta,
        "Voyage log#tur" => voyage_log_tur,
    }
    events {
        "Knight Leader#tur::OnTouch_" => knight_leader_tur_ontouch,
    }
}
