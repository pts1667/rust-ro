pub mod novice_skills;
pub mod priest_skills;
pub mod rogue_skills;
pub mod sage_skills;
pub mod swordman_skills;
pub mod thief_skills;
pub mod wizard_skills;

script_sdk_2::script_module! {
    npcs {
        "#1st5min" => rogue_skills::s_1st5min,
        "#1stmove" => rogue_skills::s_1stmove,
        "#1strecog" => rogue_skills::s_1strecog,
        "#killershow01" => rogue_skills::killershow01,
        "Alcouskou" => thief_skills::alcouskou,
        "Bag Seller" => thief_skills::bag_seller,
        "Chivalry Member" => novice_skills::chivalry_member,
        "Haijara Greg#rogueguild" => rogue_skills::haijara_greg_rogueguild,
        "Jay Greg#rogueguild" => rogue_skills::jay_greg_rogueguild,
        "Juan" => swordman_skills::juan,
        "Kienna#1st" => rogue_skills::kienna_1st,
        "Killer#Rogueguild" => rogue_skills::killer_rogueguild,
        "KnightDeThomas" => swordman_skills::knightdethomas,
        "Leon Von Frich" => swordman_skills::leon_von_frich,
        "Louis Greg#rogueguild" => rogue_skills::louis_greg_rogueguild,
        "Meow#q_wiz" => wizard_skills::meow_q_wiz,
        "Mischna" => sage_skills::mischna,
        "Nami" => novice_skills::nami,
        "Nursing Instructor" => novice_skills::nursing_instructor,
        "Simon Mayace#q_wiz" => wizard_skills::simon_mayace_q_wiz,
        "Sister Linus" => priest_skills::sister_linus,
        "Thor Greg#rogueguild" => rogue_skills::thor_greg_rogueguild,
        "Waiting Room#rogue10" => rogue_skills::waiting_room_rogue10,
    }
    events {
        "#1st5min::OnDisable" => rogue_skills::s_1st5min_ondisable,
        "#1st5min::OnEnable" => rogue_skills::s_1st5min_onenable,
        "#1st5min::OnInit" => rogue_skills::s_1st5min_oninit,
        "#1st5min::OnTimer1000" => rogue_skills::s_1st5min_ontimer1000,
        "#1st5min::OnTimer290000" => rogue_skills::s_1st5min_ontimer290000,
        "#1st5min::OnTimer310000" => rogue_skills::s_1st5min_ontimer310000,
        "#1st5min::OnTimer315000" => rogue_skills::s_1st5min_ontimer315000,
        "#1stmove::OnDisable" => rogue_skills::s_1stmove_ondisable,
        "#1stmove::OnEnable" => rogue_skills::s_1stmove_onenable,
        "#1stmove::OnInit" => rogue_skills::s_1stmove_oninit,
        "#1stmove::OnTimer3000" => rogue_skills::s_1stmove_ontimer3000,
        "#1stmove::OnTimer5000" => rogue_skills::s_1stmove_ontimer5000,
        "#1stmove::OnTimer8000" => rogue_skills::s_1stmove_ontimer8000,
        "#1stmove::OnTimer9000" => rogue_skills::s_1stmove_ontimer9000,
        "#1strecog::OnTouch_" => rogue_skills::s_1strecog_ontouch,
        "#killershow01::OnDisable" => rogue_skills::killershow01_ondisable,
        "#killershow01::OnEnable" => rogue_skills::killershow01_onenable,
        "#killershow01::OnInit" => rogue_skills::killershow01_oninit,
        "#killershow01::OnTimer1000" => rogue_skills::killershow01_ontimer1000,
        "#killershow01::OnTimer120000" => rogue_skills::killershow01_ontimer120000,
        "#killershow01::OnTimer150000" => rogue_skills::killershow01_ontimer150000,
        "Kienna#1st::OnTouch_" => rogue_skills::kienna_1st_ontouch,
        "Killer#Rogueguild::OnInit" => rogue_skills::killer_rogueguild_oninit,
        "Killer#Rogueguild::OnTouch_" => rogue_skills::killer_rogueguild_ontouch,
        "KnightDeThomas::OnTouch" => swordman_skills::knightdethomas_ontouch,
        "Leon Von Frich::OnTouch_" => swordman_skills::leon_von_frich_ontouch,
        "Waiting Room#rogue10::OnEnable" => rogue_skills::waiting_room_rogue10_onenable,
        "Waiting Room#rogue10::OnInit" => rogue_skills::waiting_room_rogue10_oninit,
        "Waiting Room#rogue10::OnStartArena" => rogue_skills::waiting_room_rogue10_onstartarena,
    }
}
