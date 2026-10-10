use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn examiner_sd_run(ctx: &Ctx, mut step: ExaminerSdStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExaminerSdStep::Start => {
                step = ExaminerSdStep::OnInit;
                continue 'machine;
            }
            ExaminerSdStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Examiner#sd")])?;
                return Err(Stop::End);
            }
            ExaminerSdStep::OnTouch => {
                if ctx.var(".s_check").get()?.number()? < 30 {
                    if ctx.var("sign_q").get()? == 30 {
                        ctx.var("sign_q").set(Val::from(32))?;
                    }
                } else if ctx.var(".s_check").get()?.number()? < 34 {
                    if ctx.var("sign_q").get()? == 30 {
                        ctx.var("sign_q").set(Val::from(33))?;
                    }
                } else if ctx.var(".s_check").get()? == 34 && ctx.var("sign_q").get()? == 30 {
                    ctx.var("sign_q").set(Val::from(34))?;
                }
                ctx.call(Function::Warp, vec![Val::from("cmd_in01"), Val::from(29), Val::from(33)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnDisable")])?;
                return Err(Stop::End);
            }
            ExaminerSdStep::OnUp => {
                ctx.var(".s_check").set((ctx.var(".s_check").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            ExaminerSdStep::OnDisable => {
                ctx.var(".s_check").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Examiner#sd")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn examiner_sd(ctx: &Ctx) -> Script {
    examiner_sd_run(ctx, ExaminerSdStep::Start, Vec::new()).map(|_| ())
}

pub fn examiner_sd_oninit(ctx: &Ctx) -> Script {
    examiner_sd_run(ctx, ExaminerSdStep::OnInit, Vec::new()).map(|_| ())
}

pub fn examiner_sd_ontouch(ctx: &Ctx) -> Script {
    examiner_sd_run(ctx, ExaminerSdStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn examiner_sd_onup(ctx: &Ctx) -> Script {
    examiner_sd_run(ctx, ExaminerSdStep::OnUp, Vec::new()).map(|_| ())
}

pub fn examiner_sd_ondisable(ctx: &Ctx) -> Script {
    examiner_sd_run(ctx, ExaminerSdStep::OnDisable, Vec::new()).map(|_| ())
}

fn wealthy_looking_merchant_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Bakerlan]")?;
    if ctx.var("sign_q").get()?.number()? < 35 {
        ctx.lines(args![
            "Umm...well then...",
            "I shall take the action for Prontera...",
            "And for Juno...",
            "Awww....",
            "Gosh, this is such a pain in the ass...",
            "Mumble mumble..."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 35 {
            ctx.lines(args![
                "Gosh.....",
                "I do not know what I should do about Juno...",
                "?",
                "Who are you?"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Nothing.:Mr. Metz sent me.")])? {
                1 => {
                    ctx.lines_as(
                        "Bakerlan",
                        args![
                            "Hmm...",
                            "I am pretty busy at this moment. So please do not interrupt me any further.",
                            "If you are looking for a job,",
                            "please inquire of my steward."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Bakerlan",
                        args![
                            "Ah....",
                            "He did?....Hmmm..",
                            "Unfortunately I am kind of busy...",
                            "will you please come back later?",
                            "Thank you."
                        ],
                    )?;
                    ctx.var("sign_q").set(Val::from(36))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("sign_q").get()? == 36 {
                ctx.lines(args![
                    "Hmm...? You've",
                    "finished all of the",
                    "tests up until Dearle's",
                    "challenge? Prove it to me",
                    "with your pieces of the",
                    "Sobbing Starlight..."
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give me a minute...:Here...")])? {
                    1 => {
                        ctx.lines_as(
                            "Bakerlan",
                            args![
                                "Well, you better",
                                "come back soon. I'm",
                                "a busy businessman, so",
                                "my time is quite precious."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        if ctx.call(Function::CountItem, vec![Val::from(7177)])?.number()? < 5 {
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "Hmm, I suppose you",
                                    "still haven't completed",
                                    "all the tests. These aren't",
                                    "enough pieces of the Sobbing",
                                    "Starlight, you know..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CountItem, vec![Val::from(7177)])? == 5 {
                            ctx.lines_as(
                                "Bakerlan",
                                args!["Ah, now I see that", "you speak the truth.", "Alright, we can begin my test."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Bakerlan", args!["First, I wish for you to make", "a delivery. The object I want you to deliver is expensive and must be handled with care. Don't lose it or you will fail. Now, speak to my steward for more information."])?;
                            ctx.var("sign_q").set(Val::from(37))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "Hm? It's not possible for",
                                    "you to have this many pieces",
                                    "of the Sobbing Starlight!",
                                    "These must be fakes. How",
                                    "dare you! Leave me!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    _ => {}
                }
            } else {
                if ctx.var("sign_q").get()?.number()? < 40 {
                    ctx.lines(args![
                        "Please speak to my",
                        "steward for all matters",
                        "related to the delivery",
                        "that you must complete",
                        "for my test."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()? == 40 {
                        ctx.lines(args![
                            "Ah yes, Mahatra tells",
                            "me that you've completed",
                            "the delivery. Well, um, he now",
                            "has another assignment for",
                            "for you to complete. So",
                            "please talk to him again."
                        ])?;
                        ctx.var("sign_q").set(Val::from(41))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 52 {
                            ctx.lines(args![
                                "You have not finished yet.",
                                "Why don't you go finish them first?",
                                "I am a busy businessman."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()? == 52 {
                            ctx.lines(args![
                                "Ah, I was informed about you.",
                                "Well, although you have your goal,",
                                "that does not necessarily mean to help that kid.",
                                "Metz must have an eye for a right person, I assume."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "Well, I wanted to assign you some various tasks",
                                    "on my own plan, but recently I am very busy",
                                    "to take care of tradings with Kunlun and Amatsu."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "I found Seyling while I was trading with Amatsu few years ago.",
                                    "I introduced her brother to the royal family of Prontera, too.",
                                    "But anyways,"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "I want to test you longer, but",
                                    "I am too busy for that. Also I don't think it is good for you.",
                                    "Let's finish the test now.",
                                    "You are qualified enough already."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Bakerlan",
                                args![
                                    "There you go.",
                                    "I am the last one who tested your qualification.",
                                    "Now bring all of the stone pieces to Metz.",
                                    "He will tell you what you need to do next."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(53))?;
                            ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                            {
                                if ctx.var("BaseLevel").get()?.number()? < 60 {
                                    ctx.call(Function::GetExperience, vec![Val::from(5000), Val::from(0)])?;
                                } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                                    ctx.call(Function::GetExperience, vec![Val::from(8000), Val::from(0)])?;
                                } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                                    ctx.call(Function::GetExperience, vec![Val::from(11000), Val::from(0)])?;
                                } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                                    ctx.call(Function::GetExperience, vec![Val::from(15000), Val::from(0)])?;
                                } else {
                                    ctx.call(Function::GetExperience, vec![Val::from(20000), Val::from(0)])?;
                                }
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()? == 97 {
                            ctx.lines(args![
                                "Umm....",
                                "Did I see you wrong? Or did Metz do?",
                                "...I do not have any more business with you.",
                                "Take care now."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("sign_q").get()? == 98 {
                            ctx.lines(args![
                                "Umm...well then...",
                                "I shall take the action for Prontera...",
                                "And for Juno...",
                                "Awww....",
                                "Gosh, this is such a pain in the ass...",
                                "Mumble mumble..."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "How is it going?",
                                "In fact, we want to do it on our own...",
                                "but we are too preoccupied with our works...",
                                "I hope you will keep up the good work.",
                                "I am also curious what that stone is."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn wealthy_looking_merchant(ctx: &Ctx) -> Script {
    wealthy_looking_merchant_body(ctx, Vec::new()).map(|_| ())
}

fn maid_s10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Seylin]")?;
    if ctx.var("sign_q").get()?.number()? < 38 {
        ctx.lines(args![
            "I'm sorry, but I'm",
            "busy cleaning the manor",
            "right now so I don't have",
            "time to talk. Oh, and would",
            "you please wipe your shoes",
            "on the mat before coming in?"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 38 {
            ctx.lines(args![
                "Leibech...?",
                "I remember that he's",
                "a regular customer who",
                "orders the strangest things.",
                "Last I heard, he was traveling",
                "near Mount Mjolnir."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Seylin",
                args![
                    "Yes, that was where",
                    "his last order was sent",
                    "to. Hm, I wonder. Do you",
                    "think he could have been",
                    "heading towards the",
                    "Schwarzwald Republic?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("sign_q").get()?.number()? < 42 {
                ctx.lines(args!["Ooooh...", "What am I going", "to do?! Oh-- You", "startled me!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Seylin",
                    args![
                        "I'm sorry, but",
                        "I was just thinking",
                        "about something. I hope",
                        "I didn't bother you. ^333333*Sigh*^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 42 {
                ctx.lines(args!["Ooooh...", "What am I going", "to do?! Oh-- You", "startled me!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Seylin",
                    args![
                        "I'm sorry, but",
                        "I was just thinking",
                        "about something. I hope",
                        "I didn't bother you. ^333333*Sigh*^000000"
                    ],
                )?;
                ctx.var("sign_q").set(Val::from(43))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()?.number()? < 51 {
                'b1: {
                    let subject1 = ctx.var("sign_q").get()?;
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(43))
                        && !subject1.loosely_equals(&Val::from(44))
                        && !subject1.loosely_equals(&Val::from(45))
                        && !subject1.loosely_equals(&Val::from(46))
                        && !subject1.loosely_equals(&Val::from(47))
                        && !subject1.loosely_equals(&Val::from(48))
                        && !subject1.loosely_equals(&Val::from(49))
                        && !subject1.loosely_equals(&Val::from(50));
                    if !matched1 && subject1.loosely_equals(&Val::from(43)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "^333333*Sigh...*^000000",
                            "Oh dear...",
                            "What can I do",
                            "about this...?"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Ignore her.:Excuse me, but Mahatra sent me...")])? {
                            1 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args![
                                        "Oh, it's no use",
                                        "worrying about",
                                        "something I can do",
                                        "nothing about. I better",
                                        "get back to work..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args!["Mister Mahatra", "sent you to me?", "Hm? Does he need", "me for anything?"],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Actually, he wants me to help you.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Seylin",
                                            args![
                                                "Oh, how very kind!",
                                                "Yes, yes, I could use",
                                                "some help right now!",
                                                "You see, it'll be my brother's",
                                                "birthday, but I'm so busy with",
                                                "work that I can't see him."
                                            ],
                                        )?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Seylin",
                                            args![
                                                "The last time I saw him,",
                                                "he looked so tired and weak",
                                                "so it's worrying me. Now, I've",
                                                "heard there's some new medicine",
                                                "called ^FF0000Vigorgra^000000. I don't know much about it, but it should help him!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Seylin",
                                            args![
                                                "Do you think you could",
                                                "find that medicine and",
                                                "bring it to me? If you don't",
                                                "want to, I understand. Plus,",
                                                "I don't know if the master",
                                                "would approve of this..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("I can't do it.:Let me think about it.:Sure, why not?")],
                                        )? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Seylin",
                                                    args![
                                                        "Oh, really?",
                                                        "Alright, I can",
                                                        "understand. I'm",
                                                        "very sorry to bother",
                                                        "you with my problems."
                                                    ],
                                                )?;
                                                ctx.var("sign_q").set(Val::from(44))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Seylin",
                                                    args![
                                                        "Oh okay.",
                                                        "Take your time.",
                                                        "If you can help",
                                                        "me, I'd really",
                                                        "appreciate it~"
                                                    ],
                                                )?;
                                                ctx.var("sign_q").set(Val::from(45))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Seylin",
                                                    args![
                                                        "Oh, thank you so much!",
                                                        "Now, I remember hearing",
                                                        "that you can find Vigorgra",
                                                        "somewhere in Al de Baran.",
                                                        "Somebody in the Alchemist",
                                                        "Guild makes it, I think."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Seylin",
                                                    args![
                                                        "I'm sorry, but I don't",
                                                        "know much about the medicine.",
                                                        "Well, aside from the fact that",
                                                        "it makes men feel young again.",
                                                        "So that must be good, right?",
                                                        "Thanks again for your help~"
                                                    ],
                                                )?;
                                                ctx.var("sign_q").set(Val::from(46))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(44)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "Hello there~",
                            "Oh, have you",
                            "changed your mind",
                            "about helping me?",
                            "I'll repay you,",
                            "of course."
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("No.:Yes.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args!["Oh really?", "I'm sorry, I...", "I just thought", "that maybe..."],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args![
                                        "Oh, thank you so much!",
                                        "Now, I remember hearing",
                                        "that you can find Vigorgra",
                                        "somewhere in Al de Baran.",
                                        "Somebody in the Alchemist",
                                        "Guild makes it, I think."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Seylin",
                                    args![
                                        "I'm sorry, but I don't",
                                        "know much about the medicine.",
                                        "Well, aside from the fact that",
                                        "it makes men feel young again.",
                                        "So that must be good, right?",
                                        "Thanks again for your help~"
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(46))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(45)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "Hello there~",
                            "So have you decided",
                            "yet? If you help me,",
                            "I'll be sure to pay",
                            "you back somehow."
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Um, I'm still thinking.:Yes.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args!["Oh okay.", "But please hurry~", "My brother's birthday", "is coming up soon!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Seylin",
                                    args![
                                        "Oh, thank you so much!",
                                        "Now, I remember hearing",
                                        "that you can find Vigorgra",
                                        "somewhere in Al de Baran.",
                                        "Somebody in the Alchemist",
                                        "Guild makes it, I think."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Seylin",
                                    args![
                                        "I'm sorry, but I don't",
                                        "know much about the medicine.",
                                        "Well, aside from the fact that",
                                        "it makes men feel young again.",
                                        "So that must be good, right?",
                                        "Thanks again for your help~"
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(46))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(46)) {
                        matched1 = true;
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(47)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "I know that you might",
                            "be able to find Vigorgra",
                            "in the Alchemist's Guild",
                            "in Al de Baran. Aside from",
                            "that, I don't know much..."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Seylin", args!["Once you get it,", "would you bring", "the Vigorgra to me?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(48)) {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.call(Function::CountItem, vec![Val::from(7044)])?.number()? < 1 {
                            ctx.lines(args![
                                "Oh, you know where",
                                "you can get Vigorgra?",
                                "That's great news!",
                                "Oh, when you have it,",
                                "bring to me, okay?"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "Hurray~!",
                                "You brought the",
                                "Vigorgra! I hope",
                                "my brother will",
                                "be happy with this!"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Seylin",
                                args![
                                    "Oh! Would you please",
                                    "bring this Vigorgra to my",
                                    "brother? His name is Maruin",
                                    "and he's working over at",
                                    "the Prontera Castle."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Seylin",
                                args![
                                    "Ah, I almost forgot!",
                                    "I just finished writing",
                                    "him this letter, so would",
                                    "you also make sure he gets",
                                    "this? Thank you so much!"
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(49))?;
                            ctx.call(Function::GetItem, vec![Val::from(7183), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(49)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "Would you please",
                            "deliver the Vigorgra",
                            "and my letter to my brother",
                            "Maruin in the Prontera Castle?",
                            "And wish him 'Happy Birthday~'"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(50)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines(args![
                            "Thanks so much",
                            "for your help again!",
                            "I want you to have this",
                            "as a token of my gratitude.",
                            "You really are a kind person~"
                        ])?;
                        ctx.var("sign_q").set(Val::from(51))?;
                        ctx.call(Function::GetItem, vec![Val::from(525), Val::from(3)])?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUPCHUP")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else if ctx.var("sign_q").get()? == 97 {
                ctx.lines(args![
                    "Oh no~!",
                    "Please don't",
                    "drag your muddy",
                    "feet over the carpet!",
                    "Would you please leave?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 98 {
                ctx.lines(args![
                    "I'm sorry, but I'm",
                    "busy cleaning the manor",
                    "right now so I don't have",
                    "time to talk. Oh, and would",
                    "you please wipe your shoes",
                    "on the mat before coming in?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "Although the master isn't",
                    "a messy person, he's pretty",
                    "unorganized. I wish he'd make",
                    "it easier for me to tell which",
                    "papers are documents and ",
                    "which ones are trash..."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Seylin",
                    args![
                        "So how have you been?",
                        "Is everything okay with you?",
                        "I hope you have a lot of good",
                        "experiences in your adventures~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn maid_s10(ctx: &Ctx) -> Script {
    maid_s10_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_s11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Maruin]")?;
    if ctx.var("sign_q").get()?.number()? < 49 {
        ctx.lines(args![
            "Welcome to",
            "Prontera Castle.",
            "Please be careful",
            "and avoid getting lost."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Maruin",
            args![
                "Oh man...",
                "I'm so tired~!",
                "Enervated, even~",
                "If I only had some",
                "kind of refreshment..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 49 {
        if (ctx.call(Function::CountItem, vec![Val::from(7044)])?.number()? < 1
            && ctx.call(Function::CountItem, vec![Val::from(7183)])?.number()? < 1)
        {
            ctx.lines(args![
                "Welcome to",
                "Prontera Castle.",
                "Please be careful",
                "and avoid getting lost."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Oh man...",
                    "I'm so tired~!",
                    "Enervated, even~",
                    "If I only had some",
                    "kind of refreshment..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Welcome to",
                "Prontera Castle.",
                "Please be careful",
                "and avoid getting los--"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args!["What's that?", "You're here for", "my sister? Um, did", "something happen...?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Oh no, she sent you a birthday gift.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Birthday...?",
                    "What are you sm--",
                    "OH. It is my birthday.",
                    "Ha! I completely forgot!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Thanks for coming",
                    "all the way out here",
                    "to deliver my present!",
                    "Wow, I wonder what she",
                    "got for me this year?"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Th-this is--!",
                    "Oh. Snap. That's right.",
                    "She's been worried about",
                    "me being tired lately. But",
                    "does she even know what",
                    "Vigorgra is supposed to...?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Never mind. It's the",
                    "thought that counts.",
                    "But for the record, it's",
                    "not like I need this stuff.",
                    "I mean, I have, you know,",
                    "a girlfr--you know what I mean."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "I'd like to pay you",
                    "back, but since I'm",
                    "on duty, all I can do",
                    "is replenish your health.",
                    "I'm not really supposed to,",
                    "but no one will say anything."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7044), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7183), Val::from(1)])?;
            ctx.var("sign_q").set(Val::from(50))?;
            ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Well, thanks for",
                    "the letter and the",
                    "male supplement, I guess.",
                    "I'll send a message to my",
                    "sister. Good luck on your",
                    "travels, alright?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if (ctx.var("sign_q").get()? == 97 || ctx.var("sign_q").get()? == 98) {
            ctx.lines(args![
                "Welcome to",
                "Prontera Castle.",
                "Please be careful",
                "and avoid getting lost."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Maruin",
                args![
                    "Oh man...",
                    "I'm so tired~!",
                    "Enervated, even~",
                    "If I only had some",
                    "kind of refreshment..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Oh hey, how's",
                "it going? Thanks",
                "again for helping",
                "out my sister, we",
                "really appreciate it."
            ])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?)
                && ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 5
            {
                ctx.next()?;
                ctx.lines_as(
                    "Maruin",
                    args![
                        "Oh...",
                        "And just between you",
                        "and me, that Vigorgra",
                        "really came in handy!",
                        "That stuff's amazing!"
                    ],
                )?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn soldier_s11(ctx: &Ctx) -> Script {
    soldier_s11_body(ctx, Vec::new()).map(|_| ())
}

fn alchemist_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Melkaba]")?;
    if ctx.var("sign_q").get()?.number()? < 46 {
        ctx.lines(args![
            "Most Alchemists seek out the",
            "Stone of Sage, but in my opinion, their goals are too short sighted. There are more important studies",
            "to be conducted in the name of science for the good of mankind..."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("...:So what are you studying?")])? {
            1 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "In the end, the ",
                        "Stone of Sage may",
                        "just be a simple rumor.",
                        "After all, thousands of",
                        "Alchemists have already",
                        "failed to create it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "What am I studying?",
                        "Well, my expertise is...",
                        "Different. If you must",
                        "know, it's a secret~",
                        "If any man found out",
                        "about my research..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 46 {
        ctx.lines(args![
            "Eh...?",
            "What do you need?",
            "If it's not important,",
            "please leave my",
            "laboratory!"
        ])?;
        ctx.next()?;
        'b2: {
            let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Vigorgra?:Oh, I'm sorry.")])?);
            let mut matched2 = false;
            let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "What?! Who told you",
                        "that I could make that?!",
                        "Oh, no matter. I suppose",
                        "you want me to get some",
                        "ready for you. Now tell me...",
                        "Why do you want it?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No, I don't want it!:I need Vigorgra for... a friend.")])? {
                    1 => {
                        ctx.lines_as(
                            "Melkaba",
                            args![
                                "You immature--!",
                                "You come all this",
                                "way to bother me?",
                                "Don't bring your",
                                "shame into my lab!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Melkaba",
                            args!["Oh right.", "Your friend.", "I understand you.", "Crystal clear."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Melkaba",
                            args![
                                "Well, since you know",
                                "about my secret research,",
                                "I'll do you a favor so long as",
                                "you don't tell anyone else!",
                                "Of course, I can't just give",
                                "you Vigorgra for free..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Melkaba",
                            args![
                                "Now, write this",
                                "down. These are the",
                                "ingredients I need to",
                                "make a bottle of Vigorgra."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Melkaba",
                            args![
                                "^ff00001 Ment",
                                "20 Honey",
                                "10 Royal Jelly",
                                "30 Bear's Footskin",
                                "1 Hinalle Leaflet",
                                "1 Empty Bottle.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Melkaba",
                            args![
                                "Alright.",
                                "Now go and",
                                "bring all those",
                                "items to me. I'll",
                                "be seeing you later."
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(47))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "There's no need",
                        "to apologize. But",
                        "I'd appreciate it if",
                        "you'd let me work",
                        "in peace."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("sign_q").get()? == 47 {
        ctx.lines(args![
            "Hmmm...",
            "So did you",
            "bring everything",
            "you need for the",
            "Vigorgra...?"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Not yet...:Yes.")])? {
            1 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "Well, let me",
                        "remind you of",
                        "what you need to",
                        "bring to me in case",
                        "you forgot already..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "^ff00001 Ment",
                        "20 Honey",
                        "10 Royal Jelly",
                        "30 Bear's Footskin",
                        "1 Hinalle Leaflet",
                        "1 Empty Bottle.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "Take your time...",
                        "In the end, you're",
                        "the one who's got",
                        "the need for it, anyway."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Melkaba",
                    args!["Great...!", "Now let me check", "to see if you have", "everything ready..."],
                )?;
                ctx.next()?;
                ctx.mes("[Melkaba]")?;
                if (((((ctx.call(Function::CountItem, vec![Val::from(526)])?.number()? < 10
                    || ctx.call(Function::CountItem, vec![Val::from(518)])?.number()? < 20)
                    || ctx.call(Function::CountItem, vec![Val::from(948)])?.number()? < 30)
                    || ctx.call(Function::CountItem, vec![Val::from(520)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(708)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? < 1)
                {
                    ctx.lines(args![
                        "Hmm, you're still",
                        "missing some of the",
                        "ingredients. Now, listen",
                        "carefully. I need you to get..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Melkaba",
                        args![
                            "^ff00001 Ment",
                            "20 Honey",
                            "10 Royal Jelly",
                            "30 Bear's Footskin",
                            "1 Hinalle Leaflet",
                            "1 Empty Bottle.^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "Good work.",
                        "Everything's here.",
                        "Alright, just this once",
                        "I'll make you a bottle",
                        "of Vigorgra."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Melkaba",
                        args![
                            "If you ask me again,",
                            "I won't speak to you!",
                            "And I'll need to charge",
                            "you 10,000 zeny."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Melkaba]")?;
                    if ctx.var("Zeny").get()?.number()? < 10000 {
                        ctx.lines(args![
                            "Huh...",
                            "It doesn't look like",
                            "you have enough zeny",
                            "right now. No matter,",
                            "I can wait until you",
                            "bring it all to me."
                        ])?;
                    } else {
                        ctx.lines(args![
                            "Alright, I'll be",
                            "taking my fee from",
                            "you now. It's not that",
                            "expensive when you",
                            "consider Vigorgra's",
                            "numerous benefits."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFMelkaba placed",
                            "all the ingredients",
                            "into a test tube and",
                            "began processing it",
                            "through some strange",
                            "electronic equipment.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......", "........."])?;
                        ctx.next()?;
                        ctx.mes("[Melkaba]")?;
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?.number()? < 98 {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PHARMACY_OK")?])?;
                            ctx.lines(args![
                                "Ah! Success!",
                                "A lifetime's supply",
                                "of Vigorgra! I know it's",
                                "just one bottle, but this",
                                "stuff is extremely potent!"
                            ])?;
                            ctx.next()?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                            ctx.call(Function::DelItem, vec![Val::from(526), Val::from(10)])?;
                            ctx.call(Function::DelItem, vec![Val::from(518), Val::from(20)])?;
                            ctx.call(Function::DelItem, vec![Val::from(948), Val::from(30)])?;
                            ctx.call(Function::DelItem, vec![Val::from(520), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(708), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                            ctx.var("sign_q").set(Val::from(48))?;
                            ctx.call(Function::GetItem, vec![Val::from(7044), Val::from(1)])?;
                            ctx.lines_as(
                                "Melkaba",
                                args![
                                    "Um, just remember",
                                    "not to take too much",
                                    "at one time. Can't have",
                                    "you getting crazy high",
                                    "blood pressure~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PHARMACY_FAIL")?])?;
                            ctx.call(Function::DelItem, vec![Val::from(526), Val::from(10)])?;
                            ctx.call(Function::DelItem, vec![Val::from(518), Val::from(20)])?;
                            ctx.call(Function::DelItem, vec![Val::from(948), Val::from(30)])?;
                            ctx.call(Function::DelItem, vec![Val::from(520), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(708), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                            ctx.lines(args![
                                "It failed?!",
                                "Of course, the mixture",
                                "was too vigorous, even",
                                "for the machinery! I'm sorry,",
                                "but would you bring the things",
                                "we need to make Vigorgra again?"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 97 {
        ctx.lines(args![
            "Most Alchemists seek out the",
            "Stone of Sage, but in my opinion, their goals are too short sighted. There are more important studies",
            "to be conducted in the name of science for the good of mankind..."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("...:So what are you studying?")])? {
            1 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "In the end, the",
                        "Stone of Sage may",
                        "just be a simple rumor.",
                        "After all, thousands of",
                        "Alchemists have already",
                        "failed to create it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "What am I studying?",
                        "Well, my expertise is...",
                        "Different. If you must",
                        "know, it's a secret~",
                        "If any man found out",
                        "about my research..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 98 {
        ctx.lines(args![
            "Most Alchemists seek out the",
            "Stone of Sage, but in my opinion, their goals are too short sighted. There are more important studies",
            "to be conducted in the name of science for the good of mankind..."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("...:So what are you studying?")])? {
            1 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "In the end, the",
                        "Stone of Sage may",
                        "just be a simple rumor.",
                        "After all, thousands of",
                        "Alchemists have already",
                        "failed to create it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Melkaba",
                    args![
                        "What am I studying?",
                        "Well, my expertise is...",
                        "Different. If you must",
                        "know, it's a secret~",
                        "If any man found out",
                        "about my research..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "Vigorgra is",
            "a miracle of",
            "modern science,",
            "but it can do more",
            "harm than good if",
            "you're not careful!"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn alchemist_sign(ctx: &Ctx) -> Script {
    alchemist_sign_body(ctx, Vec::new()).map(|_| ())
}

fn refined_steward_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Mahatra]")?;
    if ctx.var("sign_q").get()?.number()? < 37 {
        ctx.lines(args![
            "Welcome to the",
            "Alchesh Estate.",
            "The Alcheshs are the",
            "most esteemed family",
            "in all of Alberta."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Mahatra",
            args![
                "I am Mahatra Mornen,",
                "steward of this family.",
                "Feel free to ask me if",
                "you need anything and",
                "I shall do my best",
                "to assist you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 37 {
            ctx.lines(args![
                "Ah yes, the master informed",
                "me that you would be taking",
                "care of an urgent delivery for",
                "him. Please take this to a",
                "man named ^3355FFLeibech^000000."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Mahatra",
                args![
                    "It may be difficult",
                    "to find him since he",
                    "travels around the world.",
                    "Ah yes, I have heard that",
                    "he was in some other country..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mahatra",
                args![
                    "Now, the item I am",
                    "about to give you is",
                    "one of a kind. Don't lose",
                    "it or you will fall out of favor with the master..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFMahatra gives you",
                "a unique stone that has",
                "your name attached to it.^000000"
            ])?;
            ctx.var("sign_q").set(Val::from(38))?;
            ctx.call(
                Function::GetNamedItem,
                vec![Val::from(7049), ctx.call(Function::StrCharInfo, vec![Val::from(0)])?],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("sign_q").get()? == 38
                || (ctx.var("sign_q").get()? == 39 && ctx.call(Function::CountItem, vec![Val::from(7181)])?.number()? < 1))
            {
                ctx.lines(args![
                    "You haven't delivered",
                    "the Stone yet? Please take",
                    "good care of it since it's very",
                    "rare. Don't use it or lose it.",
                    "before it's delivered!",
                    "Take care, now..."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("sign_q").get()? == 39 {
                    ctx.lines(args![
                        "Ah, you've returned.",
                        "And I see you already",
                        "delivered the Stone. Great,",
                        "well done. Let me send a",
                        "message to the master, so",
                        "please visit him later."
                    ])?;
                    ctx.call(Function::DelItem, vec![Val::from(7181), Val::from(1)])?;
                    ctx.var("sign_q").set(Val::from(40))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()? == 40 {
                        ctx.lines(args![
                            "Thank you very",
                            "much for the trouble",
                            "you've gone through",
                            "on our behalf."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 41 {
                        ctx.lines(args![
                            "Greetings. Hm?",
                            "Did the master ask",
                            "you to perform another",
                            "task for him already?"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("No.:You have another assignment for me.")])? {
                            1 => {
                                ctx.lines_as("Mahatra", args!["Very well.", "Then we shall", "converse later~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Mahatra",
                                    args![
                                        "I... I do...?",
                                        "The master didn't",
                                        "leave me any instructions.",
                                        "Hmmm. Oh, I know what you can",
                                        "do! Why don't you help ^FF0000Seylin^000000? She seems bothered by something..."
                                    ],
                                )?;
                                ctx.var("sign_q").set(Val::from(42))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("sign_q").get()?.number()? < 51 {
                        ctx.lines(args![
                            "Although you may only",
                            "be doing it for the Sobbing",
                            "Starlight, I really appreciate",
                            "all your help. Seylin's a very",
                            "nice girl and doesn't deserve",
                            "to feel down..."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 51 {
                        ctx.lines(args![
                            "Yes.",
                            "I heard from Seylin.",
                            "I did not know she was worried about her brother.",
                            "Ahem ahem...",
                            "Thank you for helping her.",
                            "I will send a message to my master."
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "Mahatra]",
                            "If there is anything else, he will let you pass the test.",
                            "Good luck."
                        ])?;
                        ctx.var("sign_q").set(Val::from(52))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 98 {
                        ctx.lines(args![
                            "I'm very disappointed",
                            "in you. We put our trust",
                            "in you and you really",
                            "let us down."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mahatra",
                            args![
                                "Well, I suppose",
                                "that's the nature",
                                "of the testing. They're",
                                "designed so that only",
                                "the best of the best",
                                "can pass them."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "I hope the master listens",
                            "to me and dresses more like",
                            "a man in his position. Even if",
                            "he thinks he's just a merchant,",
                            "he's the master of this estate!",
                            "But please don't tell him that~"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn refined_steward(ctx: &Ctx) -> Script {
    refined_steward_body(ctx, Vec::new()).map(|_| ())
}

fn guard_s10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Hurubu", args!["Welcome to the", "Alchesh Estate."])?;
    if ctx.var("sign_q").get()?.number()? < 35 {
        ctx.lines(args!["If you have any business", "with the master, please", "let me know."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()?.number()? < 52 {
        ctx.lines(args!["If you have any business", "with the master, please", "let me know."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'd like to see Mr. Bakerlan.:Have a nice day.")])? {
            1 => {
                ctx.lines_as(
                    "Hurubu",
                    args![
                        "Ah, you've",
                        "come to speak",
                        "to the master?",
                        "He's not out on",
                        "business, so you",
                        "can probably see him."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Hurubu", args!["Ah...", "Good day."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("sign_q").get()? == 97 || ctx.var("sign_q").get()? == 98) {
        ctx.lines(args!["Please leave if", "you do not have", "any business here."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "Lately, it seems that",
            "the master is in a good",
            "mood. I believe that we",
            "have you to thank for that."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn guard_s10(ctx: &Ctx) -> Script {
    guard_s10_body(ctx, Vec::new()).map(|_| ())
}

fn poor_looking_merchant_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Machen]")?;
    if ctx.var("sign_q").get()?.number()? < 35 {
        ctx.lines(args![
            "Why...?",
            "How did my",
            "family's wealth",
            "and prestige just",
            "go down the drain?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Machen",
            args![
                "Ever since the",
                "Alcheshs came to Alberta,",
                "they quickly took our place",
                "as the biggest business owners",
                "here. It's our biggest shame..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("sign_q").get()?.number()? < 52 || ctx.var("sign_q").get()? == 98) {
        ctx.lines(args![
            "^333333*Sigh...*^000000",
            "How did it happen?",
            "How did the Alcheshs",
            "become so rich so quickly?"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Alchesh family?:Do you know where Mr. Bakerlan is?")])? {
            1 => {
                ctx.lines_as(
                    "Machen",
                    args![
                        "My family, the Tudas,",
                        "used to run the biggest",
                        "trading company in Alberta.",
                        "But when the Alcheshs came",
                        "fifty years ago, they started",
                        "to outdo us in everything..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Machen",
                    args![
                        "It's like they're financial",
                        "geniuses or something!",
                        "Under Bakerlan's guidance,",
                        "their estate has grown to be",
                        "one of the richest in Alberta,",
                        "if not all of Rune-Midgarts..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Machen",
                    args![
                        "Bakerlan...?",
                        "Oh, he's probably",
                        "in his ten story money",
                        "vault, swimming in his",
                        "money by now. You",
                        "know rich people."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from(".........")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Machen",
                    args![
                        "It-was-a-joke!",
                        "You see that big",
                        "mansion to the north?",
                        "That's the Alchesh Estate.",
                        "He should be home now."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Machen",
                    args![
                        "He usually travels",
                        "a lot on business,",
                        "but if you're lucky you",
                        "should be able to catch him."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 97 {
        ctx.lines(args![
            "Why...?",
            "How did my",
            "family's wealth",
            "and prestige just",
            "go down the drain?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Machen",
            args![
                "Ever since the",
                "Alcheshs came to Alberta,",
                "they quickly took our place",
                "as the biggest business owners",
                "here. It's our biggest shame..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "You know, if the Alcheshs",
            "were greedy, they actually",
            "wouldn't be as big as they",
            "are today. They've contributed",
            "to a lot of social causes and",
            "that won them a lot of support."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Machen",
            args![
                "I gotta admit...",
                "The whole family",
                "is a bunch of financial",
                "geniuses, alright."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn poor_looking_merchant(ctx: &Ctx) -> Script {
    poor_looking_merchant_body(ctx, Vec::new()).map(|_| ())
}
