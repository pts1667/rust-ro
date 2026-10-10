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
mod part_03;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;

script_sdk_2::script_module! {
    npcs {
        "#2_dun_in" => s_2_dun_in,
        "#Annonblood" => annonblood,
        "#hint01" => hint01,
        "#hun_thai_1" => hun_thai_1,
        "#hun_thai_2" => hun_thai_2,
        "#hun_thai_3" => hun_thai_3,
        "#hun_thai_4" => hun_thai_4,
        "#hun_thai_5" => hun_thai_5,
        "#hun_thai_6" => hun_thai_6,
        "#reward_tiger" => reward_tiger,
        "#th_dun1_1" => th_dun1_1,
        "#th_dun1_1_1" => th_dun1_1_1,
        "Aik#thai" => aik_thai,
        "AyoFootprint1" => ayofootprint1,
        "AyoFootprint2" => ayofootprint2,
        "AyoFootprint3" => ayofootprint3,
        "AyoFootprint4" => ayofootprint4,
        "AyoFootprint5" => ayofootprint5,
        "AyoFootprint6" => ayofootprint6,
        "AyoFootprint7" => ayofootprint7,
        "AyoFootprint8" => ayofootprint8,
        "Cook#ayo" => cook_ayo,
        "Dusit#thai" => dusit_thai,
        "Einon#ayo" => einon_ayo,
        "Fisherman" => fisherman,
        "Haggard Man" => haggard_man,
        "Merchant#ayo" => merchant_ayo,
        "Mr. Jun#ayo" => mr_jun_ayo,
        "Old Man#02" => old_man_02,
        "Powerful-Looking Woman" => powerful_looking_woman,
        "Puraim#thai1" => puraim_thai1,
        "Shaman#thai" => shaman_thai,
        "Thongpool#ayo" => thongpool_ayo,
    }
    events {
        "#Annonblood::OnTouch_" => annonblood_ontouch,
        "#hint01::OnTouch_" => hint01_ontouch,
        "#th_dun1_1::OnTouch_" => th_dun1_1_ontouch,
        "#th_dun1_1_1::OnTouch_" => th_dun1_1_1_ontouch,
        "AyoFootprint1::OnTouch_" => ayofootprint1_ontouch,
        "AyoFootprint2::OnTouch_" => ayofootprint2_ontouch,
        "AyoFootprint3::OnTouch_" => ayofootprint3_ontouch,
        "AyoFootprint4::OnTouch_" => ayofootprint4_ontouch,
        "AyoFootprint5::OnTouch_" => ayofootprint5_ontouch,
        "AyoFootprint6::OnTouch_" => ayofootprint6_ontouch,
        "AyoFootprint7::OnTouch_" => ayofootprint7_ontouch,
        "AyoFootprint8::OnTouch_" => ayofootprint8_ontouch,
        "Haggard Man::OnInit" => haggard_man_oninit,
        "Mr. Jun#ayo::OnTouch_" => mr_jun_ayo_ontouch,
        "Powerful-Looking Woman::OnTouch_" => powerful_looking_woman_ontouch,
    }
}
