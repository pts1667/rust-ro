use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum AirshipAirplane02Step {
    Start,
    OnEnable,
    OnTimer25000,
    OnTimer30000,
    OnTimer34000,
    OnTimer38000,
    OnTimer48000,
    OnTimer48010,
    OnTimer53000,
    OnTimer58000,
    OnTimer63000,
    OnTimer68000,
    OnTimer73000,
    OnTimer73500,
    OnTimer74000,
    OnTimer74500,
    OnTimer75000,
    OnTimer75500,
    OnTimer76000,
    OnTimer76500,
    OnTimer77000,
    OnTimer77500,
    OnTimer78000,
    OnTimer79000,
    OnTimer80000,
    OnTimer81000,
    OnTimer82000,
    OnTimer83000,
    OnTimer84000,
    OnTimer85000,
    OnTimer86000,
    OnTimer87000,
    OnTimer88000,
    OnTimer93000,
    OnTimer98000,
    OnTimer103000,
    OnTimer103500,
    OnTimer104000,
    OnTimer104500,
    OnTimer105000,
    OnTimer105500,
    OnTimer106000,
    OnTimer106500,
    OnTimer107000,
    OnTimer107500,
    OnTimer108000,
    OnTimer113000,
    OnTimer118000,
    OnTimer118500,
    OnTimer119000,
    OnTimer119500,
    OnTimer120000,
    OnTimer120500,
    OnTimer121000,
    OnTimer121500,
    OnTimer122000,
    OnTimer122500,
    OnTimer123000,
    OnTimer124000,
    OnTimer125000,
    OnTimer126000,
    OnTimer127000,
    OnTimer128000,
    OnTimer133000,
    OnTimer138000,
    OnTimer143000,
    OnTimer148000,
    OnTimer400000,
    OnTimer405000,
    OnTimer410000,
    OnTimer420000,
    OnTimer430000,
    OnTimer440000,
    OnTimer465000,
    OnTimer490000,
    OnTimer500000,
    OnTimer510000,
    OnTimer520000,
    OnTimer545000,
    OnTimer570000,
    OnTimer580000,
    OnTimer590000,
    OnTimer600000,
    OnMyMobDead,
    OnCaptainMobDead,
}

