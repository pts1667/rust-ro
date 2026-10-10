use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Knight2Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer180000,
    OnTimer181000,
    OnTimer182000,
}

fn knight2_run(ctx: &Ctx, mut step: Knight2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Knight2Step::Start => {
                step = Knight2Step::OnInit;
                continue 'machine;
            }
            Knight2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Knight2")])?;
                return Err(Stop::End);
            }
            Knight2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Knight2")])?;
                {
                    ctx.var(".mymobs").set(Val::from(12))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(43),
                            Val::from(42),
                            Val::from("Desert Wolf"),
                            Val::from(1106),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(43),
                            Val::from(62),
                            Val::from("Desert Wolf"),
                            Val::from(1106),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(60),
                            Val::from(68),
                            Val::from("Anacondaq"),
                            Val::from(1030),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(27),
                            Val::from(68),
                            Val::from("Anacondaq"),
                            Val::from(1030),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(60),
                            Val::from(35),
                            Val::from("Anacondaq"),
                            Val::from(1030),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(27),
                            Val::from(35),
                            Val::from("Anacondaq"),
                            Val::from(1030),
                            Val::from(1),
                            Val::from("Knight2::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(53),
                        Val::from(52),
                        Val::from("Frilldora"),
                        Val::from(1119),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(34),
                        Val::from(52),
                        Val::from("Frilldora"),
                        Val::from(1119),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(58),
                        Val::from(52),
                        Val::from("Drainliar"),
                        Val::from(1111),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(58),
                        Val::from(52),
                        Val::from("Drainliar"),
                        Val::from(1111),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(29),
                        Val::from(52),
                        Val::from("Drainliar"),
                        Val::from(1111),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(29),
                        Val::from(52),
                        Val::from("Drainliar"),
                        Val::from(1111),
                        Val::from(1),
                        Val::from("Knight2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Knight2Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_knt"), Val::from("Knight2::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Knight2")])?;
                return Err(Stop::End);
            }
            Knight2Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.lines_as("Sir Windsor", args!["...Hmm."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Windsor", args!["...One stage left."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("job_knt"), Val::from(143), Val::from(152)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight2::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight3::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Knight2Step::OnTimer180000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight2::OnDisable")])?;
                return Err(Stop::End);
            }
            Knight2Step::OnTimer181000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_knt"),
                        Val::from(24),
                        Val::from(32),
                        Val::from(63),
                        Val::from(71),
                        Val::from("prt_in"),
                        Val::from(80),
                        Val::from(100),
                    ],
                )?;
                return Err(Stop::End);
            }
            Knight2Step::OnTimer182000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Windsor Benedict#knt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn knight2(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::Start, Vec::new()).map(|_| ())
}

