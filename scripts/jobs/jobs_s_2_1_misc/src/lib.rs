pub mod blacksmith;

script_sdk_2::script_module! {
    npcs {
        "Baisulist#BLS" => blacksmith::baisulist_bls,
        "Bismarc#BLS" => blacksmith::bismarc_bls,
        "Blacksmith Guildsman#gef" => blacksmith::blacksmith_guildsman_gef,
        "Blacksmith Guildsman#moc" => blacksmith::blacksmith_guildsman_moc,
        "Guildsman#BLS" => blacksmith::guildsman_bls,
        "Guildsman#alberta" => blacksmith::guildsman_alberta,
        "Krongast#BLS" => blacksmith::krongast_bls,
        "Talpiz#BLS" => blacksmith::talpiz_bls,
        "Wickebine#BLS" => blacksmith::wickebine_bls,
    }
    events {
    }
}
