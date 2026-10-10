use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hyunmoo_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(1069)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(1070)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(1069)])?.number()? < 30)
        && ctx.call(Function::CountItem, vec![Val::from(1070)])?.number()? < 30)
    {
        ctx.lines_as("Hyunmoo", args!["You didn't bring enough mushrooms... go get some more."])?;
        ctx.next()?;
        ctx.lines_as("Hyunmoo", args!["Or is it you want to quit... do you want to quit?"])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(Val::from(runtime::select_values(ctx, &[Val::from("No.:Yes.")])?) == 1);
            let mut matched1 = false;
            let no_case1 = true;
            if matched1 {
                ctx.lines_as("Hyunmoo", args!["Then move!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.lines_as("Hyunmoo", args![".....I figured as much....you don't have a spirit."])?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("job_monk"),
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", has quit his testing to become a monk.")),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.close_window()?;
        ctx.var("monk_q").set(Val::from(16))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3027), Val::from(3028)])?;
        ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(194), Val::from(168)])?;
        return Err(Stop::End);
    } else if ((ctx.var("monk_q").get()?.number()? > 14 && ctx.var("monk_q").get()?.number()? < 25)
        && (ctx.call(Function::CountItem, vec![Val::from(1069)])? == 0 || ctx.call(Function::CountItem, vec![Val::from(1070)])? == 0))
    {
        ctx.lines_as(
            "Hyunmoo",
            args!["Nice to meet you. My name is Hyunmoo. I am in charge of the mushroom test."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hyunmoo", args!["Your task will be to gather mushrooms.", "Understand?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "Picking the mushrooms is to train your tolerance.",
                "We planted a garden in order to survive as well as to discipline our minds."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "I believe there is no better way to find true inner peace then to be one with nature.",
                "So we created our garden, however these mushrooms started sprouting up everywhere!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "What we ask of you as part of your training is to remove these mushrooms.",
                "Go help the others remove as many mushrooms as you can and bring me back",
                "enough ^FF0000Orange Net Mushrooms^000000 and ^FF0000Orange Gooey Mushroom^000000 as proof."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "Now, go get some mushrooms.",
                "Check back with me when you have picked some, I will tell you if it is enough.",
                "And remember, find peace when gardening."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hyunmoo", args!["...or do you want to quit?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("No.:Yes.")])?) == 1 {
            ctx.lines_as("Hyunmoo", args!["Alright then, keep going."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hyunmoo",
            args![".....yeah I thought as much....you don't have the spirit needed to become a monk."],
        )?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("job_monk"),
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", has quit his testing to become a monk.")),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.call(
            Function::DelItem,
            vec![Val::from(1069), ctx.call(Function::CountItem, vec![Val::from(1069)])?],
        )?;
        ctx.call(
            Function::DelItem,
            vec![Val::from(1070), ctx.call(Function::CountItem, vec![Val::from(1070)])?],
        )?;
        ctx.close_window()?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("job_monk"),
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                    + Val::from(", has quit his training to become a monk.")),
                ctx.constant("BC_MAP")?,
            ],
        )?;
        ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(194), Val::from(168)])?;
        ctx.var("monk_q").set(Val::from(16))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3027), Val::from(3028)])?;
        return Err(Stop::End);
    } else if ((ctx.var("monk_q").get()?.number()? > 14 && ctx.var("monk_q").get()?.number()? < 25)
        && (ctx.call(Function::CountItem, vec![Val::from(1069)])?.number()? > 29
            || ctx.call(Function::CountItem, vec![Val::from(1070)])?.number()? > 29))
    {
        ctx.lines_as("Hyunmoo", args!["...hmm... not bad.", "Ok, you passed."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "Go meet Tomoon for your next test.",
                "Tomoon is staying in the deepest room inside a building near this abbey."
            ],
        )?;
        ctx.var("monk_q").set(Val::from(25))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3027), Val::from(3029)])?;
        ctx.call(
            Function::DelItem,
            vec![Val::from(1069), ctx.call(Function::CountItem, vec![Val::from(1069)])?],
        )?;
        ctx.call(
            Function::DelItem,
            vec![Val::from(1070), ctx.call(Function::CountItem, vec![Val::from(1070)])?],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(194), Val::from(168)])?;
        return Err(Stop::End);
    } else if ctx.var("monk_q").get()?.number()? > 24 {
        ctx.lines_as(
            "Hyunmoo",
            args![
                "Didn't I tell you to go meet ^FF0000Tomoon^000000? Or do you want to pick some more mushrooms?",
                "Tomoon is staying in the deepest room inside a building near this abbey."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn hyunmoo_mk(ctx: &Ctx) -> Script {
    hyunmoo_mk_body(ctx, Vec::new()).map(|_| ())
}

fn hyunmoo_mk2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("monk_q").get()?.number()? < 25 {
        ctx.lines_as("Hyunmoo", args!["As I see vegetables growing, I feel myself growing within."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "As I see other monks working hard on growing vegetables,",
                "it warms my heart to see others enjoying gardening as I do."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "To be honest with you, I think gardening is the greatest thing ever...",
                "We should give thanks to the brothers who prepare our food for us through their hard work."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hyunmoo", args!["Don't forget to thank them as you go by for their hard work."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("monk_q").get()?.number()? > 24 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)) {
        ctx.lines_as(
            "Hyunmoo",
            args![
                "Didn't I tell you to go meet Tomoon? Or do you want to pick more mushrooms?",
                "Tomoon is staying in the deepest room inside a building near this abbey."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Hyunmoo", args!["As I see vegetables growing, I feel myself growing within."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "As I see other monks working hard on growing vegetables,",
                "it warms my heart to see others enjoying gardening as I do."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hyunmoo",
            args![
                "To be honest with you, I think gardening is the greatest thing ever...",
                "We should give thanks to the brothers who prepare our food for us through their hard work."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hyunmoo", args!["Don't forget to thank them as you go by for their hard work."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hyunmoo_mk2(ctx: &Ctx) -> Script {
    hyunmoo_mk2_body(ctx, Vec::new()).map(|_| ())
}

fn tomoon_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("monk_q").get()? == 25 {
        ctx.lines_as(
            "Tomoon",
            args![
                "Welcome young one.",
                "My name is Tomoon, I am in charge of the last test of spiritual training!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tomoon",
            args![
                "Now you don't need to be instructed any more than this:",
                "^990000Terminate every living thing in your way!^000000 That's all!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Tomoon", args!["While you're wandering around within the maze, if you encounter any evil creatures, just kill them! Release their tortured souls!!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Tomoon",
            args!["Do not compare us to the weakling priests! We are monks and we will always be the strongest!!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tomoon",
            args!["We are not like priests who cower behind the strength of others!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tomoon",
            args!["Now, focus!! Keep your fists tight and your eyes open! It's time to show me what you got."],
        )?;
        ctx.next()?;
        ctx.lines_as("Tomoon", args!["Let's see if you got what it takes to be a true monk!!"])?;
        ctx.close_window()?;
        ctx.var("monk_q").set(Val::from(26))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3029), Val::from(3031)])?;
        ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(88), Val::from(74)])?;
        return Err(Stop::End);
    } else if ctx.var("monk_q").get()? == 26 {
        ctx.lines_as(
            "Tomoon",
            args![
                "Hmm... you failed?",
                "Cheer up! Failure is but a process to success!",
                "Go! Start again! Kill them all!!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(88), Val::from(74)])?;
        return Err(Stop::End);
    } else if ctx.var("monk_q").get()? == 27 {
        ctx.lines_as(
            "Tomoon",
            args![
                "Excellent job!!",
                "I knew you'd make it through!",
                "Now...I will give you a secret potion which will double your physical strength."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::GetItem, vec![Val::from(506), Val::from(1)])?;
        ctx.lines(args![
            "Drink this potion and you will be able to become a monk!!!",
            "... now go back to sensei Moohae!!!"
        ])?;
        ctx.var("monk_q").set(Val::from(28))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3031), Val::from(3032)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("monk_q").get()? == 28 {
        ctx.lines_as("Tomoon", args!["I already told you, go back to sensei Moohae!!!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Tomoon", args!["....be quiet.", "....."])?;
        ctx.next()?;
        ctx.lines_as("Tomoon", args!["I will not allow anyone to cause any trouble in this abbey."])?;
        ctx.next()?;
        ctx.lines_as("Tomoon", args!["You'd better not be thinking about causing any trouble."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn tomoon_mk(ctx: &Ctx) -> Script {
    tomoon_mk_body(ctx, Vec::new()).map(|_| ())
}

fn proctor_mk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Proctor", args!["So, are you ready to undergo the spiritual training?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes!:No.:What do I have to do?")])? {
        1 => {
            ctx.lines_as("Proctor", args!["Alright! I wish you luck. If you get lost and can't find a way out, simply log out and log back in.", "Then you will return to your save point. What's that mean? Heck if I know, I'm just told to say that. Oh yes and also, please cooperate with your comrades."])?;
            ctx.close_window()?;
            ctx.var("monk_q").set(Val::from(26))?;
            ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(125), Val::from(277)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Proctor", args!["I see. Take your time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Proctor",
                args![
                    "Inside this test hall is the maze of spirits.",
                    "There are spirits inside which will block you from moving freely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Proctor", args!["If you want to exit the test hall, you must make your way to the warp portal located at the opposite side from the start point."])?;
            ctx.next()?;
            ctx.lines_as(
                "Proctor",
                args![
                    "....Oh yes and also there are monsters wandering around in the maze, please clear them.",
                    "Good luck."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn proctor_mk(ctx: &Ctx) -> Script {
    proctor_mk_body(ctx, Vec::new()).map(|_| ())
}

fn mob_monk_1_1_run(ctx: &Ctx, mut step: MobMonk11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk11Step::Start => {
                step = MobMonk11Step::OnTouch;
                continue 'machine;
            }
            MobMonk11Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(144),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(144),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(144),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(144),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk11Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_1_1(ctx: &Ctx) -> Script {
    mob_monk_1_1_run(ctx, MobMonk11Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_1_ontouch(ctx: &Ctx) -> Script {
    mob_monk_1_1_run(ctx, MobMonk11Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_1_ondisable(ctx: &Ctx) -> Script {
    mob_monk_1_1_run(ctx, MobMonk11Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_1_2_run(ctx: &Ctx, mut step: MobMonk12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk12Step::Start => {
                step = MobMonk12Step::OnTouch;
                continue 'machine;
            }
            MobMonk12Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(134),
                        Val::from(291),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(134),
                        Val::from(291),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(134),
                        Val::from(291),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(134),
                        Val::from(291),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk12Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_1_2(ctx: &Ctx) -> Script {
    mob_monk_1_2_run(ctx, MobMonk12Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_2_ontouch(ctx: &Ctx) -> Script {
    mob_monk_1_2_run(ctx, MobMonk12Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_2_ondisable(ctx: &Ctx) -> Script {
    mob_monk_1_2_run(ctx, MobMonk12Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_1_3_run(ctx: &Ctx, mut step: MobMonk13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk13Step::Start => {
                step = MobMonk13Step::OnTouch;
                continue 'machine;
            }
            MobMonk13Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(157),
                        Val::from(284),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk13Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_1_3(ctx: &Ctx) -> Script {
    mob_monk_1_3_run(ctx, MobMonk13Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_3_ontouch(ctx: &Ctx) -> Script {
    mob_monk_1_3_run(ctx, MobMonk13Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_3_ondisable(ctx: &Ctx) -> Script {
    mob_monk_1_3_run(ctx, MobMonk13Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_1_4_run(ctx: &Ctx, mut step: MobMonk14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk14Step::Start => {
                step = MobMonk14Step::OnTouch;
                continue 'machine;
            }
            MobMonk14Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(156),
                        Val::from(261),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk14Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_1_4(ctx: &Ctx) -> Script {
    mob_monk_1_4_run(ctx, MobMonk14Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_4_ontouch(ctx: &Ctx) -> Script {
    mob_monk_1_4_run(ctx, MobMonk14Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_4_ondisable(ctx: &Ctx) -> Script {
    mob_monk_1_4_run(ctx, MobMonk14Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_1_5_run(ctx: &Ctx, mut step: MobMonk15Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk15Step::Start => {
                step = MobMonk15Step::OnTouch;
                continue 'machine;
            }
            MobMonk15Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(149),
                        Val::from(268),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(149),
                        Val::from(268),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(149),
                        Val::from(268),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(149),
                        Val::from(268),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(149),
                        Val::from(268),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk15Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_1_5(ctx: &Ctx) -> Script {
    mob_monk_1_5_run(ctx, MobMonk15Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_5_ontouch(ctx: &Ctx) -> Script {
    mob_monk_1_5_run(ctx, MobMonk15Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_1_5_ondisable(ctx: &Ctx) -> Script {
    mob_monk_1_5_run(ctx, MobMonk15Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ExitMonk1Step {
    Start,
    OnTouch,
}

fn exit_monk_1_run(ctx: &Ctx, mut step: ExitMonk1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExitMonk1Step::Start => {
                step = ExitMonk1Step::OnTouch;
                continue 'machine;
            }
            ExitMonk1Step::OnTouch => {
                ctx.lines_as("Proctor", args!["You did well. Please return to Tomoon, he's waiting for you."])?;
                ctx.var("monk_q").set(Val::from(27))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_1::OnDisable")])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(196), Val::from(168)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn exit_monk_1(ctx: &Ctx) -> Script {
    exit_monk_1_run(ctx, ExitMonk1Step::Start, Vec::new()).map(|_| ())
}

pub fn exit_monk_1_ontouch(ctx: &Ctx) -> Script {
    exit_monk_1_run(ctx, ExitMonk1Step::OnTouch, Vec::new()).map(|_| ())
}

fn proctor_mk2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Proctor", args!["So, are you ready to undergo this spiritual training?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes!:No.:Check the caution for the test.")])? {
        1 => {
            ctx.lines_as("Proctor", args!["Alright! I wish you luck. If you get lost and can't find a way out, simply log out and log back in.", "Then you will return to your save point. What's that mean? Heck if I know, I'm just told to say that. Oh yes and also, please cooperate with your comrades."])?;
            ctx.close_window()?;
            ctx.var("monk_q").set(Val::from(26))?;
            ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(125), Val::from(177)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Proctor", args!["I see. Take your time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Proctor",
                args![
                    "Inside this test hall is the maze of spirits.",
                    "There are spirits inside which will block you from moving freely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Proctor", args!["If you want to exit the test hall, you must make your way to the warp portal located at the opposite side from the start point."])?;
            ctx.next()?;
            ctx.lines_as(
                "Proctor",
                args![
                    "....Oh yes and also there are monsters wandering around in the maze, please clear them.",
                    "Good luck."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn proctor_mk2(ctx: &Ctx) -> Script {
    proctor_mk2_body(ctx, Vec::new()).map(|_| ())
}

fn mob_monk_2_1_run(ctx: &Ctx, mut step: MobMonk21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk21Step::Start => {
                step = MobMonk21Step::OnTouch;
                continue 'machine;
            }
            MobMonk21Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(140),
                        Val::from(181),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(140),
                        Val::from(181),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(140),
                        Val::from(181),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(140),
                        Val::from(181),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk21Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_2_1(ctx: &Ctx) -> Script {
    mob_monk_2_1_run(ctx, MobMonk21Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_1_ontouch(ctx: &Ctx) -> Script {
    mob_monk_2_1_run(ctx, MobMonk21Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_1_ondisable(ctx: &Ctx) -> Script {
    mob_monk_2_1_run(ctx, MobMonk21Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_2_2_run(ctx: &Ctx, mut step: MobMonk22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk22Step::Start => {
                step = MobMonk22Step::OnTouch;
                continue 'machine;
            }
            MobMonk22Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(150),
                        Val::from(164),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(150),
                        Val::from(164),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(150),
                        Val::from(164),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(150),
                        Val::from(164),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk22Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_2_2(ctx: &Ctx) -> Script {
    mob_monk_2_2_run(ctx, MobMonk22Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_2_ontouch(ctx: &Ctx) -> Script {
    mob_monk_2_2_run(ctx, MobMonk22Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_2_ondisable(ctx: &Ctx) -> Script {
    mob_monk_2_2_run(ctx, MobMonk22Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_2_3_run(ctx: &Ctx, mut step: MobMonk23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk23Step::Start => {
                step = MobMonk23Step::OnTouch;
                continue 'machine;
            }
            MobMonk23Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(158),
                        Val::from(192),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk23Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_2_3(ctx: &Ctx) -> Script {
    mob_monk_2_3_run(ctx, MobMonk23Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_3_ontouch(ctx: &Ctx) -> Script {
    mob_monk_2_3_run(ctx, MobMonk23Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_3_ondisable(ctx: &Ctx) -> Script {
    mob_monk_2_3_run(ctx, MobMonk23Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_2_4_run(ctx: &Ctx, mut step: MobMonk24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk24Step::Start => {
                step = MobMonk24Step::OnTouch;
                continue 'machine;
            }
            MobMonk24Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(165),
                        Val::from(186),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk24Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_2_4(ctx: &Ctx) -> Script {
    mob_monk_2_4_run(ctx, MobMonk24Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_4_ontouch(ctx: &Ctx) -> Script {
    mob_monk_2_4_run(ctx, MobMonk24Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_4_ondisable(ctx: &Ctx) -> Script {
    mob_monk_2_4_run(ctx, MobMonk24Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_2_5_run(ctx: &Ctx, mut step: MobMonk25Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk25Step::Start => {
                step = MobMonk25Step::OnTouch;
                continue 'machine;
            }
            MobMonk25Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(162),
                        Val::from(182),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(162),
                        Val::from(182),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(162),
                        Val::from(182),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(162),
                        Val::from(182),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(162),
                        Val::from(182),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk25Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_2_5(ctx: &Ctx) -> Script {
    mob_monk_2_5_run(ctx, MobMonk25Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_5_ontouch(ctx: &Ctx) -> Script {
    mob_monk_2_5_run(ctx, MobMonk25Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_2_5_ondisable(ctx: &Ctx) -> Script {
    mob_monk_2_5_run(ctx, MobMonk25Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ExitMonk2Step {
    Start,
    OnTouch,
}

fn exit_monk_2_run(ctx: &Ctx, mut step: ExitMonk2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExitMonk2Step::Start => {
                step = ExitMonk2Step::OnTouch;
                continue 'machine;
            }
            ExitMonk2Step::OnTouch => {
                ctx.lines_as("Proctor", args!["You did well. Please return to Tomoon, he's waiting for you."])?;
                ctx.var("monk_q").set(Val::from(27))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_1::OnDisable")])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(196), Val::from(168)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn exit_monk_2(ctx: &Ctx) -> Script {
    exit_monk_2_run(ctx, ExitMonk2Step::Start, Vec::new()).map(|_| ())
}

pub fn exit_monk_2_ontouch(ctx: &Ctx) -> Script {
    exit_monk_2_run(ctx, ExitMonk2Step::OnTouch, Vec::new()).map(|_| ())
}

fn proctor_btl_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Proctor", args!["So, are you ready to undergo this spiritual training?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes!:No.:Check the caution for the test.")])? {
        1 => {
            ctx.lines_as("Proctor", args!["Alright! I wish you luck. If you get lost and can't find a way out, simply log out and log back in.", "Then you will return to your save point. What's that mean? Heck if I know, I'm just told to say that. Oh yes and also, please cooperate with your comrades."])?;
            ctx.close_window()?;
            ctx.var("monk_q").set(Val::from(26))?;
            ctx.call(Function::Warp, vec![Val::from("monk_test"), Val::from(230), Val::from(277)])?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Proctor", args!["I see. Take your time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Proctor",
                args![
                    "Inside this test hall is the maze of spirits.",
                    "There are spirits inside which will block you from moving freely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Proctor", args!["If you want to exit the test hall, you must make your way to the warp portal located at the opposite side from the start point."])?;
            ctx.next()?;
            ctx.lines_as(
                "Proctor",
                args![
                    "....Oh yes and also there are monsters wandering around in the maze, please clear them.",
                    "Good luck."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn proctor_btl_3(ctx: &Ctx) -> Script {
    proctor_btl_3_body(ctx, Vec::new()).map(|_| ())
}

fn mob_monk_3_1_run(ctx: &Ctx, mut step: MobMonk31Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk31Step::Start => {
                step = MobMonk31Step::OnTouch;
                continue 'machine;
            }
            MobMonk31Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(249),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(249),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(249),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(249),
                        Val::from(277),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk31Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn mob_monk_3_1(ctx: &Ctx) -> Script {
    mob_monk_3_1_run(ctx, MobMonk31Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_1_ontouch(ctx: &Ctx) -> Script {
    mob_monk_3_1_run(ctx, MobMonk31Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_1_ondisable(ctx: &Ctx) -> Script {
    mob_monk_3_1_run(ctx, MobMonk31Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_3_2_run(ctx: &Ctx, mut step: MobMonk32Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk32Step::Start => {
                step = MobMonk32Step::OnTouch;
                continue 'machine;
            }
            MobMonk32Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(231),
                        Val::from(296),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(231),
                        Val::from(296),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(231),
                        Val::from(296),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(231),
                        Val::from(296),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk32Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_3_2(ctx: &Ctx) -> Script {
    mob_monk_3_2_run(ctx, MobMonk32Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_2_ontouch(ctx: &Ctx) -> Script {
    mob_monk_3_2_run(ctx, MobMonk32Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_2_ondisable(ctx: &Ctx) -> Script {
    mob_monk_3_2_run(ctx, MobMonk32Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_3_3_run(ctx: &Ctx, mut step: MobMonk33Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk33Step::Start => {
                step = MobMonk33Step::OnTouch;
                continue 'machine;
            }
            MobMonk33Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(264),
                        Val::from(292),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk33Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_3_3(ctx: &Ctx) -> Script {
    mob_monk_3_3_run(ctx, MobMonk33Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_3_ontouch(ctx: &Ctx) -> Script {
    mob_monk_3_3_run(ctx, MobMonk33Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_3_ondisable(ctx: &Ctx) -> Script {
    mob_monk_3_3_run(ctx, MobMonk33Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_3_4_run(ctx: &Ctx, mut step: MobMonk34Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk34Step::Start => {
                step = MobMonk34Step::OnTouch;
                continue 'machine;
            }
            MobMonk34Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(252),
                        Val::from(284),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk34Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_3_4(ctx: &Ctx) -> Script {
    mob_monk_3_4_run(ctx, MobMonk34Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_4_ontouch(ctx: &Ctx) -> Script {
    mob_monk_3_4_run(ctx, MobMonk34Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_4_ondisable(ctx: &Ctx) -> Script {
    mob_monk_3_4_run(ctx, MobMonk34Step::OnDisable, Vec::new()).map(|_| ())
}

fn mob_monk_3_5_run(ctx: &Ctx, mut step: MobMonk35Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobMonk35Step::Start => {
                step = MobMonk35Step::OnTouch;
                continue 'machine;
            }
            MobMonk35Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("monk_test"),
                        Val::from(257),
                        Val::from(285),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MobMonk35Step::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("monk_test"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_monk_3_5(ctx: &Ctx) -> Script {
    mob_monk_3_5_run(ctx, MobMonk35Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_5_ontouch(ctx: &Ctx) -> Script {
    mob_monk_3_5_run(ctx, MobMonk35Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_monk_3_5_ondisable(ctx: &Ctx) -> Script {
    mob_monk_3_5_run(ctx, MobMonk35Step::OnDisable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ExitMonk3Step {
    Start,
    OnTouch,
}

fn exit_monk_3_run(ctx: &Ctx, mut step: ExitMonk3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExitMonk3Step::Start => {
                step = ExitMonk3Step::OnTouch;
                continue 'machine;
            }
            ExitMonk3Step::OnTouch => {
                ctx.lines_as("Proctor", args!["You did well. Please return to Tomoon, he's waiting for you."])?;
                ctx.var("monk_q").set(Val::from(27))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_1::OnDisable")])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prt_monk"), Val::from(196), Val::from(168)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn exit_monk_3(ctx: &Ctx) -> Script {
    exit_monk_3_run(ctx, ExitMonk3Step::Start, Vec::new()).map(|_| ())
}

pub fn exit_monk_3_ontouch(ctx: &Ctx) -> Script {
    exit_monk_3_run(ctx, ExitMonk3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn resetter_monk(ctx: &Ctx) -> Script {
    resetter_monk_run(ctx, ResetterMonkStep::Start, Vec::new()).map(|_| ())
}

pub fn resetter_monk_ontimer500000(ctx: &Ctx) -> Script {
    resetter_monk_run(ctx, ResetterMonkStep::OnTimer500000, Vec::new()).map(|_| ())
}

pub fn resetter_monk_oninit(ctx: &Ctx) -> Script {
    resetter_monk_run(ctx, ResetterMonkStep::OnInit, Vec::new()).map(|_| ())
}

pub fn resetter_monk_onenable(ctx: &Ctx) -> Script {
    resetter_monk_run(ctx, ResetterMonkStep::OnEnable, Vec::new()).map(|_| ())
}

fn switchreset_monkmonk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "Grrrr...",
        "All monsters in the monk job chance place have been reset.",
        "Timer's activated."
    ])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_1::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_2::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_3::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_4::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_5::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_1::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_2::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_3::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_4::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_5::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_1::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_2::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_3::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_4::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_5::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("resetter#monk::OnEnable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn switchreset_monkmonk(ctx: &Ctx) -> Script {
    switchreset_monkmonk_body(ctx, Vec::new()).map(|_| ())
}
