use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum EinbrochSmogAlertStep {
    Start,
    OnEnable,
    OnMyMobDead,
    OnTimer600000,
}

pub(super) fn einbroch_smog_alert_run(ctx: &Ctx, mut step: EinbrochSmogAlertStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EinbrochSmogAlertStep::Start => {
                return Err(Stop::End);
            }
            EinbrochSmogAlertStep::OnEnable => {
                ctx.var("$@alrdeinpoll").set(Val::from(1))?;
                ctx.call(Function::MapAnnounce, vec![Val::from("einbroch"), Val::from("This is a state of emercency! Harmful smog is reaching high levels of saturation. Residents of Einbroch must find shelter immediately."), ctx.constant("BC_MAP")?])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Einbroch Smog Alert")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Centzu#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Khowropher#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Khetine#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Sleik#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Tollaf#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Keneshiotz#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Khashurantze#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Kesunboss#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Train Station Staff#ein1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Train Station Staff#ein2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Leslie#ein_1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Tan#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Little Toby#ein-1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Airship Engineer#ein-1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Kafra Employee#ein1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Kafra Employee#ein2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Kafra Employee#ein3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Morei#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Mark#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Khemko#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Oberu#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Uwe Kleine#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Flu Mask Dealer#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Paddler#ein")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Laboratory Soldier#ein-1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Laboratory Soldier#ein-2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(82),
                        Val::from(332),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(99),
                        Val::from(328),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(122),
                        Val::from(317),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(138),
                        Val::from(319),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(147),
                        Val::from(312),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(159),
                        Val::from(316),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(173),
                        Val::from(315),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(161),
                        Val::from(311),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(147),
                        Val::from(296),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(168),
                        Val::from(282),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(175),
                        Val::from(271),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(146),
                        Val::from(274),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(160),
                        Val::from(272),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(155),
                        Val::from(256),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(179),
                        Val::from(262),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(192),
                        Val::from(248),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(212),
                        Val::from(255),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(230),
                        Val::from(250),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(246),
                        Val::from(251),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(262),
                        Val::from(254),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(253),
                        Val::from(240),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(202),
                        Val::from(245),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(181),
                        Val::from(251),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(172),
                        Val::from(238),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(146),
                        Val::from(242),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(186),
                        Val::from(226),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(173),
                        Val::from(239),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(124),
                        Val::from(248),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(120),
                        Val::from(234),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(98),
                        Val::from(234),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(101),
                        Val::from(219),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(89),
                        Val::from(208),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(96),
                        Val::from(191),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(76),
                        Val::from(194),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(60),
                        Val::from(196),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(45),
                        Val::from(194),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(34),
                        Val::from(201),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(40),
                        Val::from(184),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(64),
                        Val::from(173),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(96),
                        Val::from(173),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(41),
                        Val::from(155),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(46),
                        Val::from(131),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(46),
                        Val::from(108),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(38),
                        Val::from(93),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(55),
                        Val::from(86),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(81),
                        Val::from(81),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(107),
                        Val::from(82),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(107),
                        Val::from(104),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(123),
                        Val::from(73),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(132),
                        Val::from(87),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(125),
                        Val::from(63),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(142),
                        Val::from(64),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(150),
                        Val::from(52),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(157),
                        Val::from(37),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(179),
                        Val::from(39),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(197),
                        Val::from(46),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(217),
                        Val::from(67),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(246),
                        Val::from(54),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(228),
                        Val::from(110),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(250),
                        Val::from(118),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(273),
                        Val::from(127),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(288),
                        Val::from(138),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(281),
                        Val::from(160),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(281),
                        Val::from(192),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(291),
                        Val::from(201),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(283),
                        Val::from(218),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(268),
                        Val::from(216),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(273),
                        Val::from(196),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(262),
                        Val::from(164),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(241),
                        Val::from(180),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(216),
                        Val::from(205),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(209),
                        Val::from(198),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(224),
                        Val::from(177),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(227),
                        Val::from(163),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(208),
                        Val::from(166),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(132),
                        Val::from(87),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(149),
                        Val::from(119),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(119),
                        Val::from(36),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(84),
                        Val::from(155),
                        Val::from("Toxic Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("einbroch"),
                        Val::from(82),
                        Val::from(107),
                        Val::from("Red Fog"),
                        Val::from(1621),
                        Val::from(1),
                        Val::from("Einbroch Smog Alert::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            EinbrochSmogAlertStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("einbroch"), Val::from("Einbroch Smog Alert::OnMyMobDead")],
                    )?
                    .is_true()
                {
                    return Err(Stop::End);
                }
                step = EinbrochSmogAlertStep::OnTimer600000;
                continue 'machine;
            }
            EinbrochSmogAlertStep::OnTimer600000 => {
                ctx.call(Function::KillMonster, vec![Val::from("einbroch"), Val::from("All")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("einbroch"),
                        Val::from("Emergency status is now cancelled. Air pollution levels are now within the safety zone."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Einbroch Smog Alert")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Liotzburg#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Centzu#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Khowropher#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Khetine#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Sleik#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Tollaf#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Keneshiotz#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Khashurantze#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Kesunboss#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Train Station Staff#ein1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Train Station Staff#ein2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Leslie#ein_1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Tan#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Little Toby#ein-1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Airship Engineer#ein-1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Kafra Employee#ein1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Kafra Employee#ein2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Kafra Employee#ein3")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Morei#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Mark#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Khemko#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Oberu#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Uwe Kleine#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Flu Mask Dealer#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Paddler#ein")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Laboratory Soldier#ein-1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Laboratory Soldier#ein-2")])?;
                ctx.var("$@alrdeinpoll").set(Val::from(0))?;
                ctx.var("$einpolution").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
