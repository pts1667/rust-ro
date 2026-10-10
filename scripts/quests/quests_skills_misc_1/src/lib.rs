pub mod acolyte_skills;
pub mod archer_skills;
pub mod assassin_skills;
pub mod bard_skills;
pub mod blacksmith_skills;
pub mod crusader_skills;
pub mod dancer_skills;
pub mod hunter_skills;
pub mod knight_skills;
pub mod mage_skills;
pub mod merchant_skills;
pub mod monk_skills;

script_sdk_2::script_module! {
    npcs {
        "#tour" => knight_skills::tour,
        "Aelle#qsk_dan02" => dancer_skills::aelle_qsk_dan02,
        "Akkie#qsk_bs" => blacksmith_skills::akkie_qsk_bs,
        "Apprentice Monk#qsk_mo" => monk_skills::apprentice_monk_qsk_mo,
        "Arpesto" => hunter_skills::arpesto,
        "Assassin#realgirl" => assassin_skills::assassin_realgirl,
        "Assassin#realman" => assassin_skills::assassin_realman,
        "Bartender#bard_qskill" => bard_skills::bartender_bard_qskill,
        "Canell#qsk_dan01" => dancer_skills::canell_qsk_dan01,
        "Charlron" => merchant_skills::charlron,
        "Cleric" => acolyte_skills::cleric,
        "Customer#bard_skill01" => bard_skills::customer_bard_skill01,
        "Customer#bard_skill02" => bard_skills::customer_bard_skill02,
        "Ford#11" => crusader_skills::ford_11,
        "Gershaun_alberta" => merchant_skills::gershaun_alberta,
        "Goodman#qsk_bs" => blacksmith_skills::goodman_qsk_bs,
        "Grand Master" => knight_skills::grand_master,
        "Great Wizard" => mage_skills::great_wizard,
        "Jason" => archer_skills::jason,
        "Knight#drake" => knight_skills::knight_drake,
        "Knight#gattack" => knight_skills::knight_gattack,
        "Knight#kabuto" => knight_skills::knight_kabuto,
        "Knight#sasword" => knight_skills::knight_sasword,
        "Knight#zabii" => knight_skills::knight_zabii,
        "Monk#qsk_mo" => monk_skills::monk_qsk_mo,
        "Necko" => merchant_skills::necko,
        "Old Coffin#qsk_as" => assassin_skills::old_coffin_qsk_as,
        "Old Coffin#qsk_as2" => assassin_skills::old_coffin_qsk_as2,
        "Pastor#1011" => crusader_skills::pastor_1011,
        "Roberto" => archer_skills::roberto,
        "Soldier#277" => crusader_skills::soldier_277,
        "Spiteful-Looking Bard#bs" => bard_skills::spiteful_looking_bard_bs,
        "Stone Statue#qsk_as" => assassin_skills::stone_statue_qsk_as,
        "Stone Statue#qsk_as2" => assassin_skills::stone_statue_qsk_as2,
        "Yhelle#bard_chick1" => bard_skills::yhelle_bard_chick1,
        "Yhelle#bard_chick2" => bard_skills::yhelle_bard_chick2,
        "Yhelle#bard_chick3" => bard_skills::yhelle_bard_chick3,
        "Yhelle#bard_chick4" => bard_skills::yhelle_bard_chick4,
        "Yhelle#bard_chick5" => bard_skills::yhelle_bard_chick5,
        "Young Man#bard_q1" => bard_skills::young_man_bard_q1,
        "¡¡#crypt" => assassin_skills::crypt,
    }
    events {
        "#tour::OnTouch" => knight_skills::tour_ontouch,
        "Jason::OnTouch_" => archer_skills::jason_ontouch,
        "Necko::OnTouch_" => merchant_skills::necko_ontouch,
        "Old Coffin#qsk_as2::OnTouch_" => assassin_skills::old_coffin_qsk_as2_ontouch,
        "Old Coffin#qsk_as::OnTouch_" => assassin_skills::old_coffin_qsk_as_ontouch,
        "Spiteful-Looking Bard#bs::OnTouch" => bard_skills::spiteful_looking_bard_bs_ontouch,
        "Stone Statue#qsk_as2::OnTouch_" => assassin_skills::stone_statue_qsk_as2_ontouch,
        "Stone Statue#qsk_as::OnTouch_" => assassin_skills::stone_statue_qsk_as_ontouch,
        "Yhelle#bard_chick1::OnDisable" => bard_skills::yhelle_bard_chick1_ondisable,
        "Yhelle#bard_chick1::OnEnable" => bard_skills::yhelle_bard_chick1_onenable,
        "Yhelle#bard_chick1::OnInit" => bard_skills::yhelle_bard_chick1_oninit,
        "Yhelle#bard_chick1::OnTouch" => bard_skills::yhelle_bard_chick1_ontouch,
        "Yhelle#bard_chick2::OnDisable" => bard_skills::yhelle_bard_chick2_ondisable,
        "Yhelle#bard_chick2::OnEnable" => bard_skills::yhelle_bard_chick2_onenable,
        "Yhelle#bard_chick2::OnTouch" => bard_skills::yhelle_bard_chick2_ontouch,
        "Yhelle#bard_chick3::OnDisable" => bard_skills::yhelle_bard_chick3_ondisable,
        "Yhelle#bard_chick3::OnEnable" => bard_skills::yhelle_bard_chick3_onenable,
        "Yhelle#bard_chick3::OnTouch" => bard_skills::yhelle_bard_chick3_ontouch,
        "Yhelle#bard_chick4::OnDisable" => bard_skills::yhelle_bard_chick4_ondisable,
        "Yhelle#bard_chick4::OnEnable" => bard_skills::yhelle_bard_chick4_onenable,
        "Yhelle#bard_chick4::OnTouch" => bard_skills::yhelle_bard_chick4_ontouch,
        "Yhelle#bard_chick5::OnDisable" => bard_skills::yhelle_bard_chick5_ondisable,
        "Yhelle#bard_chick5::OnEnable" => bard_skills::yhelle_bard_chick5_onenable,
        "Yhelle#bard_chick5::OnTouch" => bard_skills::yhelle_bard_chick5_ontouch,
        "Young Man#bard_q1::OnTouch" => bard_skills::young_man_bard_q1_ontouch,
        "¡¡#crypt::OnTouch_" => assassin_skills::crypt_ontouch,
    }
}
