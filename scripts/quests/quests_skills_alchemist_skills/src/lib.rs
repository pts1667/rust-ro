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
        "Alchemist#qsk_al" => alchemist_qsk_al,
        "Beninne#qsk_al" => beninne_qsk_al,
        "Broncher#qsk_al" => broncher_qsk_al,
        "Degas#qsk_al" => degas_qsk_al,
        "Irache#qsk_al" => irache_qsk_al,
        "Kellasus#qsk_al" => kellasus_qsk_al,
        "Keshibien#qsk_al" => keshibien_qsk_al,
        "Koring#qsk_al" => koring_qsk_al,
        "Nannan#qsk_al" => nannan_qsk_al,
        "Pile of Books#qsk_al" => pile_of_books_qsk_al,
        "Pisruik#qsk_al" => pisruik_qsk_al,
        "Skrajjad#qsk_al" => skrajjad_qsk_al,
    }
    events {
    }
}
