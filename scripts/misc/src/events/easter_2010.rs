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

fn rina_buff(ctx: &Ctx) -> Result<(), Stop> {
    runtime::npc_skill(ctx, &Val::from("AL_INCAGI"), &Val::from(1), &Val::from(0), &Val::from(0))?;
    runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(1), &Val::from(0), &Val::from(0))?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum RinaEasterStep {
    Start,
    HuntingInfo,
    AfterHunting,
}

fn rina_easter_run(ctx: &Ctx, mut step: RinaEasterStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_baseexp = Val::from(0);
    let mut l_nbaseexp = Val::from(0);
    let mut l_njobexp = Val::from(0);
    let mut l_quest1 = Val::from(0);
    let mut l_quest2 = Val::from(0);
    let mut l_quest3 = Val::from(0);
    'machine: loop {
        match step {
            RinaEasterStep::Start => {
                if ctx.player().base_level()? < 40 {
                    ctx.lines_as("Rina", args!["Hi~!", "You are an adventurer like me."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rina",
                        args![
                            "I am put under a curse.",
                            "I know you want to help me,",
                            "but your experience is not enough."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::ConsumeItem, args![607])?;
                    ctx.call(Function::ConsumeItem, args![12068])?;
                    ctx.call(Function::ConsumeItem, args![12063])?;
                    ctx.call(Function::ConsumeItem, args![12053])?;
                    rina_buff(ctx)?;
                    ctx.lines_as(
                        "Rina",
                        args![
                            "I am not in the good condition,",
                            "so what I can do for you",
                            "is just like this.",
                            "Then, good bye.",
                            "Take care~!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_quest1 = ctx.call(Function::CheckQuest, args![9117])?;
                if l_quest1 == -1 {
                    {
                        ctx.call(Function::PlayBgm, args!["30.mp3"])?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Hi~!",
                                "You are an adventurer like me.",
                                "Well... In fact, I am",
                                "put under a strange curse."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "To release this curse,",
                                "somebody needs to do ^800080Oath-taking ceremony^000000",
                                "with me,",
                                "and then needs to solve",
                                "several problems instead of me."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["Let's help ^800080Rina^000000.", "Just skip it."])? == 1 {
                            ctx.call(Function::Emotion, args![constants::ET_OHNO])?;
                            ctx.lines_as(
                                "Rina",
                                args![
                                    "To help me,",
                                    "I need a competent adventurer.",
                                    "If you know those people,",
                                    "I hope you to introduce them to me later."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.call(Function::Emotion, args![constants::ET_COOL])?;
                        ctx.lines_as(
                            "Rina",
                            args!["Are you really going to help me?", "Thank you.", "You are so brave."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::PlayBgm, args!["01.mp3"])?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Then I'll start ^800080Oath-taking ceremony^000000.",
                                " ",
                                "^787878( A mysterious atmosphere hangs in the air. )^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_COUPLECASTING])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Blah blah blah...",
                                " ",
                                "Blah blah blah...",
                                " ",
                                "^787878( ... This is a strange spell. )^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::SpecialEffect, args![constants::EF_SIGNUM])?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Haaaaah~",
                                "^800080Oath-taking ceremony^000000 is done enough now.",
                                "Isn't that so simple?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, args![constants::ET_HUM])?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Okay, from no on,",
                                "you need to go on ^006400a real adventure",
                                "to release my curse^000000.",
                                "Let me know when you are ready."
                            ],
                        )?;
                        ctx.quests().start(9117)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if l_quest1 == 0 || l_quest1 == 1 {
                        step = RinaEasterStep::HuntingInfo;
                        continue 'machine;
                    } else {
                        if l_quest1 == 2 {
                            if ((ctx.quests().check(9118)? < 2 || ctx.quests().check(9119)? < 2) || ctx.quests().check(9120)? < 2)
                                || ctx.quests().check(9121)? < 2
                            {
                                if ((ctx.call(Function::CheckQuest, args![9118, constants::HUNTING])? == 2
                                    || ctx.call(Function::CheckQuest, args![9119, constants::HUNTING])? == 2)
                                    || ctx.call(Function::CheckQuest, args![9120, constants::HUNTING])? == 2)
                                    || ctx.call(Function::CheckQuest, args![9121, constants::HUNTING])? == 2
                                {
                                    ctx.lines_as(
                                        "Rina",
                                        args!["You did it.", "I can feel that my body is recovering.", "But it is not enough."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Rina",
                                        args!["^006400You have one more thing to do.^000000", "Let me know when you're ready."],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.quests().complete(9118)?;
                                    ctx.quests().complete(9119)?;
                                    ctx.quests().complete(9120)?;
                                    ctx.quests().complete(9121)?;
                                    ctx.quests().start(9122)?;
                                } else {
                                    step = RinaEasterStep::HuntingInfo;
                                    continue 'machine;
                                }
                                return Err(Stop::End);
                            }
                        }
                    }
                    step = RinaEasterStep::AfterHunting;
                    continue 'machine;
                }
            }
            RinaEasterStep::HuntingInfo => {
                ctx.lines_as(
                    "Rina",
                    args![
                        "What you need to do is",
                        "to choose one monster among",
                        "^FF0000DEVIRUCHI,^000000 ^FF0000WRAITH DEAD,^000000",
                        "^FF0000DULLAHAN,^000000 ^FF0000NIGHTMARE TERROR^000000",
                        "and then kill ^0000FF50^000000 monsters",
                        "and come back to me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rina",
                    args![
                        "It must be a tough task,",
                        "but you are the right person",
                        "who did the oath-taking ceremony.",
                        "I hope you succeed."
                    ],
                )?;
                if l_quest1.number()? < 2 {
                    ctx.quests().complete(9117)?;
                    ctx.quests().start(9118)?;
                    ctx.quests().start(9119)?;
                    ctx.quests().start(9120)?;
                    ctx.quests().start(9121)?;
                }
                ctx.close_window()?;
                rina_buff(ctx)?;
                return Err(Stop::End);
            }
            RinaEasterStep::AfterHunting => {
                l_quest2 = ctx.call(Function::CheckQuest, args![9122])?;
                if l_quest2 == 0 || l_quest2 == 1 {
                    ctx.lines_as(
                        "Rina",
                        args!["I will check one thing.", "For this task,", "You need to feel the music."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rina",
                        args!["You cannot complete the task", "without the music.", "do you have any problem?"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "- Check! -",
                        "^787878- BGM option should be turned on -^000000",
                        "^787878- in the game option menu. -^000000",
                        "^787878- Please check -^000000",
                        "^787878- whether you can listen -^000000",
                        "^787878- to the music sound. -^000000"
                    ])?;
                    ctx.next()?;
                    if ctx.menu(&["[I'm ready to listen to the BGM.]", "[I can't listen to the BGM.]"])? == 1 {
                        ctx.lines_as(
                            "Rina",
                            args!["I am so shocked that", "you cannot feel the music.", "What should we do now..."],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Emotion, args![constants::ET_CRY])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Rina",
                        args![
                            "Listen carefully",
                            "the feelings of it...",
                            "I can't hear it,",
                            "but you can hear the sound."
                        ],
                    )?;
                    ctx.next()?;
                    if ((ctx.var("oversea_event2").get()?.number()? < 520 || ctx.var("oversea_event3").get()?.number()? < 270)
                        || ctx.var("oversea_event6").get()?.number()? < 245)
                        || ctx.var("oversea_event9").get()?.number()? < 197
                    {
                        if ctx.var("oversea_event2").get()? == 520 {
                            ctx.call(Function::PlayBgm, args!["13.mp3"])?;
                        } else {
                            if ctx.var("oversea_event3").get()? == 270 {
                                ctx.call(Function::PlayBgm, args!["59.mp3"])?;
                            } else {
                                if ctx.var("oversea_event6").get()? == 245 {
                                    ctx.call(Function::PlayBgm, args!["70.mp3"])?;
                                } else {
                                    if ctx.var("oversea_event9").get()? == 197 {
                                        ctx.call(Function::PlayBgm, args!["94.mp3"])?;
                                    } else {
                                        'b1: {
                                            let subject1 = ctx.call(Function::Rand, args![1, 4])?;
                                            let mut matched1 = false;
                                            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                                                && !subject1.loosely_equals(&Val::from(2))
                                                && !subject1.loosely_equals(&Val::from(3))
                                                && !subject1.loosely_equals(&Val::from(4));
                                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                                matched1 = true;
                                            }
                                            if matched1 {
                                                ctx.call(Function::PlayBgm, args!["13.mp3"])?;
                                                ctx.var("oversea_event2").set(Val::from(520))?;
                                                break 'b1;
                                            }
                                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                                matched1 = true;
                                            }
                                            if matched1 {
                                                ctx.call(Function::PlayBgm, args!["59.mp3"])?;
                                                ctx.var("oversea_event3").set(Val::from(270))?;
                                                break 'b1;
                                            }
                                            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                                                matched1 = true;
                                            }
                                            if matched1 {
                                                ctx.call(Function::PlayBgm, args!["70.mp3"])?;
                                                ctx.var("oversea_event6").set(Val::from(245))?;
                                                break 'b1;
                                            }
                                            if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                                                matched1 = true;
                                            }
                                            if matched1 {
                                                ctx.call(Function::PlayBgm, args!["94.mp3"])?;
                                                ctx.var("oversea_event9").set(Val::from(197))?;
                                                break 'b1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ctx.lines_as(
                        "Rina",
                        args![
                            "^006400The music you're hearing now^000000",
                            "has something to do with the one specific city.",
                            "Think carefully ^006400what kind of city^000000",
                            "has similar feeling with this music."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rina",
                        args!["And...", "Go to ^006400the city", "where you can remind by this music^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rina",
                        args![
                            "After that,",
                            "find ^FF0000the strange mark^000000",
                            "around the entrances of the city.",
                            "Then you can release the curse on me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Rina", args!["It must be tough", "but I hope you good luck."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Emotion, args![constants::ET_OHNO])?;
                    return Err(Stop::End);
                } else {
                    l_quest3 = ctx.call(Function::CheckQuest, args![9123])?;
                    if l_quest3 == 0 || l_quest3 == 1 {
                        ctx.call(Function::Emotion, args![constants::ET_CHUP])?;
                        ctx.lines_as(
                            "Rina",
                            args!["You're back~!", "My curse has been released.", "Thank you so much."],
                        )?;
                        ctx.next()?;
                        if ctx.call(Function::CheckWeight, args![5852, 1])? == 0 {
                            ctx.lines_as(
                                "Rina",
                                args![
                                    "Your bag is too full.",
                                    "I have a present for you",
                                    "so make your bag lighter.",
                                    "I'll wait for you."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.quests().complete(9123)?;
                        ctx.items().give(5852, 1)?;
                        l_baseexp = (((ctx
                            .var("BaseLevel")
                            .get()?
                            .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(4))?))?)
                        .try_mul(
                            ((ctx.var("BaseLevel").get()?.try_div(Val::from(29))?)
                                + (ctx.var("BaseLevel").get()?.try_div(Val::from(6))?)),
                        )?) + ((Val::from(5).try_mul(ctx.var("BaseLevel").get()?)?).try_div(Val::from(2))?));
                        if ctx.var("advjob").get()? == 0 {
                            if ctx.player().base_level()? < 40 {
                                l_nbaseexp = l_baseexp.clone();
                            } else {
                                if ctx.player().base_level()? < 50 {
                                    l_nbaseexp =
                                        (l_baseexp.clone() + (ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?));
                                } else {
                                    if ctx.player().base_level()? < 60 {
                                        l_nbaseexp = (l_baseexp.clone()
                                            + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(20))?))?));
                                    } else {
                                        if ctx.player().base_level()? < 70 {
                                            l_nbaseexp = (l_baseexp.clone()
                                                + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                    .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(10))?))?));
                                        } else {
                                            if ctx.player().base_level()? < 80 {
                                                l_nbaseexp = (l_baseexp.clone()
                                                    + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                        .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(5))?))?));
                                            } else {
                                                if ctx.player().base_level()? < 90 {
                                                    l_nbaseexp = (l_baseexp.clone()
                                                        + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                            .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(3))?))?));
                                                } else {
                                                    if ctx.player().base_level()? < 99 {
                                                        l_nbaseexp = (l_baseexp.clone()
                                                            + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                                .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(2))?))?));
                                                    } else {
                                                        l_nbaseexp = (l_baseexp.clone()
                                                            + (((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                                .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(2))?))?)
                                                            .try_mul(Val::from(2))?));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            l_njobexp = ((((ctx
                                .var("JobLevel")
                                .get()?
                                .try_mul((ctx.var("JobLevel").get()?.try_sub(Val::from(3))?))?)
                            .try_mul(((ctx.var("JobLevel").get()?.try_div(Val::from(25))?) + Val::from(1)))?)
                                + (Val::from(16).try_sub((ctx.var("JobLevel").get()?.try_mul(Val::from(2))?))?))
                            .try_mul(Val::from(2))?);
                        } else {
                            if ctx.player().base_level()? < 30 {
                                l_nbaseexp = l_baseexp.clone();
                            } else {
                                if ctx.player().base_level()? < 40 {
                                    l_nbaseexp = (l_baseexp.clone() + (ctx.var("BaseLevel").get()?.try_mul(Val::from(10))?));
                                } else {
                                    if ctx.player().base_level()? < 50 {
                                        l_nbaseexp = (l_baseexp.clone()
                                            + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                .try_mul(Val::from(2))?));
                                    } else {
                                        if ctx.player().base_level()? < 60 {
                                            l_nbaseexp = (l_baseexp.clone()
                                                + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                    .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(10))?))?));
                                        } else {
                                            if ctx.player().base_level()? < 70 {
                                                l_nbaseexp = (l_baseexp.clone()
                                                    + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                        .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(5))?))?));
                                            } else {
                                                if ctx.player().base_level()? < 80 {
                                                    l_nbaseexp = (l_baseexp.clone()
                                                        + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                            .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(3))?))?));
                                                } else {
                                                    if ctx.player().base_level()? < 90 {
                                                        l_nbaseexp = (l_baseexp.clone()
                                                            + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                                .try_mul((ctx.var("BaseLevel").get()?.try_div(Val::from(2))?))?));
                                                    } else {
                                                        if ctx.player().base_level()? < 98 {
                                                            l_nbaseexp = (l_baseexp.clone()
                                                                + ((ctx.var("BaseLevel").get()?.try_mul(ctx.var("BaseLevel").get()?)?)
                                                                    .try_mul(ctx.var("BaseLevel").get()?)?));
                                                        } else {
                                                            l_nbaseexp = (l_baseexp.clone()
                                                                + (((ctx
                                                                    .var("BaseLevel")
                                                                    .get()?
                                                                    .try_mul(ctx.var("BaseLevel").get()?)?)
                                                                .try_mul(ctx.var("BaseLevel").get()?)?)
                                                                .try_mul(Val::from(2))?));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            l_njobexp = ((((ctx.var("JobLevel").get()?.try_mul(ctx.var("JobLevel").get()?)?)
                                .try_mul(((ctx.var("JobLevel").get()?.try_div(Val::from(5))?) + Val::from(2)))?)
                                + (Val::from(20).try_sub(ctx.var("JobLevel").get()?)?))
                            .try_mul(Val::from(3))?);
                        }
                        ctx.call(Function::GetExperience, args![l_nbaseexp.clone(), l_njobexp.clone()])?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "This is the present for you.",
                                "It's not a big one",
                                "but please take it",
                                "as a token of my gratitude."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rina",
                            args![
                                "I'll take a rest for a few days",
                                "and then I'll go on an adventure.",
                                "See you again."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::ConsumeItem, args![607])?;
                        ctx.call(Function::ConsumeItem, args![12068])?;
                        ctx.call(Function::ConsumeItem, args![12063])?;
                        ctx.call(Function::ConsumeItem, args![12053])?;
                        rina_buff(ctx)?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Rina",
                            args![
                                "Thank you for releasing my curse.",
                                "I'll take a rest for a few days",
                                "and then go on an adventure again.",
                                "Have a happy day~!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::ConsumeItem, args![607])?;
                        rina_buff(ctx)?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn rina_easter(ctx: &Ctx) -> Script {
    rina_easter_run(ctx, RinaEasterStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Hiddne01easterStep {
    Start,
    OnRelease,
}

fn hiddne01easter_run(ctx: &Ctx, mut step: Hiddne01easterStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hiddne01easterStep::Start => {
                if ctx.var("oversea_event2").get()?.number()? > 519 {
                    step = Hiddne01easterStep::OnRelease;
                    continue 'machine;
                }
                return Err(Stop::End);
            }
            Hiddne01easterStep::OnRelease => {
                ctx.call(Function::NpcSpecialEffect, args![constants::EF_PATTACK])?;
                ctx.mes("- I found ^0000FFthe strange mark^000000. -")?;
                ctx.next()?;
                ctx.call(Function::SpecialEffect, args![constants::EF_HOLYHIT])?;
                ctx.lines(args![
                    "- I can definitely feel that",
                    "- ^0000FFRina^000000 has been released",
                    "- from the curse.",
                    "- ^006400Let's go back to Rina!^000000"
                ])?;
                if ctx.quests().check(9122)? < 2 {
                    ctx.quests().complete(9122)?;
                    ctx.quests().start(9123)?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hiddne01easter(ctx: &Ctx) -> Script {
    hiddne01easter_run(ctx, Hiddne01easterStep::Start, Vec::new()).map(|_| ())
}

pub fn hiddne01easter_onrelease(ctx: &Ctx) -> Script {
    hiddne01easter_run(ctx, Hiddne01easterStep::OnRelease, Vec::new()).map(|_| ())
}

pub fn hiddne02easter(ctx: &Ctx) -> Script {
    if ctx.var("oversea_event3").get()?.number()? > 269 {
        ctx.call(Function::DoEvent, args!["#Hiddne01Easter::OnRelease"])?;
    }
    ctx.end()
}

pub fn hiddne03easter(ctx: &Ctx) -> Script {
    if ctx.var("oversea_event6").get()?.number()? > 244 {
        ctx.call(Function::DoEvent, args!["#Hiddne01Easter::OnRelease"])?;
    }
    ctx.end()
}

pub fn hiddne04easter(ctx: &Ctx) -> Script {
    if ctx.var("oversea_event9").get()?.number()? > 196 {
        ctx.call(Function::DoEvent, args!["#Hiddne01Easter::OnRelease"])?;
    }
    ctx.end()
}

pub fn rina_s_little_friend(ctx: &Ctx) -> Script {
    let mut l_ncharge = Val::from(0);
    let mut l_npercentage = Val::from(0);
    if ctx.call(Function::CheckQuest, args![9117])? == -1 {
        ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
        ctx.lines_as("Rina's Little Friend", args![".......", " ", "^787878( No response. )^000000"])?;
        return ctx.close();
    }
    if ctx.call(Function::CheckQuest, args![9123])? == 2 {
        ctx.call(Function::Emotion, args![constants::ET_MERONG])?;
        ctx.lines_as(
            "Rina's Little Friend",
            args![
                "Hi.",
                "I guess you're the trustworthy friend.",
                "Are you here to make a deal with me?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.:What do you mean?")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.var("advjob").get()? == 0 {
                    if ctx.player().base_level()? < 70 {
                        l_ncharge = Val::from(400000);
                    } else {
                        if ctx.player().base_level()? < 90 {
                            l_ncharge = Val::from(450000);
                        } else {
                            l_ncharge = Val::from(480000);
                        }
                    }
                } else {
                    l_ncharge = Val::from(500000);
                }
                if (ctx.items().count(574)? < 1 || ctx.items().count(1001)? < 20) || ctx.player().zeny()? < l_ncharge.number()? {
                    ctx.lines_as(
                        "Rina's Little Friend",
                        args!["Hmm.", "The material is not enough.", "Please check the things you need."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "Cool... if you give me the materials",
                        "I will start it right now.",
                        "Are you ready for it?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Yes, let's start it.:No, stop it.")])?) == 2 {
                    ctx.lines_as("Rina's Little Friend", args!["Heh. It's boring."])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "Okay, I will start it now.",
                        " ",
                        "^787878( A mysterious atmosphere hangs in the air. )^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::NpcSpecialEffect, args![constants::EF_ASPERSIO])?;
                ctx.items().take(574, 1)?;
                ctx.items().take(1001, 20)?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_ncharge.clone())?))?;
                l_npercentage = ctx.call(Function::Rand, args![1, 100])?;
                if l_npercentage.number()? <= 41 {
                    ctx.lines_as(
                        "Rina's Little Friend",
                        args!["Life is given to the egg.", "Aaaaah~ I got to take some rest.", "Good bye~!"],
                    )?;
                    if l_npercentage.number()? <= 12 {
                        ctx.items().give(9003, 1)?;
                    } else {
                        if l_npercentage.number()? <= 24 {
                            ctx.items().give(9005, 1)?;
                        } else {
                            if l_npercentage.number()? <= 36 {
                                ctx.items().give(9009, 1)?;
                            } else {
                                ctx.items().give(9023, 1)?;
                            }
                        }
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Emotion, args![constants::ET_SLEEPY])?;
                    return ctx.end();
                } else {
                    if l_npercentage.number()? <= 60 {
                        if l_npercentage.number()? >= 42 && l_npercentage.number()? <= 45 {
                            ctx.items().give(5852, 1)?;
                        } else {
                            ctx.items().give(12019, 5)?;
                        }
                        ctx.lines_as(
                            "Rina's Little Friend",
                            args![
                                "It seems that you have quite interesting things.",
                                "I need to take some rest.",
                                "Good bye~!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Emotion, args![constants::ET_SLEEPY])?;
                        return ctx.end();
                    } else {
                        if l_npercentage.number()? <= 85 {
                            if l_npercentage.number()? >= 61 && l_npercentage.number()? <= 75 {
                                ctx.items().give(1001, 20)?;
                                ctx.items().give(607, 1)?;
                            } else {
                                ctx.items().give(574, 1)?;
                                ctx.items().give(608, 2)?;
                            }
                            ctx.lines_as(
                                "Rina's Little Friend",
                                args![
                                    "There was no change.",
                                    "I didn't mean to do it,",
                                    "but I'm sorry...",
                                    "I hope to see you again, my friend."
                                ],
                            )?;
                            ctx.call(Function::Emotion, args![constants::ET_HUM])?;
                            return ctx.close();
                        } else {
                            ctx.items().give(12093, 2)?;
                            ctx.lines_as(
                                "Rina's Little Friend",
                                args!["Hmm.", "It became a dish.", "Looks delicious.", "Then, good bye~!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Emotion, args![constants::ET_HUNGRY])?;
                            return ctx.end();
                        }
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "You are here just to see my cute looks?",
                        "Aren't you peeking at Rina",
                        "pretending to see me?",
                        "It's funny... haha~"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Emotion, args![constants::ET_KIK])?;
                return ctx.end();
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.call(Function::PlayBgm, args!["23.mp3"])?;
                ctx.lines_as(
                    "Rina's Little Friend",
                    args!["Huh?!", "I never told you before?", "Then listen carefully."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "I have a",
                        "special ability.",
                        "It's to give a special strength",
                        "to ^0000FFthe egg^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "Well, I cannot do that unlimitedly.",
                        "There is no magic that is done forever.",
                        "So I need some additional cost."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "And I cannot sure",
                        "to what the egg will be changed.",
                        "It may be end in failure",
                        "and it can be something like Pet Egg."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Ask the cost.:Stop listening because it feels like a trick.")],
                )?) == 2
                {
                    ctx.lines_as("Rina's Little Friend", args!["Well... I don't really care.", "Good bye."])?;
                    ctx.close_window()?;
                    ctx.call(Function::PlayBgm, args!["08.mp3"])?;
                    return ctx.end();
                }
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "Do you want to make a deal?",
                        "You need",
                        "^0000FF1 Egg^000000, ^0000FF20 Light Granule^000000 and"
                    ],
                )?;
                if ctx.var("advjob").get()? == 0 {
                    if ctx.player().base_level()? < 70 {
                        ctx.mes("^B8860B400,000 Zeny.")?;
                    } else {
                        if ctx.player().base_level()? < 90 {
                            ctx.mes("^B8860B450,000 Zeny^000000.")?;
                        } else {
                            ctx.mes("^B8860B480,000 Zeny^000000.")?;
                        }
                    }
                } else {
                    ctx.mes("^B8860B500,000 Zeny^000000.")?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Rina's Little Friend",
                    args![
                        "If you are interested",
                        "make that zeny and come again.",
                        "Haha...",
                        "I'll be waiting for you."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::PlayBgm, args!["08.mp3"])?;
                ctx.call(Function::Emotion, args![constants::ET_KIK])?;
                return ctx.end();
            }
        }
    }
    ctx.call(Function::Emotion, args![constants::ET_MERONG])?;
    ctx.lines_as(
        "Rina's Little Friend",
        args![
            "Ahem!",
            "Why? Are you surprised to see me speaking?",
            "Well, I understand you.",
            "Only the person who did",
            "the oath-taking ceremony can talk with me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rina's Little Friend",
        args!["If Rina's curse is released,", "I can also", "suggest you", "an interesting thing."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rina's Little Friend",
        args![
            "I will let you know",
            "the detailed explanation later.",
            "Then see you later, my friend."
        ],
    )?;
    ctx.close()
}

pub fn traveler_01easter(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Traveler",
        args![
            "I'm a traveler",
            "and I often visit ^8B4513Prontera^000000.",
            "Hmm, I think I might",
            "see you around here before."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Listen some more.", "Just ignore it."])? == 1 {
        ctx.lines_as(
            "Traveler",
            args!["As I expected,", "you are a silent person. Ha ha.", "Good bye."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Traveler",
        args![
            "A few days ago,",
            "I saw a new adventurer",
            "who were standing",
            "near the ^8B4513Cathedral^000000."
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_THROB])?;
    ctx.lines_as(
        "Traveler",
        args![
            "That adventurer seemed pretty.",
            "But...",
            "I feel some strange energy",
            "so I don't come close to that person."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Traveler",
        args![
            "Look like",
            "you're interested in",
            "that new adventurer?",
            "Or that is just your face look. Ha ha."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["I'm interested in it.", "I don't care at all."])? == 1 {
        ctx.lines_as("Traveler", args!["Ah~ I see~!", "Then, Good bye."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Traveler",
        args![
            "Just as I expected!",
            "You can find",
            "that new adventurer",
            "on the way to the ^8B4513Cathedral^000000",
            "at the direction of 1o'clock in ^8B4513Prontera^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Traveler",
        args!["She was carrying", "^800080a big egg thing^000000.", "That's what I know."],
    )?;
    ctx.close()
}

pub fn traveler_01easter_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn traveler_01easter_ontimer5000(ctx: &Ctx) -> Script {
    ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn traveler_02easter(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Traveler",
        args![
            "A while ago,",
            "I met a lady",
            "and she said she has been",
            "under a strange curse."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Traveler",
        args![
            "I wanted to help her",
            "but my experience is not enough.",
            "So I couldn't help her."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Traveler",
        args![
            "If you are more than",
            "level ^0000FF40^000000,",
            "go to the ^8B4513Cathedral^000000",
            "at the direction of 1'o clock.",
            "I hope you can help her..."
        ],
    )?;
    ctx.close()
}

pub fn traveler_02easter_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn traveler_02easter_ontimer5000(ctx: &Ctx) -> Script {
    ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}
