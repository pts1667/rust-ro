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
mod part_05;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;
pub use part_04::*;
pub use part_05::*;

script_sdk_2::script_module! {
    npcs {
        "#fire" => fire,
        "#lou_drink1" => lou_drink1,
        "#lou_drink2" => lou_drink2,
        "#lou_path" => lou_path,
        "Chef Assistant#lou1" => chef_assistant_lou1,
        "Chef#1-2" => chef_1_2,
        "Chi Wu Ping#lou" => chi_wu_ping_lou,
        "City Hall Officer#lou" => city_hall_officer_lou,
        "Doctor#lyang" => doctor_lyang,
        "Employee#1" => employee_1,
        "Employee#2" => employee_2,
        "Employee#3" => employee_3,
        "Employee#poison" => employee_poison,
        "Familiar-Looking Patient" => familiar_looking_patient,
        "Gunpowder Expert" => gunpowder_expert,
        "Hermit" => hermit,
        "Jiang Rong#lou" => jiang_rong_lou,
        "Jiu Chi Ling#lou" => jiu_chi_ling_lou,
        "Jiu Lian Bu#1-1" => jiu_lian_bu_1_1,
        "Jiu Lian Bu#1-2" => jiu_lian_bu_1_2,
        "Lady#delivery" => lady_delivery,
        "Li Min#lou" => li_min_lou,
        "Liu Jia Lim#lou" => liu_jia_lim_lou,
        "Lord#bailong" => lord_bailong,
        "Poison King#lou" => poison_king_lou,
        "Pot#1" => pot_1,
        "Pot#2" => pot_2,
        "Soldier#1" => soldier_1,
        "Soldier#2" => soldier_2,
        "Soldier#bailong1" => soldier_bailong1,
        "Soldier#bailong2" => soldier_bailong2,
        "Soldier#bailong3" => soldier_bailong3,
        "Soldier#bailong4" => soldier_bailong4,
        "Storage Keeper#lou" => storage_keeper_lou,
        "Studying Officer#lou" => studying_officer_lou,
        "Supply Stack#1lou" => supply_stack_1lou,
        "Supply Stack#2" => supply_stack_2,
        "Supply Stack#3lou" => supply_stack_3lou,
        "Supply Stack#4lou" => supply_stack_4lou,
        "Supply Stack#5lou" => supply_stack_5lou,
        "Tool Shop Master#lou" => tool_shop_master_lou,
        "Trap#lou_in1" => trap_lou_in1,
        "Trap#lou_in2" => trap_lou_in2,
        "Trap#lou_in3" => trap_lou_in3,
        "Trap#lou_in4" => trap_lou_in4,
        "Trap#lou_in5" => trap_lou_in5,
    }
    events {
        "Chef Assistant#lou1::OnTouch_" => chef_assistant_lou1_ontouch,
        "Chef#1-2::OnTouch_" => chef_1_2_ontouch,
        "Employee#1::OnTouch_" => employee_1_ontouch,
        "Employee#2::OnTouch_" => employee_2_ontouch,
        "Employee#3::OnTouch_" => employee_3_ontouch,
        "Trap#lou_in1::OnTouch_" => trap_lou_in1_ontouch,
        "Trap#lou_in2::OnTouch_" => trap_lou_in2_ontouch,
        "Trap#lou_in3::OnTouch_" => trap_lou_in3_ontouch,
        "Trap#lou_in4::OnTouch_" => trap_lou_in4_ontouch,
        "Trap#lou_in5::OnTouch_" => trap_lou_in5_ontouch,
    }
}
