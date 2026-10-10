pub mod halloween_2009;
pub mod halloween_2013;
pub mod idul_fitri;
pub mod lunar_2008;
pub mod memorialday_2008;
pub mod rwc_2011;
pub mod rwc_2012;
pub mod stpatrick_2008;
pub mod twintowers;
pub mod valentinesday;
pub mod valentinesday_2009;
pub mod valentinesday_2012;
pub mod whiteday;
pub mod xmas;

script_sdk_2::script_module! {
    npcs {
        "Anxious Leprechaun#8pday" => stpatrick_2008::anxious_leprechaun_8pday,
        "Baker Extraordinaire" => valentinesday_2012::baker_extraordinaire,
        "Carl Orleans" => valentinesday::carl_orleans,
        "Cellerb" => idul_fitri::cellerb,
        "Chef Candycon#2013HE" => halloween_2013::chef_candycon_2013he,
        "Chicken#2013HE" => halloween_2013::chicken_2013he,
        "Dessert Manager#Val09" => valentinesday_2009::dessert_manager_val09,
        "Driller#pron" => rwc_2012::driller_pron,
        "Event Ring Maker#Val09" => valentinesday_2009::event_ring_maker_val09,
        "Father Christmas" => xmas::father_christmas,
        "Goldberg#pron" => rwc_2012::goldberg_pron,
        "Grast#Memorial" => memorialday_2008::grast_memorial,
        "Halloween Wizard#iRO09" => halloween_2009::halloween_wizard_iro09,
        "Jainie" => valentinesday::jainie,
        "Kentucky#2013HE" => halloween_2013::kentucky_2013he,
        "Lauds#Memorial" => memorialday_2008::lauds_memorial,
        "Memorial Plaque#Memorial" => memorialday_2008::memorial_plaque_memorial,
        "Miss Lunar#rat" => lunar_2008::miss_lunar_rat,
        "Packs Trader#Val09" => valentinesday_2009::packs_trader_val09,
        "Pinkamenia" => valentinesday_2012::pinkamenia,
        "Pumpkin Hat Researcher" => halloween_2009::pumpkin_hat_researcher,
        "RWC2011 Agent#2" => rwc_2011::rwc2011_agent_2,
        "Rice Mill Grandma#rat" => lunar_2008::rice_mill_grandma_rat,
        "Rice Mill Man#rat" => lunar_2008::rice_mill_man_rat,
        "Rocks#08StPattysDay" => stpatrick_2008::rocks_08stpattysday,
        "Stephen" => valentinesday::stephen,
        "Sugar" => whiteday::sugar,
        "Suspicious Coffin#2013HE" => halloween_2013::suspicious_coffin_2013he,
        "Trader#Val09" => valentinesday_2009::trader_val09,
        "Treat#2013HE" => halloween_2013::treat_2013he,
        "Trick or Treater" => halloween_2009::trick_or_treater,
        "Trick#2013HE" => halloween_2013::trick_2013he,
        "Twin Towers#tt1" => twintowers::twin_towers_tt1,
        "Valentine Vote Manager#v" => valentinesday_2009::valentine_vote_manager_v,
        "Wandering soul#2013HE" => halloween_2013::wandering_soul_2013he,
        "Wandering soul#2013HE2" => halloween_2013::wandering_soul_2013he2,
        "Wandering soul#2013HE3" => halloween_2013::wandering_soul_2013he3,
        "Wandering soul#2013HE4" => halloween_2013::wandering_soul_2013he4,
    }
    events {
        "Father Christmas::OnInit" => xmas::father_christmas_oninit,
        "Pinkamenia::OnInit" => valentinesday_2012::pinkamenia_oninit,
        "Rocks#08StPattysDay::OnTouch" => stpatrick_2008::rocks_08stpattysday_ontouch,
        "Trick or Treater::OnEnableTreat" => halloween_2009::trick_or_treater_onenabletreat,
        "Trick or Treater::OnInit" => halloween_2009::trick_or_treater_oninit,
        "Trick or Treater::OnTimer15000" => halloween_2009::trick_or_treater_ontimer15000,
        "Trick or Treater::OnTimer300000" => halloween_2009::trick_or_treater_ontimer300000,
        "Trick or Treater::OnTouch" => halloween_2009::trick_or_treater_ontouch,
    }
}
