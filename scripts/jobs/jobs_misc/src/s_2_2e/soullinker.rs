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
enum KidLink1Step {
    Start,
    OnInit,
}

fn kid_link1_run(ctx: &Ctx, mut step: KidLink1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KidLink1Step::Start => {
                if ctx.player().class()? == constants::JOB_SOUL_LINKER {
                    ctx.lines_as(
                        "Maia",
                        args![
                            "Best of luck in your",
                            "journeys. As you master",
                            "more Soul Linker skills,",
                            "you will be able to draw",
                            "more of the spirits' power",
                            "to endow upon your allies..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.player().class()? == constants::JOB_STAR_GLADIATOR {
                    ctx.mes("[Kid]")?;
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        ctx.lines(args!["Aren't you a warrior", "of the sun? I'm familiar"])?;
                    } else {
                        ctx.lines(args!["Aren't you a warrior of", "the moon? I'm familiar"])?;
                    }
                    ctx.lines(args![
                        "with your ways. After all,",
                        "the basis of both of our",
                        "skills is grounded in the",
                        "Taekwon Do job, right?"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.player().class()? != constants::JOB_TAEKWON {
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Mm? I've got nothing to",
                            "offer you. But if you know",
                            "any well experienced",
                            "practitioners of Taekwon",
                            "Do, they might benefit",
                            "from what I know."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.player().job_level()? < 40 {
                    ctx.lines_as(
                        "Kid",
                        args![
                            "So you're studying",
                            "Taekwon Do. That's good,",
                            "that's very good. Just keep",
                            "refining those skills and",
                            "stick to your training."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.player().job_level()? > 39 {
                    if ctx.var("soul_q").get()? == 0 {
                        ctx.lines_as("Kid", args!["...", "Hey you."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Did you call me?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Yeah, I called you.",
                                "Now don't make me",
                                "raise my voice, and",
                                "just get over here."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["You're awfully rude for a kid!", "Ignore him."])? == 0 {
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "You're lucky I'm",
                                    "taking an interest",
                                    "in you! I might look",
                                    "like a kid, but I'm over",
                                    "three hundred years old!"
                                ],
                            )?;
                            ctx.call(Function::Emotion, args![constants::ET_HNG])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "Now listen...",
                                    "I know that you're a",
                                    "disciple of Taekwon Do.",
                                    "It's a respectable art, but",
                                    "I've got a proposition for",
                                    "you if you want to hear it."
                                ],
                            )?;
                            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "I'm looking at you, and I can",
                                    "already tell that you're very",
                                    "spiritually inclined. You've",
                                    "got a lot of potential I don't",
                                    "wanna see wasted. Why don't",
                                    "you become a ''Soul-Linker?''"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Ha! Silly little boy~", "Soul Linker?"])? == 0 {
                                ctx.lines_as(
                                    "Kid",
                                    args![
                                        "You... You d-don't",
                                        "believe me? I'm being",
                                        "dead serious. Can you",
                                        "forget the fact that I look",
                                        "like a little kid for just one",
                                        "minute? *Psh* ...Youngsters."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "Soul Linkers communicate",
                                    "with spirits of fallen warriors",
                                    "that still wish to fight in the",
                                    "world of the living. Now, these",
                                    "warrior spirits can't fight as",
                                    "themselves in our world."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "However, since you're",
                                    "spiritually inclined, these",
                                    "spirits are attracted to you.",
                                    "With enough training, you can",
                                    "temporarily imbue the power of these spirits to your allies."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "Now, you can't imbue yourself",
                                    "with the spirits' power. Also,",
                                    "depending on your skills as",
                                    "a Soul Linker, you can only",
                                    "endow other characters of certain job classes with enchanced power."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "You'll have to enter",
                                    "a wholly different world",
                                    "to become a Soul Linker,",
                                    "but I know it'll be possible",
                                    "for you. So what do you say?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["No. At least, not now...", "Alright. What do I have to do?"])? == 0 {
                                ctx.lines_as(
                                    "Kid",
                                    args![
                                        "Ah, alright. Well,",
                                        "if you ever decide to",
                                        "become a Soul Linker,",
                                        "then please come back",
                                        "and talk to me at any time."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("soul_q").set(Val::from(1))?;
                            ctx.quests().start(6005)?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "So you want to become",
                                    "a Soul Linker? Great!",
                                    "Alright, first I need you",
                                    "to bring back a few items.",
                                    "Don't worry, I'll explain",
                                    "why you need them later."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "Now bring me",
                                    "^0000FF1 3 Carat Diamond^000000,",
                                    "^0000FF1 Immortal Heart^000000 and",
                                    "^0000FF1 Witherless Rose^000000.",
                                    "And try not to make me",
                                    "wait too long, alright?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Kid",
                            args!["Huh...?", "Wait, where are", "you going? I'm...", "I'm talking to you!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("soul_q").get()? == 1 {
                        if ctx.player().class()? == constants::JOB_TAEKWON {
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "You're back, eh?",
                                    "So did you bring",
                                    "^0000FF1 3 Carat Diamond^000000,",
                                    "^0000FF1 Immortal Heart^000000 and",
                                    "^0000FF1 Witherless Rose^000000.",
                                    "like I asked you to?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["There you are.", "No, not yet..."])? == 0 {
                                if ctx.items().count(732)? > 0 && ctx.items().count(929)? > 0 && ctx.items().count(748)? > 0 {
                                    ctx.items().take(732, 1)?;
                                    ctx.items().take(929, 1)?;
                                    ctx.items().take(748, 1)?;
                                    ctx.var("soul_q").set(Val::from(2))?;
                                    ctx.quests().change(6005, 6006)?;
                                    ctx.lines_as(
                                        "Kid",
                                        args![
                                            "Great, I see that you've",
                                            "brought everything. But",
                                            "before we begin, let me",
                                            "introduce myself. My name",
                                            "is Maia, and I've been alive for more than three hundred years."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Maia",
                                        args![
                                            "Without giving away too many",
                                            "of the details, I've been divinely charged with the duty of finding",
                                            "and recruiting more Soul Linkers. That's part of the reason why",
                                            "I haven't, you know, passed on."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Maia",
                                        args![
                                            "Anyway, I still need to finish",
                                            "preparations with the materials",
                                            "that you just brought, so would",
                                            "you come back in a little bit?",
                                            "Then, we'll talk once again."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Kid",
                                    args![
                                        "Mm...?",
                                        "Hey. You forgot",
                                        "a few things. Now",
                                        "go back and bring",
                                        "everything that I ask",
                                        "for this time, okay?"
                                    ],
                                )?;
                                ctx.call(Function::Emotion, args![constants::ET_HNG])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kid",
                                    args![
                                        "I know I just told you",
                                        "what we need, but I'm",
                                        "going to remind you again:",
                                        "^0000FF1 3 Carat Diamond^000000,",
                                        "^0000FF1 Immortal Heart^000000 and",
                                        "^0000FF1 Witherless Rose^000000."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Kid",
                                args![
                                    "Mm. That's fine.",
                                    "Although I have all",
                                    "the time to spare in",
                                    "the world, I don't like",
                                    "to wait for very long."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.var("soul_q").set(Val::from(0))?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "You've become a warrior",
                                "of the Sun, the Moon and",
                                "the Stars instead? I had no",
                                "idea you had that potential.",
                                "I suppose I can't blame you..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("soul_q").get()? == 2 {
                        if ctx.var("SkillPoint").get()?.is_true() {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "You still have some",
                                    "unallocated Skill Points.",
                                    "Use them all to learn some",
                                    "Taekwon Do skills, and then",
                                    "return when you're ready."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var(".soullinkertest").get()? == 1 {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "Right now, someone else",
                                    "is completing the ceremony",
                                    "to become a Soul Linker.",
                                    "Would you please wait until",
                                    "it's finished? Then, when I'm",
                                    "available, I'll attend to you."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.npc().do_event("Timer#link3::OnEnable")?;
                        ctx.var(".soullinkertest").set(Val::from(1))?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Great, I've finished",
                                "the preparations. Now",
                                "we'll proceed with the",
                                "ceremony to change",
                                "you into a Soul Linker.",
                                "Now close your eyes..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.warp("job_soul", 30, 30)?;
                        return Err(Stop::End);
                    } else if ctx.var("soul_q").get()?.number()? > 2 {
                        ctx.lines_as("Maia", args!["Are you ready to", "enter the depths", "of your mind again?"])?;
                        ctx.next()?;
                        if ctx.menu(&["No", "Yes"])? == 0 {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "Well then, come",
                                    "back to me when you",
                                    "think you are ready.",
                                    "Until then, I'll be",
                                    "waiting right here."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var(".soullinkertest").get()? == 1 {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "Right now, someone else",
                                    "is completing the ceremony",
                                    "to become a Soul Linker.",
                                    "Would you please wait until",
                                    "it's finished? Then, when I'm",
                                    "available, I'll attend to you."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.npc().do_event("Timer#link3::OnEnable")?;
                        ctx.var(".soullinkertest").set(Val::from(1))?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Alright then, close",
                                "your eyes and relax.",
                                "We'll go back into the",
                                "depths of your mind."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.warp("job_soul", 30, 30)?;
                        return Err(Stop::End);
                    }
                }
                step = KidLink1Step::OnInit;
                continue 'machine;
            }
            KidLink1Step::OnInit => {
                ctx.var(".soullinkertest").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn kid_link1(ctx: &Ctx) -> Script {
    kid_link1_run(ctx, KidLink1Step::Start, Vec::new()).map(|_| ())
}

pub fn kid_link1_oninit(ctx: &Ctx) -> Script {
    kid_link1_run(ctx, KidLink1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MaiaLink2Step {
    Start,
    OnTouch,
}

fn maia_link2_run(ctx: &Ctx, mut step: MaiaLink2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaiaLink2Step::Start => {
                step = MaiaLink2Step::OnTouch;
                continue 'machine;
            }
            MaiaLink2Step::OnTouch => {
                if ctx.player().class()? == constants::JOB_TAEKWON {
                    if ctx.player().job_level()? < 40 {
                        ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Hm? How did you come",
                                "here? You're not qualified",
                                "for this ceremony yet. Come,I will bring you back to Morocc..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.warp("morocc", 157, 47)?;
                        return Err(Stop::End);
                    }
                    if ctx.var("soul_q").get()? == 2 {
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Do you recognize this",
                                "place? Right now, we're",
                                "inside your mind. The spirits",
                                "of warriors that have died",
                                "hover here, waiting for you",
                                "to call upon their power."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Right now, there are only",
                                "a few of them here, but if",
                                "you continue to train, you",
                                "will be able to call upon",
                                "more spirits as a Soul Linker."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.var("soul_q").set(Val::from(3))?;
                        ctx.quests().change(6006, 6007)?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "We can only remain in",
                                "your mind for 3 minutes.",
                                "I suggest that you speak",
                                "to the spirits while you",
                                "have the opportunity."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("soul_q").get()? == 3 {
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Listen to what",
                                "spirits are tending to say.",
                                "There is a reason why",
                                "they cannot move on",
                                "to the next world."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("soul_q").get()? == 4 {
                        ctx.lines_as(
                            "Maia",
                            args![
                                "I believe that you are",
                                "now ready to become",
                                "a Soul Linker. However,",
                                "you may continue to",
                                "speak with the spirits",
                                "if that is what you wish."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["Converse more with the spirits", "Become a Soul Linker"])? == 0 {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "Alright. Try to hurry",
                                    "since we can remain in",
                                    "your mind for a limited",
                                    "time. Although, we can",
                                    "go back inside your mind",
                                    "if you talk to me later..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.call(Function::IsMounting, vec![])?.is_true() {
                            ctx.lines_as(
                                "Maia",
                                args![
                                    "You are on a riding pet,",
                                    "so you cannot change your job.",
                                    "Please unequip your riding pet and try again!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Maia",
                            args![
                                "Then let us begin the",
                                "ceremony. These items will",
                                "be used to endow you with",
                                "the ability to borrow the power",
                                "of the fallen warriors and lend",
                                "it to your friends in battle."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Maia", args!["This Witherless Rose will", "wither away instead of you..."])?;
                        ctx.call(
                            Function::NpcSpecialEffect,
                            args![constants::EF_MAPPILLAR2, constants::AREA, "Maia#link2"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "This Witherless Rose will",
                                "wither away instead of you...",
                                "This Immortal Heart will cease",
                                "to pump blood, instead of yours. "
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "This Witherless Rose will",
                                "wither away instead of you...",
                                "This Immortal Heart will cease",
                                "to pump blood, instead of yours. This Diamond will turn to dust,",
                                "in place of your mortal body."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "The dead who wish",
                                "to continue fighting...",
                                "Will fight for you! Use your",
                                "powers as a Soul Linker",
                                "wisely and for just purposes."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("SkillPoint").get()?.is_true() {
                            ctx.mes(
                                "^0000ffYou still have unused skill points. Please use all remaining skill points and try again!^000000",
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.quests().complete(6008)?;
                        shared::other_global_functions::job_change(ctx, args![constants::JOB_SOUL_LINKER])?;
                        shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                        ctx.var("soul_q").set(Val::from(0))?;
                        ctx.lines_as(
                            "Maia",
                            args![
                                "I wish the best of luck",
                                "in your new life. Surround",
                                "yourself with allies, and the",
                                "spirits will be able to protect",
                                "you and help you fight in your battles. Farewell for now, friend."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                        ctx.npc().do_event("Timer#link3::OnDisable")?;
                        ctx.warp("morocc", 157, 47)?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                    ctx.lines_as(
                        "Maia",
                        args![
                            "Hmm...?",
                            "The time for you",
                            "to be here has not",
                            "arrived. Let's go",
                            "back to Morocc..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.warp("morocc", 157, 47)?;
                    return Err(Stop::End);
                }
                ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                if ctx.player().class()? == constants::JOB_SOUL_LINKER {
                    ctx.lines_as(
                        "Maia",
                        args![
                            "The time has come for",
                            "you to venture out into the",
                            "wide world! More Soul Linkers",
                            "will definitely be needed in the ongoing battle against evil..."
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        "Maia",
                        args![
                            "That's strange...",
                            "You're not supposed to",
                            "be here. Let me guide",
                            "you back to Morocc..."
                        ],
                    )?;
                }
                ctx.close_window()?;
                ctx.warp("morocc", 157, 47)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maia_link2(ctx: &Ctx) -> Script {
    maia_link2_run(ctx, MaiaLink2Step::Start, Vec::new()).map(|_| ())
}

pub fn maia_link2_ontouch(ctx: &Ctx) -> Script {
    maia_link2_run(ctx, MaiaLink2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn monk_spirit_link4(ctx: &Ctx) -> Script {
    if ctx.var("soul_q").get()? == 2 {
        ctx.lines_as(
            "Monk Spirit",
            args![
                "Who am I...?",
                "I think... I think",
                "it would be best if",
                "you spoke to Maya first...",
                "Who and what I am requires",
                "a complicated explanation..."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("soul_q").get()?.number()? > 2 {
        ctx.lines_as(
            "Monk Spirit",
            args![
                "In life, my peers did",
                "their best to assure me",
                "that I accomplish all that",
                "I could as a Monk. Still...",
                "Still I would never be fully",
                "satisfied with my skills."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Monk Spirit",
            args![
                "In death, I had many regrets,",
                "never having the chance to pass",
                "my skills down to future Monks.",
                "Lending my power to others ",
                "is the only chance that I can",
                "possibly have to do this."
            ],
        )?;
        ctx.next()?;
        ctx.var("soul_q").set(Val::from(4))?;
        if ctx.call(Function::CheckQuest, args![6008])? == -1 {
            ctx.quests().change(6007, 6008)?;
        }
        ctx.lines_as(
            "Monk Spirit",
            args![
                "I beg of you...",
                "I need you to help",
                "me fully realize the",
                "true potential of the",
                "Monks of today."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Monk Spirit", args!["..."])?;
    ctx.close()
}

pub fn sage_spirit_link5(ctx: &Ctx) -> Script {
    if ctx.var("soul_q").get()? == 2 {
        ctx.lines_as(
            "Sage Spirit",
            args![
                "Speak to Maia.",
                "I'm afraid I may",
                "confuse you if Maia",
                "doesn't first explain",
                "your present situation..."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("soul_q").get()?.number()? > 2 {
        ctx.lines_as(
            "Sage Spirit",
            args![
                "My pursuit of knowledge",
                "granted me incredible power:",
                "in life, I could have destroyed",
                "anything I wanted. Few Sages",
                "could even reach my level..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sage Spirit",
            args![
                "I died, but I was never able",
                "to pass on to the next world.",
                "I still want to use my abilities.I want to use my knowledge",
                "to build what pleases me,",
                "and to destroy as I please."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sage Spirit",
            args![
                "It is enough if I can",
                "lend my power to a Sage",
                "that is worthy of receiving",
                "it. But to do that, I shall",
                "require your help. I beg you,",
                "let me become your spirit ally."
            ],
        )?;
        ctx.var("soul_q").set(Val::from(4))?;
        if ctx.call(Function::CheckQuest, args![6008])? == -1 {
            ctx.quests().change(6007, 6008)?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Sage Spirit",
            args![
                "I believe that you",
                "are the only one who",
                "has a chance of bringing",
                "rest to my troubled soul..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Sage Spirit", args!["..."])?;
    ctx.close()
}

pub fn alchemist_spirit_link7(ctx: &Ctx) -> Script {
    if ctx.var("soul_q").get()? == 2 {
        ctx.lines_as(
            "Alchemist Spirit",
            args![
                "Oh! I really want to",
                "speak to you, but what",
                "I have to say won't make",
                "much sense unless you",
                "talk to Maia first. But yes,",
                "I really need your help."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("soul_q").get()?.number()? > 2 {
        ctx.lines_as(
            "Alchemist Spirit",
            args![
                "Without exagerrating, I was",
                "the fastest Alchemist in my",
                "time. In fact, I may even be",
                "the fastest Alchemist ever.",
                "But then I grew arrogant, and",
                "killed myself in an accident."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Alchemist Spirit",
            args![
                "But death would not stifle",
                "my skill. In fact, I've even",
                "improved my skill since I've",
                "passed away. I cannot go",
                "on to the next world until I've",
                "passed on my techniques..."
            ],
        )?;
        ctx.var("soul_q").set(Val::from(4))?;
        if ctx.call(Function::CheckQuest, args![6008])? == -1 {
            ctx.quests().change(6007, 6008)?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Alchemist Spirit",
            args![
                "I'm powerless as a spirit,",
                "but with your help, I can",
                "influence the Alchemists of",
                "today and help them refine",
                "their skills. I beseech you,",
                "please give me this chance..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Alchemist Spirit", args!["..."])?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum TimerLink3Step {
    Start,
    OnEnable,
    OnDisable,
    OnTimer60000,
    OnTimer120000,
    OnTimer180000,
    OnTimer181000,
    OnTimer182000,
    OnTimer183000,
}

fn timer_link3_run(ctx: &Ctx, mut step: TimerLink3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerLink3Step::Start => {
                return Err(Stop::End);
            }
            TimerLink3Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerLink3Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                return Err(Stop::End);
            }
            TimerLink3Step::OnTimer60000 => {
                step = TimerLink3Step::OnTimer120000;
                continue 'machine;
            }
            TimerLink3Step::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, args!["job_soul"])? == 0 {
                    ctx.call(Function::StopNpcTimer, vec![])?;
                    ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                }
                return Err(Stop::End);
            }
            TimerLink3Step::OnTimer180000 => {
                step = TimerLink3Step::OnTimer181000;
                continue 'machine;
            }
            TimerLink3Step::OnTimer181000 => {
                step = TimerLink3Step::OnTimer182000;
                continue 'machine;
            }
            TimerLink3Step::OnTimer182000 => {
                ctx.call(Function::MapWarp, args!["job_soul", "morocc", 157, 47])?;
                return Err(Stop::End);
            }
            TimerLink3Step::OnTimer183000 => {
                ctx.call(Function::MapWarp, args!["job_soul", "morocc", 157, 47])?;
                ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn timer_link3(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::Start, Vec::new()).map(|_| ())
}

pub fn timer_link3_onenable(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn timer_link3_ondisable(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer60000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer120000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer180000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer181000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer181000, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer182000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer182000, Vec::new()).map(|_| ())
}

pub fn timer_link3_ontimer183000(ctx: &Ctx) -> Script {
    timer_link3_run(ctx, TimerLink3Step::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn soul_linker_var(ctx: &Ctx) -> Script {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Soul Linker Var",
        args![
            "I can reset the Soul Linker",
            "NPCs if a Soul Linker candidate",
            "encounters a problem during the",
            "end of the job quest. Please do",
            "not use this function if players are still in the Quest Map."
        ],
    )?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0])?.number()? < 1 {
        ctx.lines_as("Soul Linker Var", args!["Password", "is incorrect."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Soul Linker Var",
        args!["Would you like to", "reset the Soul Linker", "Global Variable?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Reset", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Soul Linker Var",
                args!["The Soul Linker", "Job Quest NPCs", "have been reset."],
            )?;
            ctx.call(Function::SetVariableOfNpc, args![".soullinkertest", "Kid#link1", 0, 0])?;
            ctx.close()
        }
        1 => {
            ctx.lines_as("Soul Linker Var", args!["You have canceled", "this command."])?;
            ctx.close()
        }
        _ => Ok(()),
    }
}
