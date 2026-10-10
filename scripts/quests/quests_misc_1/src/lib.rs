pub mod bunnyband;
pub mod counteragent_mixture;
pub mod doomed_swords;
pub mod doomed_swords_quest;
pub mod gunslinger_quests;
pub mod juice_maker;
pub mod mage_solution;
pub mod monstertamers;
pub mod mrsmile;
pub mod ninja_quests;

script_sdk_2::script_module! {
    npcs {
        "Aure Dupon#cm" => counteragent_mixture::aure_dupon_cm,
        "Basshu" => ninja_quests::basshu,
        "Boshuu" => ninja_quests::boshuu,
        "Cetsu#magum" => doomed_swords::cetsu_magum,
        "Craftsman Kaibara" => ninja_quests::craftsman_kaibara,
        "Dollshoi" => mage_solution::dollshoi,
        "F. Harrison" => gunslinger_quests::f_harrison,
        "Garrison" => gunslinger_quests::garrison,
        "Ghatu#magum" => doomed_swords::ghatu_magum,
        "Ingrid" => gunslinger_quests::ingrid,
        "Kafra Employee#bunny" => bunnyband::kafra_employee_bunny,
        "Lab Director" => gunslinger_quests::lab_director,
        "Louitz#cm" => counteragent_mixture::louitz_cm,
        "Marianne#juice" => juice_maker::marianne_juice,
        "Marx Hansen#juice" => juice_maker::marx_hansen_juice,
        "Middle-Aged Man#magum1" => doomed_swords_quest::middle_aged_man_magum1,
        "Monster Tamer#alb" => monstertamers::monster_tamer_alb,
        "Monster Tamer#alde" => monstertamers::monster_tamer_alde,
        "MonsterTamer_izlude" => monstertamers::monstertamer_izlude,
        "Morgenstein#cm" => counteragent_mixture::morgenstein_cm,
        "Morrison#juice" => juice_maker::morrison_juice,
        "Munak's Grandma" => monstertamers::munak_s_grandma,
        "Mysterious Man#magum" => doomed_swords::mysterious_man_magum,
        "Nain#magum" => doomed_swords::nain_magum,
        "Old Man#magum1" => doomed_swords_quest::old_man_magum1,
        "Ponka-Hontas" => mage_solution::ponka_hontas,
        "Ravey" => gunslinger_quests::ravey,
        "Smile Assistance" => mrsmile::smile_assistance,
        "Tetsu" => ninja_quests::tetsu,
        "Toshu" => ninja_quests::toshu,
        "Vanessa" => gunslinger_quests::vanessa,
        "Veeyop#magum" => doomed_swords::veeyop_magum,
        "Young Man#magum1" => doomed_swords_quest::young_man_magum1,
    }
    events {
    }
}
