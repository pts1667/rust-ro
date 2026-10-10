pub mod quests_aldebaran;
pub mod quests_geffen;
pub mod quests_izlude;
pub mod quests_lutie;
pub mod quests_niflheim;
pub mod quests_payon;
pub mod quests_umbala;
pub mod quests_yuno;

script_sdk_2::script_module! {
    npcs {
        "#!@#$%" => quests_umbala::script,
        "#Graveyard1" => quests_niflheim::graveyard1,
        "#Graveyard2" => quests_niflheim::graveyard2,
        "#Piano" => quests_niflheim::piano,
        "#Skulldoor" => quests_umbala::skulldoor,
        "#unpc" => quests_umbala::unpc,
        "A Citizen of Juno#juno" => quests_yuno::a_citizen_of_juno_juno,
        "Alreg#nif" => quests_niflheim::alreg_nif,
        "Bain#juno" => quests_yuno::bain_juno,
        "Bajin" => quests_yuno::bajin,
        "Bertztan" => quests_umbala::bertztan,
        "Billik" => quests_niflheim::billik,
        "Blacksmith" => quests_geffen::blacksmith,
        "Boy" => quests_payon::boy,
        "Chabimatan" => quests_umbala::chabimatan,
        "CiCi#juno" => quests_yuno::cici_juno,
        "Crayu#nif" => quests_niflheim::crayu_nif,
        "Edgar_izlude" => quests_izlude::edgar_izlude,
        "Eric" => quests_geffen::eric,
        "Erious#nif" => quests_niflheim::erious_nif,
        "Feylin" => quests_niflheim::feylin,
        "Granny" => quests_payon::granny,
        "Kato#juno" => quests_yuno::kato_juno,
        "Kuzkahina#nif" => quests_niflheim::kuzkahina_nif,
        "Metto#juno" => quests_yuno::metto_juno,
        "Muriniel's Cottage#juno" => quests_yuno::muriniel_s_cottage_juno,
        "Mystic Lady" => quests_payon::mystic_lady,
        "Nia#yagu" => quests_geffen::nia_yagu,
        "Phrenetan" => quests_umbala::phrenetan,
        "Piano3" => quests_niflheim::piano3,
        "Sage Esklah#juno" => quests_yuno::sage_esklah_juno,
        "Sage Syklah#juno" => quests_yuno::sage_syklah_juno,
        "Sage Yklah#juno" => quests_yuno::sage_yklah_juno,
        "Stangckle#juno" => quests_yuno::stangckle_juno,
        "Trader#01" => quests_aldebaran::trader_01,
        "Umpokoriohtan" => quests_umbala::umpokoriohtan,
        "Utan Chief" => quests_umbala::utan_chief,
        "Utan Shaman" => quests_umbala::utan_shaman,
        "Vending Machine" => quests_lutie::vending_machine,
        "Vending Machine Man" => quests_lutie::vending_machine_man,
        "Wagan#juno" => quests_yuno::wagan_juno,
        "Wainatan" => quests_umbala::wainatan,
        "Witch#nif" => quests_niflheim::witch_nif,
        "Young man#12" => quests_payon::young_man_12,
    }
    events {
        "#!@#$%::OnInit" => quests_umbala::script_oninit,
        "#!@#$%::OnTouch_" => quests_umbala::script_ontouch,
        "#Graveyard1::OnTouch_" => quests_niflheim::graveyard1_ontouch,
        "#Graveyard2::OnTouch_" => quests_niflheim::graveyard2_ontouch,
        "#Piano::OnTouch_" => quests_niflheim::piano_ontouch,
        "#unpc::OnInit" => quests_umbala::unpc_oninit,
        "#unpc::OnTouch_" => quests_umbala::unpc_ontouch,
        "Piano3::OnTouch_" => quests_niflheim::piano3_ontouch,
        "Umpokoriohtan::OnInit" => quests_umbala::umpokoriohtan_oninit,
    }
}
