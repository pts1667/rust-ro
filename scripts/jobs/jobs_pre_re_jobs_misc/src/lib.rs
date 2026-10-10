pub mod s_2_2;

script_sdk_2::script_module! {
    npcs {
        "Monster Summon#cr5" => s_2_2::crusader::monster_summon_cr5,
        "Summoner#cr5" => s_2_2::crusader::summoner_cr5,
    }
    events {
        "Monster Summon#cr5::OnEnd" => s_2_2::crusader::monster_summon_cr5_onend,
        "Monster Summon#cr5::OnInit" => s_2_2::crusader::monster_summon_cr5_oninit,
        "Monster Summon#cr5::OnStart" => s_2_2::crusader::monster_summon_cr5_onstart,
        "Monster Summon#cr5::OnTouch_" => s_2_2::crusader::monster_summon_cr5_ontouch,
        "Summoner#cr5::OnTouch_" => s_2_2::crusader::summoner_cr5_ontouch,
    }
}
