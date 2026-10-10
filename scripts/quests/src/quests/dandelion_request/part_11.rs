use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MaoManagerStep {
    Start,
    OnStart,
    OnStop,
    OnTimer300000,
    OnTimer305000,
    OnTimer310000,
    OnTimer315000,
    OnTimer320000,
    OnTimer325000,
    OnTimer327000,
    OnTimer329000,
    OnTimer331000,
    OnTimer333000,
    OnTimer335000,
    OnTimer1800000,
    OnMyMobDead,
}

fn mao_manager_run(ctx: &Ctx, mut step: MaoManagerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaoManagerStep::Start => {
                step = MaoManagerStep::OnStart;
                continue 'machine;
            }
            MaoManagerStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnStop => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("morocc"), Val::from("#mao_manager::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao1::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao4::OnEnter")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("How dare... you... interrupt... the holy ritual..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer305000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("...The seal... no... it won't... it's not..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer310000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("...Accursed humans... interfering with... my revival..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer315000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("...Take this... blessing... of blood... Accept your.... death..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer320000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("...Beg for... my forgiveness... in the afterlife...!"),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer325000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("morocc"),
                        Val::from("???: Morocc Satan's angry! Oh Freya, please protect us from the evil being..."),
                        Val::from(1),
                        Val::from(8087790),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(185),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(185),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(185),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(185),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(185),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(191),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(191),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(191),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(191),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(191),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer327000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(194),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(194),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(194),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(194),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(194),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(197),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(197),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(197),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(197),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(197),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(200),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(200),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(200),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(200),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(200),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer329000 => {
                ctx.call(
                    Function::Announce,
                    vec![
                        Val::from("Kafra Pavianne: This is the Morocc Kafra Branch! The m-monsters are... Aaaaahhh!"),
                        ctx.constant("BC_ALL")?,
                        Val::from(7396315),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(203),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(203),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(203),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(203),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(203),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(206),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(206),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(206),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(206),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(206),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(209),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(209),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(209),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(209),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(209),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer331000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(163),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(161),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(159),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(157),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(155),
                        Val::from(212),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer333000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(138),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(140),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(142),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(146),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(148),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(150),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(152),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(154),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(156),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(158),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(160),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(162),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(164),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(166),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(168),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(170),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(172),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(174),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(176),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(178),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(180),
                        Val::from(162),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(138),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(140),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(142),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(146),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(148),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(150),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(152),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(154),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(156),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(158),
                        Val::from(160),
                        Val::from("Satan's Despair"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(160),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(162),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(162),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(164),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(166),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(168),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(170),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(172),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(174),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(176),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(178),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(180),
                        Val::from(160),
                        Val::from("Satan's Wrath"),
                        Val::from(1154),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer335000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(138),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(140),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(142),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(146),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(148),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(150),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(152),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(154),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(156),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(158),
                        Val::from(158),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(160),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(162),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(164),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(166),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(168),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(170),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(172),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(174),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(176),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(178),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(180),
                        Val::from(158),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Satan's Despair"),
                        Val::from(1041),
                        Val::from(20),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("morocc"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Satan's Wrath"),
                        Val::from(1041),
                        Val::from(20),
                        Val::from("#mao_manager::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnTimer1800000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("morocc"), Val::from("#mao_manager::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao1::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao4::OnEnter")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MaoManagerStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mao_manager(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::Start, Vec::new()).map(|_| ())
}

pub fn mao_manager_onstart(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnStart, Vec::new()).map(|_| ())
}

pub fn mao_manager_onstop(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnStop, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer300000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer305000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer305000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer310000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer310000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer315000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer315000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer320000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer320000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer325000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer325000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer327000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer327000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer329000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer329000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer331000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer331000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer333000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer333000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer335000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer335000, Vec::new()).map(|_| ())
}

pub fn mao_manager_ontimer1800000(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnTimer1800000, Vec::new()).map(|_| ())
}

pub fn mao_manager_onmymobdead(ctx: &Ctx) -> Script {
    mao_manager_run(ctx, MaoManagerStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn simon_mao_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Simon",
        args!["...No. How many", "times must I tell you?", "I'm not going back."],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_SWEAT")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Simon#mao")])?,
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Kimmie", args!["Please...", "We really need", "you back home..."])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_CRY")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Kimmie")])?,
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Simon",
        args![
            "I'm sorry, Kimmie.",
            "I have to follow my",
            "own path. You'll have",
            "to make do without me.",
            "Besides, you're strong",
            "and don't really need me..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kimmie",
        args![
            "Y-you can't do this!",
            "Why did you leave us?",
            "I d-didn't want to tell you",
            "this, but Jimmy's been getting",
            "in trouble with the law and..."
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_KEK")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Kimmie")])?,
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Simon",
        args![
            "Then it's his own",
            "fault. I don't care, I'm",
            "still not going home...",
            "I've got to be responsible",
            "for myself before I can even",
            "think of taking care of others."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Kimmie", args!["Shut up! Shut up!", "Come back to us!"])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "I have no idea what's",
            "going on, but it seems",
            "pretty bad. Sometimes it's",
            "good not to get involved",
            "with other people's problems."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn simon_mao(ctx: &Ctx) -> Script {
    simon_mao_body(ctx, Vec::new()).map(|_| ())
}

fn morocc_invasion_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines(args![
        ((Val::from("A total of ") + ctx.var("$maoattack").get()?) + Val::from(" users completed")),
        "the Satan Morocc: Lin Quest.",
        "There are 2 requirements to",
        "summon Satan Morocc's troops.",
        "1) 50 users must complete",
        "this Satan Morocc: Lin Quest."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "2) One of these users that",
        "completed the Lin Quest must",
        "approach the Dandelion Org",
        "NPCs at the outskirts of Morocc to trigger the monster invasion.",
        "Change the GlobalVar to affect status of Satan Morocc invasion?"
    ])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Set Globalvar"), Val::from("Cancel")])?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args![
                "Please choose the number",
                "of times that the Lin Quest",
                "will be recorded as completed",
                "on the server. 0 is essentially",
                "a reset; 50 will immediately",
                "make the invasion available."
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("49"), Val::from("50"), Val::from("0")])? {
                1 => {
                    ctx.lines(args![
                        "The GlobalVar has been",
                        "set to 49. The Lin Quest",
                        "must be completed 1 more",
                        "time before the Satan Morocc",
                        "invasion can be triggered."
                    ])?;
                    ctx.var("$maoattack").set(Val::from(49))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines(args![
                        "The GlobalVar has been",
                        "set to 50. The Satan Morocc",
                        "invasion can now be triggered."
                    ])?;
                    ctx.var("$maoattack").set(Val::from(50))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines(args![
                        "The GlobalVar has been",
                        "set to 0. The Lin Quest",
                        "must be completed 1 more",
                        "time before the Satan Morocc",
                        "invasion can be triggered."
                    ])?;
                    ctx.var("$maoattack").set(Val::from(0))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("You have canceled.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn morocc_invasion_manager(ctx: &Ctx) -> Script {
    morocc_invasion_manager_body(ctx, Vec::new()).map(|_| ())
}
