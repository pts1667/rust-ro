pub mod stargladiator;

script_sdk_2::script_module! {
    npcs {
        "Beeryu#job_star" => stargladiator::beeryu_job_star,
        "Cheehee#job_star" => stargladiator::cheehee_job_star,
        "Daru#job_star" => stargladiator::daru_job_star,
        "Moohyun#job_star" => stargladiator::moohyun_job_star,
        "Wandering Master#job_sta" => stargladiator::wandering_master_job_sta,
    }
    events {
        "Moohyun#job_star::OnTouch_" => stargladiator::moohyun_job_star_ontouch,
    }
}
