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
mod part_04;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;
pub use part_04::*;

script_sdk_2::script_module! {
    npcs {
        "Arena#1" => arena_1,
        "Arena#2" => arena_2,
        "Arena#3" => arena_3,
        "Arena#Doorkeeper" => arena_doorkeeper,
        "Biology Professor#sa" => biology_professor_sa,
        "Dean of the Academy#sa" => dean_of_the_academy_sa,
        "History Professor#sa" => history_professor_sa,
        "Physics Professor#sa" => physics_professor_sa,
        "Practical Examination P" => practical_examination_p,
        "Staff of the Academy#a" => staff_of_the_academy_a,
        "Test Helper#sg" => test_helper_sg,
        "Test Helper#talk" => test_helper_talk,
        "Waiting Room#sg" => waiting_room_sg,
        "Written Test Professor#s" => written_test_professor_s,
    }
    events {
        "Arena#1::OnEnable" => arena_1_onenable,
        "Arena#1::OnInit" => arena_1_oninit,
        "Arena#1::OnMyMobDead" => arena_1_onmymobdead,
        "Arena#1::OnReset" => arena_1_onreset,
        "Arena#1::OnTimer1000" => arena_1_ontimer1000,
        "Arena#1::OnTimer123000" => arena_1_ontimer123000,
        "Arena#1::OnTimer153000" => arena_1_ontimer153000,
        "Arena#1::OnTimer173000" => arena_1_ontimer173000,
        "Arena#1::OnTimer183000" => arena_1_ontimer183000,
        "Arena#1::OnTimer184000" => arena_1_ontimer184000,
        "Arena#1::OnTimer185000" => arena_1_ontimer185000,
        "Arena#1::OnTimer186000" => arena_1_ontimer186000,
        "Arena#1::OnTimer2000" => arena_1_ontimer2000,
        "Arena#1::OnTimer3000" => arena_1_ontimer3000,
        "Arena#1::OnTimer33000" => arena_1_ontimer33000,
        "Arena#1::OnTimer63000" => arena_1_ontimer63000,
        "Arena#1::OnTimer93000" => arena_1_ontimer93000,
        "Arena#2::OnEnable" => arena_2_onenable,
        "Arena#2::OnInit" => arena_2_oninit,
        "Arena#2::OnMyMobDead" => arena_2_onmymobdead,
        "Arena#2::OnReset" => arena_2_onreset,
        "Arena#2::OnTimer1000" => arena_2_ontimer1000,
        "Arena#2::OnTimer123000" => arena_2_ontimer123000,
        "Arena#2::OnTimer153000" => arena_2_ontimer153000,
        "Arena#2::OnTimer173000" => arena_2_ontimer173000,
        "Arena#2::OnTimer183000" => arena_2_ontimer183000,
        "Arena#2::OnTimer184000" => arena_2_ontimer184000,
        "Arena#2::OnTimer185000" => arena_2_ontimer185000,
        "Arena#2::OnTimer186000" => arena_2_ontimer186000,
        "Arena#2::OnTimer2000" => arena_2_ontimer2000,
        "Arena#2::OnTimer33000" => arena_2_ontimer33000,
        "Arena#2::OnTimer63000" => arena_2_ontimer63000,
        "Arena#2::OnTimer93000" => arena_2_ontimer93000,
        "Arena#3::OnDisable" => arena_3_ondisable,
        "Arena#3::OnEnable" => arena_3_onenable,
        "Arena#3::OnInit" => arena_3_oninit,
        "Arena#Doorkeeper::OnDisable" => arena_doorkeeper_ondisable,
        "Arena#Doorkeeper::OnEnable" => arena_doorkeeper_onenable,
        "Arena#Doorkeeper::OnInit" => arena_doorkeeper_oninit,
        "Arena#Doorkeeper::OnMyMobDead" => arena_doorkeeper_onmymobdead,
        "Arena#Doorkeeper::OnReset" => arena_doorkeeper_onreset,
        "Arena#Doorkeeper::OnTimer1000" => arena_doorkeeper_ontimer1000,
        "Arena#Doorkeeper::OnTimer30000" => arena_doorkeeper_ontimer30000,
        "Arena#Doorkeeper::OnTimer50000" => arena_doorkeeper_ontimer50000,
        "Arena#Doorkeeper::OnTimer60000" => arena_doorkeeper_ontimer60000,
        "Arena#Doorkeeper::OnTimer61000" => arena_doorkeeper_ontimer61000,
        "Arena#Doorkeeper::OnTimer62000" => arena_doorkeeper_ontimer62000,
        "Arena#Doorkeeper::OnTimer63000" => arena_doorkeeper_ontimer63000,
        "Test Helper#sg::OnEnable" => test_helper_sg_onenable,
        "Test Helper#sg::OnInit" => test_helper_sg_oninit,
        "Test Helper#sg::OnTimer2000" => test_helper_sg_ontimer2000,
        "Test Helper#sg::OnTimer4000" => test_helper_sg_ontimer4000,
        "Test Helper#sg::OnTimer5000" => test_helper_sg_ontimer5000,
        "Test Helper#sg::OnTimer7000" => test_helper_sg_ontimer7000,
        "Test Helper#sg::OnTimer9000" => test_helper_sg_ontimer9000,
        "Waiting Room#sg::OnEnable" => waiting_room_sg_onenable,
        "Waiting Room#sg::OnInit" => waiting_room_sg_oninit,
        "Waiting Room#sg::OnStartArena" => waiting_room_sg_onstartarena,
    }
}
