#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

mod common;
mod part_01;
mod part_02;
mod part_03;

pub use part_01::*;
pub use part_02::*;
pub use part_03::*;

script_sdk_2::script_module! {
    npcs {
        "#Monkeybra" => monkeybra,
        "Angelo#br" => angelo_br,
        "Botanist Karmen#bra" => botanist_karmen_bra,
        "Brasilis Girl#bra" => brasilis_girl_bra,
        "Candy Maker" => candy_maker,
        "Carpet#bra" => carpet_bra,
        "Cherto" => cherto,
        "Curator#bra" => curator_bra,
        "Daniel#bra" => daniel_bra,
        "Door#bra" => door_bra,
        "Fabio#bra" => fabio_bra,
        "Faucet#bra" => faucet_bra,
        "Ghost#bra" => ghost_bra,
        "Ghost#bra_end" => ghost_bra_end,
        "Iara#nk" => iara_nk,
        "Jaguar#bra" => jaguar_bra,
        "Lucia#brasilis" => lucia_brasilis,
        "Mage Paje#bra" => mage_paje_bra,
        "Mariana#bra" => mariana_bra,
        "Marta#bra" => marta_bra,
        "Mirror#bra" => mirror_bra,
        "Monkey#bra" => monkey_bra,
        "Native Warrior#nk" => native_warrior_nk,
        "Open Manhole#todunbra" => open_manhole_todunbra,
        "Pedro#bra" => pedro_bra,
        "Pipe#bra" => pipe_bra,
        "Pipe#brafild" => pipe_brafild,
        "Poring#bra" => poring_bra,
        "Puppy#bra" => puppy_bra,
        "Recluse#bra" => recluse_bra,
        "Shaman#nk" => shaman_nk,
        "Strange Kid#bra" => strange_kid_bra,
        "Toilet#bra" => toilet_bra,
        "Toucan#bra" => toucan_bra,
        "Water lily#bra" => water_lily_bra,
        "inbathroom#bra" => inbathroom_bra,
    }
    events {
        "Angelo#br::OnGo" => angelo_br_ongo,
        "Angelo#br::OnInit" => angelo_br_oninit,
        "Angelo#br::OnTimer10000" => angelo_br_ontimer10000,
        "Brasilis Girl#bra::OnTouch" => brasilis_girl_bra_ontouch,
        "Ghost#bra::OnInit" => ghost_bra_oninit,
        "Ghost#bra_end::OnInit" => ghost_bra_end_oninit,
        "Iara#nk::OnTouch" => iara_nk_ontouch,
        "Jaguar#bra::OnTouch_" => jaguar_bra_ontouch,
        "Lucia#brasilis::OnInit" => lucia_brasilis_oninit,
        "Lucia#brasilis::OnTimer7000" => lucia_brasilis_ontimer7000,
        "Pedro#bra::OnTalk" => pedro_bra_ontalk,
        "Puppy#bra::OnDisable" => puppy_bra_ondisable,
        "Puppy#bra::OnEnable" => puppy_bra_onenable,
        "Puppy#bra::OnInit" => puppy_bra_oninit,
        "Recluse#bra::OnTouchNPC" => recluse_bra_ontouchnpc,
        "Toucan#bra::OnTouch" => toucan_bra_ontouch,
        "inbathroom#bra::OnTouch_" => inbathroom_bra_ontouch,
    }
}
