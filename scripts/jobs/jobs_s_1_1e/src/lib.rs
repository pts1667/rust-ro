pub mod gunslinger;
pub mod ninja;
pub mod taekwon;

script_sdk_2::script_module! {
    npcs {
        "Akagi" => ninja::akagi,
        "Kuuga Gai#nq" => ninja::kuuga_gai_nq,
        "Master Miller" => gunslinger::master_miller,
        "Phoenix" => taekwon::phoenix,
        "Suspicious Man#nq" => ninja::suspicious_man_nq,
        "Wise Bull Horn" => gunslinger::wise_bull_horn,
    }
    events {
    }
}
