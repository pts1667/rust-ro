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
        "#paypuzz1" => paypuzz1,
        "#prt_key-1" => prt_key_1,
        "#prt_key-1-1" => prt_key_1_1,
        "Buddha Statue#paypuzz6" => buddha_statue_paypuzz6,
        "Clanux Heffron#hellion" => clanux_heffron_hellion,
        "Dried Fish#paypuzz3" => dried_fish_paypuzz3,
        "Enoz#hellion" => enoz_hellion,
        "Grout'he Tuccok#hellion" => grout_he_tuccok_hellion,
        "Hidden Cave#hellion" => hidden_cave_hellion,
        "Old Scholar Tyus#hellion" => old_scholar_tyus_hellion,
        "Pile of Stone#paypuzz2" => pile_of_stone_paypuzz2,
        "Sage Welshyun#hellion" => sage_welshyun_hellion,
        "Unknown Machine#prt_key" => unknown_machine_prt_key,
        "Vat#paypuzz4" => vat_paypuzz4,
        "Wooden Floor#paypuzz5" => wooden_floor_paypuzz5,
    }
    events {
        "#paypuzz1::OnTouch_" => paypuzz1_ontouch,
        "#prt_key-1-1::OnTouch_" => prt_key_1_1_ontouch,
        "Buddha Statue#paypuzz6::OnTouch_" => buddha_statue_paypuzz6_ontouch,
        "Hidden Cave#hellion::OnTouch_" => hidden_cave_hellion_ontouch,
        "Old Scholar Tyus#hellion::OnTouch_" => old_scholar_tyus_hellion_ontouch,
        "Sage Welshyun#hellion::OnTouch_" => sage_welshyun_hellion_ontouch,
    }
}
