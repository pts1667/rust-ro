use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum ChivalryCaptainKntStep {
    Start,
    LMission,
}

fn chivalry_captain_knt_run(ctx: &Ctx, mut step: ChivalryCaptainKntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ChivalryCaptainKntStep::Start => {
                ctx.mes("[Captain Herman]")?;
                if ctx.var("Upper").get()? == 1 {
                    ctx.mes("Hm? You're... What is it about you? I've been an honorable Knight for a long time, but I cannot understand this feeling I'm getting from you...")?;
                    ctx.next()?;
                    ctx.lines_as("Captain Herman", args!["May god bless your body and soul, warrior. I hope you will show your courage and protect those who are weaker than you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                        ctx.lines(args!["Ah, a member of our Chivalry.", "I hope you are living up to my expectations. We have vowed to be strong for our kingdom, even if death is upon us..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
                        ctx.lines(args!["Welcome,", "this is the", "Prontera Chivalry.", "What brings you here?"])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "I want to change my job to Swordman.:I want to change my job to a Knight.:Just visiting.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Captain Herman",
                                    args![
                                        "A-ha~",
                                        "A Swordman, you say?",
                                        ((Val::from("I'm sorry, ")
                                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                Val::from("lad")
                                            } else {
                                                Val::from("lass")
                                            }))
                                            + Val::from(", but you've")),
                                        "come to the wrong place!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Captain Herman", args!["This isn't the Swordsman guild, it's the Prontera Chivalry! If you wish to become a Swordman, visit the Swordman Guild located in Izlude."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Captain Herman",
                                    args![
                                        "Ah, I see that you have great ambition. But you must first become a Swordman before becoming",
                                        "a Knight. One step at a time..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Captain Herman", args!["First, visit the Swordman guild in Izlude. Then, come visit us again once you have become a well experienced Swordman."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as("Captain Herman", args!["Aha~", "You must have lots of free time. Why don't you go hunt some monsters instead of wandering about aimlessly?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.mes("Welcome. We, the proud Knights of the Prontera Chivalry, will give our lives for king and country! Please enjoy your stay.")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if ctx.var("knight_q").get()? == 0 {
                    ctx.lines(args!["Welcome, this is", "the Prontera Chivalry.", "What brings you here?"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I want to change my job to a Knight.:Just visiting.")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Captain Herman",
                            args![
                                "Ohh...",
                                ((Val::from("A young ")
                                    + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                        Val::from("man")
                                    } else {
                                        Val::from("lady")
                                    }))
                                    + Val::from(" who wishes")),
                                "to become a Knight!",
                                "Our Prontera Chivalry",
                                "will assist you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Captain Herman", args!["First of all, I am the captain of the Prontera Chivalry, Herman Phon Efesirsus. I'm pleased to meet young people eager to join the Prontera Chivalry."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Captain Herman",
                            args![
                                "We only accept Swordmen",
                                "who are at least Job Level 40.",
                                "We cannot consider applicants that are not yet experienced enough to become Knights."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Captain Herman",
                            args![
                                "Once you apply, and we find",
                                "you eligible, we will begin the job change procedure. Would you",
                                "like to apply now?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Yes, I would like to apply.:I'd like to think about it please.")],
                        )?) == 1
                        {
                            ctx.mes("[Captain Herman]")?;
                            if ctx.var("JobLevel").get()?.number()? < 40 {
                                ctx.mes("Ah, you are not yet ready to become a Knight! Didn't I specifically mention the Job Level 40 requirement?")?;
                                ctx.next()?;
                                ctx.lines_as("Captain Herman", args!["Of course I understand your strong desire to join us, but now is not the time. Go out and fight some more monsters. We will be here waiting."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.var("SkillPoint").get()?.is_true() {
                                ctx.lines(args![
                                    "Ah...!",
                                    "You cannot change jobs if you have unused skill points remaining. Return when you have used",
                                    "all of your skill points."
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("knight_q").set(Val::from(1))?;
                            ctx.call(Function::SetQuest, vec![Val::from(9000)])?;
                            ctx.lines(args![
                                "Let me see...",
                                "Your name is",
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("...")),
                                "Is that right?"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Captain Herman",
                                args![
                                    "Let me explain the job change procedure. You must visit a series of Knights and pass each",
                                    "of their tests."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Captain Herman", args!["Once all the tests are completed, every Knight involved in your testing will gather and discuss your performance."])?;
                            ctx.next()?;
                            ctx.lines_as("Captain Herman", args!["The Knights must unanimously approve of you before you can join the Prontera Chivalry. If only one person objects, you must", "start over."])?;
                            ctx.next()?;
                            ctx.lines_as("Captain Herman", args!["But I believe if you persist with an earnest heart, you shall be acknowledged by the Knights and ultimately recognized as a member of our Chivalry."])?;
                            ctx.next()?;
                            ctx.lines_as("Captain Herman", args!["So, let's not waste any more time talking! Go and meet these Knights and begin their tests. Once you have completed all of the tests, come back to me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Captain Herman",
                            args![
                                "Oh...!",
                                "Well, I don't want to pressure you. Take your time and think it over. Return when you are ready to",
                                "change jobs, for we will be waiting."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Captain Herman",
                        args!["Come to think of it, aren't you a Swordman? It looks like you've encountered many foes in battle."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Captain Herman",
                        args!["You should consider changing jobs to a Knight. Come and talk to me if you are interested."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Captain Herman",
                        args!["Please take", "your time in", "looking around.", "Good day."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("knight_q").get()? == 1 {
                        ctx.lines(args![
                            "Mmm?",
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(",")),
                            "what can I do for you?",
                            "Ah, you don't know",
                            "who to visit?"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Captain Herman", args!["I believe the Knights in charge of testing have set an order in which you must visit them. I suppose it helps the testing process."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Captain Herman",
                            args![
                                "First, go and visit",
                                "Sir Andrew for your first test. Don't be too nervous, he'll explain everything once you talk to him."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("knight_q").get()? == 2 {
                            chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(0)])?;
                        } else {
                            if ctx.var("knight_q").get()? == 3 {
                                chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(0)])?;
                            } else {
                                if ctx.var("knight_q").get()? == 4 {
                                    chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(1)])?;
                                    ctx.mes("It appears that you have finished one test. Let's see. Sir Andrew, who must this Swordman visit next?")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Sir Andrew",
                                        args![
                                            "I said to",
                                            "visit Sir Siracuse.",
                                            "Funny, I thought I told",
                                            "you. Did I forget...?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Captain Herman", args!["Did you hear?", "Go to Sir Siracuse and take his test. Once you complete his test, do not forget who you're supposed to visit next as well."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("knight_q").get()? == 5 {
                                        chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(0)])?;
                                    } else {
                                        if ctx.var("knight_q").get()? == 6 {
                                            chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(1)])?;
                                            ctx.lines(args![
                                                "Let's see...",
                                                "You've completed two tests.",
                                                "Sir Siracuse, who must this Swordman visit next?"
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines_as("Sir Siracuse", args!["Oh...!", "Um, who was next...?", "Right! Sir Windsor!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Captain Herman", args!["Head over to", "Sir Windsor Benedict for your next test. Listen carefully to the Knights in charge of testing so that you don't feel lost, alright?"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("knight_q").get()? == 7 {
                                                chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(0)])?;
                                            } else {
                                                if ctx.var("knight_q").get()? == 8 {
                                                    chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::LMission, vec![Val::from(1)])?;
                                                    ctx.lines(args!["Sir Windor...?", "Who must this", "Swordman visit", "next?"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sir Windsor", args!["..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sir Windsor", args!["...", "......"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Sir Windsor", args!["...Amy Beatrice."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Captain Herman", args!["Ah, go and visit", "Lady Amy and take her test. Make sure you pay attention to who you must go to for your next test."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("knight_q").get()? == 9 {
                                                        chivalry_captain_knt_run(
                                                            ctx,
                                                            ChivalryCaptainKntStep::LMission,
                                                            vec![Val::from(0)],
                                                        )?;
                                                    } else {
                                                        if ctx.var("knight_q").get()? == 10 {
                                                            chivalry_captain_knt_run(
                                                                ctx,
                                                                ChivalryCaptainKntStep::LMission,
                                                                vec![Val::from(1)],
                                                            )?;
                                                            ctx.lines(args![
                                                                "Let's see...",
                                                                "Lady Amy, who",
                                                                "must this Swordman",
                                                                "visit next?"
                                                            ])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Lady Amy",
                                                                args!["Oh...", "I said to visit", "Sir Edmond!", "Tee hee~"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "Now, go and speak",
                                                                    "to Sir Edmond. He will",
                                                                    "be in charge of your",
                                                                    "next test."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("knight_q").get()? == 11 {
                                                            chivalry_captain_knt_run(
                                                                ctx,
                                                                ChivalryCaptainKntStep::LMission,
                                                                vec![Val::from(0)],
                                                            )?;
                                                        } else if ctx.var("knight_q").get()? == 12 {
                                                            chivalry_captain_knt_run(
                                                                ctx,
                                                                ChivalryCaptainKntStep::LMission,
                                                                vec![Val::from(1)],
                                                            )?;
                                                            ctx.lines(args![
                                                                "Don't you only have to visit one more person? The Knight in",
                                                                "charge of the final test",
                                                                "is Sir Gray Prospheiro."
                                                            ])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Sir Edmond",
                                                                args![
                                                                    "This world operates according",
                                                                    "to the law of cause and effect.",
                                                                    "All will be revealed in the end."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args!["Be alert and do", "your best, as this", "is the last test."],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args!["Return to me", "after you have", "completed the", "final test."],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("knight_q").get()? == 13 {
                                                            ctx.lines(args!["Finish the last test.", "Once that is complete, all the Knights involved in your testing shall gather, and we will evaluate your performance."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("knight_q").get()? == 14 {
                                                            if ctx.var("SkillPoint").get()?.is_true() {
                                                                ctx.lines(args!["Oh...!", "You cannot change jobs if you have unused skill points remaining. Return once you have distributed all your skill points."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                            ctx.lines(args!["Oh, have you completed all the tests? But not everyone who completes the tests can", "become a Knight."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Captain Herman", args!["During the test we see how loyal, honorable and strong you are. We also see if you were courteous and if you know the value of modesty and reverence."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Captain Herman", args!["Through this process, I have also observed your actions. All seven of our opinions will be reflected in the decision of your job change."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "Then...",
                                                                    "We shall listen",
                                                                    "to everyone's thoughts!",
                                                                    "Andrew, what do you think?"
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.mes("[Sir Andrew]")?;
                                                            if ctx.var("JobLevel").get()? == 50 {
                                                                ctx.lines(args![
                                                                    "What can I say?",
                                                                    "I approve!",
                                                                    "Having lived as",
                                                                    "a Swordsman up",
                                                                    "until now",
                                                                    "is enough."
                                                                ])?;
                                                            } else {
                                                                ctx.lines(args![
                                                                    "This one has",
                                                                    "gathered items",
                                                                    "that are troublesome",
                                                                    "to obtain. I approve!",
                                                                    ((Val::from("I believe ")
                                                                        + (if ctx
                                                                            .var("Sex")
                                                                            .get()?
                                                                            .loosely_equals(&ctx.constant("SEX_MALE")?)
                                                                        {
                                                                            Val::from("")
                                                                        } else {
                                                                            Val::from("s")
                                                                        }))
                                                                        + Val::from(
                                                                            "he will continue to be loyal after becoming a Knight."
                                                                        ))
                                                                ])?;
                                                            }
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args!["Hmm.", "What a nice review.", "Siracuse, what are your thoughts?"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Sir Siracuse",
                                                                args![
                                                                    "Heh, very well. Not quite what",
                                                                    "I would want, but hopefully will become better in the future."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Sir Siracuse", args!["After becoming a Knight, you must build a good reputation through honor. Ehh... I approve."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args!["Okay...", "Windsor,", "what about you?"],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Sir Windsor", args!["..."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Sir Windsor", args!["...", "......"])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Sir Windsor", args!["....Approved."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "I don't think",
                                                                    "he disapproves.",
                                                                    "Then, let's listen",
                                                                    "to Amy's opinion."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.mes("[Lady Amy]")?;
                                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                                ctx.lines(args!["Mmm~ He's so polite!", "He'll grow to be a wonderful Knight. And he's got such cute widdle cheeeeks~ Hee hee!"])?;
                                                            } else {
                                                                ctx.lines(args![
                                                                    "Mmm~ She should be great!",
                                                                    "She's very courteous and also very cute, so a few more points! Heh~",
                                                                    "I shouldn't be saying things like this!"
                                                                ])?;
                                                            }
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "Well...",
                                                                    "A strange review,",
                                                                    "but I believe",
                                                                    "she approves.",
                                                                    "Edmond, speak",
                                                                    "your mind."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.mes("[Sir Edmond]")?;
                                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                                ctx.lines(args!["He seems a little rough, but something bright shines within him. With polish and refinement, his true value will shine forth", "as the sun."])?;
                                                            } else {
                                                                ctx.lines(args!["It's difficult to see, but there is a spiritual beauty within her. With polish and refinement, her true value will glow as resplendently", "as the moon."])?;
                                                            }
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args!["Lastly...", "Gray. I would like", "to hear your thoughts."],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Gray", args![((Val::from("A young ") + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) { Val::from("gentleman") } else { Val::from("lady") })) + Val::from(" coming here with the determination to become a Knight is enough..."))])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "Everyone",
                                                                    "has approved.",
                                                                    "No one has opposed.",
                                                                    "Then I shall tell",
                                                                    "you my opinion."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Captain Herman", args!["My decision is..."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Captain Herman", args!["I approve."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Captain Herman", args!["You may not have finished all the tests perfectly, but you have all the necessary qualities to become", "a Knight."])?;
                                                            ctx.next()?;
                                                            ctx.call(Function::CompleteQuest, vec![Val::from(9012)])?;
                                                            shared::other_global_functions::job_change(
                                                                ctx,
                                                                vec![ctx.constant("JOB_KNIGHT")?],
                                                            )?;
                                                            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "I hereby declare",
                                                                    "you a member of",
                                                                    "the Prontera Chivalry.",
                                                                    "Protect the weak and",
                                                                    "live with honor."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.call(Function::GetItem, vec![Val::from(656), Val::from(7)])?;
                                                            ctx.lines_as("Captain Herman", args!["Oh...", "We have prepared a small gift to congratulate you on your job change. Please use it when you are in battle as you honorably protect others."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Captain Herman",
                                                                args![
                                                                    "Go forth!",
                                                                    "The future of",
                                                                    "Rune-Midgarts",
                                                                    "rests on your",
                                                                    "shoulders!"
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                step = ChivalryCaptainKntStep::LMission;
                continue 'machine;
            }
            ChivalryCaptainKntStep::LMission => {
                ctx.lines(args![
                    "Mmm?",
                    ((Val::from("Swordman ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                    "How are the tests?"
                ])?;
                if runtime::arg(&args, 0, Val::from(0)).is_true() {
                    ctx.lines(args!["Ah~ You do not know", "who to visit next?"])?;
                    ctx.next()?;
                    ctx.mes("[Captain Herman]")?;
                    return Ok(Val::from(0));
                } else {
                    ctx.lines(args!["It may be difficult,", "but do your best."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn chivalry_captain_knt(ctx: &Ctx) -> Script {
    chivalry_captain_knt_run(ctx, ChivalryCaptainKntStep::Start, Vec::new()).map(|_| ())
}

fn sir_andrew_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_items: Vec<Val> = Vec::new();
    ctx.mes("[Sir Andrew]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.lines(args!["You must be", "a member of", "the Chivalry.", "How are you doing?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "You must work diligently to gather food as well as save zeny to buy equipment. Save everything you",
                    "find in battle, even the smallest Jellopy."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args!["But it's not good", "to be too greedy.", "After all, we are Knights."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["Hey there,", "little Novice.", "Welcome to the", "Prontera Chivalry."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args!["You might that you're", "weak right now, but someday", "you'll become stronger."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args!["Dream of a bright future, and go look forward on the path that you choose to take."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "We, the members of the",
                "Prontera Chivalry, are putting our best effort in protecting peace in this world."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "Even during the battles we face each and every day, we dream of",
                    "a bright future that is to come."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.lines(args![
            "We, the members of the",
            "Prontera Chivalry, are putting our best effort in protecting peace in this world."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Andrew",
            args![
                "Even during the battles we face each and every day, we dream of",
                "a bright future that is to come."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 1 {
        ctx.lines(args!["Good day.", "May I help you", "with something?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to take the test.:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "Ah...",
                    "You wish",
                    "to become a Knight.",
                    "Your name is",
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(",")),
                    "correct?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "I am a Knight of",
                    "the Prontera Chivalry,",
                    "Andrew Shylock.",
                    "I am in charge of",
                    "your first test."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Sir Andrew", args!["I will be testing your sense of loyalty. Every Knight must possess this virtue. For this exam, you will be gathering prizes from", "the battlefield."])?;
            ctx.next()?;
            if ctx.var("JobLevel").get()? == 50 {
                ctx.lines_as(
                    "Sir Andrew",
                    args![
                        "Mmm...?",
                        "Hold on there.",
                        "You look like you've",
                        "mastered being",
                        "a Swordsman."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Andrew",
                    args![
                        "Impressive...!",
                        "On second thought,",
                        "I don't think your",
                        "loyalty needs to",
                        "be tested."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Andrew", args!["Please go to my fellow Knight, Sir Siracuse, as he will give you your next test. Well done in mastering the Swordman job."])?;
                ctx.var("knight_q").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(9000), Val::from(9003)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Andrew",
                args!["Without", "further ado,", "let's begin!", "Go and gather the", "following items..."],
            )?;
            ctx.next()?;
            ctx.mes("[Sir Andrew]")?;
            let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
            if subject1 == 1 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1040), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(7006), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(931), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(1057), false);
                runtime::local_set(&mut l_items, &Val::from(base + 7), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 8), Val::from(903), false);
                runtime::local_set(&mut l_items, &Val::from(base + 9), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 10), Val::from(1028), false);
                runtime::local_set(&mut l_items, &Val::from(base + 11), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 12), Val::from(2), false);
            } else if subject1 == 2 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1042), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(950), false);
                runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(1032), false);
                runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(966), false);
                runtime::local_set(&mut l_items, &Val::from(base + 7), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 8), Val::from(7031), false);
                runtime::local_set(&mut l_items, &Val::from(base + 9), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 10), Val::from(946), false);
                runtime::local_set(&mut l_items, &Val::from(base + 11), Val::from(5), false);
                runtime::local_set(&mut l_items, &Val::from(base + 12), Val::from(3), false);
            }
            ctx.var("knight_q").set(runtime::local_get(&l_items, &Val::from(12), false))?;
            if ctx.var("knight_q").get()? == 2 {
                ctx.call(Function::ChangeQuest, vec![Val::from(9000), Val::from(9001)])?;
            } else {
                ctx.call(Function::ChangeQuest, vec![Val::from(9000), Val::from(9002)])?;
            }
            ctx.lines(args![
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(1), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(3), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(5), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(4), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(7), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(6), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(9), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(8), false)])?)
                    + Val::from("^000000 and")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(11), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(10), false)])?)
                    + Val::from("^000000,"))
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "I shall be",
                    "waiting here for",
                    "you to bring the",
                    "items I've listed.",
                    "See you soon~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Sir Andrew", args!["Well, then...", "Good day."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (ctx.var("knight_q").get()? == 2 || ctx.var("knight_q").get()? == 3) {
        ctx.lines(args![
            "Welcome back~",
            "Did you gather",
            "all the items?",
            "Let's check and see..."
        ])?;
        ctx.next()?;
        let subject2 = ctx.var("knight_q").get()?;
        if subject2 == 2 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1040), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(7006), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(931), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(1057), false);
            runtime::local_set(&mut l_items, &Val::from(base + 7), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 8), Val::from(903), false);
            runtime::local_set(&mut l_items, &Val::from(base + 9), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 10), Val::from(1028), false);
            runtime::local_set(&mut l_items, &Val::from(base + 11), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 12), Val::from(0), false);
        } else if subject2 == 3 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1042), false);
            runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(950), false);
            runtime::local_set(&mut l_items, &Val::from(base + 3), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 4), Val::from(1032), false);
            runtime::local_set(&mut l_items, &Val::from(base + 5), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 6), Val::from(966), false);
            runtime::local_set(&mut l_items, &Val::from(base + 7), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 8), Val::from(7031), false);
            runtime::local_set(&mut l_items, &Val::from(base + 9), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 10), Val::from(946), false);
            runtime::local_set(&mut l_items, &Val::from(base + 11), Val::from(5), false);
            runtime::local_set(&mut l_items, &Val::from(base + 12), Val::from(0), false);
        }
        if (((((runtime::op(
            &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?,
            ">=",
            &runtime::local_get(&l_items, &Val::from(1), false),
        )?
        .is_true()
            && runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(2), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(3), false),
            )?
            .is_true())
            && runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(4), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(5), false),
            )?
            .is_true())
            && runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(6), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(7), false),
            )?
            .is_true())
            && runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(8), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(9), false),
            )?
            .is_true())
            && runtime::op(
                &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(10), false)])?,
                ">=",
                &runtime::local_get(&l_items, &Val::from(11), false),
            )?
            .is_true())
        {
            ctx.lines_as(
                "Sir Andrew",
                args![
                    "Perfect! We appreciate your effort in gathering these items. Thesee will be used to support the Chivalry's finances."
                ],
            )?;
            ctx.next()?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(0), false),
                    runtime::local_get(&l_items, &Val::from(1), false),
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(2), false),
                    runtime::local_get(&l_items, &Val::from(3), false),
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(4), false),
                    runtime::local_get(&l_items, &Val::from(5), false),
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(6), false),
                    runtime::local_get(&l_items, &Val::from(7), false),
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(8), false),
                    runtime::local_get(&l_items, &Val::from(9), false),
                ],
            )?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(10), false),
                    runtime::local_get(&l_items, &Val::from(11), false),
                ],
            )?;
            if ctx.var("knight_q").get()? == 2 {
                ctx.call(Function::ChangeQuest, vec![Val::from(9001), Val::from(9003)])?;
            } else {
                ctx.call(Function::ChangeQuest, vec![Val::from(9002), Val::from(9003)])?;
            }
            ctx.var("knight_q").set(Val::from(4))?;
            ctx.lines_as("Sir Andrew", args!["Please visit my fellow Knight, Sir Siracuse, and continue the tests with the dedication and loyalty you've shown to me this day."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sir Andrew",
            args![
                "Wait, wait...",
                "I think you're",
                "still missing some",
                "items. In case you",
                "forgot, let me",
                "remind you..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Andrew",
            args![
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(1), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(3), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(5), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(4), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(7), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(6), false)])?)
                    + Val::from("^000000,")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(9), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(8), false)])?)
                    + Val::from("^000000 and")),
                ((((Val::from("^236B8E") + runtime::local_get(&l_items, &Val::from(11), false)) + Val::from(" "))
                    + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(10), false)])?)
                    + Val::from("^000000,"))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Andrew",
            args![
                "Now, please take this test seriously and with sincerity.",
                "Now, I'll be waiting for you",
                "to complete this task."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 4 {
        ctx.mes(
            "Did you have something you needed to ask me? You should go and take the next test. Hurry, Sir Siracuse is waiting for you~",
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 14 {
        ctx.mes("You must have finished all the tests. Good job! You should go see our Captain so that we can all give our evaluation.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "Did you have something you needed to ask me? You should go and take your next test. Do your best.",
            "I know you can do it!"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sir_andrew_knt(ctx: &Ctx) -> Script {
    sir_andrew_knt_body(ctx, Vec::new()).map(|_| ())
}

fn sir_siracuse_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Sir Siracuse]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.lines(args![
                "Hey there!",
                "How are you doing?",
                "The Chivalry's been",
                "doing pretty well."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "We've been",
                    "testing new members,",
                    "but not all of them",
                    "show as much promise",
                    "as you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "I hope these new recruits all behave themselves, and don't",
                    "bring shame to the Chivalry."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args!["If you catch any of the new guys acting in a way unbecoming of a Knight, scold them for me please?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["Oh?", "What is a Novice", "doing here?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args!["Are you interested in becoming a Knight? You just can't change into a Knight from a Novice, you know."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "First, you have",
                    "to become a well",
                    "experienced Swordman",
                    "before you can consider",
                    "becoming a Knight."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Offense and defense.",
                "Is there a way to have both without compromising one or the other?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "Two-handed weapons greatly",
                    "improve your offense but decrease your defenses. Is there something that can overcome this weakness?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args!["A weapon or some sort", "of technique like that", "would help Knights greatly..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.lines(args![
            "Offense and defense.",
            "Is there a way to have both without compromising one or the other?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Siracuse",
            args![
                "Two-handed weapons greatly",
                "improve your offense but decrease your defenses. Is there something that can overcome this weakness?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Siracuse",
            args!["A weapon or some sort", "of technique like that", "would help Knights greatly..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("knight_q").get()? == 1 {
            ctx.lines(args!["Eh?", "Do you have", "something to", "ask me?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Oh, to become",
                        "a Knight? Come to",
                        "think of it, aren't",
                        ((Val::from("you the ")
                            + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                Val::from("guy")
                            } else {
                                Val::from("girl")
                            }))
                            + Val::from(" that")),
                        "just applied?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Let's see...",
                        "Your name was",
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "But, before you come to me, you must visit the others. The way",
                        "I see it, you haven't proven that you know the basics. But I'll",
                        "reconsider once you",
                        "pass the first test."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Hmmm...?", "Alright.", "It's just that", "you had that", "look on your", "face."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("knight_q").get()? == 2 || ctx.var("knight_q").get()? == 3) {
            ctx.lines(args!["Eh?", "Do you have", "something to", "ask me?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as("Sir Siracuse", args!["Hahaha~!", "Aren't you supposed to be taking Andrew's test? You can't just skip that, you know! All of our tests are important."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Speak to Sir Andrew first.",
                        "My test for you will come after you've finished his test."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Hmmm...?", "Alright.", "It's just that", "you had that", "look on your", "face."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("knight_q").get()? == 4 || ctx.var("knight_q").get()? == 5) {
            if ctx.var("knight_q").get()? == 4 {
                ctx.lines(args!["Oh?", "Do you have", "something to", "ask me?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Sir Andrew sent me to take your test.:Oh, nothing.")],
                )?) == 1
                {
                    ctx.lines_as("Sir Siracuse", args!["I see, you've passed the first test. Very well, I'll make some time for you. Let me introduce myself. My name is James Siracuse."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Siracuse", args!["This test will measure how much you know about Knighthood. More importantly, I want to know your thoughts about honor."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Siracuse", args!["Don't be nervous, I won't keep you too long. These will be quick questions. Plus, you still have to see the others, right?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Siracuse",
                        args!["Well then,", "let's begin.", "Please answer", "promptly."],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Sir Siracuse",
                        args!["Hmmm...?", "Alright.", "It's just that", "you had that", "look on your", "face."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else if ctx.var("knight_q").get()? == 5 {
                ctx.lines(args!["What...", "You again?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I wish to take the test again.:Oh, nothing.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Sir Siracuse",
                        args!["Is that right?", "Are you sure you're", "prepared this time?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sir Siracuse", args!["Alright then,", "here we go again..."])?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Sir Siracuse",
                        args!["Hmmm...?", "Alright.", "It's just that", "you had that", "look on your", "face."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as("Sir Siracuse", args!["A Knight must possess great strength, defense, speed, and the skill to wield a Two-Handed Sword. Which of the following weapons are not affected by the Two Hand Quicken skill?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Katana:Slayer:Broadsword:Flamberge")])?) != 4 {
                ctx.var("knight_q").set(Val::from(5))?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Wrong!",
                        "That's a Two-Handed Sword!",
                        "Are you sure you want to be a Knight? You don't even know the basics..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Siracuse", args!["If you're not sure about anything, go into town and ask any Knight. You need to learn more about Knights before applying for the job!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Good, now let me ask about some skills. Which of the following is not necessary to learn Bowling Bash?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Two Handed Sword Mastery Lv.5:Magnum Break Lv.3:Provoke Lv.10:Bash Lv.10",
                )],
            )?) != 3
            {
                ctx.var("knight_q").set(Val::from(5))?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Wrong!",
                        "You need that to learn Bowling Bash! You should learn more about the Knight class before applying for the job!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Sir Siracuse", args!["Knights can also use Spears, unlike other jobs, and have skills related to Spears as well. What skills are not necessary to learn the skill Brandish Spear?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Pierce Lv.5:Spear Stab Lv.3:Spear Boomerang Lv.3:Peco Peco Ride Lv.1")],
            )?) != 3
            {
                ctx.var("knight_q").set(Val::from(5))?;
                ctx.lines_as("Sir Siracuse", args!["Wrong! You need to learn that to learn Brandish Spear! How can you not know about Knights if you want to become one?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "If you aren't sure about anything, go into town and ask any Knight",
                        "for help. Come back after you've learned more about Knights."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Sir Siracuse", args!["Some Spears also have magical attributes, just like spells. Of the following, which can attack a Nightmare, which has the Ghost attribute?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Zephyrus:Lance:Bill Guisarme:Crescent Scythe")],
            )?) != 1
            {
                ctx.var("knight_q").set(Val::from(5))?;
                ctx.lines_as("Sir Siracuse", args!["Wrong! You'll be doing absolutely no damage with that type of Spear! Come back after you've learned more about Knights!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args!["If you have a question, just ask any Knight in town. This is basic knowledge for us!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Sir Siracuse", args!["When you become a Knight you can ride a Peco Peco. However, your attack speed decreases once you're mounted on a Peco Peco."])?;
            ctx.next()?;
            ctx.lines_as("Sir Siracuse", args!["But, you can counter this speed decrease as you learn the Cavalier Mastery skill. What percentage of your normal attack speed will you have after learning Level 3 Cavalier Mastery?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "70 % of normal attack speed:80 % of normal attack speed:90 % of normal attack speed:100 % of normal attack speed",
                )],
            )?) != 2
            {
                ctx.var("knight_q").set(Val::from(5))?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Wrong!",
                        "Don't bother riding a Peco Peco if you don't know about Cavalier Mastery!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Siracuse",
                    args!["You better come back after you've learned a little more about Knights!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "Good, good...",
                    "I'm pretty sure you know a decent amount about Knights. Now, let me ask you some personal questions about Knights."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args!["What should you do when you run into a Novice asking for help in town?"],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Tell the Novice of a reasonable hunting area.:Let the Novice fight while you take the damage.:Give the Novice a bunch of Zeny and items.",
                )],
            )? {
                1 => {
                    ctx.lines_as("Sir Siracuse", args!["Of course, even a Novice needs to learn how to be independent. Giving good guidance to Novices is one of the best things we can do."])?;
                    ctx.next()?;
                }
                2 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as("Sir Siracuse", args!["You have the wrong idea. Do you really believe that is helping the Novice? Give a man a fish, he will eat for a day. Teach him to fish, he will eat for a lifetime!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as("Sir Siracuse", args!["Do you really believe that this will truly help the poor Novice? It's generous but, they will not know the true value of zeny and items until they earn it themselves."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Alright...", "Now, how should", "you act within", "a party?"],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Protect everyone in the front of the battle.:Gather monsters and destroy them at once.:Get as many items possible, at all cost.",
                )],
            )? {
                1 => {
                    ctx.lines_as("Sir Siracuse", args!["That's it! Our strength and attacks are very important in a party. All Knights should engage in a battle with that mindset."])?;
                    ctx.next()?;
                }
                2 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as("Sir Siracuse", args!["Are you crazy? Don't you realize the flaw in that kind of thinking? You can't control large mobs. What if they kill you? Who will protect the innocent?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as(
                        "Sir Siracuse",
                        args![
                            "I see your greed and we will have none of it here! It seems you do not truly care for others!",
                            "Get lost!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Lastly...", "what's the most", "important value", "a Knight must have?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Honor:Wealth:Status")])? {
                1 => {
                    ctx.lines_as(
                        "Sir Siracuse",
                        args!["Right, above all else, Knights must be honorable! We live and die for honor! Always keep that in mind."],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as("Sir Siracuse", args!["You're scum! You strive to become a Knight for personal wealth? Get lost! We will not accept someone like you in our Chivalry!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.var("knight_q").set(Val::from(5))?;
                    ctx.lines_as("Sir Siracuse", args!["So you're trying to become famous through the Chivalry? That's pathetic. We won't accept someone like you in our Chivalry!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.var("knight_q").set(Val::from(6))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(9003), Val::from(9004)])?;
            ctx.lines_as("Sir Siracuse", args!["Well then,", "this is the", "end of my test."])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "For your next",
                    "test, please go",
                    "see Sir Windsor.",
                    "He's very quiet,",
                    "but don't let that",
                    "get to you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("knight_q").get()? == 6 {
            ctx.lines(args!["Oh?", "Do you have", "something to", "ask me?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Sir Siracuse",
                    args![
                        "Hey...",
                        "You already took my test, didn't you? You're done here. You should go visit Sir Windsor now..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Siracuse",
                args!["Hmmm...?", "Alright.", "It's just that", "you had that", "look on your", "face."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("knight_q").get()? == 14 {
            ctx.lines(args!["Mmm...?", "You finished", "everyone else's", "tests as well?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Siracuse",
                args![
                    "Well then,",
                    "go and see the",
                    "captain. We'll all",
                    "be there to evaluate",
                    "your performance."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Hey again.",
                "Did you need something?",
                "Sorry, but I'm busy at the moment. You should go and finish the rest of your tests."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn sir_siracuse_knt(ctx: &Ctx) -> Script {
    sir_siracuse_knt_body(ctx, Vec::new()).map(|_| ())
}

fn sir_windsor_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_mes_s = Val::from("");
    ctx.lines_as("Sir Windsor", args!["..."])?;
    ctx.next()?;
    ctx.lines_as("Sir Windsor", args!["...", "......"])?;
    ctx.next()?;
    ctx.mes("[Sir Windsor]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            ctx.mes("Protect.")?;
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.lines(args!["...Go play", "outside."])?;
        } else {
            ctx.mes("...Hmpf.")?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("knight_q").get()? == 0 {
        ctx.mes("...What?")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()?.number()? >= 1 && ctx.var("knight_q").get()?.number()? <= 5) {
        ctx.mes("...What?")?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I would like to take the test to change jobs.:Oh, nothing.")],
        )?) == 1
        {
            ctx.lines_as("Sir Windsor", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Sir Windsor", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as("Sir Windsor", args!["...It's not my turn."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sir Windsor", args!["..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("knight_q").get()? == 6 || ctx.var("knight_q").get()? == 7) {
        if ctx.var("knight_q").get()? == 6 {
            l_mes_s = Val::from("Sir Siracuse sent me to you.:Oh, nothing.");
            ctx.mes(".....What?")?;
            ctx.next()?;
        } else {
            l_mes_s = Val::from("I want to try again!:...");
            ctx.next()?;
        }
        if Val::from(runtime::select_values(ctx, &[l_mes_s.clone()])?) == 1 {
            ctx.lines_as("Sir Windsor", args!["..."])?;
            ctx.next()?;
            ctx.var("knight_q").set(Val::from(7))?;
            if ctx.call(Function::CheckQuest, vec![Val::from(9004)])? != -1 {
                ctx.call(Function::ChangeQuest, vec![Val::from(9004), Val::from(9005)])?;
            }
            ctx.lines_as("Sir Windsor", args!["...", "......"])?;
            ctx.next()?;
            ctx.mes("[Sir Windsor]")?;
            if ctx.var("knight_q").get()? == 6 {
                ctx.mes("...Follow me.")?;
            } else {
                ctx.mes("...Fine.")?;
                ctx.next()?;
                ctx.lines_as("Sir Windsor", args!["...This way."])?;
            }
            ctx.close_window()?;
            if ctx.call(Function::CheckQuest, vec![Val::from(9006)])? == -1 {
                ctx.call(Function::ChangeQuest, vec![Val::from(9005), Val::from(9006)])?;
            }
            ctx.call(Function::Warp, vec![Val::from("job_knt"), Val::from(89), Val::from(101)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Sir Windsor", args!["..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("knight_q").get()? == 14 {
        ctx.lines(args!["...Talk to", "the captain."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["...You're", "done here."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sir_windsor_knt(ctx: &Ctx) -> Script {
    sir_windsor_knt_body(ctx, Vec::new()).map(|_| ())
}

fn knight_windsor_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    ctx.lines_as("Sir Windsor", args!["..."])?;
    ctx.next()?;
    ctx.lines_as("Sir Windsor", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as("Sir Windsor", args!["...Question?"])?;
    ctx.next()?;
    l_i = Val::from(runtime::select_values(
        ctx,
        &[Val::from(
            "What kind of test is this?:How do I take the test?:I'd like to leave.:No.",
        )],
    )?);
    ctx.lines_as("Sir Windsor", args!["..."])?;
    if l_i.clone() == 4 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.next()?;
    ctx.lines_as("Sir Windsor", args!["...", "......"])?;
    let subject1 = l_i.clone();
    if subject1 == 1 {
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...You fight monsters."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...Kill them all."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...Three stages.", "Beat them all."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["......3 minutes", "for each stage."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args![".........."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 2 {
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...Go in the", "waiting room."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...Then it", "will begin."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...You have to wait", "if someone is testing."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["...You can go in", "after that person."])?;
        ctx.next()?;
        ctx.lines_as("Sir Windsor", args!["..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if subject1 == 3 {
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("prt_in"), Val::from(80), Val::from(100)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn knight_windsor_knt(ctx: &Ctx) -> Script {
    knight_windsor_knt_body(ctx, Vec::new()).map(|_| ())
}

fn windsor_benedict_knt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn windsor_benedict_knt(ctx: &Ctx) -> Script {
    windsor_benedict_knt_body(ctx, Vec::new()).map(|_| ())
}

fn windsor_benedict_knt_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Windsor Benedict#knt")])?;
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Room"),
            Val::from(20),
            Val::from("Windsor Benedict#knt::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn windsor_benedict_knt_oninit(ctx: &Ctx) -> Script {
    windsor_benedict_knt_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn windsor_benedict_knt_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("job_knt"), Val::from("Knight1::OnMyMobDead")],
    )?;
    ctx.call(
        Function::KillMonster,
        vec![Val::from("job_knt"), Val::from("Knight2::OnMyMobDead")],
    )?;
    ctx.call(
        Function::KillMonster,
        vec![Val::from("job_knt"), Val::from("Knight3::OnMyMobDead")],
    )?;
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("job_knt"), Val::from(43), Val::from(146)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight1::OnEnable")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn windsor_benedict_knt_onstartarena(ctx: &Ctx) -> Script {
    windsor_benedict_knt_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn windsor_benedict_knt_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn windsor_benedict_knt_onstart(ctx: &Ctx) -> Script {
    windsor_benedict_knt_onstart_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Knight1Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer180000,
    OnTimer181000,
    OnTimer182000,
}

fn knight1_run(ctx: &Ctx, mut step: Knight1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Knight1Step::Start => {
                step = Knight1Step::OnInit;
                continue 'machine;
            }
            Knight1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Knight1")])?;
                return Err(Stop::End);
            }
            Knight1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Knight1")])?;
                {
                    ctx.var(".mymobs").set(Val::from(12))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(39),
                            Val::from(150),
                            Val::from("Dustiness"),
                            Val::from(1114),
                            Val::from(1),
                            Val::from("Knight1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(47),
                            Val::from(150),
                            Val::from("Dustiness"),
                            Val::from(1114),
                            Val::from(1),
                            Val::from("Knight1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(39),
                            Val::from(142),
                            Val::from("Dustiness"),
                            Val::from(1114),
                            Val::from(1),
                            Val::from("Knight1::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_knt"),
                            Val::from(47),
                            Val::from(142),
                            Val::from("Dustiness"),
                            Val::from(1114),
                            Val::from(1),
                            Val::from("Knight1::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(137),
                        Val::from("Piere"),
                        Val::from(1160),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(137),
                        Val::from("Andre"),
                        Val::from(1095),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(137),
                        Val::from("Deniro"),
                        Val::from(1105),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(155),
                        Val::from("Piere"),
                        Val::from(1160),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(155),
                        Val::from("Andre"),
                        Val::from(1095),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(43),
                        Val::from(155),
                        Val::from("Deniro"),
                        Val::from(1105),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(35),
                        Val::from(146),
                        Val::from("Argos"),
                        Val::from(1100),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_knt"),
                        Val::from(52),
                        Val::from(146),
                        Val::from("Argos"),
                        Val::from(1100),
                        Val::from(1),
                        Val::from("Knight1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Knight1Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_knt"), Val::from("Knight1::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Knight1")])?;
                return Err(Stop::End);
            }
            Knight1Step::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.lines_as("Sir Windsor", args!["..."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Windsor", args!["...On to", "the next level."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("job_knt"), Val::from(43), Val::from(52)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight1::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Knight2::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Knight1Step::OnTimer180000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight1::OnDisable")])?;
                return Err(Stop::End);
            }
            Knight1Step::OnTimer181000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_knt"),
                        Val::from(24),
                        Val::from(126),
                        Val::from(63),
                        Val::from(165),
                        Val::from("prt_in"),
                        Val::from(80),
                        Val::from(100),
                    ],
                )?;
                return Err(Stop::End);
            }
            Knight1Step::OnTimer182000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Knight1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Windsor Benedict#knt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn knight1(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::Start, Vec::new()).map(|_| ())
}

pub fn knight1_oninit(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn knight1_onenable(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn knight1_ondisable(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn knight1_onmymobdead(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn knight1_ontimer180000(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn knight1_ontimer181000(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnTimer181000, Vec::new()).map(|_| ())
}

pub fn knight1_ontimer182000(ctx: &Ctx) -> Script {
    knight1_run(ctx, Knight1Step::OnTimer182000, Vec::new()).map(|_| ())
}
