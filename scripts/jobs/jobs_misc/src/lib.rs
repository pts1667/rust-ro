pub mod novice;
pub mod s_2_1a;
pub mod s_2_2a;
pub mod s_2_2e;
pub mod valkyrie;

script_sdk_2::script_module! {
    npcs {
        "Alchemist Spirit#link7" => s_2_2e::soullinker::alchemist_spirit_link7,
        "Assassin Cross#Valkyrie" => s_2_1a::assassincross::assassin_cross_valkyrie,
        "Biochemist#Valkyrie" => s_2_2a::creator::biochemist_valkyrie,
        "Book of Ymir" => valkyrie::book_of_ymir,
        "Champion#Valkyrie" => s_2_2a::champion::champion_valkyrie,
        "Esseray#sn" => novice::supernovice::esseray_sn,
        "Gypsy#Valkyrie" => s_2_2a::gypsy::gypsy_valkyrie,
        "Heart of Ymir" => valkyrie::heart_of_ymir,
        "High Priest#Valkyrie" => s_2_1a::highpriest::high_priest_valkyrie,
        "High Wizard#Valkyrie" => s_2_1a::highwizard::high_wizard_valkyrie,
        "Kafra Employee#sn" => novice::supernovice::kafra_employee_sn,
        "Kid#link1" => s_2_2e::soullinker::kid_link1,
        "Lord Knight#Valkyrie" => s_2_1a::lordknight::lord_knight_valkyrie,
        "Maia#link2" => s_2_2e::soullinker::maia_link2,
        "MasterSmith#Valkyrie" => s_2_1a::whitesmith::mastersmith_valkyrie,
        "Metheus Sylphe#Library" => valkyrie::metheus_sylphe_library,
        "Minstrel#Valkyrie" => s_2_2a::clown::minstrel_valkyrie,
        "Monk Spirit#link4" => s_2_2e::soullinker::monk_spirit_link4,
        "Paladin#Valkyrie" => s_2_2a::paladin::paladin_valkyrie,
        "Sage Spirit#link5" => s_2_2e::soullinker::sage_spirit_link5,
        "Scholar#Valkyrie" => s_2_2a::professor::scholar_valkyrie,
        "Sniper#Valkyrie" => s_2_1a::sniper::sniper_valkyrie,
        "Soul Linker Var" => s_2_2e::soullinker::soul_linker_var,
        "Stalker#Valkyrie" => s_2_2a::stalker::stalker_valkyrie,
        "Teleporter" => valkyrie::teleporter,
        "Timer#link3" => s_2_2e::soullinker::timer_link3,
        "Tzerero#sn" => novice::supernovice::tzerero_sn,
        "Valkyrie#" => valkyrie::valkyrie,
    }
    events {
        "Kid#link1::OnInit" => s_2_2e::soullinker::kid_link1_oninit,
        "Maia#link2::OnTouch_" => s_2_2e::soullinker::maia_link2_ontouch,
        "Timer#link3::OnDisable" => s_2_2e::soullinker::timer_link3_ondisable,
        "Timer#link3::OnEnable" => s_2_2e::soullinker::timer_link3_onenable,
        "Timer#link3::OnTimer120000" => s_2_2e::soullinker::timer_link3_ontimer120000,
        "Timer#link3::OnTimer180000" => s_2_2e::soullinker::timer_link3_ontimer180000,
        "Timer#link3::OnTimer181000" => s_2_2e::soullinker::timer_link3_ontimer181000,
        "Timer#link3::OnTimer182000" => s_2_2e::soullinker::timer_link3_ontimer182000,
        "Timer#link3::OnTimer183000" => s_2_2e::soullinker::timer_link3_ontimer183000,
        "Timer#link3::OnTimer60000" => s_2_2e::soullinker::timer_link3_ontimer60000,
    }
}
