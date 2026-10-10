use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum RaceTimer11Step {
    Start,
    OnEnable,
    OnInit,
    OnTimer10000,
    OnTimer30000,
    OnTimer90000,
    OnTimer210000,
    OnTimer270000,
    OnTimer272000,
    OnTimer330000,
}

pub(super) fn race_timer1_1_run(ctx: &Ctx, mut step: RaceTimer11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RaceTimer11Step::Start => {
                step = RaceTimer11Step::OnEnable;
                continue 'machine;
            }
            RaceTimer11Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#race_timer1-1")])?;
                step = RaceTimer11Step::OnInit;
                continue 'machine;
            }
            RaceTimer11Step::OnInit => {
                ctx.var("$@mon_time_1_1").set(Val::from(2))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Single Monster Race will soon begin. We hope to see many of you participate!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Single Monster Race Arena has just opened."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                ctx.var("$@mon_time_1_1").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Race Progress Timer::OnEnable")])?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer90000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Single Monster Race arena is now open. Participants should enter the Arena as soon as they can."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer210000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from(
                            "The entrance to the Single Monster Race Arena will close shortly. Participants, please enter the arena now.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer270000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Single Monster Race Arena's entrance will soon close."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer272000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("Participants, please enter the Arena before the doors close."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer11Step::OnTimer330000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The race is now starting. If you missed your chance to enter this race, please try again next time~!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                ctx.var("$@mon_time_1_1").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("#race_timer1-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RaceProgressTimerStep {
    Start,
    OnEnable,
    OnTimer1000,
    OnTimer7000,
    OnTimer10000,
    OnTimer120000,
    OnTimer123000,
    OnTimer240000,
    OnTimer243000,
    OnTimer300000,
    OnDisable,
    OnInit,
}

pub(super) fn race_progress_timer_run(ctx: &Ctx, mut step: RaceProgressTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_line = Val::from(0);
    let mut l_tired = Val::from(0);
    'machine: loop {
        match step {
            RaceProgressTimerStep::Start => {
                step = RaceProgressTimerStep::OnEnable;
                continue 'machine;
            }
            RaceProgressTimerStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Race Progress Timer")])?;
                l_c = Val::from(1);
                'l1: loop {
                    if !(l_c.clone().number()? < 7) {
                        break 'l1;
                    }
                    'b1: {
                        l_line = ctx.call(Function::Rand, vec![Val::from(1), Val::from(70)])?;
                        ctx.call(Function::EnableNpc, vec![(Val::from("starting#") + l_c.clone())])?;
                        ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#1"))])?;
                        if l_line.clone().number()? <= 10 {
                            l_tired = ctx.call(Function::Rand, vec![Val::from(50), Val::from(60)])?;
                            ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#5"))])?;
                            ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#6"))])?;
                        } else {
                            if l_line.clone().number()? <= 30 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(40), Val::from(60)])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#5"))])?;
                                if l_tired.clone().number()? >= 50 {
                                    ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#2"))])?;
                                }
                            } else if l_line.clone().number()? <= 40 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(30), Val::from(50)])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#1"))])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#2"))])?;
                                if l_tired.clone().number()? < 40 {
                                    ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#3"))])?;
                                }
                            } else if l_line.clone().number()? <= 50 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(20), Val::from(40)])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#1"))])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Luk") + l_c.clone()) + Val::from("#2"))])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#2"))])?;
                                ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#3"))])?;
                                if l_tired.clone().number()? < 30 {
                                    ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#4"))])?;
                                }
                            } else if l_line.clone().number()? <= 60 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(10), Val::from(30)])?;
                                l_i = Val::from(1);
                                'l2: loop {
                                    if !(l_i.clone().number()? <= 3) {
                                        break 'l2;
                                    }
                                    'b2: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Luk") + l_c.clone()) + Val::from("#")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                l_i = Val::from(2);
                                'l3: loop {
                                    if !(l_i.clone().number()? <= 4) {
                                        break 'l3;
                                    }
                                    'b3: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Tire") + l_c.clone()) + Val::from("#")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                if l_tired.clone().number()? < 20 {
                                    ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#5"))])?;
                                }
                            } else if l_line.clone().number()? <= 70 {
                                l_tired = ctx.call(Function::Rand, vec![Val::from(0), Val::from(20)])?;
                                l_i = Val::from(1);
                                'l4: loop {
                                    if !(l_i.clone().number()? <= 4) {
                                        break 'l4;
                                    }
                                    'b4: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Luk") + l_c.clone()) + Val::from("#")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                l_i = Val::from(2);
                                'l5: loop {
                                    if !(l_i.clone().number()? <= 5) {
                                        break 'l5;
                                    }
                                    'b5: {
                                        ctx.call(
                                            Function::EnableNpc,
                                            vec![(((Val::from("Tire") + l_c.clone()) + Val::from("#")) + l_i.clone())],
                                        )?;
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                if l_tired.clone().number()? < 10 {
                                    ctx.call(Function::EnableNpc, vec![((Val::from("Tire") + l_c.clone()) + Val::from("#6"))])?;
                                }
                            }
                        }
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mr_1_luk") + l_c.clone()),
                            l_line.clone(),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mr_1_tire") + l_c.clone()),
                            l_tired.clone(),
                            &mut [
                                (".@c", runtime::LocalMut::Scalar(&mut l_c)),
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@line", runtime::LocalMut::Scalar(&mut l_line)),
                                (".@tired", runtime::LocalMut::Scalar(&mut l_tired)),
                            ],
                        )?;
                    }
                    l_c = (l_c.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer1000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Ticket Helper#single")])?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Welcome to the Monster Race Arena."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Feel free to inquire at the help desk whenever you have questions."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer120000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("The Single Monster Race will start in 3 minutes."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Please ask a Ticket Helper if you wish to wager on the race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer240000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("The Single Monster Race will start shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer243000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Please ask a Ticket Helper if you wish to wager on the race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("The Monster Race has already begun. Good luck to all the participants."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                ctx.var("$@monster_race").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Ticket Helper#single")])?;
                l_i = Val::from(1);
                'l6: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l6;
                    }
                    'b6: {
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![((((Val::from("Runner No. ") + l_i.clone()) + Val::from("#")) + l_i.clone()) + Val::from("::OnEnable"))],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RaceProgressTimerStep::OnDisable => {
                step = RaceProgressTimerStep::OnInit;
                continue 'machine;
            }
            RaceProgressTimerStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Race Progress Timer")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MedalDistributorSingleStep {
    Start,
    OnEnable,
    OnTimer1000,
    OnTimer4000,
    OnTimer7000,
    OnTimer10000,
    OnTimer13000,
    OnTimer240000,
    OnTimer243000,
    OnTimer246000,
    OnTimer249000,
    OnTimer252000,
    OnTimer300000,
    OnTimer342000,
    OnInit,
}

