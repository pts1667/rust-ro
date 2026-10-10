pub mod quest_alligator;
pub mod quest_caramel;
pub mod quest_coco;
pub mod quest_creamy;
pub mod quest_demonpungus;
pub mod quest_disguiseloliruri;
pub mod quest_dokebi;
pub mod quest_dryad;
pub mod quest_fabre;
pub mod quest_frilldora;
pub mod quest_goat;
pub mod quest_golem;
pub mod quest_hode;
pub mod quest_leafcat;
pub mod quest_mantis;
pub mod quest_pecopeco;
pub mod quest_pupa;
pub mod quest_zhupolong;

script_sdk_2::script_module! {
    npcs {
        "Cuir#Gator_Hunt" => quest_alligator::cuir_gator_hunt,
        "Deadman" => quest_disguiseloliruri::deadman,
        "Dragon Hunter" => quest_zhupolong::dragon_hunter,
        "Gregor#PecoPeco_Hunt" => quest_pecopeco::gregor_pecopeco_hunt,
        "Halgus#Pupa_Hunt" => quest_pupa::halgus_pupa_hunt,
        "Laertes#Creamy_Hunt" => quest_creamy::laertes_creamy_hunt,
        "Langry#Fabre_Hunt" => quest_fabre::langry_fabre_hunt,
        "Lella#LeafCat_Hunt" => quest_leafcat::lella_leafcat_hunt,
        "Lemly#Frilldora_Hunt" => quest_frilldora::lemly_frilldora_hunt,
        "Li#Dokebi_Hunt" => quest_dokebi::li_dokebi_hunt,
        "Lilla#Dryad_Hunt" => quest_dryad::lilla_dryad_hunt,
        "Local Villager#hunt" => quest_demonpungus::local_villager_hunt,
        "Mantis Researcher" => quest_mantis::mantis_researcher,
        "Nutters#Coco_Hunt" => quest_coco::nutters_coco_hunt,
        "Private Jeremy#hunt" => quest_golem::private_jeremy_hunt,
        "Shone#Hode_Hunt" => quest_hode::shone_hode_hunt,
        "Vegetable Farmer#Goat" => quest_goat::vegetable_farmer_goat,
        "Yullo#Caramel_Hunt" => quest_caramel::yullo_caramel_hunt,
    }
    events {
    }
}
