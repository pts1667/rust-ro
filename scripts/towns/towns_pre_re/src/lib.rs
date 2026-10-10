pub mod merchants;

script_sdk_2::script_module! {
    npcs {
        "Bullet Dealer Tony#alb" => merchants::ammo_dealer::bullet_dealer_tony_alb,
        "Hair Dresser" => merchants::hair_style::hair_dresser,
        "Hair Dresser#li" => merchants::hair_style::hair_dresser_li,
        "Jovovich" => merchants::hair_dyer::jovovich,
        "Magazine Dealer Kenny" => merchants::ammo_boxes::magazine_dealer_kenny,
    }
    events {
    }
}
