pub mod s_2006_headgears;
pub mod s_2008_headgears;

script_sdk_2::script_module! {
    npcs {
        "?" => s_2006_headgears::script,
        "Chungwolmang" => s_2006_headgears::chungwolmang,
        "Ghenirhemin" => s_2006_headgears::ghenirhemin,
        "Han Garam" => s_2006_headgears::han_garam,
        "Myu#08_hat" => s_2008_headgears::myu_08_hat,
        "Orc Lady#2008hat03" => s_2008_headgears::orc_lady_2008hat03,
        "Sakjul" => s_2006_headgears::sakjul,
        "Trainee#2008hat01" => s_2008_headgears::trainee_2008hat01,
    }
    events {
        "Orc Lady#2008hat03::OnTouch" => s_2008_headgears::orc_lady_2008hat03_ontouch,
    }
}
