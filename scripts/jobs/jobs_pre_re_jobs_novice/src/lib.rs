pub mod novice;

script_sdk_2::script_module! {
    npcs {
        "Bruce#nv" => novice::bruce_nv,
        "Bulletin Board#nv" => novice::bulletin_board_nv,
        "Entrance Guard#nv" => novice::entrance_guard_nv,
        "Guard#nv1" => novice::guard_nv1,
        "Guard#nv2" => novice::guard_nv2,
        "Guide Soldier#nv1" => novice::guide_soldier_nv1,
        "Hanson#nv" => novice::hanson_nv,
        "Helper#nv" => novice::helper_nv,
        "Instructor#nv" => novice::instructor_nv,
        "Interfaces Tutor#nv1" => novice::interfaces_tutor_nv1,
        "Item Tutor#nv" => novice::item_tutor_nv,
        "Kafra Employee#nv1" => novice::kafra_employee_nv1,
        "Receptionist#nv1" => novice::receptionist_nv1,
        "Shion#nv1" => novice::shion_nv1,
        "Skill Tutor#nv" => novice::skill_tutor_nv,
        "Somatology Instructor" => novice::somatology_instructor,
        "Test Examiner#nv1" => novice::test_examiner_nv1,
        "Trainer#nv1" => novice::trainer_nv1,
        "Understandings of Skills" => novice::understandings_of_skills,
    }
    events {
    }
}
