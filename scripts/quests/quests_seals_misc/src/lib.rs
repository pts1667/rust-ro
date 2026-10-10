pub mod god_global;
pub mod god_weapon_creation;
pub mod seal_status;
pub mod sleipnir_seal;

script_sdk_2::script_module! {
    npcs {
        "#god_hopewarp1" => god_weapon_creation::god_hopewarp1,
        "#que_godnpc1" => god_weapon_creation::que_godnpc1,
        "Friar#G5" => sleipnir_seal::friar_g5,
        "Godly Item Quests#god" => god_weapon_creation::godly_item_quests_god,
        "Golbal var" => god_global::golbal_var,
        "Grunburti#1" => god_weapon_creation::grunburti_1,
        "Grunburti#god" => god_weapon_creation::grunburti_god,
        "Manager#G" => sleipnir_seal::manager_g,
        "Noyee#G" => sleipnir_seal::noyee_g,
        "Researcher#G1" => sleipnir_seal::researcher_g1,
        "Researcher#G2" => sleipnir_seal::researcher_g2,
        "Researcher#G3" => sleipnir_seal::researcher_g3,
        "Researcher#G4" => sleipnir_seal::researcher_g4,
        "Sign Post#god" => seal_status::sign_post_god,
        "Slab#G" => sleipnir_seal::slab_g,
        "Switch#God0" => sleipnir_seal::switch_god0,
        "Switch#God1" => sleipnir_seal::switch_god1,
        "Switch#God2" => sleipnir_seal::switch_god2,
        "Switch#God3" => sleipnir_seal::switch_god3,
        "Switch#God4" => sleipnir_seal::switch_god4,
        "god_failwarp#1" => god_weapon_creation::god_failwarp_1,
        "god_sl_w0" => sleipnir_seal::god_sl_w0,
        "god_sl_w1" => sleipnir_seal::god_sl_w1,
        "god_sl_w2" => sleipnir_seal::god_sl_w2,
        "god_sl_w3" => sleipnir_seal::god_sl_w3,
        "god_sl_w4" => sleipnir_seal::god_sl_w4,
        "god_wep_warpmaster" => god_weapon_creation::god_wep_warpmaster,
    }
    events {
        "#god_hopewarp1::OnInit" => god_weapon_creation::god_hopewarp1_oninit,
        "#god_hopewarp1::OnReset" => god_weapon_creation::god_hopewarp1_onreset,
        "#god_hopewarp1::OnStartArena" => god_weapon_creation::god_hopewarp1_onstartarena,
        "Golbal var::OnInit" => god_global::golbal_var_oninit,
        "Grunburti#god::OnEnable" => god_weapon_creation::grunburti_god_onenable,
        "Grunburti#god::OnTimer10000" => god_weapon_creation::grunburti_god_ontimer10000,
        "Grunburti#god::OnTimer610000" => god_weapon_creation::grunburti_god_ontimer610000,
        "Grunburti#god::OnTimer612000" => god_weapon_creation::grunburti_god_ontimer612000,
        "Grunburti#god::OnTimer615000" => god_weapon_creation::grunburti_god_ontimer615000,
        "god_failwarp#1::OnInit" => god_weapon_creation::god_failwarp_1_oninit,
        "god_failwarp#1::OnTouch_" => god_weapon_creation::god_failwarp_1_ontouch,
        "god_sl_w0::OnTouch_" => sleipnir_seal::god_sl_w0_ontouch,
        "god_sl_w1::OnTouch_" => sleipnir_seal::god_sl_w1_ontouch,
        "god_sl_w2::OnTouch_" => sleipnir_seal::god_sl_w2_ontouch,
        "god_sl_w3::OnTouch_" => sleipnir_seal::god_sl_w3_ontouch,
        "god_sl_w4::OnTouch_" => sleipnir_seal::god_sl_w4_ontouch,
        "god_wep_warpmaster::OnDisable" => god_weapon_creation::god_wep_warpmaster_ondisable,
        "god_wep_warpmaster::OnEnable" => god_weapon_creation::god_wep_warpmaster_onenable,
    }
}
