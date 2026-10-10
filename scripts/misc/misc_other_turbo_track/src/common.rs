use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MasterTtMainStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer7000,
    OnTimer9000,
    OnTimer11000,
    OnTimer13000,
    OnTimer15000,
    OnTimer17000,
    OnTimer18000,
    OnTimer19000,
    OnTimer20000,
    OnTimer21000,
    OnTimer22000,
    OnTimer23000,
    OnTimer30000,
    OnTimer83000,
    OnTimer143000,
    OnTimer203000,
    OnTimer263000,
    OnTimer323000,
    OnTimer383000,
    OnTimer443000,
    OnTimer503000,
    OnTimer563000,
    OnTimer623000,
    OnTimer683000,
    OnTimer743000,
    OnTimer803000,
    OnTimer863000,
    OnTimer893000,
    OnTimer903000,
    OnTimer913000,
    OnTimer918000,
    OnTimer919000,
    OnTimer920000,
    OnTimer921000,
    OnTimer922000,
    OnTimer923000,
    OnTimer925000,
    OnTimer927000,
    OnInit,
    RName,
    AfterRName,
}

pub(super) fn master_tt_main_run(ctx: &Ctx, mut step: MasterTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rn_s = Val::from("");
    let mut l_s = Val::from(0);
    let mut l_string_s = Val::from("");
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            MasterTtMainStep::Start => {
                step = MasterTtMainStep::OnEnable;
                continue 'machine;
            }
            MasterTtMainStep::OnEnable => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Master#") + l_w_s.clone())])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnDisable => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Master#") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("Welcome to the Turbo Track."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer9000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("The game will be hosted for 15 minutes and at least one person must complete the entire course."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("We hope you will do your best."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer13000 => {
                ctx.call(Function::MapAnnounce, vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from("The game will begin after a 5 second countdown. Everyone, please take your positions behind the Starting Line."), ctx.constant("BC_MAP")?, Val::from("0x33FF66")])?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("The countdown will commence shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer17000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 5 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer18000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 4 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer19000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 3 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer20000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 2 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer21000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 1 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer22000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("- 0 -"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer23000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("Now! The race has begun! Go Go Go!!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-1"))],
                )?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-2"))],
                )?;
                if ctx.call(Function::StrNpcInfo, vec![Val::from(4)])? == "turbo_n_1" {
                    ctx.var("$@start_time").set(ctx.call(Function::GetTimeTick, vec![Val::from(0)])?)?;
                }
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from(
                            "Remember that this is a 15 minute race. After 15 minutes, everyone will be transported out of the race track.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer83000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 14 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer143000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 13 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer203000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 12 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer263000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 11 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer323000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 10 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer383000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 9 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer443000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 8 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer503000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 7 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer563000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 6 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer623000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 5 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer683000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 4 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer743000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 3 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer803000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 2 minutes left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer863000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 1 minute left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer893000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 30 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer903000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 20 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer913000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 10 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer918000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 5 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer919000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 4 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer920000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 3 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer921000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 2 seconds left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer922000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("You have 1 second left."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer923000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("Time's up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer925000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("The race is over."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnTimer927000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("[Everyone will be transported to a Waiting Room.]"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("turbo_room"),
                        Val::from(71),
                        Val::from(89),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Broadcast#") + l_w_s.clone())])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("Master#") + l_w_s.clone()) + Val::from("::OnDisable"))],
                )?;
                if ctx.call(Function::StrNpcInfo, vec![Val::from(4)])? == "turbo_n_1" {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Solo Mode#n1::OnEnable")])?;
                } else {
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(master_tt_main_run(ctx, MasterTtMainStep::RName, vec![l_w_s.clone()])? + Val::from("::OnEnable"))],
                    )?;
                }
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-1"))],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-2"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("snake#") + l_w_s.clone()) + Val::from("::OnReset"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("hunting#") + l_w_s.clone()) + Val::from("::OnReset"))],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("bing2#") + l_w_s.clone())])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Winner Helper#TBT_") + l_w_s.clone())])?;
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
                )?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Disposable_Switch#") + l_w_s.clone())])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_1#") + l_w_s.clone())])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_2#") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            MasterTtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Master#") + l_w_s.clone())])?;
                return Err(Stop::End);
                step = MasterTtMainStep::AfterRName;
                continue 'machine;
            }
            MasterTtMainStep::RName => {
                l_string_s = runtime::arg(&args, 0, Val::from(0));
                l_s = (if runtime::strlen(&l_string_s.clone()).number()? > 2 {
                    runtime::substr(&l_string_s.clone(), &Val::from(1), &Val::from(2))?
                } else {
                    runtime::charat(
                        &l_string_s.clone(),
                        &(runtime::strlen(&l_string_s.clone()).try_sub(Val::from(1))?),
                    )?
                });
                l_rn_s = ((((if runtime::compare(
                    &ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                    &(Val::from("_e_") + l_s.clone()),
                )
                .is_true()
                {
                    Val::from("Expert mode")
                } else {
                    Val::from("Normal mode")
                }) + Val::from(" - "))
                    + l_s.clone())
                    + Val::from(" person"));
                return Ok(l_rn_s.clone());
                return Ok(Val::from(0));
            }
            MasterTtMainStep::AfterRName => {
                return Ok(Val::from(0));
            }
        }
    }
}