pub(super) fn medal_distributor_single_run(ctx: &Ctx, mut step: MedalDistributorSingleStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_insa = Val::from(0);
    let mut l_j = Val::from(0);
    'machine: loop {
        match step {
            MedalDistributorSingleStep::Start => {
                if !(ctx.call(Function::CheckWeight, vec![Val::from("Spawn"), Val::from(200)])?.is_true()) {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "I'm sorry, but I can't",
                            "reward you with any medals",
                            "until you make more space",
                            "available in your Inventory."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.call(Function::CountItem, vec![Val::from("Monster_Ticket")])?.is_true() {
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Hello there~",
                            "If you've wagered on the",
                            "winning monster in a recent",
                            "race, then you can exchange",
                            "your game ticket here for",
                            "some Prize Medals."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Medal Distributor",
                        args![
                            "Please remember that you can",
                            "only exchange winning Game",
                            "Tickets for Prize Medals right",
                            "after the race finishes. Prize",
                            "Medals may be given to Wayne",
                            "in Hugel in exchange for items."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("$@monster_race").get()?.is_true() {
                        if ctx.var("monster_race_1").get()?.loosely_equals(&ctx.var("$@monster_race").get()?) {
                            ctx.lines_as(
                                "Medal Distributor",
                                args![
                                    "Oh, congratulations, you",
                                    "have a winning ticket for",
                                    "a Single Monster Race! So ",
                                    "would you like to exchange your",
                                    "Racing Ticket for Prize Medals?"
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please.:No, thanks.")])?) == 2 {
                                ctx.lines_as(
                                    "Medal Distributor",
                                    args![
                                        "Um, are you sure? You ",
                                        "can only exchange a winning",
                                        "Racing Ticket for Prize Medals",
                                        "for a short time after the race. If you made a mistake, you",
                                        "should ask me again quickly."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Medal Distributor",
                                args![
                                    "Let me see your ticket...",
                                    "Oh! Congratulations, you",
                                    "won! May I have your name?"
                                ],
                            )?;
                            ctx.next()?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            if l_input_s
                                .clone()
                                .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            {
                                l_insa = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
                                ctx.lines_as(
                                    "Medal Distributor",
                                    args![
                                        ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                        "You can exchange this",
                                        "ticket for a Prize Medal by",
                                        "entering your ticket exchange",
                                        "number now. Your ticket",
                                        ((Val::from("exchange number is ^FF0000") + l_insa.clone()) + Val::from("^000000."))
                                    ],
                                )?;
                                ctx.next()?;
                                let (input, status) = runtime::input_number(ctx, None, None)?;
                                l_input = input;
                                if !(l_input.clone().is_true()) {
                                    ctx.lines_as(
                                        "Medal Distributor",
                                        args!["Oh? You don't want", "to exchange your", "winning ticket?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if l_input.clone().loosely_equals(&l_insa.clone()) {
                                    ctx.lines_as(
                                        "Medal Distributor",
                                        args![
                                            "Thank you! You entered",
                                            "the correct number...",
                                            "Everything seems to",
                                            "be in order. Alright!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Medal Distributor",
                                        args![
                                            "Now please accept your",
                                            "Prize Medals! You can",
                                            "exchange these with",
                                            "Wayne in Hugel for some",
                                            "interesting items. Thank you~"
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
                                    ctx.var("monster_race_1").set(Val::from(0))?;
                                    ctx.call(Function::GetItem, vec![Val::from(7515), Val::from(4)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Medal Distributor",
                                    args![
                                        "I'm sorry, but it seems",
                                        "that you entered the",
                                        "incorrect ticket number.",
                                        "Would you mind coming",
                                        "back again in a while?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Medal Distributor",
                                args![
                                    "What's this...?",
                                    "I think there's a problem...",
                                    "Did you enter your name",
                                    "incorrectly? Please check",
                                    "your name, and then try again."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Medal Distributor",
                            args![
                                "You may not have wagered",
                                "on the winning monster in",
                                "the last race, but I hope that",
                                "you get lucky next time~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Medal Distributor",
                    args![
                        "Hello there~",
                        "If you've wagered on the",
                        "winning monster in a recent",
                        "race, then you can exchange",
                        "your game ticket here for",
                        "some Prize Medals."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Medal Distributor",
                    args![
                        "Please remember that you can",
                        "only exchange winning Game",
                        "Tickets for Prize Medals right",
                        "after the race finishes. Prize",
                        "Medals may be given to Wayne",
                        "in Hugel in exchange for items."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Medal Distributor#single")])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("The Monster Race is finished! Congratulations to all the winners!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Please give your Racing Ticket to the Medal Distributor if you bet on the winning monster."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("You have 5 minutes to exchange a winning ticket for Prize Medals from the Medal Distributor."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("All tickets become void after this 5 minute period, so winners should claim their prize now."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Please leave the Race Arena before this 5 minute period elapses. Thank you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer240000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Attention. We will being preparing for the next race shortly..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer243000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("We will close the Racing Arena in 1 minute to prepare for the next race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer246000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Participants in the last race should leave the arena as soon as possible."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer249000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Thank you for your cooperation."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer252000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("We hope that you enjoyed the Monster Race arena. Come back again soon~"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.var("$@mr_1_tire1").set(Val::from(0))?;
                ctx.var("$@mr_1_luk1").set(ctx.var("$@mr_1_tire1").get()?)?;
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l1;
                    }
                    'b1: {
                        l_j = Val::from(1);
                        'l2: loop {
                            if !(l_j.clone().number()? <= 6) {
                                break 'l2;
                            }
                            'b2: {
                                ctx.call(
                                    Function::DisableNpc,
                                    vec![(((Val::from("Tire") + l_i.clone()) + Val::from("#")) + l_j.clone())],
                                )?;
                                ctx.call(
                                    Function::DisableNpc,
                                    vec![(((Val::from("Luk") + l_i.clone()) + Val::from("#")) + l_j.clone())],
                                )?;
                            }
                            l_j = (l_j.clone() + Val::from(1));
                        }
                        ctx.call(Function::EnableNpc, vec![(Val::from("Luk1#") + l_i.clone())])?;
                        ctx.call(Function::EnableNpc, vec![(Val::from("Tire1#") + l_i.clone())])?;
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mr_1_luk") + l_i.clone()),
                            Val::from(0),
                            &mut [
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@input", runtime::LocalMut::Scalar(&mut l_input)),
                                (".@input$", runtime::LocalMut::Scalar(&mut l_input_s)),
                                (".@insa", runtime::LocalMut::Scalar(&mut l_insa)),
                                (".@j", runtime::LocalMut::Scalar(&mut l_j)),
                            ],
                        )?;
                        runtime::setd(
                            ctx,
                            &(Val::from("$@mr_1_tire") + l_i.clone()),
                            Val::from(0),
                            &mut [
                                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                (".@input", runtime::LocalMut::Scalar(&mut l_input)),
                                (".@input$", runtime::LocalMut::Scalar(&mut l_input_s)),
                                (".@insa", runtime::LocalMut::Scalar(&mut l_insa)),
                                (".@j", runtime::LocalMut::Scalar(&mut l_j)),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer300000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#race_timer1-1::OnEnable")])?;
                ctx.var("$@monster_race").set(Val::from(0))?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("p_track01"), Val::from("hugel"), Val::from(63), Val::from(73)],
                )?;
                return Err(Stop::End);
            }
            MedalDistributorSingleStep::OnTimer342000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = MedalDistributorSingleStep::OnInit;
                continue 'machine;
            }
            MedalDistributorSingleStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Medal Distributor#single")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RaceTimer21Step {
    Start,
    OnEnable,
    OnInit,
    OnTimer10000,
    OnTimer30000,
    OnTimer90000,
    OnTimer210000,
    OnTimer270000,
    OnTimer272000,
    OnTimer330000,
}

pub(super) fn race_timer2_1_run(ctx: &Ctx, mut step: RaceTimer21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RaceTimer21Step::Start => {
                step = RaceTimer21Step::OnEnable;
                continue 'machine;
            }
            RaceTimer21Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#race_timer2-1")])?;
                step = RaceTimer21Step::OnInit;
                continue 'machine;
            }
            RaceTimer21Step::OnInit => {
                ctx.var("$@mon_time_2_1").set(Val::from(2))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Dual Monster Race will soon begin. We hope to see many of you participate!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Dual Monster Race Arena has just opened."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                ctx.var("$@mon_time_2_1").set(Val::from(1))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#race_timer2-2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("TrapGlobal#race02::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Ticket Helper#2")])?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer90000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Dual Monster Race arena is now open. Participants should enter the Arena as soon as they can."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer210000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from(
                            "The entrance to the Dual Monster Race Arena will close shortly. Participants, please enter the arena now.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer270000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The Dual Monster Race Arena's entrance will soon close."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer272000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("Participants, please enter the Arena before the doors close."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer21Step::OnTimer330000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("hugel"),
                        Val::from("The race is now starting. If you missed your chance to enter this race, please try again next time~!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xffb6c1"),
                    ],
                )?;
                ctx.var("$@mon_time_2_1").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("#race_timer2-1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RaceTimer22Step {
    Start,
    OnEnable,
    OnTimer5000,
    OnTimer7000,
    OnTimer120000,
    OnTimer122000,
    OnTimer240000,
    OnTimer242000,
    OnTimer300000,
    OnInit,
}

pub(super) fn race_timer2_2_run(ctx: &Ctx, mut step: RaceTimer22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RaceTimer22Step::Start => {
                step = RaceTimer22Step::OnEnable;
                continue 'machine;
            }
            RaceTimer22Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#race_timer2-2")])?;
                ctx.var("$@mon_time_2_2").set(Val::from(0))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Welcome to the Monster Race Arena."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Feel free to inquire at the help desk whenever you have questions."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer120000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("The Dual Monster Race will start in 3 minutes."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer122000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Please ask a Ticket Helper if you wish to wager on the race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer240000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("The Dual Monster Race will start shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer242000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Please ask a Ticket Helper if you wish to wager on the race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("The Monster Race is starting now. Good luck, everybody!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x87ceeb"),
                    ],
                )?;
                ctx.var("$@mon_time_2_2").set(Val::from(1))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Ticket Helper#2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#poring1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#lunatic1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#savagebebe1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#desertwolf1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#deviruchi1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#baphomet1::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#race_timer2-2")])?;
                return Err(Stop::End);
            }
            RaceTimer22Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#race_timer2-2")])?;
                ctx.var("$@mon_time_2_2").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RaceTimer23Step {
    Start,
    OnEnable,
    OnTimer3000,
    OnTimer6000,
    OnTimer9000,
    OnTimer12000,
    OnTimer15000,
    OnTimer240000,
    OnTimer243000,
    OnTimer246000,
    OnTimer249000,
    OnTimer252000,
    OnTimer300000,
    OnInit,
}

pub(super) fn race_timer2_3_run(ctx: &Ctx, mut step: RaceTimer23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RaceTimer23Step::Start => {
                step = RaceTimer23Step::OnEnable;
                continue 'machine;
            }
            RaceTimer23Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#race_timer2-3")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("The Monster Race is finished! Congratulations to all the winners!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Please give your Racing Ticket to the Medal Distributor if you bet on the winning monster."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer9000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("You have 5 minutes to exchange a winning ticket for Prize Medals from the Medal Distributor."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("All tickets become void after this 5 minute period, so winners should claim their prize now."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Please leave the Race Arena before this 5 minute period elapses. Thank you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer240000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Attention. We will being preparing for the next race shortly..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer243000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("We will close the Racing Arena in 1 minute to prepare for the next race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer246000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Participants in the last race should leave the arena as soon as possible."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer249000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("Thank you for your cooperation."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer252000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track02"),
                        Val::from("We hope that you enjoyed the Monster Race arena. Come back again soon~"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RaceTimer23Step::OnTimer300000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("p_track02"), Val::from("hugel"), Val::from(63), Val::from(73)],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Medal Distributor#medal")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#race_timer2-1::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = RaceTimer23Step::OnInit;
                continue 'machine;
            }
            RaceTimer23Step::OnInit => {
                ctx.var("$@mon_race_2_2").set(Val::from(0))?;
                ctx.var("$@mon_race_2_1").set(ctx.var("$@mon_race_2_2").get()?)?;
                ctx.call(Function::DisableNpc, vec![Val::from("#race_timer2-3")])?;
                return Err(Stop::End);
            }
        }
    }
}
