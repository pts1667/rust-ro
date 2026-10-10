use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum GenRo1Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn gen_ro_1_run(ctx: &Ctx, mut step: GenRo1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GenRo1Step::Start => {
                step = GenRo1Step::OnTouch;
                continue 'machine;
            }
            GenRo1Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(200),
                            Val::from(389),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("gen_ro#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(201),
                            Val::from(389),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("gen_ro#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#2::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#3::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#4::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            GenRo1Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("gen_ro#1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            GenRo1Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn gen_ro_1(ctx: &Ctx) -> Script {
    gen_ro_1_run(ctx, GenRo1Step::Start, Vec::new()).map(|_| ())
}

pub fn gen_ro_1_ontouch(ctx: &Ctx) -> Script {
    gen_ro_1_run(ctx, GenRo1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn gen_ro_1_ondisable(ctx: &Ctx) -> Script {
    gen_ro_1_run(ctx, GenRo1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn gen_ro_1_onmymobdead(ctx: &Ctx) -> Script {
    gen_ro_1_run(ctx, GenRo1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GenRo2Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn gen_ro_2_run(ctx: &Ctx, mut step: GenRo2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GenRo2Step::Start => {
                step = GenRo2Step::OnTouch;
                continue 'machine;
            }
            GenRo2Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(100),
                            Val::from(389),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("gen_ro#2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#1::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            GenRo2Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("gen_ro#2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            GenRo2Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn gen_ro_2(ctx: &Ctx) -> Script {
    gen_ro_2_run(ctx, GenRo2Step::Start, Vec::new()).map(|_| ())
}

pub fn gen_ro_2_ontouch(ctx: &Ctx) -> Script {
    gen_ro_2_run(ctx, GenRo2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn gen_ro_2_ondisable(ctx: &Ctx) -> Script {
    gen_ro_2_run(ctx, GenRo2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn gen_ro_2_onmymobdead(ctx: &Ctx) -> Script {
    gen_ro_2_run(ctx, GenRo2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GenRo3Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn gen_ro_3_run(ctx: &Ctx, mut step: GenRo3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GenRo3Step::Start => {
                step = GenRo3Step::OnTouch;
                continue 'machine;
            }
            GenRo3Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(145),
                            Val::from(389),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("gen_ro#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(143),
                            Val::from(389),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("gen_ro#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#2::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            GenRo3Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("gen_ro#3::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            GenRo3Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn gen_ro_3(ctx: &Ctx) -> Script {
    gen_ro_3_run(ctx, GenRo3Step::Start, Vec::new()).map(|_| ())
}

pub fn gen_ro_3_ontouch(ctx: &Ctx) -> Script {
    gen_ro_3_run(ctx, GenRo3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn gen_ro_3_ondisable(ctx: &Ctx) -> Script {
    gen_ro_3_run(ctx, GenRo3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn gen_ro_3_onmymobdead(ctx: &Ctx) -> Script {
    gen_ro_3_run(ctx, GenRo3Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GenRo4Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn gen_ro_4_run(ctx: &Ctx, mut step: GenRo4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GenRo4Step::Start => {
                step = GenRo4Step::OnTouch;
                continue 'machine;
            }
            GenRo4Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(200),
                            Val::from(389),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("gen_ro#4::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#3::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            GenRo4Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("gen_ro#4::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            GenRo4Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn gen_ro_4(ctx: &Ctx) -> Script {
    gen_ro_4_run(ctx, GenRo4Step::Start, Vec::new()).map(|_| ())
}

pub fn gen_ro_4_ontouch(ctx: &Ctx) -> Script {
    gen_ro_4_run(ctx, GenRo4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn gen_ro_4_ondisable(ctx: &Ctx) -> Script {
    gen_ro_4_run(ctx, GenRo4Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn gen_ro_4_onmymobdead(ctx: &Ctx) -> Script {
    gen_ro_4_run(ctx, GenRo4Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum OnewayToGuStep {
    Start,
    OnTouch,
}

fn oneway_to_gu_run(ctx: &Ctx, mut step: OnewayToGuStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OnewayToGuStep::Start => {
                step = OnewayToGuStep::OnTouch;
                continue 'machine;
            }
            OnewayToGuStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("gen_ro#4::OnDisable")])?;
                ctx.var("rogue_q").set(Val::from(17))?;
                ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(367), Val::from(10)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn oneway_to_gu(ctx: &Ctx) -> Script {
    oneway_to_gu_run(ctx, OnewayToGuStep::Start, Vec::new()).map(|_| ())
}

pub fn oneway_to_gu_ontouch(ctx: &Ctx) -> Script {
    oneway_to_gu_run(ctx, OnewayToGuStep::OnTouch, Vec::new()).map(|_| ())
}

fn aragham_junior_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rogue_q").get()? == 9 {
        ctx.lines_as("Aragham Jr.", args!["Oh, you must be", "from the Rogue Guild..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Aragham Jr.",
            args![
                "My name is",
                "Aragham Junior,",
                "Rogue of the Desert.",
                "Are you ready to learn",
                "how to be a Rogue?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Aragham Jr.", args!["See, as a Rogue, our motto is, '^0000FFAvoid the strong! Be malicious to the weak!^000000' That rule especially goes true for monsters."])?;
        ctx.next()?;
        ctx.lines_as(
            "Aragham Jr.",
            args!["Avoid the strong!", "Be malicious to the weak!", "It's a simple rule..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aragham Jr.",
            args!["Now, remember it as you go through ^0000FFthe Underground Tunnel^000000. Try to walk all the way to the Rogue Guild."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aragham Jr.",
            args!["There will be a few monsters, but don't worry. I know you're strong. Alright, are you ready to go or what?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes, let's go.:Nah~")])?) == 1 {
            ctx.lines_as("Aragham Jr.", args!["Alright...", "Good luck, then."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            ctx.var("rogue_q").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2022), Val::from(2026)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Aragham Jr.",
            args!["Fine, fine.", "Take your time", "and come back", "when you're ready."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rogue_q").get()? == 13 {
        ctx.lines_as("Aragham Jr.", args!["Oh, you're back.", "I think you'll do well this time. Another motto Rogues have is '^0000FFFailure teaches success^000000.' Well, then again..."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Re-Test:Cancel")])?) == 1 {
            ctx.lines_as("Aragham Jr.", args!["Good luck."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Aragham Jr.",
            args!["Fine, fine.", "Take your time", "and come back", "when you're ready."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
        ctx.lines_as(
            "Aragham Jr.",
            args!["Huh...?", "Who are you?!", "You're not from", "the Rogue Guild!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Aragham Jr.",
            args!["You've come here to kill me, haven't you? N-no! I'm can't die yet! Get lost! Otherwise, I'll kill you first!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Aragham Jr.",
            args![
                "Hey...",
                "what brings",
                "you back here?",
                "Why don't you",
                "take a rest",
                "before you leave?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn aragham_junior_rg(ctx: &Ctx) -> Script {
    aragham_junior_rg_body(ctx, Vec::new()).map(|_| ())
}

fn hollgrehenn_junior_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rogue_q").get()? == 11 {
        ctx.lines_as("Hollgrehenn Jr.", args!["Huh...", "From the", "Rogue guild, huh?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["I'm Hollgrehenn Junior. I tend to a lot of our underground business. So are you ready to take on my test?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Hollgrehenn Jr.", args!["We Rogues share this motto: ^0000FFAvoid the strong! Be malicious to the weak!^000000 This rule applies to any threat, especially monsters."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["It's easy to remember.", "Just don't forget to put it into practice. You got it?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["For my test, you'll go through the ^0000FFUnderground Tunnel^000000. Follow it all the way back to the Rogue Guild."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["There are some monsters there, but that'll be part of your training. Now, are you ready to go or not?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I am.:Nah~")])?) == 1 {
            ctx.lines_as("Hollgrehenn Jr.", args!["Good luck."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            ctx.var("rogue_q").set(Val::from(15))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2024), Val::from(2026)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["Take your time.", "Come back here", "when you're ready."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rogue_q").get()? == 15 {
        ctx.lines_as("Hollgrehenn Jr.", args!["Huh.", "You failed.", "Gonna try again?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Re-Test:Cancel.")])?) == 1 {
            ctx.lines_as("Hollgrehenn Jr.", args!["Good luck."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["Take your time.", "Come back here", "when you're ready."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
        ctx.lines_as("Hollgrehenn Jr.", args!["Huh...?", "You're not from", "the Rogue Guild..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["You better get out", "of here right now", "if you know what's", "good for you..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["Now...", "Beat it before", "I change my mind", "about killing you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Hollgrehenn Jr.",
            args!["Hey...", "Come to visit?", "We Rogues gotta", "stick together, huh?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hollgrehenn_junior_rg(ctx: &Ctx) -> Script {
    hollgrehenn_junior_rg_body(ctx, Vec::new()).map(|_| ())
}

fn antonio_junior_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rogue_q").get()? == 10 {
        ctx.lines_as(
            "Antonio Jr.",
            args![
                "You're from",
                "the Rogue guild?",
                "If you wanna learn",
                "about becoming a Rogue,",
                "then shut up and stay put."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Antonio Jr.",
            args!["^0000FFAvoid the strong! Be malicious to the weak!^000000 That's our motto for battling monsters."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Antonio Jr.",
            args!["Show no mercy when you fight weaker monsters, and try to keep away from stronger monsters."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Antonio Jr.",
            args!["Now, I want you to walk all the way to the Rogue Guild through this ^0000FFUnderground Tunnel^000000."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Antonio Jr.",
            args!["There are monsters there, but if you avoid the strong and be malicious to the weak, you'll be fine."],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Let's go!:W-wait~")])?) == 1 {
            ctx.lines_as(
                "Antonio Jr.",
                args![
                    "I hope you do",
                    "not fail this test",
                    "You can only become",
                    "a Rogue if you pass..."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            ctx.var("rogue_q").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2023), Val::from(2026)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Antonio Jr.",
            args![
                "I don't have time",
                "to fool around with",
                "you. Hurry up, get",
                "ready, then take",
                "the test."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rogue_q").get()? == 14 {
        ctx.lines_as(
            "Antonio Jr.",
            args!["You failed...?", "I guess that's life.", "Are you gonna try", "again or what?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Re-test:Cancel")])?) == 1 {
            ctx.lines_as(
                "Antonio Jr.",
                args![
                    "Remember, I'm doing",
                    "you a favor here...",
                    "Now, don't come back",
                    "until you're a Rogue."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(15), Val::from(105)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Antonio Jr.",
            args![
                "I don't have time",
                "to fool around with",
                "you. Hurry up, get",
                "ready, then take",
                "the test."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
        ctx.lines(args!["Huh...?", "Who are you?!", "You're not from", "the Rogue Guild!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Antonio Jr.",
            args![
                "You've come here to kill me?! I won't let you!! Come on, give me your best shot! You can't fight if I rip out your eyes!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Antonio Jr.",
            args!["Hey, how's it goin'?", "Take it easy, and just", "relax before you leave."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn antonio_junior_rg(ctx: &Ctx) -> Script {
    antonio_junior_rg_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QuestOutStep {
    Start,
    OnTouch,
}

fn quest_out_run(ctx: &Ctx, mut step: QuestOutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QuestOutStep::Start => {
                step = QuestOutStep::OnTouch;
                continue 'machine;
            }
            QuestOutStep::OnTouch => {
                ctx.var("rogue_q").set(Val::from(16))?;
                ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(378), Val::from(113)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn quest_out(ctx: &Ctx) -> Script {
    quest_out_run(ctx, QuestOutStep::Start, Vec::new()).map(|_| ())
}

pub fn quest_out_ontouch(ctx: &Ctx) -> Script {
    quest_out_run(ctx, QuestOutStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue1Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_1_run(ctx: &Ctx, mut step: MobRogue1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue1Step::Start => {
                step = MobRogue1Step::OnTouch;
                continue 'machine;
            }
            MobRogue1Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(14),
                            Val::from(187),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(188),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(16),
                            Val::from(189),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(17),
                            Val::from(187),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(18),
                            Val::from(188),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(19),
                            Val::from(189),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#1::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue1Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue1Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_1(ctx: &Ctx) -> Script {
    mob_rogue_1_run(ctx, MobRogue1Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_1_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_1_run(ctx, MobRogue1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_1_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_1_run(ctx, MobRogue1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_1_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_1_run(ctx, MobRogue1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue2Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_2_run(ctx: &Ctx, mut step: MobRogue2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue2Step::Start => {
                step = MobRogue2Step::OnTouch;
                continue 'machine;
            }
            MobRogue2Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(276),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(13),
                            Val::from(276),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(11),
                            Val::from(276),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#2::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue2Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue2Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_2(ctx: &Ctx) -> Script {
    mob_rogue_2_run(ctx, MobRogue2Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_2_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_2_run(ctx, MobRogue2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_2_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_2_run(ctx, MobRogue2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_2_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_2_run(ctx, MobRogue2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue3Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_3_run(ctx: &Ctx, mut step: MobRogue3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue3Step::Start => {
                step = MobRogue3Step::OnTouch;
                continue 'machine;
            }
            MobRogue3Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(336),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(336),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(336),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(15),
                            Val::from(336),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#4::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#7::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#8::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue3Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#3::OnMyMObDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue3Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_3(ctx: &Ctx) -> Script {
    mob_rogue_3_run(ctx, MobRogue3Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_3_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_3_run(ctx, MobRogue3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_3_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_3_run(ctx, MobRogue3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_3_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_3_run(ctx, MobRogue3Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue4Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_4_run(ctx: &Ctx, mut step: MobRogue4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue4Step::Start => {
                step = MobRogue4Step::OnTouch;
                continue 'machine;
            }
            MobRogue4Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(39),
                            Val::from(341),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#4::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(40),
                            Val::from(341),
                            Val::from("Ghoul"),
                            Val::from(1036),
                            Val::from(1),
                            Val::from("mob_rogue#4::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(41),
                            Val::from(341),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#4::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(42),
                            Val::from(341),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#4::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue4Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#4::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue4Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_4(ctx: &Ctx) -> Script {
    mob_rogue_4_run(ctx, MobRogue4Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_4_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_4_run(ctx, MobRogue4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_4_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_4_run(ctx, MobRogue4Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_4_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_4_run(ctx, MobRogue4Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue5Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_5_run(ctx: &Ctx, mut step: MobRogue5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue5Step::Start => {
                step = MobRogue5Step::OnTouch;
                continue 'machine;
            }
            MobRogue5Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(92),
                            Val::from(334),
                            Val::from("Khalitzburg"),
                            Val::from(1132),
                            Val::from(1),
                            Val::from("mob_rogue#5::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#2::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#3::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue5Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#5::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue5Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_5(ctx: &Ctx) -> Script {
    mob_rogue_5_run(ctx, MobRogue5Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_5_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_5_run(ctx, MobRogue5Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_5_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_5_run(ctx, MobRogue5Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_5_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_5_run(ctx, MobRogue5Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue6Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_6_run(ctx: &Ctx, mut step: MobRogue6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue6Step::Start => {
                step = MobRogue6Step::OnTouch;
                continue 'machine;
            }
            MobRogue6Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(57),
                            Val::from(301),
                            Val::from("Khalitzburg"),
                            Val::from(1132),
                            Val::from(1),
                            Val::from("mob_rogue#6::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#2::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#3::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue6Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#6::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue6Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_6(ctx: &Ctx) -> Script {
    mob_rogue_6_run(ctx, MobRogue6Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_6_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_6_run(ctx, MobRogue6Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_6_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_6_run(ctx, MobRogue6Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_6_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_6_run(ctx, MobRogue6Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue7Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_7_run(ctx: &Ctx, mut step: MobRogue7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue7Step::Start => {
                step = MobRogue7Step::OnTouch;
                continue 'machine;
            }
            MobRogue7Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(101),
                            Val::from(264),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#7::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(102),
                            Val::from(264),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#7::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue7Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#7::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue7Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_7(ctx: &Ctx) -> Script {
    mob_rogue_7_run(ctx, MobRogue7Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_7_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_7_run(ctx, MobRogue7Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_7_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_7_run(ctx, MobRogue7Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_7_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_7_run(ctx, MobRogue7Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue8Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_8_run(ctx: &Ctx, mut step: MobRogue8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue8Step::Start => {
                step = MobRogue8Step::OnTouch;
                continue 'machine;
            }
            MobRogue8Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(140),
                            Val::from(312),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#8::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue8Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#8::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue8Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_8(ctx: &Ctx) -> Script {
    mob_rogue_8_run(ctx, MobRogue8Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_8_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_8_run(ctx, MobRogue8Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_8_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_8_run(ctx, MobRogue8Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_8_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_8_run(ctx, MobRogue8Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue9Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_9_run(ctx: &Ctx, mut step: MobRogue9Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue9Step::Start => {
                step = MobRogue9Step::OnTouch;
                continue 'machine;
            }
            MobRogue9Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(139),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(149),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(140),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(141),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(150),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(151),
                            Val::from(246),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#9::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue9Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#9::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue9Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_9(ctx: &Ctx) -> Script {
    mob_rogue_9_run(ctx, MobRogue9Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_9_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_9_run(ctx, MobRogue9Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_9_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_9_run(ctx, MobRogue9Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_9_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_9_run(ctx, MobRogue9Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue10Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_10_run(ctx: &Ctx, mut step: MobRogue10Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue10Step::Start => {
                step = MobRogue10Step::OnTouch;
                continue 'machine;
            }
            MobRogue10Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(176),
                            Val::from(211),
                            Val::from("Ghoul"),
                            Val::from(1036),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(176),
                            Val::from(212),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(176),
                            Val::from(213),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(177),
                            Val::from(214),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(177),
                            Val::from(211),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(177),
                            Val::from(212),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(177),
                            Val::from(213),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(177),
                            Val::from(214),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(178),
                            Val::from(211),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(178),
                            Val::from(212),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(178),
                            Val::from(213),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(178),
                            Val::from(214),
                            Val::from("Archer Skeleton"),
                            Val::from(1016),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(179),
                            Val::from(211),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(179),
                            Val::from(212),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(179),
                            Val::from(213),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(179),
                            Val::from(214),
                            Val::from("Zombie"),
                            Val::from(1015),
                            Val::from(1),
                            Val::from("mob_rogue#10::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#4::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#5::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#6::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue10Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#10::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue10Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_10(ctx: &Ctx) -> Script {
    mob_rogue_10_run(ctx, MobRogue10Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_10_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_10_run(ctx, MobRogue10Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_10_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_10_run(ctx, MobRogue10Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_10_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_10_run(ctx, MobRogue10Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue11Step {
    Start,
    OnTouch,
}

fn mob_rogue_11_run(ctx: &Ctx, mut step: MobRogue11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue11Step::Start => {
                step = MobRogue11Step::OnTouch;
                continue 'machine;
            }
            MobRogue11Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#7::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#8::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#9::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#10::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_11(ctx: &Ctx) -> Script {
    mob_rogue_11_run(ctx, MobRogue11Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_11_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_11_run(ctx, MobRogue11Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue12Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_12_run(ctx: &Ctx, mut step: MobRogue12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue12Step::Start => {
                step = MobRogue12Step::OnTouch;
                continue 'machine;
            }
            MobRogue12Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(90),
                            Val::from(187),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#12::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(90),
                            Val::from(183),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#12::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(90),
                            Val::from(190),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#12::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue12Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#12::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue12Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_12(ctx: &Ctx) -> Script {
    mob_rogue_12_run(ctx, MobRogue12Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_12_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_12_run(ctx, MobRogue12Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_12_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_12_run(ctx, MobRogue12Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_12_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_12_run(ctx, MobRogue12Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue13Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_13_run(ctx: &Ctx, mut step: MobRogue13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue13Step::Start => {
                step = MobRogue13Step::OnTouch;
                continue 'machine;
            }
            MobRogue13Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(236),
                            Val::from(186),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#13::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(238),
                            Val::from(186),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#13::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(234),
                            Val::from(186),
                            Val::from("Abysmal Knight"),
                            Val::from(1219),
                            Val::from(1),
                            Val::from("mob_rogue#13::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#12::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue13Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#13::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue13Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_13(ctx: &Ctx) -> Script {
    mob_rogue_13_run(ctx, MobRogue13Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_13_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_13_run(ctx, MobRogue13Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_13_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_13_run(ctx, MobRogue13Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_13_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_13_run(ctx, MobRogue13Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue14Step {
    Start,
    OnTouch,
}

fn mob_rogue_14_run(ctx: &Ctx, mut step: MobRogue14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue14Step::Start => {
                step = MobRogue14Step::OnTouch;
                continue 'machine;
            }
            MobRogue14Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#13::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_14(ctx: &Ctx) -> Script {
    mob_rogue_14_run(ctx, MobRogue14Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_14_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_14_run(ctx, MobRogue14Step::OnTouch, Vec::new()).map(|_| ())
}
