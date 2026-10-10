#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

#[derive(Clone, Copy, Debug)]
enum VoteTimerEinStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer60000,
}

fn ein_pick_kafra_timer(ctx: &Ctx, kafra_event: &str) -> Script {
    if ctx.var("$dts_kafrawins").get()?.number()? < 2 {
        ctx.npc().do_event("Vote Timer2#ein::OnEnable")
    } else {
        ctx.npc().do_event(kafra_event)
    }
}

fn ein_timer_after_result(ctx: &Ctx, kafra_event: &str) -> Script {
    if ctx.var("$dts_jondawins").get()? == 0 {
        ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
    } else if ctx.var("$dts_jondawins").get()? == 1 {
        // Same as the else branch; the separate check keeps the variable reads of the original.
        ein_pick_kafra_timer(ctx, kafra_event)?;
    } else {
        ein_pick_kafra_timer(ctx, kafra_event)?;
    }
    ctx.call(Function::StopNpcTimer, args![])?;
    Ok(())
}

fn vote_timer_ein_run(ctx: &Ctx, mut step: VoteTimerEinStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VoteTimerEinStep::Start => {
                step = VoteTimerEinStep::OnInit;
                continue 'machine;
            }
            VoteTimerEinStep::OnInit => {
                if ctx.var("$dts").get()?.is_true() {
                    ctx.var("$dts").set(Val::from(0))?;
                    ctx.var("$dtsvote").set(Val::from(0))?;
                    ctx.var("$dtsday").set(Val::from(0))?;
                }
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            VoteTimerEinStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            VoteTimerEinStep::OnTimer60000 => {
                if ctx.var("$dts_result").get()? == 1 {
                    ein_timer_after_result(ctx, "Vote Timer3#ein::OnEnable")?;
                } else if ctx.var("$dts_result").get()? == 2 {
                    ein_timer_after_result(ctx, "Vote Timer4#ein::OnEnable")?;
                } else if ctx.var("$dts_time").get()? == 1440 {
                    if ctx.var("$dts_jondavotes").get()?.number()? >= 20 || ctx.var("$dts_kafravotes").get()?.number()? >= 20 {
                        if ctx.var("$dts_jondavotes").get()?.number()? > ctx.var("$dts_kafravotes").get()?.number()? {
                            if ctx.var("$dts_jondawins").get()? == 0 {
                                if ctx.var("$dts_kafrawins").get()?.number()? < 2 {
                                    ctx.var("$dts_kafrawins").set(ctx.var("$dts_kafrawins").get()? + Val::from(1))?;
                                    ctx.var("$dts_result").set(Val::from(1))?;
                                    ctx.var("$dts_time").set(Val::from(0))?;
                                    ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                    ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                                } else if ctx.var("$dts_kafrawins").get()? == 2 {
                                    ctx.var("$dts_result").set(Val::from(1))?;
                                    ctx.var("$dts_time").set(Val::from(0))?;
                                    ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                    ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                                }
                            } else if ctx.var("$dts_jondawins").get()? == 1 {
                                if ctx.var("$dts_kafrawins").get()?.number()? < 2 {
                                    ctx.var("$dts_kafrawins").set(ctx.var("$dts_kafrawins").get()? + Val::from(1))?;
                                    ctx.var("$dts_result").set(Val::from(1))?;
                                    ctx.var("$dts_time").set(Val::from(0))?;
                                    ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                    ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                                } else {
                                    ctx.var("$dts_time").set(Val::from(0))?;
                                    ctx.var("$dts_result").set(Val::from(1))?;
                                    ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                    ctx.npc().do_event("Vote Timer3#ein::OnEnable")?;
                                }
                            } else if ctx.var("$dts_kafrawins").get()?.number()? < 2 {
                                ctx.var("$dts_kafrawins").set(ctx.var("$dts_kafrawins").get()? + Val::from(1))?;
                                ctx.var("$dts_result").set(Val::from(1))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                            } else {
                                ctx.var("$dts_result").set(Val::from(1))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#6::OnEnable")?;
                                ctx.npc().do_event("Vote Timer3#ein::OnEnable")?;
                            }
                        } else if ctx.var("$dts_kafrawins").get()? == 0 {
                            if ctx.var("$dts_jondawins").get()?.number()? < 2 {
                                ctx.var("$dts_jondawins").set(ctx.var("$dts_jondawins").get()? + Val::from(1))?;
                                ctx.var("$dts_result").set(Val::from(2))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                                ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                            } else {
                                ctx.var("$dts_result").set(Val::from(2))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                                ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                            }
                        } else if ctx.var("$dts_kafrawins").get()? == 1 {
                            if ctx.var("$dts_jondawins").get()?.number()? < 2 {
                                ctx.var("$dts_jondawins").set(ctx.var("$dts_jondawins").get()? + Val::from(1))?;
                                ctx.var("$dts_result").set(Val::from(2))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                                ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                            } else {
                                ctx.var("$dts_result").set(Val::from(2))?;
                                ctx.var("$dts_time").set(Val::from(0))?;
                                ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                                ctx.npc().do_event("Vote Timer4#ein::OnEnable")?;
                            }
                        } else if ctx.var("$dts_jondawins").get()?.number()? < 2 {
                            ctx.var("$dts_jondawins").set(ctx.var("$dts_jondawins").get()? + Val::from(1))?;
                            ctx.var("$dts_result").set(Val::from(2))?;
                            ctx.var("$dts_time").set(Val::from(0))?;
                            ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                            ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                        } else {
                            ctx.var("$dts_result").set(Val::from(2))?;
                            ctx.var("$dts_time").set(Val::from(0))?;
                            ctx.npc().do_event("Scrutiny Association#7::OnEnable")?;
                            ctx.npc().do_event("Vote Timer4#ein::OnEnable")?;
                        }
                    } else {
                        ctx.var("$dts_result").set(Val::from(3))?;
                        ctx.var("$dts_time").set(Val::from(0))?;
                        ctx.npc().do_event("Vote Timer#ein::OnEnable")?;
                    }
                } else {
                    ctx.var("$dts_time").set(ctx.var("$dts_time").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer#ein::OnEnable")?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn vote_timer_ein(ctx: &Ctx) -> Script {
    vote_timer_ein_run(ctx, VoteTimerEinStep::Start).map(|_| ())
}

pub fn vote_timer_ein_oninit(ctx: &Ctx) -> Script {
    vote_timer_ein_run(ctx, VoteTimerEinStep::OnInit).map(|_| ())
}

pub fn vote_timer_ein_onenable(ctx: &Ctx) -> Script {
    vote_timer_ein_run(ctx, VoteTimerEinStep::OnEnable).map(|_| ())
}

pub fn vote_timer_ein_ontimer60000(ctx: &Ctx) -> Script {
    vote_timer_ein_run(ctx, VoteTimerEinStep::OnTimer60000).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VoteTimer2EinStep {
    Start,
    OnEnable,
    OnTimer60000,
}

fn vote_timer2_ein_run(ctx: &Ctx, mut step: VoteTimer2EinStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VoteTimer2EinStep::Start => {
                step = VoteTimer2EinStep::OnEnable;
                continue 'machine;
            }
            VoteTimer2EinStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            VoteTimer2EinStep::OnTimer60000 => {
                ctx.call(Function::StopNpcTimer, args![])?;
                if ctx.var("$dts_periodcheck").get()? == 8640 {
                    ctx.var("$dts_periodcheck").set(Val::from(0))?;
                    ctx.var("$dts_result").set(Val::from(0))?;
                    ctx.var("$dts_kafravotes").set(Val::from(0))?;
                    ctx.var("$dts_jondavotes").set(Val::from(0))?;
                    ctx.var("$dts_votecount").set(ctx.var("$dts_votecount").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer#ein::OnEnable")?;
                } else {
                    ctx.var("$dts_periodcheck").set(ctx.var("$dts_periodcheck").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer2#ein::OnEnable")?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn vote_timer2_ein(ctx: &Ctx) -> Script {
    vote_timer2_ein_run(ctx, VoteTimer2EinStep::Start).map(|_| ())
}

pub fn vote_timer2_ein_onenable(ctx: &Ctx) -> Script {
    vote_timer2_ein_run(ctx, VoteTimer2EinStep::OnEnable).map(|_| ())
}

pub fn vote_timer2_ein_ontimer60000(ctx: &Ctx) -> Script {
    vote_timer2_ein_run(ctx, VoteTimer2EinStep::OnTimer60000).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VoteTimer3EinStep {
    Start,
    OnEnable,
    OnTimer60000,
}

fn vote_timer3_ein_run(ctx: &Ctx, mut step: VoteTimer3EinStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VoteTimer3EinStep::Start => {
                step = VoteTimer3EinStep::OnEnable;
                continue 'machine;
            }
            VoteTimer3EinStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            VoteTimer3EinStep::OnTimer60000 => {
                ctx.call(Function::StopNpcTimer, args![])?;
                if ctx.var("$dts_periodcheck").get()? == 8640 {
                    ctx.var("$dts_periodcheck").set(Val::from(0))?;
                    ctx.var("$dts_result").set(Val::from(0))?;
                    ctx.var("$dts_kafravotes").set(Val::from(0))?;
                    ctx.var("$dts_jondavotes").set(Val::from(0))?;
                    ctx.var("$dts_jondawins")
                        .set(Val::from(ctx.var("$dts_jondawins").get()?.number()? - 1))?;
                    ctx.var("$dts_votecount").set(ctx.var("$dts_votecount").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer#ein::OnEnable")?;
                } else {
                    ctx.var("$dts_periodcheck").set(ctx.var("$dts_periodcheck").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer3#ein::OnEnable")?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn vote_timer3_ein(ctx: &Ctx) -> Script {
    vote_timer3_ein_run(ctx, VoteTimer3EinStep::Start).map(|_| ())
}

pub fn vote_timer3_ein_onenable(ctx: &Ctx) -> Script {
    vote_timer3_ein_run(ctx, VoteTimer3EinStep::OnEnable).map(|_| ())
}

pub fn vote_timer3_ein_ontimer60000(ctx: &Ctx) -> Script {
    vote_timer3_ein_run(ctx, VoteTimer3EinStep::OnTimer60000).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VoteTimer4EinStep {
    Start,
    OnEnable,
    OnTimer60000,
}

fn vote_timer4_ein_run(ctx: &Ctx, mut step: VoteTimer4EinStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VoteTimer4EinStep::Start => {
                step = VoteTimer4EinStep::OnEnable;
                continue 'machine;
            }
            VoteTimer4EinStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            VoteTimer4EinStep::OnTimer60000 => {
                ctx.call(Function::StopNpcTimer, args![])?;
                if ctx.var("$dts_periodcheck").get()? == 8640 {
                    ctx.var("$dts_periodcheck").set(Val::from(0))?;
                    ctx.var("$dts_result").set(Val::from(0))?;
                    ctx.var("$dts_kafravotes").set(Val::from(0))?;
                    ctx.var("$dts_jondavotes").set(Val::from(0))?;
                    ctx.var("$dts_kafrawins")
                        .set(Val::from(ctx.var("$dts_kafrawins").get()?.number()? - 1))?;
                    ctx.var("$dts_votecount").set(ctx.var("$dts_votecount").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer#ein::OnEnable")?;
                } else {
                    ctx.var("$dts_periodcheck").set(ctx.var("$dts_periodcheck").get()? + Val::from(1))?;
                    ctx.npc().do_event("Vote Timer4#ein::OnEnable")?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn vote_timer4_ein(ctx: &Ctx) -> Script {
    vote_timer4_ein_run(ctx, VoteTimer4EinStep::Start).map(|_| ())
}

pub fn vote_timer4_ein_onenable(ctx: &Ctx) -> Script {
    vote_timer4_ein_run(ctx, VoteTimer4EinStep::OnEnable).map(|_| ())
}

pub fn vote_timer4_ein_ontimer60000(ctx: &Ctx) -> Script {
    vote_timer4_ein_run(ctx, VoteTimer4EinStep::OnTimer60000).map(|_| ())
}

pub fn kafra_voting_staff_yuno(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![9]).map(|_| ())
}

pub fn kafra_voting_staff_prt(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_moc(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_gef(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_pay(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_alb(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_alde(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![2]).map(|_| ())
}

pub fn kafra_voting_staff_lght(ctx: &Ctx) -> Script {
    shared::kafras_dts_warper::f_votekafra(ctx, args![9]).map(|_| ())
}

const COOL_EVENT_STAFF: &str = "Cool Event Corp. Voting Staff";

fn cool_event_thanks(ctx: &Ctx) -> Script {
    ctx.lines_as(
        COOL_EVENT_STAFF,
        args![
            "Always be assured that",
            "Cool Event Corp. will do",
            "everything in its power to",
            "ensure the satisfaction of",
            "its customers, young and old",
            "and big and small. Thank you~"
        ],
    )
}

fn dungeon_teleport(ctx: &Ctx, map: &str, x: i32, y: i32) -> Script {
    if ctx.player().zeny()? >= 4000 {
        ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
        ctx.fx().cutin("zonda_01", 255)?;
        ctx.warp(map, x, y)?;
        return ctx.end();
    }
    ctx.lines_as(
        COOL_EVENT_STAFF,
        args![
            "I'm sorry, but you do",
            "not have enough zeny to",
            "teleport to this destination.",
            "The teleport fee is 4,000 zeny."
        ],
    )
}

fn cool_event_offer_vote(ctx: &Ctx) -> Script {
    ctx.lines_as(
        COOL_EVENT_STAFF,
        args![
            "Cool Event Corp.,",
            "if chosen to provide the",
            "Dungeon Teleport Service,",
            "will teleport adventurers to",
            "the following dungeons..."
        ],
    )?;
    ctx.next()?;
    ctx.lines(args![" ", " "])?;
    if ctx.var("$dts_jondawins").get()? == 0 {
        ctx.mes("^FF0000Byalan Dungeon, Level 3^000000")?;
    } else if ctx.var("$dts_jondawins").get()? == 1 {
        ctx.lines(args![
            "^FF0000Byalan Dungeon, Level 3^000000",
            "^FF0000Clock Tower, 3rd Floor^000000"
        ])?;
    } else {
        ctx.lines(args![
            "^FF0000Byalan Dungeon, Level 3^000000",
            "^FF0000Clock Tower, 3rd Floor^000000",
            "^FF0000Glast Heim Entrance^000000"
        ])?;
    }
    ctx.next()?;
    ctx.lines_as(
        COOL_EVENT_STAFF,
        args![
            "If you are interested in",
            "these destinations, then",
            "it would be in your best",
            "interest to vote for us.",
            "Would you like to vote",
            "for Cool Event Corp.?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["No", "Yes"])? == 0 {
        ctx.lines_as(
            COOL_EVENT_STAFF,
            args![
                "Ah, I see... Well, if you",
                "happen to change your mind,",
                "feel free to come back and",
                "cast your vote for Cool Event",
                "Corp, alright? Have a nice day~"
            ],
        )
    } else {
        ctx.var("lhz_vote").set(ctx.var("$dts_votecount").get()? + Val::from(1))?;
        ctx.var("$dts_jondavotes").set(ctx.var("$dts_jondavotes").get()? + Val::from(1))?;
        ctx.lines_as(
            COOL_EVENT_STAFF,
            args![
                "Thank you for your vote!",
                "It's customers like you who",
                "ensure the success and great",
                "service that you have come to",
                "expect from Cool Event Corp.",
                "Thank you and have a nice day~"
            ],
        )
    }
}

pub fn cool_event_staff(ctx: &Ctx) -> Script {
    shared::other_global_functions::f_cleargarbage(ctx, args![])?;
    ctx.fx().cutin("zonda_01", 2)?;
    ctx.lines_as(
        COOL_EVENT_STAFF,
        args![
            "Hello! Don't forget to make",
            "your voice be heard and make",
            "sure you vote in the elections",
            "between Cool Event Corp. and",
            "Kafra Corporation for control of the Dungeon Teleport Service!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Reason for Election", "Cast a Vote", "Use Teleport Service", "Cancel"])? {
        0 => {
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "Cool Event Corp. has been",
                    "planning to provide a new",
                    "Dungeon Teleport Service to",
                    "its customers, a service not",
                    "already provided by the Kafra",
                    "Corporation. However..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "Kafra Corporation, which",
                    "already monopolizes the",
                    "public teleportation market,",
                    "actually also had plans to",
                    "provide a similar service."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "Because of technological",
                    "limitations, only one company",
                    "can be chosen as the provider",
                    "of this Dungeon Teleport Service. Hence, we will let the customers",
                    "decide through these elections."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "Multiple elections will be",
                    "held so that our customers",
                    "can test out the special services of each company for themselves.",
                    "However, keep in mind that you must be eligible in order to vote."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "For voter eligibility",
                    "details, please visit our",
                    "headquarters in the city of",
                    "Lighthalzen located in the",
                    "Schwarzwald Republic.",
                    "Thank you for your time."
                ],
            )?;
        }
        1 => {
            if ctx.var("$dts_result").get()? == 0 {
                if ctx.var("lhz_vote").get()?.number()? <= ctx.var("$dts_votecount").get()?.number()? {
                    cool_event_offer_vote(ctx)?;
                } else {
                    ctx.lines_as(
                        COOL_EVENT_STAFF,
                        args![
                            "I'm sorry, but you've",
                            "already cast your vote",
                            "in this election. However,",
                            "please don't let that stop you",
                            "from voting for Cool Event",
                            "Corp. in the next election~"
                        ],
                    )?;
                }
            } else if ctx.var("$dts_result").get()? == 3 {
                if ctx.var("lhz_vote").get()?.number()? <= ctx.var("$dts_votecount").get()?.number()? {
                    ctx.lines_as(
                        COOL_EVENT_STAFF,
                        args![
                            "Unfortunately, there wasn't",
                            "enough voter turnout in the",
                            "last election, so we're holding",
                            "another election to determine",
                            "which company will provide the",
                            "Dungeon Teleport Service."
                        ],
                    )?;
                    ctx.next()?;
                    cool_event_offer_vote(ctx)?;
                } else {
                    ctx.lines_as(
                        COOL_EVENT_STAFF,
                        args![
                            "Unfortunately, there wasn't",
                            "enough voter turnout in the",
                            "last election, so we're holding",
                            "another election to determine",
                            "which company will provide the",
                            "Dungeon Teleport Service."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        COOL_EVENT_STAFF,
                        args![
                            "We appreciate that",
                            "you've already participated",
                            "in this second election by",
                            "casting your vote. Thank",
                            "you for your support~"
                        ],
                    )?;
                }
            } else {
                ctx.lines_as(
                    COOL_EVENT_STAFF,
                    args![
                        "I'm sorry, but an election is",
                        "not currently being held at this time. Please come and cast your",
                        "vote at the next election to decide which company will provide the",
                        "Dungeon Teleport Service."
                    ],
                )?;
            }
        }
        2 => {
            if ctx.var("$dts_result").get()? == 1 {
                ctx.lines_as(
                    COOL_EVENT_STAFF,
                    args![
                        "Please remember that we",
                        "cannot accept Free Warp Tickets",
                        "or award Special Reserve Points",
                        "for this service. Now, please",
                        "choose your destination."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("$dts_jondawins").get()? == 0 {
                    match ctx.menu(&["Byalan Dungeon, Level 3 -> 4,000 z", "Cancel"])? {
                        0 => dungeon_teleport(ctx, "iz_dun02", 234, 206)?,
                        _ => cool_event_thanks(ctx)?,
                    }
                } else if ctx.var("$dts_jondawins").get()? == 1 {
                    match ctx.menu(&["Byalan Dungeon, Level 3 -> 4,000 z", "Clock Tower, 3rd Floor -> 4,000 z", "Cancel"])? {
                        0 => dungeon_teleport(ctx, "iz_dun02", 234, 206)?,
                        1 => dungeon_teleport(ctx, "c_tower3", 64, 143)?,
                        _ => cool_event_thanks(ctx)?,
                    }
                } else {
                    match ctx.menu(&[
                        "Byalan Dungeon, Level 3 -> 4,000 z",
                        "Clock Tower, 3rd Floor -> 4,000 z",
                        "Glast Heim Entrance -> 4,000 z",
                        "Cancel",
                    ])? {
                        0 => dungeon_teleport(ctx, "iz_dun02", 234, 206)?,
                        1 => dungeon_teleport(ctx, "c_tower3", 64, 143)?,
                        2 => dungeon_teleport(ctx, "glast_01", 368, 303)?,
                        _ => cool_event_thanks(ctx)?,
                    }
                }
            } else if ctx.var("$dts_result").get()? == 2 {
                ctx.lines_as(
                    COOL_EVENT_STAFF,
                    args![
                        "I'm sorry, but Cool Event",
                        "Corp. does not currently offer",
                        "the Dungeon Teleport Service",
                        "due to the results of the last",
                        "election. Please vote for us",
                        "next time, alright? Good day~"
                    ],
                )?;
            } else {
                ctx.lines_as(
                    COOL_EVENT_STAFF,
                    args![
                        "I'm sorry, but the",
                        "Dungeon Teleport Service is",
                        "unavailable during elections",
                        "and will be reactivated after the election results are announced.",
                        "Thank you and have a nice day."
                    ],
                )?;
            }
        }
        3 => {
            ctx.lines_as(
                COOL_EVENT_STAFF,
                args![
                    "Cool Event Corp. is always",
                    "working to make sure that",
                    "not only are our customers",
                    "satisfied, but that we also",
                    "exceed your utmost standards.",
                    "Thank you and have a good day."
                ],
            )?;
        }
        _ => {}
    }
    ctx.close_window()?;
    ctx.fx().cutin("zonda_01", 255)?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum ScrutinyAssociation5Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer7200000,
}

fn scrutiny_association_5_run(ctx: &Ctx, mut step: ScrutinyAssociation5Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ScrutinyAssociation5Step::Start => {
                step = ScrutinyAssociation5Step::OnInit;
                continue 'machine;
            }
            ScrutinyAssociation5Step::OnInit => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            ScrutinyAssociation5Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            ScrutinyAssociation5Step::OnTimer7200000 => {
                ctx.call(Function::StopNpcTimer, args![])?;
                if ctx.var("$dts_result").get()? == 0 || ctx.var("$dts_result").get()? == 3 {
                    ctx.call(
                        Function::Announce,
                        args![
                            "Currently, the Dungeon Teleport Service Provider Election is being held in all major cities. Your participation is appreciated.",
                            constants::BC_ALL,
                            "0x70dbdb"
                        ],
                    )?;
                }
                ctx.npc().do_event("Scrutiny Association#5::OnEnable")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn scrutiny_association_5(ctx: &Ctx) -> Script {
    scrutiny_association_5_run(ctx, ScrutinyAssociation5Step::Start).map(|_| ())
}

pub fn scrutiny_association_5_oninit(ctx: &Ctx) -> Script {
    scrutiny_association_5_run(ctx, ScrutinyAssociation5Step::OnInit).map(|_| ())
}

pub fn scrutiny_association_5_onenable(ctx: &Ctx) -> Script {
    scrutiny_association_5_run(ctx, ScrutinyAssociation5Step::OnEnable).map(|_| ())
}

pub fn scrutiny_association_5_ontimer7200000(ctx: &Ctx) -> Script {
    scrutiny_association_5_run(ctx, ScrutinyAssociation5Step::OnTimer7200000).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ScrutinyAssociation6Step {
    Start,
    OnEnable,
    OnTimer1000,
    OnTimer5000,
    OnTimer10000,
    OnTimer15000,
}

fn scrutiny_association_6_run(ctx: &Ctx, mut step: ScrutinyAssociation6Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ScrutinyAssociation6Step::Start => {
                step = ScrutinyAssociation6Step::OnEnable;
                continue 'machine;
            }
            ScrutinyAssociation6Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            ScrutinyAssociation6Step::OnTimer1000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "All the votes for the Dungeon Teleport Service Provider Election have been received and counted.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation6Step::OnTimer5000 => {
                ctx.call(
                    Function::Announce,
                    args!["The results are now in...", constants::BC_ALL, "0x70dbdb"],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation6Step::OnTimer10000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "This time, Cool Event Corp. will be the Dungeon Teleport Service Provider.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation6Step::OnTimer15000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "Many thanks to all of you who have voted and shown your support.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn scrutiny_association_6(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::Start).map(|_| ())
}

pub fn scrutiny_association_6_onenable(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::OnEnable).map(|_| ())
}

pub fn scrutiny_association_6_ontimer1000(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::OnTimer1000).map(|_| ())
}

pub fn scrutiny_association_6_ontimer5000(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::OnTimer5000).map(|_| ())
}

pub fn scrutiny_association_6_ontimer10000(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::OnTimer10000).map(|_| ())
}

pub fn scrutiny_association_6_ontimer15000(ctx: &Ctx) -> Script {
    scrutiny_association_6_run(ctx, ScrutinyAssociation6Step::OnTimer15000).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ScrutinyAssociation7Step {
    Start,
    OnEnable,
    OnTimer1000,
    OnTimer5000,
    OnTimer10000,
    OnTimer15000,
}

fn scrutiny_association_7_run(ctx: &Ctx, mut step: ScrutinyAssociation7Step) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ScrutinyAssociation7Step::Start => {
                step = ScrutinyAssociation7Step::OnEnable;
                continue 'machine;
            }
            ScrutinyAssociation7Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            ScrutinyAssociation7Step::OnTimer1000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "All the votes for the Dungeon Teleport Service Provider Election have been received and counted.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation7Step::OnTimer5000 => {
                ctx.call(
                    Function::Announce,
                    args!["The results are now in...", constants::BC_ALL, "0x70dbdb"],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation7Step::OnTimer10000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "This time, Kafra Corporation will be the Dungeon Teleport Service Provider.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            ScrutinyAssociation7Step::OnTimer15000 => {
                ctx.call(
                    Function::Announce,
                    args![
                        "Many thanks to all of you who have voted and shown your support.",
                        constants::BC_ALL,
                        "0x70dbdb"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn scrutiny_association_7(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::Start).map(|_| ())
}

pub fn scrutiny_association_7_onenable(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::OnEnable).map(|_| ())
}

pub fn scrutiny_association_7_ontimer1000(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::OnTimer1000).map(|_| ())
}

pub fn scrutiny_association_7_ontimer5000(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::OnTimer5000).map(|_| ())
}

pub fn scrutiny_association_7_ontimer10000(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::OnTimer10000).map(|_| ())
}

pub fn scrutiny_association_7_ontimer15000(ctx: &Ctx) -> Script {
    scrutiny_association_7_run(ctx, ScrutinyAssociation7Step::OnTimer15000).map(|_| ())
}
