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
        "Adventurer#1" => adventurer_1,
        "Apple of Idun" => apple_of_idun,
        "Bard#2" => bard_2,
        "Bard#3" => bard_3,
        "Bard#4" => bard_4,
        "Battle Songs" => battle_songs,
        "Little Girl#Jorti" => little_girl_jorti,
        "Luke's Songs Vol.1" => luke_s_songs_vol_1,
        "Old Book#bq" => old_book_bq,
        "Old Man#bq1" => old_man_bq1,
        "Representative#bq" => representative_bq,
    }
    events {
    }
}