pub fn knight2_oninit(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn knight2_onenable(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn knight2_ondisable(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn knight2_onmymobdead(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn knight2_ontimer180000(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn knight2_ontimer181000(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnTimer181000, Vec::new()).map(|_| ())
}

pub fn knight2_ontimer182000(ctx: &Ctx) -> Script {
    knight2_run(ctx, Knight2Step::OnTimer182000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Knight3Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer180000,
    OnTimer181000,
    OnTimer182000,
}

fn knight3_run(ctx: &Ctx, mut step: Knight3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Knight3Step::Start => {
                step = Knight3Step::OnInit;
                continue 'machine;
            }
            Knight3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Knight3")])?;
                return Err(Stop::End);
            }
            Knight3Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Knight3")])?;
                {
                    ctx.var(".mymobs").set(Val::from(7))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(147),
                            Val::from(167),
                            Val::from("Goblin Archer"),
                            Val::from(1258),
                            Val::from(1),
                            Val::from("Knight3::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(136),
                            Val::from(158),
                            Val::from("Steam Goblin"),
                            Val::from(1280),
                            Val::from(1),
                            Val::from("Knight3::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(136),
                        Val::from(152),
                        Val::from("Goblin"),
                        Val::from(1122),
                        Val::from(1),
                        Val::from("Knight3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(150),
                        Val::from(152),
                        Val::from("Goblin"),
                        Val::from(1123),
                        Val::from(1),
                        Val::from("Knight3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(145),
                        Val::from("Goblin"),
                        Val::from(1124),
                        Val::from(1),
                        Val::from("Knight3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(167),
                        Val::from("Goblin"),
                        Val::from(1125),
                        Val::from(1),
                        Val::from("Knight3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(139),
                        Val::from(167),
                        Val::from("Goblin"),
                        Val::from(1126),
                        Val::from(1),
                        Val::from("Knight3::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Knight3Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_knt"), Val::from("Knight3::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Knight3")])?;
                return Err(Stop::End);
            }
            Knight3Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.lines_as("Sir Windsor", args!["..."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Windsor", args!["...Very good."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Windsor", args!["...Go talk to", "Amy Beatrice now."])?;
                    ctx.close_window()?;
                    ctx.var("knight_q").set(Val::from(8))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(9007)])? == -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(9006), Val::from(9007)])?;
                    }
                    ctx.call(Function::Warp, vec![Val::from("prt_in"), Val::from(80), Val::from(100)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight3::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Windsor Benedict#knt::OnStart")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Knight3Step::OnTimer180000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight3::OnDisable")])?;
                return Err(Stop::End);
            }
            Knight3Step::OnTimer181000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_knt"),
                        Val::from(124),
                        Val::from(132),
                        Val::from(163),
                        Val::from(171),
                        Val::from("prt_in"),
                        Val::from(80),
                        Val::from(100),
                    ],
                )?;
                return Err(Stop::End);
            }
            Knight3Step::OnTimer182000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Windsor Benedict#knt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn knight3(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::Start, Vec::new()).map(|_| ())
}

pub fn knight3_oninit(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn knight3_onenable(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn knight3_ondisable(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn knight3_onmymobdead(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn knight3_ontimer180000(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn knight3_ontimer181000(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnTimer181000, Vec::new()).map(|_| ())
}

pub fn knight3_ontimer182000(ctx: &Ctx) -> Script {
    knight3_run(ctx, Knight3Step::OnTimer182000, Vec::new()).map(|_| ())
}

fn lady_amy_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_knight_t = Val::from(0);
    ctx.mes("[Lady Amy]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.lines(args!["Oh...!", "I wonder, why", "have you come", "to visit me?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args![
                    "You're not having",
                    "trouble as a Knight,",
                    "are you? Well, I think",
                    "you're doing well~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Lady Amy", args!["Of course~", "You're a member of", "the Prontera Chivalry~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["Aww~", "What a cute", "little Novice!", "Soooooo cute!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args![
                    "Heh heh...",
                    "Are you interested",
                    "in becoming a Knight",
                    "later on? You'd be",
                    "a great Knight~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args!["Remember, you're", "going to be a Knight,", "alright? Promise?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args!["Welcome to", "the Prontera Chivalry~"])?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args!["We're only Knights,", "but hope you enjoy", "your stay here.", "Heh heh~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.lines(args![
            "Ooh, you're",
            "a Swordsman...?",
            "Did you come to",
            "change jobs to",
            "a Knight?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["To apply, talk", "to the captain", "all the way over", "there. Hee hee~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()?.number()? >= 1 && ctx.var("knight_q").get()?.number()? <= 7) {
        ctx.lines(args!["Hmmm?", "Why did you", "come to Amy?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Lady Amy",
                args![
                    "Mmm~",
                    "You applied to change jobs! Okay! You'll soon be a Knight with that kind of determination!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args!["But...", "You have to go", "to the other Knights", "before talking to Amy."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args!["I'd love to test", "you from the beginning,", "but I'm not allowed to.", "Hee hee~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lady Amy", args!["Aww~", "Alright..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()? == 8 || ctx.var("knight_q").get()? == 9) {
        if ctx.var("knight_q").get()? == 8 {
            ctx.lines(args!["Hmmm?", "Why did you", "come to Amy?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Sir Windsor told me to--:Oh, nothing.")],
            )?) == 1
            {
                if ctx.call(Function::CheckQuest, vec![Val::from(9008)])? == -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(9007), Val::from(9008)])?;
                }
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "Oh!",
                        "No need to say",
                        "anything more.",
                        "Welcome! It's time",
                        "to take Amy's test!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "My name is Amy Beatrice,",
                        "a proud Lady Knight of the Prontera Chivalry. Amy's test will test your etiquette as a Knight~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "I'll tell you a story and you choose an answer whenever",
                        "I ask a question. Your etiquette will be judged on your answers."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Amy",
                    args!["So listen carefully", "and answer as if you're", "already a Knight, okay?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Lady Amy", args!["Then,", "let's begin!"])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Lady Amy", args!["Aww...", "Alright~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("knight_q").get()? == 9 {
            ctx.lines(args!["Hmmm?", "Why did you", "come to Amy?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "Mmm~?",
                        "Have you learned",
                        "what you did wrong",
                        "last time? If you",
                        "fail again, I'm going",
                        "to be mad!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "So listen carefully",
                        "and answer as if you",
                        "are a Knight.",
                        "Well then,",
                        "let's begin!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lady Amy",
                    args![
                        "You are a Knight and you are looking for a party in Morocc.",
                        "How would you go about doing so?"
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as("Lady Amy", args!["Aww...", "Alright~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.lines(args![
            "You are a Knight and you are looking for a party in Morocc.",
            "How would you go about doing so?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Shout out that you are looking for a party.:Open a chat room and wait.:Look for people seeking Knights.",
            )],
        )?) != 1
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args![
                "You have formed a party with equal leveled players. There's a Priest, a Wizard, a Hunter, an Assassin, and a Blacksmith..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["The six of you decide to go hunt and have decided to go to the Pyramids."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["You reach Level 4", "of the Pyramids", "with your party.", "What should you do?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Check out the area and plan ahead.:Gather monsters for your party members.:Lead the party slowly at the front.",
            )],
        )?) != 2
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args!["But some rude players came with a group of monsters and disappeared! What should you do?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Keep the monsters from reaching the party.:Defend while the party retreats.:Run away on your Peco Peco.",
            )],
        )?) != 3
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as("Lady Amy", args!["Luckily, you all lived through the crisis. But as you walk, you find a person, who is not in your party, collapsed on the ground."])?;
        ctx.next()?;
        ctx.lines_as("Lady Amy", args!["The person is asking politely for help. What should you do?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Ask your party's Priest to help.:Say you will help for Zeny.:Ignore and move on.",
            )],
        )?) == 1
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args!["You must bid farewell to your party members because you must go somewhere else."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["But you find", "a rare item during", "the battle. What", "should you do?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Give it to who deserves it the most.:Pretend like nothing happened and keep it.:Decide with party who gets it.",
            )],
        )?) != 2
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as("Lady Amy", args!["You end up with the item and you go to Prontera to sell it. There are many people with shops and chat rooms opened selling items."])?;
        ctx.next()?;
        ctx.lines_as("Lady Amy", args!["What should you", "do to sell your item?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Shout out loud to everyone.:Open a chat room and wait.:Inquire if there is anyone that is interested.",
            )],
        )?) != 1
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args![
                "While you are waiting,",
                "someone comes and begs",
                "for items and zeny.",
                "What do you do?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Give them some Zeny and items.:Simply ignore them.:Give suggestions for a place to hunt.",
            )],
        )?) == 3
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args!["Now you decide to go to the Hidden Temple by yourself. You happily ride on your Peco Peco."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["But you run into", "someone that is lost.", "What should you do?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Tell the person how to reach the exit.:Lead the person to the exit.:Give a Butterfly Wing.",
            )],
        )?) != 3
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args![
                "You've been hunting for a while, and now you're low on HP!",
                "It's red now, which is very dangerous."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args![
                "Ah, then a Priest",
                "happens to walk by.",
                "How would you ask",
                "the Priest for a Heal?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Would it be possible to get a heal please?:Can I have a heal?:Heal plz!!",
            )],
        )?) == 1
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as(
            "Lady Amy",
            args!["You are now very", "exhausted and it's time", "to go back to town."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["You then find", "a rare item on", "the street.", "What should", "you do?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Pick it up and keep it.:Ask around to find the owner.:Simply walk by.")],
        )?) != 1
        {
            l_knight_t = (l_knight_t.clone() + Val::from(10));
        }
        ctx.lines_as("Lady Amy", args!["Okay,", "that was the", "end of my test!"])?;
        ctx.next()?;
        ctx.mes("[Lady Amy]")?;
        if l_knight_t.clone() == 100 {
            ctx.var("knight_q").set(Val::from(10))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9008), Val::from(9009)])?;
            ctx.mes("Well done, that kind of mentality is needed for a Knight! For your next test, visit Sir Edmond, please~")?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args!["I'll have nice comments about you for the captain. Do well on the tests you have left, okay?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_knight_t.clone() == 90 {
            ctx.var("knight_q").set(Val::from(10))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9008), Val::from(9009)])?;
            ctx.lines(args![
                "Well, it wasn't perfect,",
                "but I think you know enough",
                "about etiquette to be",
                "a fine Knight."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lady Amy",
                args![
                    "Now, it's time for you to go to Sir Edmond for your next test. Do well on the rest of your tests. You better promise~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.var("knight_q").set(Val::from(9))?;
        ctx.lines(args![
            "Mmm...",
            "To be honest, I don't think your attitude is good enough to be a Knight quite yet."
        ])?;
        ctx.next()?;
        ctx.lines_as("Lady Amy", args!["If you really act like that, everyone will think Knights are rude! Think about how you answered my questions and come again later."])?;
        ctx.next()?;
        ctx.lines_as("Lady Amy", args!["If you want,", "I'll let you", "retake the test, okay?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 10 {
        ctx.lines(args!["Hmmm?", "Why did you", "come to Amy?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["You have to go to", "Sir Edmond for your", "next test, okay?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 14 {
        ctx.lines(args!["Wow~", "Now it's time for", "everyone to decide", "on your job change!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lady Amy",
            args!["Let's go talk to our", "captain. Don't worry", "too much. It should", "be okay."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["Hmmm?", "Why did you", "come to Amy?"])?;
        ctx.next()?;
        ctx.lines_as("Lady Amy", args!["You still have", "other tests to take.", "Hurry and finish~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn lady_amy_knt(ctx: &Ctx) -> Script {
    lady_amy_knt_body(ctx, Vec::new()).map(|_| ())
}

fn sir_edmond_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Sir Edmond]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.lines(args!["Think of your", "mind as if it were", "flowing water."])?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["Flowing water", "avoids obstacles,", "going on its way..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Edmond",
                args!["Knights must be", "able to pass things,", "like calm water, in", "any situation."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("Trees with deep roots don't sway with the wind. The fact that powerful skills must be built on strong basics is immutable...")?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["Your future", "can even be", "decided now..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("Everything in this world exists in harmony. Living without disrupting this harmony is the right way to live...")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.mes("Those with ominous thoughts will only dream such dreams. It's better to have no dreams at all than to have dreams of sadness and despair.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()?.number()? >= 1 && ctx.var("knight_q").get()?.number()? <= 9) {
        ctx.lines(args!["What is it...", "Wandering Swordman?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sir Edmond",
                args![
                    "A seed must first be nestled",
                    "in the earth before the seed may sprout. Then, the sprout must grow leaves before its buds blossom into flowers..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["If not...", "The flower will", "be incomplete."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Edmond",
                args!["Go to the others", "first, so that you", "may find your path..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sir Edmond",
            args![
                "The life that you",
                "want will soon be",
                "before your eyes.",
                "Everything will",
                "come in perfect",
                "order."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 10 {
        ctx.lines(args!["What is it...", "Wandering Swordman."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Lady Amy sent me.:Oh, nothing.")])?) == 1 {
            ctx.lines_as(
                "Sir Edmond",
                args!["It is now time to take my test. Please do your best, as you have done on the other tests."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Edmond",
                args!["My name is", "Edmond Groster.", "I am a member of", "the Prontera Chivalry."],
            )?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["Knights are in the position for others to follow. Therefore, you must modestly think about the world's order and have the personality to fit the role you will play."])?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["You must not make careless decisions. Your will should bend as the reeds or be as firm as stone when the situation calls for it."])?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["You must not kill monsters without reason and not take joy in doing so. Take this time to quietly think about this on your own..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Edmond",
                args!["Then, we shall", "begin the test.", "Keep in mind", "the quality of", "reverence."],
            )?;
            ctx.close_window()?;
            ctx.var("knight_q").set(Val::from(11))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9009), Val::from(9010)])?;
            ctx.call(Function::Warp, vec![Val::from("job_knt"), Val::from(143), Val::from(57)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sir Edmond", args!["The life you want", "will soon be before", "your eyes."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 11 {
        ctx.lines(args!["What is it...", "Wandering Swordman?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I'm sorry, I didn't mean to...:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sir Edmond",
                args!["You were too careless in the last test. A Knight's sword exists to protect others, not to torment weaker monsters."],
            )?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["In a world where everything exists in harmony, you can't have humans continuously destroying without purpose. This principle applies to the real world, not to this test alone."])?;
            ctx.next()?;
            ctx.lines_as("Sir Edmond", args!["The test", "shall begin.", "Show me your", "patience..."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("job_knt"), Val::from(143), Val::from(57)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sir Edmond",
            args![
                "The life that you",
                "want will soon be",
                "before your eyes.",
                "Everything will",
                "come in perfect",
                "order."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()? == 12 || ctx.var("knight_q").get()? == 13) {
        ctx.lines(args![
            "I have seen your character for myself. It is now time for you to take the last test. Go and speak",
            "to Sir Gray..."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "Go and speak",
            "to our captain.",
            "The time has come",
            "for all of us to",
            "evaluate your",
            "performance."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sir_edmond_knt(ctx: &Ctx) -> Script {
    sir_edmond_knt_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TimerKntStep {
    Start,
    OnTimer300000,
    OnTimer300500,
    OnTimer301500,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
}

fn timer_knt_run(ctx: &Ctx, mut step: TimerKntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerKntStep::Start => {
                step = TimerKntStep::OnTimer300000;
                continue 'machine;
            }
            TimerKntStep::OnTimer300000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#knt")])?;
                return Err(Stop::End);
            }
            TimerKntStep::OnTimer300500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#knt::OnDisable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#knt")])?;
                return Err(Stop::End);
            }
            TimerKntStep::OnTimer301500 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#knt::OnEnable")])?;
                return Err(Stop::End);
            }
            TimerKntStep::OnInit => {
                step = TimerKntStep::OnEnable;
                continue 'machine;
            }
            TimerKntStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Timer#knt")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(141),
                        Val::from(57),
                        Val::from("Poring"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(145),
                        Val::from(57),
                        Val::from("Poring"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(55),
                        Val::from("Poring"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(59),
                        Val::from("Poring"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(141),
                        Val::from(55),
                        Val::from("Lunatic"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(141),
                        Val::from(59),
                        Val::from("Lunatic"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(145),
                        Val::from(55),
                        Val::from("Lunatic"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(145),
                        Val::from(59),
                        Val::from("Lunatic"),
                        Val::from(1063),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(139),
                        Val::from(57),
                        Val::from("Chonchon"),
                        Val::from(1011),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(147),
                        Val::from(57),
                        Val::from("Chonchon"),
                        Val::from(1011),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(53),
                        Val::from("Chonchon"),
                        Val::from(1011),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(143),
                        Val::from(61),
                        Val::from("Chonchon"),
                        Val::from(1011),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(165),
                        Val::from(54),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(165),
                        Val::from(57),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(122),
                        Val::from(54),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(122),
                        Val::from(57),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Timer#knt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerKntStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_knt"), Val::from("Timer#knt::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Timer#knt")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#knt")])?;
                return Err(Stop::End);
            }
            TimerKntStep::OnMyMobDead => {
                ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(353), Val::from(251)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn timer_knt(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::Start, Vec::new()).map(|_| ())
}

pub fn timer_knt_ontimer300000(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn timer_knt_ontimer300500(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnTimer300500, Vec::new()).map(|_| ())
}

pub fn timer_knt_ontimer301500(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnTimer301500, Vec::new()).map(|_| ())
}

pub fn timer_knt_oninit(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnInit, Vec::new()).map(|_| ())
}

pub fn timer_knt_onenable(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn timer_knt_ondisable(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn timer_knt_onmymobdead(ctx: &Ctx) -> Script {
    timer_knt_run(ctx, TimerKntStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WarpKntStep {
    Start,
    OnInit,
    OnTouch,
}

fn warp_knt_run(ctx: &Ctx, mut step: WarpKntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WarpKntStep::Start => {
                step = WarpKntStep::OnInit;
                continue 'machine;
            }
            WarpKntStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#knt")])?;
                return Err(Stop::End);
            }
            WarpKntStep::OnTouch => {
                ctx.var("knight_q").set(Val::from(12))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(9010), Val::from(9011)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_in"), Val::from(80), Val::from(100)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_knt(ctx: &Ctx) -> Script {
    warp_knt_run(ctx, WarpKntStep::Start, Vec::new()).map(|_| ())
}

pub fn warp_knt_oninit(ctx: &Ctx) -> Script {
    warp_knt_run(ctx, WarpKntStep::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_knt_ontouch(ctx: &Ctx) -> Script {
    warp_knt_run(ctx, WarpKntStep::OnTouch, Vec::new()).map(|_| ())
}

fn sir_gray_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_knight_t = Val::from(0);
    ctx.mes("[Sir Gray]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.lines(args![
                "The glint of light",
                "that shines off this blade cannot be put into words. This is the weapon every Knight must have."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args!["Yes...", "^3355FFClaymore^000000!", "Every Knight", "would want one!"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("About ^3355FFClaymore^000000:Buy Claymore:End Conversation")])? {
                1 => {
                    ctx.lines_as("Sir Gray", args!["Claymore, one of the best among the famous swords you can attain in Rune-Midgarts' Prontera!! Its value is priceless when considered by a Knight."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Gray", args!["Now, the Prontera Chivalry is making these fabulous Claymores. For Knights, they are only ^3355FF74,000^000000 Zeny."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Gray", args!["But not only that, you need", "1 ^3355FFSteel^000000 because of the Claymore's characteristics. If you like, I can create one for you. For the honor of the Prontera Chivalry!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 1800 {
                        ctx.lines_as("Sir Gray", args!["Oh no...", "It seems that you are carrying too many things. You don't have enough space for a heavy Claymore in your inventory."])?;
                        ctx.next()?;
                        ctx.lines_as("Sir Gray", args!["Why don't you", "go and organize", "your items first."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ((ctx.var("Zeny").get()?.number()? > 73999
                            && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 0)
                            && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?))
                        {
                            ctx.lines_as(
                                "Sir Gray",
                                args![
                                    "You are ready!",
                                    "You must know the",
                                    "true value of",
                                    "a Claymore!",
                                    "I shall make",
                                    "it right now!!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Sir Gray", args!["The basics of", "making the Claymore", "is easy. Watch~!"])?;
                            ctx.next()?;
                            ctx.lines(args!["^3355FF*Stir Stir*^000000", "^3355FF*Ooncha Ooncha*^000000"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sir Gray",
                                args![
                                    "Okay, it's ready!",
                                    "Every Knight's pride:",
                                    "a fine ^3355FFClaymore^000000.",
                                    "You attained a reliable item.",
                                    "It'll be a good companion on your adventures."
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(74000))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(1163), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "I realize you may really want a Claymore, but I can't make it without the materials.",
                                "^3355FF74,000 zeny^000000 and ^3355FF1 Steel!^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Come back when", "you have everything", "ready. I shall be", "waiting..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                3 => {
                    ctx.lines_as("Sir Gray", args!["Any Knight should be able to wield a Claymore as if it were an extension of their body. I used to look forward to brandishing my Claymore in battle..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["Believe it", "or not, I was", "once a Novice", "as well."])?;
            ctx.next()?;
            ctx.lines_as("Sir Gray", args!["I never really planned to become a Knight, but I did decide to become a strong person. Somehow, along my journeys, I ended up joining the Prontera Chivalry. Ha ha ha!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["Young one,", "use your time", "wisely."])?;
        ctx.next()?;
        ctx.lines_as("Sir Gray", args!["No point in", "harboring regret", "once time has passed."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.lines(args!["Young one,", "use your time", "wisely."])?;
        ctx.next()?;
        ctx.lines_as("Sir Gray", args!["No point in", "harboring regret", "once time has passed."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()? == 12 || ctx.var("knight_q").get()? == 13) {
        if ctx.var("knight_q").get()? == 12 {
            ctx.lines(args!["Oh...", "A young Swordman.", "Yes, what can", "I do for you?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as("Sir Gray", args!["Hoho, I see.", "So you took", "everyone else's", "test?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Gray",
                    args!["Then shall", "we begin mine?", "It's not really", "a test though."],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["Let's talk", "casually,", "shall we?"])?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["First...", "Why did you", "decide to become", "a Knight?"])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Sir Gray", args!["Take care!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("knight_q").get()? == 13 {
            ctx.lines(args!["Ah, you again.", "What brings you", "to me?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I've been thinking a lot.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as("Sir Gray", args!["Is that so...", "I wonder if you", "truly have..."])?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["Then...", "Like last time,", "I will ask again..."])?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["First...", "Why did you", "decide to become", "a Knight?"])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Sir Gray", args!["Take care!", "Health is", "every man's", "treasure!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        match runtime::select_values(
            ctx,
            &[Val::from(
                "To become stronger...:To help my guild...:Because I'm unsatisfied with myself right now...",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Sir Gray",
                    args![
                        "To become stronger, you say?",
                        "Yes, Knights are indeed strong.",
                        "But why gain strength?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["Is it to show off to others? To attain fame? Or do you have a diferent reason? What do you think is so good about gaining strength as a Knight?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Gain wealth and fame.:I can protect myself.:I can protect others.")],
                )? {
                    1 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(10));
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Of course, wealth and fame have their place in the world. But we as Knights must live for higher virtues."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Sir Gray", args!["Good thinking. You must first be able to protect yourself in order to protect others. To this end, you must constantly train, and never give in to laziness."])?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Ah, a wonderful idea. A Knight's strength must be used to protect the weak and defend righteousness."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Sadly, there are a few Knights who shame us by forgetting the ideals that should be basic to Knighthood..."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as("Sir Gray", args!["Ah, to help your guild, or maybe even your party. Our wise and benevolent King Tristram the 3rd gave us these golden words..."])?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["^8B7500Beyond the calm river, lies a dangerous waterfall. Therefore, you must always be prepared for everything...^000000"])?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["So how do you", "think you can", "help your guild?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "My guild needs me.:I can help gather funds for my guild.:I can protect my guild members.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Anyone, anywhere in this world,",
                                "has a place where they are needed. Never neglect someone in need, even if he is not a guild member."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(10));
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Of course wealth is important.", "But we Knights must live for higher virtues."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Ah, a wonderful idea. A Knight's strength must be used to protect the weak and defend righteousness."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Sadly, there are a few Knights who shame us by forgetting the ideals that should be basic to Knighthood..."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            3 => {
                l_knight_t = (l_knight_t.clone() + Val::from(5));
                ctx.lines_as(
                    "Sir Gray",
                    args![
                        "Satisfaction, you say.",
                        "It seems like you are",
                        "already a fine Swordman.",
                        "Is there a particular reason you wish to be a Knight?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Gray", args!["I don't know about", "Swordmen, but Knights do not allow self-indulgence. There are those so obsessed with gaining strength that they cannot control themselves."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Gray",
                    args!["So...", "What part of yourself", "are you not satisfied", "with right now?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Skills.:Goal.:Appearance.")])? {
                    1 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(5));
                        ctx.lines_as("Sir Gray", args!["Skill is something you gain with experience as a Knight. It cannot be your highest goal. Otherwise, you'll never be satisfied as a Knight."])?;
                        ctx.next()?;
                    }
                    2 => {
                        l_knight_t = (l_knight_t.clone().try_sub(Val::from(5))?);
                        ctx.lines_as("Sir Gray", args!["I see...", "Always having a goal is very important. You may be full of ideas upon becoming a Knight, but that may change with time."])?;
                        ctx.next()?;
                    }
                    3 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(5));
                        ctx.lines_as("Sir Gray", args!["Oh no...", "What you see isn't what really counts. A Swordman may be stronger than a Knight, and even Knight may grow weak if he becomes lazy."])?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ctx.lines_as(
            "Sir Gray",
            args![
                "I understand your thoughts,",
                "but there are those who wish to",
                "become Knights without thinking."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Gray",
            args!["Those are the ones who instigate problems and shame the honor of Knights, bringing irreversible results."],
        )?;
        ctx.next()?;
        ctx.lines_as("Sir Gray", args!["The same goes for you as well. Once you become a Knight, you can never become a Swordman again. The duties and responsibilities of a Knight will always be with you."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Gray",
            args!["If you become a Knight right away, what are you going to do first?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "I am going to go straight to battle.:There are those waiting for me.:I will learn more about Knights.",
            )],
        )? {
            1 => {
                ctx.lines_as("Sir Gray", args!["Battle...?", "And then?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "I will grow within a short period of time.:I would like to test my ability as a Knight.:I would like to go to more challenging places.",
                    )],
                )? {
                    1 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(10));
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Don't be in too much of a hurry to become strong. Even if you become",
                                "a Knight, you are still yourself."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Sir Gray", args!["Testing yourself is a good thing. It's okay to be happy about how you change, but don't forget about the true qualities of being a Knight."])?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Even if you become a Knight, you are not changing your inner self. No need to overwork yourself.",
                                "Relax and take things step by step."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as("Sir Gray", args!["Who is", "waiting for you?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("My friends.:My Guild members.:My Lover.")])? {
                    1 => {
                        ctx.lines_as("Sir Gray", args!["I see, they would share in the joy of your achievements. Don't ever lose your kind heart, and always give help to your friends."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Those who would share in your happiness and hardship. As a Knight, you must always protect them."],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "Oh, youth!",
                                "Becoming a Knight",
                                "for your beloved!",
                                ((Val::from("Always protect ")
                                    + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                        Val::from("her")
                                    } else {
                                        Val::from("him")
                                    }))
                                    + Val::from("...")),
                                "Even at the sacrifice",
                                "of your own life!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Gray",
                            args!["Also...", "Love them forever.", "Sincere affection", "is hard to find."],
                        )?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            3 => {
                ctx.lines_as("Sir Gray", args!["Good attitude...", "What do you plan", "on learning?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Comfortable places for Knights to go...:The different paths of a Knight...:Ways to get more money as a Knight...",
                    )],
                )? {
                    1 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(5));
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "There are many places that are comfortable or uncomfortable in this world. However Knights must",
                                "be able to survive anywhere."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Sir Gray",
                            args![
                                "There are many similar Knights outside in the world. Think of them as your seniors and ask many questions."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    3 => {
                        l_knight_t = (l_knight_t.clone() + Val::from(15));
                        ctx.lines_as("Sir Gray", args!["Oh no. Do you hold wealth as a priority of being a Knight? We're not meant to be that way. Come again when you have thought", "more about it..."])?;
                        ctx.next()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ctx.lines_as(
            "Sir Gray",
            args![
                "Oh no, we've been",
                "talking too much...",
                "I apologize for",
                "keeping you here",
                "for so long."
            ],
        )?;
        ctx.next()?;
        if l_knight_t.clone() == 0 {
            ctx.var("knight_q").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9011), Val::from(9012)])?;
            ctx.lines_as("Sir Gray", args!["I enjoyed talking with you. You remind me of myself as a young recruit. Shall we talk to the captain and decide on your", "job change?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args!["Don't worry too", "much, I have a very", "high opinion of you.", "Now, go~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_knight_t.clone() == 5 {
            ctx.var("knight_q").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9011), Val::from(9012)])?;
            ctx.lines_as(
                "Sir Gray",
                args!["I enjoyed speaking with you. You can think about the principles of Knighthood more once you become a Knight."],
            )?;
            ctx.next()?;
            ctx.lines_as("Sir Gray", args!["Then, shall we go to the captain and decide on your job change? Don't worry too much. You are good enough to be a Knight!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_knight_t.clone() == 10 {
            ctx.var("knight_q").set(Val::from(14))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9011), Val::from(9012)])?;
            ctx.lines_as(
                "Sir Gray",
                args![
                    "I enjoyed talking with you. Although, there were some",
                    "things that bothered me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args!["You should go", "to the captain", "so we can decide", "on your job change."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args![
                    "Don't worry too much, coming to take my test means the others have acknowledged you as well.",
                    "Go now...!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.var("knight_q").set(Val::from(13))?;
            ctx.lines_as("Sir Gray", args!["Conversing", "with young ones", "is always enjoyable..."])?;
            ctx.next()?;
            ctx.lines_as("Sir Gray", args!["But it seems as though your dream is elsewhere, or that your focus is hazy. Spend more time as a Swordman, and come back", "to me later."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args!["If you truly wish to become a Knight, you must change your outlook first. Then, we shall see."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("knight_q").get()? == 14 {
        ctx.lines(args!["I told you", "to go to", "the captain."])?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Gray",
            args!["Everyone will", "carefully make", "their decision,", "so go now!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["Oh...", "A young Swordman.", "What can I do for you?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sir Gray",
                args!["Hoho~", "There are many", "other younger", "Knights in here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Gray",
                args!["If you talk", "to all of them,", "I may review", "you as well."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sir Gray", args!["Take care!", "Health is", "every man's", "treasure!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sir_gray_knt(ctx: &Ctx) -> Script {
    sir_gray_knt_body(ctx, Vec::new()).map(|_| ())
}