pub(super) fn airship_airplane02_run(ctx: &Ctx, mut step: AirshipAirplane02Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_mob_id = Val::from(0);
    let mut l_mob_list: Vec<Val> = Vec::new();
    let mut l_x: Vec<Val> = Vec::new();
    let mut l_y: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            AirshipAirplane02Step::Start => {
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer25000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are heading to Izlude."),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain: Attention, all passengers."),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer34000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain: We are being approached by a group of unidentified creatures."),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer38000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain: All passengers on deck, please find shelter inside the ship!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Airship Staff#airplane01")])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer48000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(245),
                        Val::from(57),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(247),
                        Val::from(59),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(249),
                        Val::from(52),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(243),
                        Val::from(62),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(239),
                        Val::from(52),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(234),
                        Val::from(56),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(227),
                        Val::from(49),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(233),
                        Val::from(41),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_x, &Val::from(base + 0), Val::from(251), false);
                runtime::local_set(&mut l_x, &Val::from(base + 1), Val::from(245), false);
                runtime::local_set(&mut l_x, &Val::from(base + 2), Val::from(234), false);
                runtime::local_set(&mut l_x, &Val::from(base + 3), Val::from(233), false);
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_y, &Val::from(base + 0), Val::from(47), false);
                runtime::local_set(&mut l_y, &Val::from(base + 1), Val::from(53), false);
                runtime::local_set(&mut l_y, &Val::from(base + 2), Val::from(46), false);
                runtime::local_set(&mut l_y, &Val::from(base + 3), Val::from(58), false);
                l_i = Val::from(0);
                'l1: loop {
                    if !(l_i.clone().number()? < 4) {
                        break 'l1;
                    }
                    'b1: {
                        let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                        if subject2 == 1 {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("airplane_01"),
                                    runtime::local_get(&l_x, &l_i.clone(), false),
                                    runtime::local_get(&l_y, &l_i.clone(), false),
                                    Val::from("Drainliar"),
                                    Val::from(1111),
                                    Val::from(1),
                                    Val::from("Airship#airplane02::OnMyMobDead"),
                                ],
                            )?;
                        } else if subject2 == 2 {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("airplane_01"),
                                    runtime::local_get(&l_x, &l_i.clone(), false),
                                    runtime::local_get(&l_y, &l_i.clone(), false),
                                    Val::from("Rotar Zairo"),
                                    Val::from(1392),
                                    Val::from(1),
                                    Val::from("Airship#airplane02::OnMyMobDead"),
                                ],
                            )?;
                        } else if subject2 == 3 {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("airplane_01"),
                                    runtime::local_get(&l_x, &l_i.clone(), false),
                                    runtime::local_get(&l_y, &l_i.clone(), false),
                                    Val::from("Farmiliar"),
                                    Val::from(1005),
                                    Val::from(1),
                                    Val::from("Airship#airplane02::OnMyMobDead"),
                                ],
                            )?;
                        } else if subject2 == 4 {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("airplane_01"),
                                    runtime::local_get(&l_x, &l_i.clone(), false),
                                    runtime::local_get(&l_y, &l_i.clone(), false),
                                    Val::from("Picky"),
                                    Val::from(1049),
                                    Val::from(1),
                                    Val::from("Airship#airplane02::OnMyMobDead"),
                                ],
                            )?;
                        } else if subject2 == 5 {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("airplane_01"),
                                    runtime::local_get(&l_x, &l_i.clone(), false),
                                    runtime::local_get(&l_y, &l_i.clone(), false),
                                    Val::from("Steel Chonchon"),
                                    Val::from(1042),
                                    Val::from(1),
                                    Val::from("Airship#airplane02::OnMyMobDead"),
                                ],
                            )?;
                        }
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_mob_list, &Val::from(base + 0), Val::from(1111), false);
                runtime::local_set(&mut l_mob_list, &Val::from(base + 1), Val::from(1392), false);
                runtime::local_set(&mut l_mob_list, &Val::from(base + 2), Val::from(1005), false);
                runtime::local_set(&mut l_mob_list, &Val::from(base + 3), Val::from(1049), false);
                runtime::local_set(&mut l_mob_list, &Val::from(base + 4), Val::from(1042), false);
                l_mob_id = runtime::local_get(&l_mob_list, &ctx.call(Function::Rand, vec![Val::from(5)])?, false);
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(243),
                        Val::from(60),
                        Val::from("--ja--"),
                        l_mob_id.clone(),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(228),
                        Val::from(54),
                        Val::from("--ja--"),
                        l_mob_id.clone(),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(232),
                        Val::from(41),
                        Val::from("--ja--"),
                        l_mob_id.clone(),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer48010 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(238),
                        Val::from(56),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(239),
                        Val::from(56),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(240),
                        Val::from(50),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(241),
                        Val::from(56),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(247),
                        Val::from(51),
                        Val::from("Gremlin"),
                        Val::from(1632),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(237),
                        Val::from(44),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(233),
                        Val::from(54),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(237),
                        Val::from(62),
                        Val::from("Beholder"),
                        Val::from(1633),
                        Val::from(1),
                        Val::from("Airship#airplane02::OnCaptainMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer53000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Attendant: Captain Tarlock, we're in trouble! The monsters are heading to the propellers!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer58000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: What?! I've got to stop them!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer63000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Airship Captain#01")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Airship Captain#02")])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer68000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_ANGER")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: You ugly, godforsaken creatures... Get off my ship!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer73000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("*Kzzz...Drrrr...Boom! CRASH!*"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer73500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer74000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer74500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer75000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom10#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer75500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer76000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer76500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer77000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer77500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom10#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer78000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Engineer: Oh no! We've got a problem with the Number One Rear Engine!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer79000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer80000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer81000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer82000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom10#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer83000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Pilot: Hurry! Get the women, old people and children somewhere safe first! Hurry!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer84000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer85000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer86000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer87000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer88000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_ANGER")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: You dirty monsters are dealing with this ship's captain..."),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer93000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_ANGER")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: I'll protect this ship and my crew with my life!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer98000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HNG")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: Here goes! Special Exodus Joker XIII Doom Rifle!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer103000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("*Bang! Bang Bang! Bang Bang! Bang Bang Bang!*"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer103500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom1#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer104000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom2#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer104500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom3#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer105000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom4#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer105500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom5#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer106000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom0#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer106500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom1#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer107000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom2#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer107500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom4#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer108000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom5#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("airplane_01"), Val::from("Airship#airplane02::OnCaptainMobDead")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer113000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HNG")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: Filthy animals! Stop ruining my ship!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer118000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("*Bang! Bang Bang! Bang Bang! Bang Bang Bang!*"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer118500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer119000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer119500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer120000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer120500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom10#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer121000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer121500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer122000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer122500 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_HIT5")?, ctx.constant("AREA")?, Val::from("Airship Captain#02")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer123000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom8#airplane"),
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("*Boom! Boom Boom! Boom Boom! Boom!*"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer124000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom9#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer125000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom10#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer126000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom6#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer127000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SUI_EXPLOSION")?,
                        ctx.constant("AREA")?,
                        Val::from("boom7#airplane"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer128000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_FRET")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: There's... Too many to handle!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer133000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Pilot: Captain, sir, the situation is getting critical!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer138000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SWEAT")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: We'll need all the help we can get!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer143000 => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SORRY")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Airship Captain#02")])?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Captain Tarlock: All hands and any passenger who can fight! We've got to drive away these monsters!"),
                        ctx.constant("BC_MAP")?,
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer148000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Airship Captain#02")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Airship Captain#01")])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer400000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("airplane_01"), Val::from("Airship#airplane02::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer405000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Monster threat eliminated. The Airship is now returning to normal operation."),
                        ctx.constant("BC_MAP")?,
                        Val::from(65280),
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("Airship Staff#airplane01")])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer410000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We will arrive in Izlude shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer420000 => {
                ctx.var("$@airplanelocation2").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnUnhide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnUnhide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Welcome to Izlude. Have a safe trip."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer430000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are currently in Izlude. The Airship will take off shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer440000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnHide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnHide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("The Airship is now taking off. Our next destination is Juno."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer465000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are heading to Juno."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer490000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We will arrive in Juno shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer500000 => {
                ctx.var("$@airplanelocation2").set(Val::from(2))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnUnhide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnUnhide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Welcome to Juno. Have a safe trip."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer510000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are currently in Juno. The Airship will leave shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer520000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnHide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnHide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("The Airship is leaving the ground. Our next destination is Rachel."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF8200"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer545000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are heading to Rachel."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF8200"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer570000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We will arrive in Rachel shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF8200"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer580000 => {
                ctx.var("$@airplanelocation2").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnUnhide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnUnhide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("Welcome to Rachel. Have a safe trip."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF8200"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer590000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("We are currently in Rachel. The Airship will take off shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF8200"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnTimer600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-3::OnHide")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#AirshipWarp-4::OnHide")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("airplane_01"),
                        Val::from("The Airship is now taking off. Our next destination is Izlude."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::SetVariableOfNpc,
                    vec![Val::from(".moninv"), Val::from("International_Airship"), Val::from(0), Val::from(2)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("International_Airship::OnEnable")])?;
                return Err(Stop::End);
            }
            AirshipAirplane02Step::OnMyMobDead => {
                step = AirshipAirplane02Step::OnCaptainMobDead;
                continue 'machine;
            }
            AirshipAirplane02Step::OnCaptainMobDead => {
                return Err(Stop::End);
            }
        }
    }
}
