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

pub use part_01::*;
pub use part_02::*;

script_sdk_2::script_module! {
    npcs {
        "BBQ Boy#cmd" => bbq_boy_cmd,
        "BBQ Papa#cmd" => bbq_papa_cmd,
        "BBQ Visitor#cmd" => bbq_visitor_cmd,
        "Campground Boy#cmd" => campground_boy_cmd,
        "Campground Lad#cmd" => campground_lad_cmd,
        "Camping Maiden#cmd" => camping_maiden_cmd,
        "Camping Youth#cmd" => camping_youth_cmd,
        "Chief#cmd" => chief_cmd,
        "Hair Ornament Girl" => hair_ornament_girl,
        "Hullaris#cmd" => hullaris_cmd,
        "Kichiri#cmd" => kichiri_cmd,
        "Magatu#cmd" => magatu_cmd,
        "Manzi#cmd" => manzi_cmd,
        "Meteurengut#cmd" => meteurengut_cmd,
        "Nigirboran#cmd" => nigirboran_cmd,
        "Rakusa#cmd" => rakusa_cmd,
        "Toruna#cmd" => toruna_cmd,
        "Traveler#head" => traveler_head,
        "Won#cmd" => won_cmd,
        "Zaka#cmd" => zaka_cmd,
    }
    events {
    }
}
