use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn race_timer1_1(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::Start, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_onenable(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_oninit(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnInit, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer10000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer30000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer90000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer90000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer210000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer210000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer270000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer270000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer272000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer272000, Vec::new()).map(|_| ())
}

pub fn race_timer1_1_ontimer330000(ctx: &Ctx) -> Script {
    race_timer1_1_run(ctx, RaceTimer11Step::OnTimer330000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_onenable(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer1000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer7000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer10000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer120000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer123000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer240000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer243000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer243000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ontimer300000(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_ondisable(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn race_progress_timer_oninit(ctx: &Ctx) -> Script {
    race_progress_timer_run(ctx, RaceProgressTimerStep::OnInit, Vec::new()).map(|_| ())
}

fn ticket_helper_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_m = Val::from(0);
    ctx.lines_as(
        "Ticket Helper",
        args![
            "Welcome to the",
            "Monster Race Arena.",
            "If you'd like to participate",
            "in the ^3131FFSingle Monster Race^000000,",
            "then please select 1 out of",
            "the 6 monsters from the list."
        ],
    )?;
    ctx.next()?;
    if !(ctx.call(Function::CheckWeight, vec![Val::from("Spawn"), Val::from(200)])?.is_true()) {
        ctx.lines_as(
            "Ticket Helper",
            args![
                "Wait, wait...",
                "I can't give you",
                "anything right now.",
                "You're carrying way",
                "too many things..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_m = (Val::from(runtime::select_values(
        ctx,
        &[
            Val::from("Monster Status"),
            Val::from("Monster 1"),
            Val::from("Monster 2"),
            Val::from("Monster 3"),
            Val::from("Monster 4"),
            Val::from("Monster 5"),
            Val::from("Monster 6"),
        ],
    )?)
    .try_sub(Val::from(1))?);
    if l_m.clone() == 0 {
        l_i = Val::from(1);
        'l1: loop {
            if !(l_i.clone().number()? <= 6) {
                break 'l1;
            }
            'b1: {
                ctx.lines(args![
                    ((((((Val::from("Monster ") + l_i.clone()) + Val::from(" [^CC6600Luck^000000: "))
                        + runtime::getd(
                            ctx,
                            &(Val::from("$@mr_1_luk") + l_i.clone()),
                            &[(".@i", runtime::Local::Scalar(&l_i)), (".@m", runtime::Local::Scalar(&l_m))]
                        )?)
                        + Val::from("] [^EE0000HP^000000: "))
                        + runtime::getd(
                            ctx,
                            &(Val::from("$@mr_1_tire") + l_i.clone()),
                            &[(".@i", runtime::Local::Scalar(&l_i)), (".@m", runtime::Local::Scalar(&l_m))]
                        )?)
                        + Val::from("]"))
                ])?;
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("$@mon_time_1_1").get()? == 1 {
            if ctx.var("monster_race_1").get()?.is_true() {
                ctx.lines_as(
                    "Ticket Helper",
                    args![
                        "You have selected",
                        ((Val::from("Monster ^FF0000") + ctx.var("monster_race_1").get()?) + Val::from("^000000 for the")),
                        "Single Monster Race.",
                        "The start of the race",
                        "will be announced soon,",
                        "so please wait. Thank you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Ticket Helper",
                args![
                    ((Val::from("You've chosen Monster ") + l_m.clone()) + Val::from("?")),
                    "Alright then, please wait",
                    "until the start of the race is",
                    "announced. If the monster you",
                    "picked wins, then please use this ticket to redeem your prize."
                ],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(7514), Val::from(1)])?;
            ctx.var("monster_race_1").set(l_m.clone())?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Ticket Helper",
            args![
                "I'm sorry, but a Monster",
                "Race is now in progress.",
                "If you'd like to participate, then please wait for the next race."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ticket_helper_single(ctx: &Ctx) -> Script {
    ticket_helper_single_body(ctx, Vec::new()).map(|_| ())
}

fn ticket_helper_single_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Ticket Helper#single")])?;
    return Err(Stop::End);
}

pub fn ticket_helper_single_oninit(ctx: &Ctx) -> Script {
    ticket_helper_single_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RunnerMainStep {
    Start,
    OnEnable,
    OnDisable,
    OnTouchNPC,
    OnInit,
}

fn runner_main_run(ctx: &Ctx, mut step: RunnerMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_mob: Vec<Val> = Vec::new();
    let mut l_n = Val::from(0);
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    'machine: loop {
        match step {
            RunnerMainStep::Start => {
                step = RunnerMainStep::OnEnable;
                continue 'machine;
            }
            RunnerMainStep::OnEnable => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.call(Function::EnableNpc, vec![])?;
                let position = ctx
                    .call(Function::GetMapXy, vec![ctx.constant("BL_NPC")?])?
                    .into_array()
                    .ok_or_else(|| Stop::Error("Invalid position".into()))?;
                l_m_s = position[0].clone();
                l_x = position[1].clone();
                l_y = position[2].clone();
                let base = Val::from(1).number()?;
                runtime::local_set(&mut l_mob, &Val::from(base + 0), Val::from(1725), false);
                runtime::local_set(&mut l_mob, &Val::from(base + 1), Val::from(1726), false);
                runtime::local_set(&mut l_mob, &Val::from(base + 2), Val::from(1727), false);
                runtime::local_set(&mut l_mob, &Val::from(base + 3), Val::from(1728), false);
                runtime::local_set(&mut l_mob, &Val::from(base + 4), Val::from(1730), false);
                runtime::local_set(&mut l_mob, &Val::from(base + 5), Val::from(1729), false);
                l_n = runtime::atoi(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?);
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("p_track01"),
                        Val::from(58),
                        l_y.clone(),
                        ((Val::from("The ") + shared::other_global_functions::f_getnumsuffix(ctx, vec![l_n.clone()])?)
                            + Val::from(" Racer")),
                        runtime::local_get(&l_mob, &l_n.clone(), false),
                        Val::from(1),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            RunnerMainStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        Val::from("p_track01"),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            RunnerMainStep::OnTouchNPC => {
                l_n = runtime::atoi(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?);
                ctx.var("$@monster_race").set(l_n.clone())?;
                l_i = Val::from(1);
                'l1: loop {
                    if !(l_i.clone().number()? <= 6) {
                        break 'l1;
                    }
                    'b1: {
                        if l_n.clone().loosely_equals(&l_i.clone()) {
                            break 'b1;
                        }
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![((((Val::from("Runner No. ") + l_i.clone()) + Val::from("#")) + l_i.clone()) + Val::from("::OnDisable"))],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.call(Function::Sleep, vec![Val::from(1000)])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("We have a winner...!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(Function::Sleep, vec![Val::from(1000)])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        ((Val::from("Monster ") + l_n.clone()) + Val::from(" is the winner of this race!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(Function::Sleep, vec![Val::from(4000)])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        ((Val::from("If you wagered on Monster ") + l_n.clone())
                            + Val::from(" in this race, talk to the Medal Distributor to receive your prize!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(Function::Sleep, vec![Val::from(2000)])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("p_track01"),
                        Val::from("Please remember that we can distribute Prize Medals for only 5 minutes after each race."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                ctx.call(Function::Sleep, vec![Val::from(1000)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Medal Distributor#single::OnEnable")])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        Val::from("p_track01"),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            RunnerMainStep::OnInit => {
                if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? != "" {
                    ctx.call(Function::DisableNpc, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn runner_main(ctx: &Ctx) -> Script {
    runner_main_run(ctx, RunnerMainStep::Start, Vec::new()).map(|_| ())
}

pub fn runner_main_onenable(ctx: &Ctx) -> Script {
    runner_main_run(ctx, RunnerMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn runner_main_ondisable(ctx: &Ctx) -> Script {
    runner_main_run(ctx, RunnerMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn runner_main_ontouchnpc(ctx: &Ctx) -> Script {
    runner_main_run(ctx, RunnerMainStep::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn runner_main_oninit(ctx: &Ctx) -> Script {
    runner_main_run(ctx, RunnerMainStep::OnInit, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::Start, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_onenable(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer1000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer4000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer7000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer10000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer13000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer13000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer240000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer243000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer243000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer246000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer246000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer249000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer249000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer252000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer252000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer300000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_ontimer342000(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnTimer342000, Vec::new()).map(|_| ())
}

pub fn medal_distributor_single_oninit(ctx: &Ctx) -> Script {
    medal_distributor_single_run(ctx, MedalDistributorSingleStep::OnInit, Vec::new()).map(|_| ())
}

fn exit_guide_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Exit Guide",
        args![
            "If you have a winning Racing",
            "Ticket, please make sure that",
            "you redeem it for Prize Medals",
            "now. All Racing Tickets become",
            "void once the next race begins."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Exit Guide",
        args![
            "If you wish to leave",
            "the arena, then I can guide",
            "you outside. Would you like",
            "to leave the arena right now?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 2 {
        ctx.lines_as(
            "Exit Guide",
            args!["Alright, then.", "Just let me know", "whenever you're", "ready to leave."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.call(Function::CountItem, vec![Val::from(7514)])?.is_true()) {
        ctx.lines_as(
            "Exit Guide",
            args![
                "Thank you for",
                "your patronage, and",
                "I hope that you come",
                "visit us again soon~"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Exit Guide",
            args![
                "In accordance with our",
                "policies, I must take your",
                "Racing Ticket before you leave.",
                "Thank you for your patronage,",
                "and I hope you enjoy your time",
                "here in the Monster Race Arena."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
    }
    ctx.var("monster_race_1").set(Val::from(0))?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(63), Val::from(73)])?;
    return Err(Stop::End);
}

pub fn exit_guide_single(ctx: &Ctx) -> Script {
    exit_guide_single_body(ctx, Vec::new()).map(|_| ())
}

fn eckar_ellebird_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Eckar Ellebird",
        args![
            "Welcome to the biggest",
            "attraction in Hugel, the",
            "Monster Race Arena.",
            "How may I help you today?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Monster Race Info:Enter Monster Race")],
    )?) == 1
    {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Monster Races originated from",
                "simple children's games in which",
                "Cute Pets would race against each other. This grew into an adult",
                "pastime that is so popular, we've built a racing arena in Hugel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Our Monster Race Arena hosts",
                "two types of monster races. First, we have the Single Monster Race,",
                "in which those that wagered on the 1st place monster are rewarded."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Then, we have the Dual Monster",
                "Race in which the house odds and rewards are greater than in Single",
                "Monster Races: you must wager on 2 monsters, and they must place in",
                "1st and 2nd for you to win."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Although a small entrance",
                "fee is required, we only use",
                "the money to give rewards to",
                "participants and maintain this",
                "arena. Therefore, we're not",
                "profiting from this enterprise."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Also, we prohibit others",
                "from making personal bets",
                "and wagers, using items and",
                "zeny, based on the outcomes",
                "of these races. That kind of",
                "gambling is illegal here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Once you enter the Race Arena, you will receive a Racing Ticket.",
                "Keep in mind that winning Racing Tickets can only be exchanged for",
                "Prize Medals during a 5 minute window after the end of the race."
            ],
        )?;
        ctx.next()?;
    }
    ctx.lines_as(
        "Eckar Ellebird",
        args![
            "The entrance fee for all races",
            "in the Monster Race Arena is",
            "2,000 zeny. If you'd like to wager on a Dual Monster Race, then",
            "please ask my brother Erenes,",
            "and he will help you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eckar Ellebird",
        args![
            "Otherwise, I'll help get you",
            "started if you're interested",
            "in a Single Monster Race.",
            "Would you like wager on",
            "a Single Monster Race?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please.:No, thanks.")])?) == 2 {
        ctx.lines_as(
            "Eckar Ellebird",
            args!["Very well. I hope that", "you enjoy your time here", "in the Monster Race Arena~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.call(Function::CheckWeight, vec![Val::from("Spawn"), Val::from(700)])?.is_true()) {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Oh, wow. You're carrying",
                "an awful lot of stuff... Yeah,",
                "you better put some of it away",
                "in Kafra Storage or something."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("Zeny").get()?.number()? < 2000 {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "I'm sorry, but you",
                "don't have enough",
                "money to pay the",
                "2,000 zeny entrance fee. "
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from("Monster_Ticket")])?.is_true() {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Hm? What are you doing",
                "with an expired Racing Ticket?",
                "Well, I better get rid of it for you before it can get mixed up",
                "with your new Racing Ticket."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
        ctx.next()?;
        if ctx.var("$@mon_time_1_1").get()? == 1 {
            if ctx.var("Zeny").get()?.number()? < 2000 {
                ctx.lines_as(
                    "Eckar Ellebird",
                    args![
                        "I'm sorry, but you",
                        "don't have enough",
                        "money to pay the",
                        "2,000 zeny entrance fee. "
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Eckar Ellebird",
                args![
                    "Alright, I think you",
                    "should be all set. I hope",
                    "that you enjoy the race~",
                    "Let me guide you inside",
                    "the Monster Race Arena now."
                ],
            )?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
            ctx.var("monster_race_1").set(Val::from(0))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("p_track01"), Val::from(75), Val::from(41)])?;
            return Err(Stop::End);
        }
    } else if ctx.var("$@mon_time_1_1").get()? == 1 {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Thanks, I hope that",
                "you enjoy this race.",
                "Let me guide you now",
                "to the Monster Race Arena."
            ],
        )?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
        ctx.var("monster_race_1").set(Val::from(0))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("p_track01"), Val::from(75), Val::from(41)])?;
        return Err(Stop::End);
    }
    if ctx.var("$@mon_time_1_1").get()? == 2 {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "We're still finishing our",
                "preparations for the next",
                "Single Monster Race, so",
                "we ask that you please",
                "wait a little while longer..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$@monster_race").get()? == 0 {
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "Right now, a Monster Race",
                "is in progress. It's too late to place a wager, but if you'd like",
                "to watch, the fee is 500 zeny",
                "for spectators. Would you like to enter the Monster Race Arena?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Enter:Cancel")])?) == 2 {
            ctx.lines_as(
                "Eckar Ellebird",
                args![
                    "Alright, then. If you'd like",
                    "to wager on a monster",
                    "race, please wait for the",
                    "current race to finish. I hope",
                    "that you enjoy your time here",
                    "in the Monster Race Arena~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("Zeny").get()?.number()? > 499 {
            ctx.lines_as("Eckar Ellebird", args!["Thank you~", "I hope you enjoy", "watching this race!"])?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
            ctx.var("monster_race_1").set(Val::from(0))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("p_track01"), Val::from(75), Val::from(41)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Eckar Ellebird",
            args![
                "I'm sorry, but you don't",
                "have enough money to pay",
                "the 500 zeny spectator fee."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Eckar Ellebird",
        args![
            "I'm sorry, but a monster",
            "race has just ended, so we're",
            "having the 5 minute period in",
            "which the winners can claim",
            "their Prize Medals. The gate",
            "will open soon, so please wait."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn eckar_ellebird_single(ctx: &Ctx) -> Script {
    eckar_ellebird_single_body(ctx, Vec::new()).map(|_| ())
}

fn eckar_ellebird_single_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Eckar Ellebird#single")])?;
    return Err(Stop::End);
}

pub fn eckar_ellebird_single_onenable(ctx: &Ctx) -> Script {
    eckar_ellebird_single_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn eckar_ellebird_single_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Eckar Ellebird#single")])?;
    return Err(Stop::End);
}

pub fn eckar_ellebird_single_ondisable(ctx: &Ctx) -> Script {
    eckar_ellebird_single_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn game_guide_single_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Game Guide",
        args!["Welcome to the", "Monster Race Arena.", "How can I help you?"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Monster Race Info:Wager Info:Ticket Redemption Info")])? {
        1 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "Monster Races originated from",
                    "simple children's games in which",
                    "Cute Pets would race against each other. This grew into an adult",
                    "pastime that is so popular, we've built a racing arena in Hugel."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Our Monster Race Arena hosts",
                    "two types of monster races. First, we have the Single Monster Race,",
                    "in which those that wagered on the 1st place monster are rewarded."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Then, we have the Dual Monster",
                    "Race in which the house odds and rewards are greater than in Single",
                    "Monster Races: you must wager on 2 monsters, and they must place in",
                    "1st and 2nd for you to win."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Although a small entrance",
                    "fee is required, we only use",
                    "the money to give rewards to",
                    "participants and maintain this",
                    "arena. Therefore, we're not",
                    "profiting from this enterprise."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Also, we prohibit others",
                    "from making personal bets",
                    "and wagers, using items and",
                    "zeny, based on the outcomes",
                    "of these races. That kind of",
                    "gambling is illegal here..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "Once you enter the Race Arena, you will receive a Racing Ticket.",
                    "Keep in mind that winning Racing Tickets can only be exchanged for",
                    "Prize Medals during a 5 minute window after the end of the race."
                ],
            )?;
            ctx.next()?;
        }
        2 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "Before placing a wager, you",
                    "must get a free Racing Ticket",
                    "from the Ticket Helper. There,",
                    "I've marked the Ticket Helper",
                    "on your Mini-Map, so you can",
                    "find him pretty easily."
                ],
            )?;
            ctx.call(
                Function::ViewPoint,
                vec![Val::from(1), Val::from(73), Val::from(22), Val::from(1), Val::from(16724821)],
            )?;
            ctx.next()?;
        }
        3 => {
            ctx.lines_as(
                "Game Guide",
                args![
                    "If you wagered on the winner",
                    "of a Single Monster Race, or",
                    "on the 1st or 2nd place winners",
                    "in a Dual Monster Race, then",
                    "you can exchange your Racing",
                    "Ticket for Prize Medals."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "However, you must exchange",
                    "your Racing Ticket with the",
                    "Medal Distributor within the",
                    "5 minute window after the end",
                    "of the race. ^FF0000Your ticket becomes^FFFFFF ^FF0000 void after these 5 minutes.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "When this 5 minute window",
                    "elapses, you will be teleported outside, and we will immediately",
                    "begin preparing for the next race. Make sure that you remember this",
                    "information when you wager."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Game Guide",
                args![
                    "If you haven't received",
                    "your free Racing Ticket,",
                    "then please visit the Ticket",
                    "Helper. There, I've just marked",
                    "his location on your Mini-Map."
                ],
            )?;
            ctx.call(
                Function::ViewPoint,
                vec![Val::from(1), Val::from(67), Val::from(45), Val::from(2), Val::from(13525760)],
            )?;
            ctx.next()?;
        }
        _ => {}
    }
    ctx.lines_as(
        "Game Guide",
        args![
            "Thank you, and",
            "I hope you enjoy",
            "your time here in the",
            "Monster Racing Arena."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn game_guide_single(ctx: &Ctx) -> Script {
    game_guide_single_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Starting1Step {
    Start,
    OnTouchNPC,
    OnInit,
}

fn starting_1_run(ctx: &Ctx, mut step: Starting1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_speed = Val::from(0);
    let mut l_start = Val::from(0);
    'machine: loop {
        match step {
            Starting1Step::Start => {
                step = Starting1Step::OnTouchNPC;
                continue 'machine;
            }
            Starting1Step::OnTouchNPC => {
                l_start = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if l_start.clone().number()? < 11 {
                    l_speed = Val::from(60);
                } else {
                    if l_start.clone().number()? < 21 {
                        l_speed = Val::from(70);
                    } else {
                        if l_start.clone().number()? < 31 {
                            l_speed = Val::from(80);
                        } else {
                            if l_start.clone().number()? < 41 {
                                l_speed = Val::from(90);
                            } else {
                                if l_start.clone().number()? < 51 {
                                    l_speed = Val::from(100);
                                } else if l_start.clone().number()? < 61 {
                                    l_speed = Val::from(110);
                                } else if l_start.clone().number()? < 71 {
                                    l_speed = Val::from(120);
                                } else if l_start.clone().number()? < 81 {
                                    l_speed = Val::from(130);
                                } else if l_start.clone().number()? < 91 {
                                    l_speed = Val::from(140);
                                } else {
                                    l_speed = Val::from(150);
                                }
                            }
                        }
                    }
                }
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_WALKSPEED")?, Val::from(5000), l_speed.clone()],
                )?;
                return Err(Stop::End);
            }
            Starting1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn starting_1(ctx: &Ctx) -> Script {
    starting_1_run(ctx, Starting1Step::Start, Vec::new()).map(|_| ())
}

pub fn starting_1_ontouchnpc(ctx: &Ctx) -> Script {
    starting_1_run(ctx, Starting1Step::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn starting_1_oninit(ctx: &Ctx) -> Script {
    starting_1_run(ctx, Starting1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Luk1Step {
    Start,
    OnTouchNPC,
    OnInit,
}

fn luk_1_run(ctx: &Ctx, mut step: Luk1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_speed = Val::from(0);
    let mut l_start = Val::from(0);
    'machine: loop {
        match step {
            Luk1Step::Start => {
                step = Luk1Step::OnTouchNPC;
                continue 'machine;
            }
            Luk1Step::OnTouchNPC => {
                l_start = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if l_start.clone().number()? < 61 {
                    l_speed = Val::from(110);
                } else if l_start.clone().number()? < 71 {
                    l_speed = Val::from(120);
                } else if l_start.clone().number()? < 81 {
                    l_speed = Val::from(130);
                } else if l_start.clone().number()? < 91 {
                    l_speed = Val::from(140);
                } else {
                    l_speed = Val::from(150);
                }
                ctx.call(
                    Function::StartStatus,
                    vec![ctx.constant("SC_WALKSPEED")?, Val::from(10000), l_speed.clone()],
                )?;
                return Err(Stop::End);
            }
            Luk1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn luk_1(ctx: &Ctx) -> Script {
    luk_1_run(ctx, Luk1Step::Start, Vec::new()).map(|_| ())
}

pub fn luk_1_ontouchnpc(ctx: &Ctx) -> Script {
    luk_1_run(ctx, Luk1Step::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn luk_1_oninit(ctx: &Ctx) -> Script {
    luk_1_run(ctx, Luk1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Luk2Step {
    Start,
    OnTouchNPC,
    OnInit,
}

fn luk_2_run(ctx: &Ctx, mut step: Luk2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_start = Val::from(0);
    let mut l_time = Val::from(0);
    'machine: loop {
        match step {
            Luk2Step::Start => {
                step = Luk2Step::OnTouchNPC;
                continue 'machine;
            }
            Luk2Step::OnTouchNPC => {
                l_start = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if l_start.clone().number()? < 61 {
                    l_time = Val::from(1000);
                } else if l_start.clone().number()? < 71 {
                    l_time = Val::from(2000);
                } else if l_start.clone().number()? < 81 {
                    l_time = Val::from(3000);
                } else if l_start.clone().number()? < 91 {
                    l_time = Val::from(4000);
                }
                if l_time.clone().is_true() {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_STUN")?, l_time.clone(), Val::from(0)],
                    )?;
                }
                return Err(Stop::End);
            }
            Luk2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn luk_2(ctx: &Ctx) -> Script {
    luk_2_run(ctx, Luk2Step::Start, Vec::new()).map(|_| ())
}

pub fn luk_2_ontouchnpc(ctx: &Ctx) -> Script {
    luk_2_run(ctx, Luk2Step::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn luk_2_oninit(ctx: &Ctx) -> Script {
    luk_2_run(ctx, Luk2Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Tire1Step {
    Start,
    OnTouchNPC,
    OnInit,
}

fn tire_1_run(ctx: &Ctx, mut step: Tire1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_start = Val::from(0);
    let mut l_time = Val::from(0);
    'machine: loop {
        match step {
            Tire1Step::Start => {
                step = Tire1Step::OnTouchNPC;
                continue 'machine;
            }
            Tire1Step::OnTouchNPC => {
                l_start = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if l_start.clone().number()? < 61 {
                    l_time = Val::from(1000);
                } else if l_start.clone().number()? < 71 {
                    l_time = Val::from(2000);
                } else if l_start.clone().number()? < 81 {
                    l_time = Val::from(3000);
                } else if l_start.clone().number()? < 91 {
                    l_time = Val::from(4000);
                }
                if l_time.clone().is_true() {
                    ctx.call(
                        Function::StartStatus,
                        vec![ctx.constant("SC_SLEEP")?, l_time.clone(), Val::from(0)],
                    )?;
                }
                return Err(Stop::End);
            }
            Tire1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tire_1(ctx: &Ctx) -> Script {
    tire_1_run(ctx, Tire1Step::Start, Vec::new()).map(|_| ())
}

pub fn tire_1_ontouchnpc(ctx: &Ctx) -> Script {
    tire_1_run(ctx, Tire1Step::OnTouchNPC, Vec::new()).map(|_| ())
}

pub fn tire_1_oninit(ctx: &Ctx) -> Script {
    tire_1_run(ctx, Tire1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn race_timer2_1(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::Start, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_onenable(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_oninit(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnInit, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer10000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer30000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer90000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer90000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer210000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer210000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer270000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer270000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer272000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer272000, Vec::new()).map(|_| ())
}

pub fn race_timer2_1_ontimer330000(ctx: &Ctx) -> Script {
    race_timer2_1_run(ctx, RaceTimer21Step::OnTimer330000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::Start, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_onenable(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer5000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer7000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer120000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer122000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer122000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer240000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer242000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer242000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_ontimer300000(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn race_timer2_2_oninit(ctx: &Ctx) -> Script {
    race_timer2_2_run(ctx, RaceTimer22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn race_timer2_3(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::Start, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_onenable(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer3000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer6000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer9000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer9000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer12000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer15000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer240000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer243000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer243000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer246000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer246000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer249000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer249000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer252000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer252000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_ontimer300000(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn race_timer2_3_oninit(ctx: &Ctx) -> Script {
    race_timer2_3_run(ctx, RaceTimer23Step::OnInit, Vec::new()).map(|_| ())
}

fn eckar_erenes_double_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Eckar Erenes",
        args![
            "Welcome to the",
            "Monster Race Arena,",
            "the pride and joy of",
            "the village of Hugel!",
            "How may I help you?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Monster Race Info:Enter Monster Race")],
    )?) == 1
    {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Monster Races originated from",
                "simple children's games in which",
                "Cute Pets would race against each other. This grew into an adult",
                "pastime that is so popular, we've built a racing arena in Hugel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Our Monster Race Arena hosts",
                "two types of monster races. First, we have the Single Monster Race,",
                "in which those that wagered on the 1st place monster are rewarded."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Then, we have the Dual Monster",
                "Race in which the house odds and rewards are greater than in Single",
                "Monster Races: you must wager on 2 monsters, and they must place in",
                "1st and 2nd for you to win."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Although a small entrance",
                "fee is required, we only use",
                "the money to give rewards to",
                "participants and maintain this",
                "arena. Therefore, we're not",
                "profiting from this enterprise."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Also, we prohibit others",
                "from making personal bets",
                "and wagers, using items and",
                "zeny, based on the outcomes",
                "of these races. That kind of",
                "gambling is illegal here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Once you enter the Race Arena, you will receive a Racing Ticket.",
                "Keep in mind that winning Racing Tickets can only be exchanged for",
                "Prize Medals during a 5 minute window after the end of the race."
            ],
        )?;
        ctx.next()?;
    }
    ctx.lines_as(
        "Eckar Erenes",
        args![
            "The entrance fee for all races",
            "in the Monster Race Arena is",
            "2,000 zeny. If you'd like to wager on a Single Monster Race,",
            "then please ask my brother",
            "Ellebird to help you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Eckar Erenes",
        args![
            "Otherwise, I'll help get you",
            "started if you're interested",
            "in a Dual Monster Race.",
            "Would you like to wager",
            "on a Dual Monster Race?"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please.:No, thanks.")])?) == 2 {
        ctx.lines_as(
            "Eckar Erenes",
            args!["Very well. I hope that", "you enjoy your time here", "in the Monster Race Arena~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx
        .call(Function::CheckWeight, vec![Val::from("Jellopy"), Val::from(700)])?
        .is_true())
    {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Hmm... You're toting",
                "too many things with you",
                "right now. You better put",
                "some of your stuff away in",
                "Kafra Storage before you can",
                "wager on any monster races..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("Zeny").get()?.number()? < 2000 {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "I'm sorry, but you",
                "don't have enough",
                "money to pay the",
                "2,000 zeny entrance fee. "
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7514)])?.is_true() {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Hm? What are you doing",
                "with an expired Racing Ticket?",
                "Well, I better get rid of it for you before it can get mixed up",
                "with your new Racing Ticket."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7514), Val::from(1)])?;
        ctx.next()?;
        if ctx.var("$@mon_time_2_1").get()? == 1 {
            ctx.lines_as(
                "Eckar Erenes",
                args![
                    "Alright, I think you",
                    "should be all set. I hope",
                    "that you enjoy the race~",
                    "Let me guide you inside",
                    "the Monster Race Arena now."
                ],
            )?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
            ctx.var("monster_race_2_2").set(Val::from(0))?;
            ctx.var("monster_race_2_1").set(ctx.var("monster_race_2_2").get()?)?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("p_track02"), Val::from(75), Val::from(41)])?;
            return Err(Stop::End);
        } else if ctx.var("$@mon_time_2_1").get()? == 2 {
            ctx.lines_as(
                "Eckar Erenes",
                args![
                    "We're still finishing our",
                    "preparations for the next",
                    "Double Monster Race, so",
                    "we ask that you please",
                    "wait a little while longer..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("$@mon_time_2_1").get()? == 1 {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Thanks, I hope that",
                "you enjoy this race.",
                "Let me guide you now",
                "to the Monster Race Arena."
            ],
        )?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
        ctx.var("monster_race_2_2").set(Val::from(0))?;
        ctx.var("monster_race_2_1").set(ctx.var("monster_race_2_2").get()?)?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("p_track02"), Val::from(75), Val::from(41)])?;
        return Err(Stop::End);
    } else if ctx.var("$@mon_time_2_1").get()? == 2 {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "We're still finishing our",
                "preparations for the next",
                "Double Monster Race, so",
                "we ask that you please",
                "wait a little while longer..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("$@mon_time_2_2").get()? == 1 {
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "Right now, a Monster Race",
                "is in progress. It's too late to place a wager, but if you'd like",
                "to watch, the fee is 500 zeny",
                "for spectators. Would you like to enter the Monster Race Arena?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Enter:Cancel")])?) == 2 {
            ctx.lines_as(
                "Eckar Erenes",
                args![
                    "Alright, then. If you'd like",
                    "to wager on a monster",
                    "race, please wait for the",
                    "current race to finish. I hope",
                    "that you enjoy your time here",
                    "in the Monster Race Arena~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("Zeny").get()?.number()? > 499 {
            ctx.lines_as("Eckar Erenes", args!["Thank you~", "I hope you enjoy", "watching this race!"])?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500))?))?;
            ctx.var("monster_race_2_2").set(Val::from(0))?;
            ctx.var("monster_race_2_1").set(ctx.var("monster_race_2_2").get()?)?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("p_track02"), Val::from(75), Val::from(41)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Eckar Erenes",
            args![
                "I'm sorry, but you don't",
                "have enough money to pay",
                "the 500 zeny spectator fee."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Eckar Erenes",
        args![
            "I'm sorry, but a monster",
            "race has just ended, so we're",
            "having the 5 minute period in",
            "which the winners can claim",
            "their Prize Medals. The gate",
            "will open soon, so please wait."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn eckar_erenes_double(ctx: &Ctx) -> Script {
    eckar_erenes_double_body(ctx, Vec::new()).map(|_| ())
}

fn eckar_erenes_double_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Eckar Erenes#double")])?;
    return Err(Stop::End);
}

pub fn eckar_erenes_double_onenable(ctx: &Ctx) -> Script {
    eckar_erenes_double_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn eckar_erenes_double_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Eckar Erenes#double")])?;
    return Err(Stop::End);
}

pub fn eckar_erenes_double_ondisable(ctx: &Ctx) -> Script {
    eckar_erenes_double_ondisable_body(ctx, Vec::new()).map(|_| ())
}
