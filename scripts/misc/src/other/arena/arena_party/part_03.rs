use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force10mob2PartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_10mob_2_party_run(ctx: &Ctx, mut step: Force10mob2PartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force10mob2PartyStep::Start => {
                step = Force10mob2PartyStep::OnEnable;
                continue 'machine;
            }
            Force10mob2PartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Samurai Spector"),
                        Val::from(1542),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(179),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_10mob-2#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force10mob2PartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_10mob-2#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force10mob2PartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_10mob-2#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::OnExit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On10_End")])?;
                    ctx.var("$arn_partyc").set(Val::from(0))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_10mob_2_party(ctx: &Ctx) -> Script {
    force_10mob_2_party_run(ctx, Force10mob2PartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_10mob_2_party_onenable(ctx: &Ctx) -> Script {
    force_10mob_2_party_run(ctx, Force10mob2PartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_10mob_2_party_onreset(ctx: &Ctx) -> Script {
    force_10mob_2_party_run(ctx, Force10mob2PartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_10mob_2_party_onmymobdead(ctx: &Ctx) -> Script {
    force_10mob_2_party_run(ctx, Force10mob2PartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExitPartyStep {
    Start,
    OnTouch,
}

fn force_exit_party_run(ctx: &Ctx, mut step: ForceExitPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExitPartyStep::Start => {
                step = ForceExitPartyStep::OnTouch;
                continue 'machine;
            }
            ForceExitPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_exitmob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_party(ctx: &Ctx) -> Script {
    force_exit_party_run(ctx, ForceExitPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_party_ontouch(ctx: &Ctx) -> Script {
    force_exit_party_run(ctx, ForceExitPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExitmobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_exitmob_party_run(ctx: &Ctx, mut step: ForceExitmobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExitmobPartyStep::Start => {
                step = ForceExitmobPartyStep::OnEnable;
                continue 'machine;
            }
            ForceExitmobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Farewell"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("I hate you"),
                        Val::from(1543),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("I like chocolate"),
                        Val::from(1472),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("You like it, huh?"),
                        Val::from(1472),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Sorry"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Tristram II"),
                        Val::from(1562),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("I am hungry"),
                        Val::from(1468),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Bye"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Take care"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Sexy Body"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Pressure"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Take it easy"),
                        Val::from(1491),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Are you gonna hurt me?"),
                        Val::from(1555),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Merchant"),
                        Val::from(1428),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Ms. Kim"),
                        Val::from(1472),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Martial Art"),
                        Val::from(1472),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Part-timer"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Boss"),
                        Val::from(1562),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Old Yellow Box"),
                        Val::from(1474),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Bat"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Extra"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Milk Merchant"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Darling"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Oh noes!"),
                        Val::from(1491),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("I am not a Wraith"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Mom Wraith"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Dad Wraith"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Book[3]"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(177),
                        Val::from("Exchange Diary"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_exitmob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ForceExitmobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_exitmob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            ForceExitmobPartyStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exitmob_party(ctx: &Ctx) -> Script {
    force_exitmob_party_run(ctx, ForceExitmobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_exitmob_party_onenable(ctx: &Ctx) -> Script {
    force_exitmob_party_run(ctx, ForceExitmobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_exitmob_party_onreset(ctx: &Ctx) -> Script {
    force_exitmob_party_run(ctx, ForceExitmobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_exitmob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_exitmob_party_run(ctx, ForceExitmobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn staff_party_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Staff",
        args![
            "You did a good job.",
            "Even if you have failed to clear a time attack battle, I will reward you with a small amount of arena points."
        ],
    )?;
    ctx.next()?;
    if ctx.var("arena_point").get()? == 30000 {
        ctx.lines_as(
            "Staff",
            args![
                "Uh huh!",
                "You already have enough arena points.",
                "Please spend some arena points later. When I see you next time, I will make sure to give you some reward."
            ],
        )?;
        ctx.next()?;
    } else {
        ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(1)))?;
    }
    ctx.lines_as("Staff", args!["Let me guide you outside. I hope you had a good time."])?;
    ctx.close_window()?;
    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
    ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
    return Err(Stop::End);
}

pub fn staff_party_1(ctx: &Ctx) -> Script {
    staff_party_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_party_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_arnparty_s = Val::from("");
    if runtime::op(&ctx.var("$arena_minptend").get()?, "<", &ctx.var("$arena_minptst").get()?)?.is_true() {
        if runtime::op(&ctx.var("$arena_secptend").get()?, "<", &ctx.var("$arena_secptst").get()?)?.is_true() {
            ctx.var("@record_minpt").set(
                (((Val::from(60).try_sub(ctx.var("$arena_minptst").get()?)?) + ctx.var("$arena_minptend").get()?).try_sub(Val::from(1))?),
            )?;
            ctx.var("@record_secpt")
                .set(((Val::from(60).try_sub(ctx.var("$arena_secptst").get()?)?) + ctx.var("$arena_secptend").get()?))?;
        } else {
            ctx.var("@record_minpt")
                .set(((Val::from(60).try_sub(ctx.var("$arena_minptst").get()?)?) + ctx.var("$arena_minptend").get()?))?;
            ctx.var("@record_secpt")
                .set((ctx.var("$arena_secptend").get()?.try_sub(ctx.var("$arena_secptst").get()?)?))?;
        }
    } else {
        if runtime::op(&ctx.var("$arena_secptend").get()?, "<", &ctx.var("$arena_secptst").get()?)?.is_true() {
            ctx.var("@record_minpt")
                .set(((ctx.var("$arena_minptend").get()?.try_sub(ctx.var("$arena_minptst").get()?)?).try_sub(Val::from(1))?))?;
            ctx.var("@record_secpt")
                .set(((Val::from(60).try_sub(ctx.var("$arena_secptst").get()?)?) + ctx.var("$arena_secptend").get()?))?;
        } else {
            ctx.var("@record_minpt")
                .set((ctx.var("$arena_minptend").get()?.try_sub(ctx.var("$arena_minptst").get()?)?))?;
            ctx.var("@record_secpt")
                .set((ctx.var("$arena_secptend").get()?.try_sub(ctx.var("$arena_secptst").get()?)?))?;
        }
    }
    ctx.var("@gappt").set(
        (((Val::from(60).try_mul(ctx.var("$top_ptmin").get()?)?) + ctx.var("$top_ptsec").get()?)
            .try_sub(((Val::from(60).try_mul(ctx.var("@record_minpt").get()?)?) + ctx.var("@record_secpt").get()?))?),
    )?;
    ctx.lines_as(
        "Staff",
        args![
            "Wow, you did a good job~ ",
            ((Val::from("Your name is...^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                + Val::from("^000000, isn't it?")),
            ((Val::from("^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                + Val::from("^000000, total time you spent to pass the battle.."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((Val::from("is ") + ctx.var("@record_minpt").get()?) + Val::from("minutes ")) + ctx.var("@record_secpt").get()?)
                + Val::from("seconds.")),
            "Congratulations!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((Val::from("The fastest party among people who cleared party arena time force battle is ^3131FF")
                + ctx.var("$arena_pttopn$").get()?)
                + Val::from("^000000."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((((Val::from("^3131FF") + ctx.var("$arena_pttopn$").get()?) + Val::from("^000000's running time was ^3131FF"))
                + ctx.var("$top_ptmin").get()?)
                + Val::from("^000000minutes ^3131FF"))
                + ctx.var("$top_ptsec").get()?)
                + Val::from("^000000seconds."))
        ],
    )?;
    ctx.next()?;
    if ctx.var("@gappt").get()?.number()? < 0 {
        ctx.lines_as(
            "Staff",
            args!["Although you failed to make a new record, I hope you will succeed next time."],
        )?;
        ctx.next()?;
        if ctx.var("arena_point").get()?.number()? > 29980 {
            ctx.lines_as(
                "Staff",
                args![
                    "Then let me reward you with some arena points....eh?",
                    "Your arena points have exceeded the maximum amount. I cannot give you more points until you spend some points."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args!["You can check the amount of arena points you have in the arena waiting room."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "I hope you had a good time and let me guide you to the entrance of arena.",
                    "Thank you."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(40)))?;
            ctx.lines_as("Staff", args!["Let me reward you some arena points.", "If you wish to check the amount of arena points you have, please go talk to ^3131FFVendigos^000000 at the arena entrance."])?;
            ctx.next()?;
            ctx.lines_as("Staff", args!["Let me guide you to the entrance of arena.", "See you later~"])?;
            ctx.close_window()?;
        }
        if ctx
            .call(
                Function::IsPartyLeader,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
            )?
            .loosely_equals(&Val::from(1))
        {
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_pt::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#pt::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Ponox::OnStart")])?;
        }
        ctx.call(Function::SpecialEffect, vec![ctx.var("eh_hit5").get()?])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#pt::OnNomal1")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Staff", args!["Wow! You have renewed the record!", "What a great job!"])?;
        ctx.next()?;
        if ctx
            .call(
                Function::IsPartyLeader,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
            )?
            .loosely_equals(&Val::from(1))
        {
            ctx.lines_as(
                "Staff",
                args![
                    "You can record you and your party members on ^FF0000the hall of Arena Time Force Battle party ^000000.",
                    "When you enter a name, the name will be remained on the top unless someone make a new record."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args!["Please enter a name within 10 letters which can represent you and your party members."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Ok."), Val::from("Let me think.")])? {
                1 => {
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_arnparty_s = input;
                    ctx.lines_as(
                        "Staff",
                        args![((Val::from("You have entered ^3131FF") + l_arnparty_s.clone()) + Val::from("^000000. Is it correct?"))],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])? {
                        1 => {
                            ctx.var("$top_ptmin").set(ctx.var("@record_minpt").get()?)?;
                            ctx.var("$top_ptsec").set(ctx.var("@record_secpt").get()?)?;
                            ctx.var("$arena_pttopn$").set(l_arnparty_s.clone())?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_pt")])?;
                            ctx.lines_as("Staff", args!["Your record has been entered."])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as("Staff", args!["Please take your time and think up a nice name."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Staff", args!["Please take your time and think up a nice name."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if ctx.var("arena_point").get()?.number()? > 29900 {
            ctx.lines_as(
                "Staff",
                args![
                    "Then let me reward you with some arena points....eh?",
                    "Your arena points have exceeded the maximum amount. I cannot give you more points until you spend some points."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args!["You can check the amount of arena points you have in the arena waiting room."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "I hope you had a good time and let me guide you to the entrance of arena.",
                    "Thank you."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.lines_as(
                "Staff",
                args![
                    "Let me reward you with some arena points.",
                    "At the same time, since you have renewed the record you will receive an extra amount of the points this time."
                ],
            )?;
            ctx.next()?;
            ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(100)))?;
            ctx.lines_as("Staff", args!["Let me reward you some arena points.", "If you wish to check the amount of arena points you have, please go talk to ^3131FFVendigos^000000 at the arena entrance."])?;
            ctx.next()?;
            ctx.lines_as("Staff", args!["Let me guide you to the entrance of arena.", "See you later~"])?;
            ctx.close_window()?;
        }
        if ctx
            .call(
                Function::IsPartyLeader,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(1)])?],
            )?
            .loosely_equals(&Val::from(1))
        {
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_pt::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#pt::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Ponox::OnStart")])?;
        }
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        return Err(Stop::End);
    }
}

pub fn staff_party_2(ctx: &Ctx) -> Script {
    staff_party_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimerPtStep {
    Start,
    OnEnter,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_pt_run(ctx: &Ctx, mut step: ArnTimerPtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimerPtStep::Start => {
                step = ArnTimerPtStep::OnEnter;
                continue 'machine;
            }
            ArnTimerPtStep::OnEnter => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimerPtStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast informs you about the restriction for party arena."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimerPtStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("For a smooth game play, exit warp portal will be activated in 1 minute."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimerPtStep::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("prt_are_in"), Val::from("Please proceed your battle quickly as possible in order to avoid disadvantage. Thank you for your cooperation."), Val::from(0), Val::from(16764416)])?;
                return Err(Stop::End);
            }
            ArnTimerPtStep::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#pt::OnTimeOver2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_pt::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_pt::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#pt::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ponox::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimerPtStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_pt(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_onenter(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_pt_onstop(ctx: &Ctx) -> Script {
    arn_timer_pt_run(ctx, ArnTimerPtStep::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarpPtStep {
    Start,
    OnOut,
}

fn arn_warp_pt_run(ctx: &Ctx, mut step: ArnWarpPtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarpPtStep::Start => {
                step = ArnWarpPtStep::OnOut;
                continue 'machine;
            }
            ArnWarpPtStep::OnOut => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from(66),
                        Val::from(143),
                        Val::from(81),
                        Val::from(126),
                        Val::from("arena_room"),
                        Val::from(100),
                        Val::from(75),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_warp_pt(ctx: &Ctx) -> Script {
    arn_warp_pt_run(ctx, ArnWarpPtStep::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_pt_onout(ctx: &Ctx) -> Script {
    arn_warp_pt_run(ctx, ArnWarpPtStep::OnOut, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CastPtStep {
    Start,
    OnTimeOver1,
    OnNomal1,
    OnNomal2,
    OnTimeOver2,
}

fn cast_pt_run(ctx: &Ctx, mut step: CastPtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CastPtStep::Start => {
                step = CastPtStep::OnTimeOver1;
                continue 'machine;
            }
            CastPtStep::OnTimeOver1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated due to an error occurred during battle."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            CastPtStep::OnNomal1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            CastPtStep::OnNomal2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            CastPtStep::OnTimeOver2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated due to an error occurred in the waiting room."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cast_pt(ctx: &Ctx) -> Script {
    cast_pt_run(ctx, CastPtStep::Start, Vec::new()).map(|_| ())
}

pub fn cast_pt_ontimeover1(ctx: &Ctx) -> Script {
    cast_pt_run(ctx, CastPtStep::OnTimeOver1, Vec::new()).map(|_| ())
}

pub fn cast_pt_onnomal1(ctx: &Ctx) -> Script {
    cast_pt_run(ctx, CastPtStep::OnNomal1, Vec::new()).map(|_| ())
}

pub fn cast_pt_onnomal2(ctx: &Ctx) -> Script {
    cast_pt_run(ctx, CastPtStep::OnNomal2, Vec::new()).map(|_| ())
}

pub fn cast_pt_ontimeover2(ctx: &Ctx) -> Script {
    cast_pt_run(ctx, CastPtStep::OnTimeOver2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlloffPtStep {
    Start,
    OnEnable,
    OnInit,
}

fn alloff_pt_run(ctx: &Ctx, mut step: AlloffPtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlloffPtStep::Start => {
                step = AlloffPtStep::OnEnable;
                continue 'machine;
            }
            AlloffPtStep::OnEnable => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-1#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-2#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_exitmob#party::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01start#party")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02start#party")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_03start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_10start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_05")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_03")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_03")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09_10")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_10_09")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09_exit")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arena_p")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnTimerOff")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Slipslowrun#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arn_warp_pt")])?;
                ctx.var("$arn_partywait").set(Val::from(0))?;
                ctx.var("$arn_partyc").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_pt::OnStop")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Slipslowrun#party")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("arena_p")])?;
                return Err(Stop::End);
            }
            AlloffPtStep::OnInit => {
                if (!(ctx.var("$top_ptmin").get()?.is_true()) && !(ctx.var("$top_ptsec").get()?.is_true())) {
                    ctx.var("$top_ptmin").set(Val::from(10))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alloff_pt(ctx: &Ctx) -> Script {
    alloff_pt_run(ctx, AlloffPtStep::Start, Vec::new()).map(|_| ())
}

pub fn alloff_pt_onenable(ctx: &Ctx) -> Script {
    alloff_pt_run(ctx, AlloffPtStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn alloff_pt_oninit(ctx: &Ctx) -> Script {
    alloff_pt_run(ctx, AlloffPtStep::OnInit, Vec::new()).map(|_| ())
}
