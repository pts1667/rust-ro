pub mod acolyte;
pub mod archer;
pub mod mage;
pub mod merchant;
pub mod swordman;
pub mod thief;

script_sdk_2::script_module! {
    npcs {
        "1_blank_1_a" => swordman::s_1_blank_1_a,
        "2_blank_1_a" => swordman::s_2_blank_1_a,
        "3_blank_1_a" => swordman::s_3_blank_1_a,
        "Archer Guildsman#archer" => archer::archer_guildsman_archer,
        "Ascetic#2aco" => acolyte::ascetic_2aco,
        "Ascetic#3aco" => acolyte::ascetic_3aco,
        "Ascetic#aco" => acolyte::ascetic_aco,
        "Bookshelf" => mage::bookshelf,
        "Cleric#aco" => acolyte::cleric_aco,
        "Comrade" => thief::comrade,
        "Guild Staff#mer" => merchant::guild_staff_mer,
        "Kafra Employee#mer" => merchant::kafra_employee_mer,
        "Mae#swd_1_success" => swordman::mae_swd_1_success,
        "Mage Guildsman" => mage::mage_guildsman,
        "Medic#2swd_2" => swordman::medic_2swd_2,
        "Medic#swd_1" => swordman::medic_swd_1,
        "Merchant Guildsman#mer" => merchant::merchant_guildsman_mer,
        "Merchant#mer" => merchant::merchant_mer,
        "Mixing Machine" => mage::mixing_machine,
        "Mr. Irrelevant" => thief::mr_irrelevant,
        "Student#mer" => merchant::student_mer,
        "Swordman#swd_1" => swordman::swordman_swd_1,
        "Swordman#swd_2" => swordman::swordman_swd_2,
        "Swordman#swd_3" => swordman::swordman_swd_3,
        "Test Hall Staff#2swd_3" => swordman::test_hall_staff_2swd_3,
        "Test Hall Staff#swd_1" => swordman::test_hall_staff_swd_1,
        "Test Hall Staff#swd_2" => swordman::test_hall_staff_swd_2,
        "Test Hall Staff#swd_4" => swordman::test_hall_staff_swd_4,
        "Test Hall Staff#swd_5" => swordman::test_hall_staff_swd_5,
        "Test Hall Staff#swd_6" => swordman::test_hall_staff_swd_6,
        "Test Hall Staff#swd_7" => swordman::test_hall_staff_swd_7,
        "Test Hall Staff#swd_8" => swordman::test_hall_staff_swd_8,
        "Test Hall Staff#swd_9" => swordman::test_hall_staff_swd_9,
        "Thief Guide" => thief::thief_guide,
    }
    events {
        "1_blank_1_a::OnTouch" => swordman::s_1_blank_1_a_ontouch,
        "2_blank_1_a::OnTouch" => swordman::s_2_blank_1_a_ontouch,
        "3_blank_1_a::OnTouch" => swordman::s_3_blank_1_a_ontouch,
    }
}
