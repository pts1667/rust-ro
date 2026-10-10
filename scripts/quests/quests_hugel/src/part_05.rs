use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn el_schatt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_memory").get()? == 2 {
        ctx.lines_as(
            "El Schatt",
            args![
                "What? What's that look for?",
                "Oh, you must have spoken to",
                "Manainne. Look, memories are",
                "nice and all, but I don't dwell",
                "on the past. I think about the",
                "future. That's what's best."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "El Schatt",
            args![
                "Hey, I've got some great",
                "memories of this place too,",
                "but you know what? I've decided",
                "to let go of them before they",
                "get too painful. That's life.",
                "Besides, it's not all bad."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "El Schatt",
            args![
                "My father's been wanting to",
                "build a shop here for so long,",
                "and now, I think it's about time. And this business will help the",
                "entire community. We should've",
                "done this a long time ago..."
            ],
        )?;
        ctx.next()?;
        ctx.var("hg_memory").set(Val::from(3))?;
        ctx.lines_as(
            "El Schatt",
            args![
                "C'mon... Take a look",
                "around. I won't deny that",
                "this is a quaint and lovely",
                "town, but how much money do",
                "you think these people enjoy?",
                "I need to look to the future..."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8058), Val::from(8059)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_memory").get()? == 3 {
            ctx.lines_as(
                "El Schatt",
                args![
                    "This is the perfect",
                    "location for shops.",
                    "Yeah, I'll build a whole",
                    "plaza, and rent it out to the",
                    "merchants. Build up some",
                    "commerce, attract businesses..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "El Schatt",
                args![
                    "Everyone should just",
                    "forget about that tree.",
                    "When you think about it,",
                    "it's just a waste of space..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_memory").get()? == 4 {
                ctx.var("hg_memory").set(Val::from(5))?;
                ctx.lines_as(
                    "El Schatt",
                    args![
                        "You've spoken with my father?",
                        "Hmpf! Don't tell me you're here",
                        "to change my mind. I'm building",
                        "those shops, and that's final.",
                        "It's useless to try to convince",
                        "me otherwise. Understand?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "El Schatt",
                    args![
                        "It's true that I used",
                        "to think differently...",
                        "I'd protect that old tree",
                        "with my life, I won't deny",
                        "it. But that's an old story that should be buried in the past..."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8060), Val::from(8061)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_memory").get()? == 5 {
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "The past is behind me",
                            "as it should be. Now",
                            "I must focus on building",
                            "my family's businesses.",
                            "Father was right all along..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_memory").get()? == 7 {
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "What did you say...?",
                            "You met Kanainne?",
                            "Listen, that's not funny.",
                            "I won't tolerate that kind",
                            "of sick joke, so get out",
                            "of here before I get angry."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_memory").get()? == 10 {
                    ctx.lines_as(
                        "El Schatt",
                        args!["No way...!", "Kanainne is dead!", "There's no way", "I can believe this!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Manainne", args!["......", ".........", "............"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_memory").get()? == 11 {
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "K-Kanainne! ^333333*Sob*^000000",
                            "I've missed you so much!",
                            "Why did you leave me?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Oh, my dear,",
                            "sweet El Schatt...",
                            "I'm so sorry. But",
                            "that was my fate...",
                            "It was my time to go...",
                            "You have to let me go..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "I can't take it anymore!",
                            "It hurts too much without you!",
                            "I can't stop thinking about us,",
                            "and all the memories... all of",
                            "the memories hurt so much..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Is that why you want to",
                            "cut down that tree? Because",
                            "it reminds you too much of",
                            "all of our days together?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "Y-yes! Seeing it makes",
                            "me so angry and so sad!",
                            "I wanted to get rid of it!",
                            "If I don't forget you, then",
                            "what will I do? I c-can't",
                            "forget about you, c-can I?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Will destroying that tree",
                            "make you happy? Do you think",
                            "you'll be happy if you forget",
                            "all about me? I know you're",
                            "not like that, El Schatt.",
                            "You're not that cold..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "No... I don't want",
                            "to forget you. All of",
                            "our good times. But I miss",
                            "you so much... I think I'm",
                            "going to go insane! I don't",
                            "know if I can endure this pain!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Please be strong for me,",
                            "El Schatt. Someday, time",
                            "will heal your wounds, and",
                            "your memories of me will only",
                            "bring you fondness, not pain.",
                            "I miss you El Schatt, and I..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("El Schatt", args!["Kanainne...?", "K-Kanaine?", "What's happening?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "El Schatt, I'm so sorry...",
                            "But I have to go now. I can't",
                            "stay in the world of the living",
                            "for too long, and my time limit",
                            "is almost up. I still... I..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "N-no! Please, stay with",
                            "me! I'm not ready to lose",
                            "you again! D-don't leave",
                            "me again! Nooo! Kanainne!",
                            "I need you here with me!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "I can't do that...",
                            "But please, El Schatt...",
                            "Promise that you'll be",
                            "yourself from now on...",
                            "Be kind... And follow",
                            "your true dreams..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Don't cut down",
                            "that tree... And make...",
                            "make beautiful music.",
                            "Please finish the song",
                            "that you made for me for...",
                            "for both of our sakes..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("El Schatt", args!["Anything for you,", "Kanainne. I vow it..."])?;
                    ctx.next()?;
                    ctx.lines_as("Manainne", args!["El Schatt...", "I still... I still..."])?;
                    ctx.next()?;
                    ctx.lines_as("El Schatt", args!["Y-yes...", "Kanainne...?"])?;
                    ctx.next()?;
                    ctx.lines_as("Manainne", args!["......", ".........", "............"])?;
                    ctx.next()?;
                    ctx.mes("^3355FF*Thud!*^000000")?;
                    ctx.next()?;
                    ctx.lines_as("El Schatt", args!["Kanainne...?", "Kanainne!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "...I...",
                            "...El Schatt...",
                            "She's gone again.",
                            "It's... I'm Manainne...",
                            "^333333*Sniff*^000000 She had to go back..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                    ctx.var("hg_memory").set(Val::from(12))?;
                    ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(8063)])?;
                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "Oh... Oh dear",
                            "sweet God... My...",
                            "My dear Kanainne...",
                            "No.... No... Please..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_memory").get()? == 12 {
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "...Would you please",
                            "leave me alone? I have...",
                            "I have an awful lot to",
                            "think about. And there's",
                            "music that I have to write..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "El Schatt",
                        args![
                            "Father was right...",
                            "This area is the perfect",
                            "place to build some stores.",
                            "All that's left is to chop down",
                            "that old tree. It's in the way",
                            "of a whole lot of things..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn el_schatt(ctx: &Ctx) -> Script {
    el_schatt_body(ctx, Vec::new()).map(|_| ())
}

fn torpy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 49 {
        if !(ctx.var("hg_ubu01").get()?.is_true()) {
            ctx.lines_as("Torpy", args!["D-Daddy...!", "Daddy where", "are you?! Wah~!"])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("What's wrong?:Ignore")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Torpy",
                        args![
                            "My Daddy hasn't",
                            "been home for a few",
                            "day now. I'm afraid that",
                            "something must have",
                            "happened to him. *Sniff*"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Your Dad is missing?",
                            "Why don't you tell me",
                            "about the last time",
                            "that you saw him?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Torpy",
                        args![
                            "Well, we were playing",
                            "Hide-and-Seek, and then",
                            "it was his turn to hide,",
                            "and then, a-and then...",
                            "I never found him!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Really? Tell me more...:What, is that all...?")])? {
                        1 => {
                            ctx.lines_as(
                                "Torpy",
                                args![
                                    "Th-then my Mom got",
                                    "mad at Dad for hiding",
                                    "for so long. But when",
                                    "she went to find him, she",
                                    "disappeared too! I-I'm all",
                                    "alone now! Waaaaaah~!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_THINK")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Don't worry, I'll look",
                                    "for your parents. I'm",
                                    "sure that I can find at",
                                    "least one of them just",
                                    "outside of town."
                                ],
                            )?;
                            ctx.var("hg_ubu01").set(Val::from(1))?;
                            ctx.call(Function::SetQuest, vec![Val::from(12044)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Don't worry, your",
                                    "Dad will probably",
                                    "show up sooner",
                                    "or later. Well, lots",
                                    "of luck, kid."
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_THINK")?,
                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Torpy",
                        args!["D-Daddy...!", "Mommiiiiiiiiie!", "Where are yoooou?!", "Wuh-Waaaaaaaaaah!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("hg_ubu01").get()? == 2 {
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Oh...",
                        "Mommy was just",
                        "outside of town?",
                        "*Sob* Okay, I hope my",
                        "Daddy comes back soon..."
                    ],
                )?;
                ctx.var("hg_ubu01").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(12045), Val::from(12046)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 6 {
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Y-you found my Daddy!",
                        "Hooray! I was so worried",
                        "about him! Th-thank you",
                        "so much for your help!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Um, um, where is my",
                        "Daddy hiding anyway?",
                        "Oh? A barrel? Where",
                        "was it again? Heh heh!",
                        "Now I can find him!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Oh, would you go talk",
                        "to my Mom, and tell her",
                        "to come here please? And",
                        "also tell her that Daddy",
                        "is safe! See you later~"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "^333333...............................",
                        "Hee hee~ Once I find",
                        "my Daddy, he has to get me",
                        "a girlfriend! He promised~!^000000"
                    ],
                )?;
                ctx.var("hg_ubu01").set(Val::from(7))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 7 {
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Would you please hurry",
                        "and talk to my Mom? She",
                        "should be just outside of?",
                        "this town, where you last",
                        "saw her. Bye bye for now~"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "^333333...............................",
                        "Oh yeah! Should I have",
                        "Mom help me pick out my",
                        "new girlfriend? Hmm, I dunno.",
                        "Wh-what if they become friends?",
                        "Um, that might not be good...^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 8 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Daddy promised to get",
                        "me a girlfriend if I beat him",
                        "at Hide-and-Seek! Hahaha!",
                        "I won, so now he has to do it!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Wait, but should I let",
                        "my Dad pick my girlfriend?",
                        "Maybe I should pick her...",
                        "Yeah, I'll pick the perfectest",
                        "girl that I can find! Yaaay~"
                    ],
                )?;
                ctx.var("hg_ubu01").set(Val::from(9))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 9 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    "Torpy",
                    args![
                        "Maybe I should wait until",
                        "I'm older to have a girlfriend.",
                        "I mean, what if the girl I pick",
                        "now gets uglier when she",
                        "grows up? Or what if she gets really fat? I better be careful..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFTorpy is still a young,",
                    "naive child that has much",
                    "to learn about girlfriends,",
                    "or just people, in general.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Torpy",
                    args![
                        "^333333*Sniff*^000000 D-Daddy's gone!",
                        "Will you help me find my",
                        "Daddy please? I h-have",
                        "to find him now! H-Hurry!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as(
            "Torpy",
            args!["Waaah~!", "I have to find", "my Daddy! Daddy...!", "Where are yooooou?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn torpy(ctx: &Ctx) -> Script {
    torpy_body(ctx, Vec::new()).map(|_| ())
}

fn torpy_s_mom_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_ubu01").get()? == 0 {
        ctx.lines_as(
            "Torpy's Mom",
            args![
                "Where did he go this time?!",
                "If my husband doesn't have",
                "a good excuse for not coming",
                "home, then he better come",
                "up with one or he'll be sorry!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_ubu01").get()? == 1 {
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Hmm? Oh, you talked to",
                    "my little Torpy? He thinks his",
                    "father and I just vanished?",
                    "Oh, I'm so sorry, I think there's been some misunderstanding!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Although Torpy's father has",
                    "been missing for a little",
                    "while, I just stepped out of",
                    "the house this morning. So",
                    "I haven't even been gone for",
                    "a few hours, much less a day."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "You see, my husband is",
                    "a hunter, so it's usual for him",
                    "not to come home for a few",
                    "days. Still, he's never been",
                    "gone for this long before,",
                    "so I've been looking for him..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "I've been getting a little",
                    "worried myself, honestly.",
                    "He's not in any of his usual",
                    "hunting grounds. Hopefully,",
                    "if I wait here long enough,",
                    "he'll show up sooner or later."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "If he doesn't show up",
                    "today, I better search",
                    "for him inside town. Maybe",
                    "he's hiding somewhere,",
                    "like at his friend's house.",
                    "Why is like always like this?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Listen, would you please",
                    "tell my son Torpy that I'm",
                    "fine and will be coming back",
                    "home soon? Also, if you see",
                    "my husband inside town, tell",
                    "him that he better come home!"
                ],
            )?;
            ctx.var("hg_ubu01").set(Val::from(2))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(12044), Val::from(12045)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_ubu01").get()? == 2 {
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Listen, would you please",
                    "tell my son Torpy that I'm",
                    "fine and will be coming back",
                    "home soon? Also, if you see",
                    "my husband inside town, tell",
                    "him that he better come home!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("hg_ubu01").get()? == 6 || ctx.var("hg_ubu01").get()? == 7) {
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "What?! He was hiding",
                    "inside a barrel this",
                    "entire time?! Oh, no.",
                    "I'm sorry, my husband has",
                    "caused you so much trouble."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "I really appreciate",
                    "everything you've done",
                    "to help us find him. I know",
                    "it's not much, but I hope",
                    "you accept this as our thanks.",
                    "Goodbye for now, adventurer~"
                ],
            )?;
            ctx.call(
                Function::SpecialEffect,
                vec![
                    (if ctx.var("hg_ubu01").get()? == 6 {
                        ctx.constant("EF_CONE")?
                    } else {
                        ctx.constant("EF_MVP")?
                    }),
                ],
            )?;
            ctx.var("hg_ubu01").set(Val::from(8))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(12048)])?;
            ctx.call(Function::GetItem, vec![Val::from(12065), Val::from(3)])?;
            ctx.call(Function::GetExperience, vec![Val::from(50000), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_ubu01").get()? == 8 {
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Sometimes I wonder if",
                    "Torpy is too mature for",
                    "his age, or if my husband",
                    "is too immature. In a strange",
                    "way, those two really take",
                    "after each other. Oh, well..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_ubu01").get()? == 9 {
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "When is that husband",
                    "of mine going to come",
                    "out of that barrel? He",
                    "didn't make another weird",
                    "bet with Torpy, did he? Oh,",
                    "the trouble with those two..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Torpy's Mom",
                args![
                    "Where did he go this time?!",
                    "If my husband doesn't have",
                    "a good excuse for not coming",
                    "home, then he better come",
                    "up with one or he'll be sorry!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn torpy_s_mom(ctx: &Ctx) -> Script {
    torpy_s_mom_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_barrel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_ubu01").get()? == 3 {
        ctx.lines_as(
            "Strange Man",
            args![
                "So... hun...gry...",
                "F-food... Give me...",
                "F-food... Must... replenish...",
                "h-health... with... Steamed...",
                "Crab... Nippers... Please..."
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines(args![
            "^3355FFIt sounds like",
            "this barrel wants",
            "Steamed Crab Nippers.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'll bring you Steamed Crab Nippers:Ignore")])? {
            1 => {
                ctx.lines_as(
                    "Strange Man",
                    args![
                        "Please... cook me...",
                        "Steamed... Crab Nippers...!",
                        "You... n-need.... 10 Green",
                        "Herbs... 1 Yellow Potions...",
                        "and 10... N-Nippers... to",
                        "c-cook th-them. Ugh..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hold on,", "I have to cook", "them for you?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Strange Man",
                    args!["Y-yes...", "It's... It's the", "o-only... possible...", "Way... P-please help..."],
                )?;
                ctx.var("hg_ubu01").set(Val::from(4))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(12046), Val::from(12047)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Strange Man",
                    args![
                        "D-don't... leave...",
                        "me... Here... Th-the...",
                        "hunger...! It... It can't...",
                        "It c-can't be... d-denied!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Weirdo! Why don't you",
                        "get out of that barrel,",
                        "and get your Steamed",
                        "Crab Nippers yourself?!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("hg_ubu01").get()? == 4 {
        if ctx.call(Function::CountItem, vec![Val::from(12051)])?.is_true() {
            ctx.lines_as(
                "Strange Man",
                args![
                    "Th-that... tantalizing...",
                    "scent... It's S-Steamed...",
                    "Crab... Nippers... Oh God...",
                    "It's b-been so... l-long...",
                    "G-Give me! G-Give me now!"
                ],
            )?;
            ctx.next()?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines(args![
                "^3355FFYou dumped the",
                "Steamed Crab Nippers",
                "into the barrel where",
                "it was quickly devoured.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(12051), Val::from(1)])?;
            ctx.var("hg_ubu01").set(Val::from(5))?;
            ctx.next()?;
            ctx.lines_as(
                "Strange Man",
                args![
                    "Ha ha ha!",
                    "Back in the game, baby!",
                    "Man, it's been, about what,",
                    "3 days since I last ate?",
                    "It's good to be alive!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Strange Man#Hugel::OnEnable")])?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Strange Man",
                args![
                    "Please... cook me...",
                    "Steamed... Crab Nippers...!",
                    "You... n-need.... 10 Green",
                    "Herbs... 1 Yellow Potions...",
                    "and 10... N-Nippers... to",
                    "c-cook th-them. Ugh..."
                ],
            )?;
            ctx.next()?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_THINK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hold on,", "I have to cook", "them for you?!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Strange Man",
                args!["Y-yes...", "It's... It's the", "o-only... possible...", "Way... P-please help..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("hg_ubu01").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(12051)])?.is_true() {
                ctx.lines_as(
                    "Strange Man",
                    args![
                        "Th-that... tantalizing...",
                        "scent... It's S-Steamed...",
                        "Crab... Nippers... Oh God...",
                        "It's b-been so... l-long...",
                        "G-Give me! G-Give me now!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args![
                    "^3355FFYou dumped the",
                    "Steamed Crab Nippers",
                    "into the barrel where",
                    "it was quickly devoured.^000000"
                ])?;
                ctx.call(Function::DelItem, vec![Val::from(12051), Val::from(1)])?;
                ctx.var("hg_ubu01").set(Val::from(5))?;
                ctx.next()?;
                ctx.lines_as(
                    "Strange Man",
                    args![
                        "Ha ha ha!",
                        "Back in the game, baby!",
                        "Man, it's been, about what,",
                        "3 days since I last ate?",
                        "It's good to be alive!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Strange Man#Hugel::OnEnable")])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Strange Man",
                    args![
                        "Please... cook me...",
                        "Steamed... Crab Nippers...!",
                        "You... n-need.... 10 Green",
                        "Herbs... 1 Yellow Potions...",
                        "and 10... N-Nippers... to",
                        "c-cook th-them. Ugh..."
                    ],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Hold on,", "I have to cook", "them for you?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Strange Man",
                    args!["Y-yes...", "It's... It's the", "o-only... possible...", "Way... P-please help..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("hg_ubu01").get()? == 6 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args![
                    "^3355FFFor now, you should",
                    "tell Torpy and his mom",
                    "where Torpy's father is",
                    "hiding so they won't",
                    "worry about him so much."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 7 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args![
                    "^800080Torpy's mom might want to know about^000000",
                    "^800080this extraordinary barrel.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 8 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.mes("^800080How long does Torpy's father plan to stay within the barrel?^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("hg_ubu01").get()? == 9 {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args![
                    "^800080That barrel will become a symbolic object^000000",
                    "^800080that pays tribute to the firm will of Torpy's father^000000",
                    "^800080who is trying hard to avoid keeping the promise with his son to find his wife.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_THINK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.lines(args![
                    "^800080You hear something bustling from the hole in the barrel.^000000",
                    "^800080However, it doesn't seem to be a big deal.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn suspicious_barrel(ctx: &Ctx) -> Script {
    suspicious_barrel_body(ctx, Vec::new()).map(|_| ())
}

fn strange_man_hugel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_ubu01").get()? == 5 {
        ctx.lines_as(
            "Strange Man",
            args![
                "Ahhhh... Thanks for",
                "the free meal! It was",
                "so invigorating! Well,",
                "I better climb back",
                "inside that barrel. See",
                "ya round, adventurer~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hold it!", "Why are you hiding", "inside that barrel?!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Strange Man",
            args!["Huh? Oh, I'm just", "playing Hide-and-Seek", "with my son, that's all."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "But you haven't",
                "eaten in days! Wait,",
                "hold on, you wouldn't",
                "happen to be Torpy's",
                "father now, would you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Strange Man",
            args![
                "Oh, you ran into my boy?",
                "Has he given up on our",
                "little wager yet? Because",
                "I'd really hate to lose..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Um, your wife and son",
                "are really worried about",
                "you. Shouldn't you be going",
                "home as soon as you can?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Strange Man",
            args![
                "No, I can't! Not until",
                "Torpy gives up! I made",
                "a really silly promise",
                "with my son, and if I lose",
                "this game of Hide-and-Seek...",
                "Well, I just can't lose! Bye!"
            ],
        )?;
        ctx.var("hg_ubu01").set(Val::from(6))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12047), Val::from(12048)])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFTorpy's father has",
            "hidden himself again,",
            "but at least now you",
            "can go back to Torpy's",
            "Mom and Torpy, and let them",
            "both know where he is hiding.^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Strange Man#Hugel")])?;
        ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Barrel")])?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Strange Man",
            args![
                "Hahahaha!",
                "Steamed Crab Nippers!",
                "That really hit the spot!",
                "I ate so much, I won't",
                "have to eat again for days!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn strange_man_hugel(ctx: &Ctx) -> Script {
    strange_man_hugel_body(ctx, Vec::new()).map(|_| ())
}

fn strange_man_hugel_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Strange Man#Hugel")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Suspicious Barrel")])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn strange_man_hugel_onenable(ctx: &Ctx) -> Script {
    strange_man_hugel_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn strange_man_hugel_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Strange Man#Hugel")])?;
    return Err(Stop::End);
}

pub fn strange_man_hugel_oninit(ctx: &Ctx) -> Script {
    strange_man_hugel_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn strange_man_hugel_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Strange Man#Hugel")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Suspicious Barrel")])?;
    return Err(Stop::End);
}

pub fn strange_man_hugel_ontimer60000(ctx: &Ctx) -> Script {
    strange_man_hugel_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn cellette_lavit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    if !(ctx.var("hg_odeng").get()?.is_true()) {
        ctx.lines_as(
            "Cellette",
            args![
                "Hi there, thanks for",
                "visiting our lovely town!",
                "Why don't you come in and",
                "have a taste of authentic",
                "Hugel cuisine? I guarantee",
                "that you won't regret it~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cellette",
            args![
                "My name is Cellette Lavit,",
                "and I proudly serve Fish Cake",
                "Soup, Hugel's specialty dish.",
                "All the tourists that've tried",
                "it have loved it, and I have",
                "many regular customers~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cellette",
            args![
                "This dish has something",
                "of a cult following, and it's",
                "not widely popularized, but I'm",
                "sure that it'll be considered",
                "representative of Schwarzwald",
                "Republic cuisine someday."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cellette",
            args![
                "Listen, you look like an",
                "able adventurer, so would",
                "you consider working part time",
                "for me? I have many orders for",
                "Fish Cake Soup, so there's",
                "no way I can make deliveries..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cellette",
            args![
                "If you work for me, I'll",
                "waive the price: I'll give you",
                "1 Fish Cake Soup for each",
                "delivery that you complete.",
                "But if you're not interested,",
                "you can just buy some now~"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Sure, I'll work for you!:I want to buy Fish Cake Soup.:See ya.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "Oh, thank you so much!",
                        "You can't imagine how busy",
                        "I've been, and how much I need the help! Now, let's get started~"
                    ],
                )?;
                ctx.next()?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if subject2 == 1 {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "Alright, please take this",
                            "order of Fish Cake Soup to",
                            "my friend Neha. I'm grateful",
                            "that she's a regular customer--",
                            "she's a good friend, but I'm sure that she loves the soup as well."
                        ],
                    )?;
                    ctx.var("hg_odeng").set(Val::from(1))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8064)])?;
                    ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 2 {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "Right, take this next order",
                            "of Fish Cake Soup to Maewan,",
                            "another one of my regulars.",
                            "He's been buying my soup ever",
                            "since I helped him start his business. Nice of him, isn't it?"
                        ],
                    )?;
                    ctx.var("hg_odeng").set(Val::from(2))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8065)])?;
                    ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 3 {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "Okay, this order of Fish",
                            "Cake Soup is ready! Please",
                            "take it to my roommate Layoma.",
                            "But... Be careful. She's got",
                            "a thing about punctuality.",
                            "Just try not to be late..."
                        ],
                    )?;
                    ctx.var("hg_odeng").set(Val::from(3))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8066)])?;
                    ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 4 {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "Quick, take this Fish",
                            "Paste Soup to Erjan. Um,",
                            "but be careful, he's really",
                            "nitpicky... And he likes to",
                            "have everything in such",
                            "and such and order..."
                        ],
                    )?;
                    ctx.var("hg_odeng").set(Val::from(4))?;
                    ctx.call(Function::SetQuest, vec![Val::from(8067)])?;
                    ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "Oh, you'd like to try my",
                        "Fish Cake Soup, eh? Good",
                        "choice, I know you'll enjoy",
                        "its delicious, hearty flavor~",
                        "Each order costs 100 zeny."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Order a Fish Cake Soup:Cancel")])? {
                    1 => {
                        if ctx.var("Zeny").get()?.number()? > 99 {
                            ctx.lines_as(
                                "Cellette",
                                args![
                                    "Thanks! Enjoy your",
                                    "bowl of Fish Cake Soup.",
                                    "I hope you come and ",
                                    "visit me again, okay?"
                                ],
                            )?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Cellette",
                                args![
                                    "Oh, what's this?",
                                    "You don't have enough",
                                    "money? Well, come back",
                                    "after you save 100 zeny.",
                                    "That's a really good price",
                                    "for my gourmet soup, you know."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "If you're ever hungry,",
                                "or just want to enjoy",
                                "a delicious meal, come",
                                "and have some of my soup~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "Oh? You're not interested",
                        "in trying my Fish Cake Soup",
                        "at all? Well, if you change",
                        "your mind, please come",
                        "back. I just know you'll love",
                        "the taste if you give it a try."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("hg_odeng").get()? == 1 {
        if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
            ctx.lines_as(
                "Cellette",
                args![
                    "Haven't you left already?",
                    "My friend Neha is waiting",
                    "for you to deliver her order",
                    "of Fish Cake Soup. Please",
                    "try to get it to her before the",
                    "soup gets cold, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Cellette",
                args![
                    "What's that? You lost",
                    "the Fish Cake Soup you're",
                    "supposed to deliver to Neha?",
                    "Oh well, I suppose I'll have",
                    "to give you this fresh bowl",
                    "of soup that I just made..."
                ],
            )?;
            ctx.next()?;
            if ctx.var("Zeny").get()?.number()? > 99 {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "But you need to be",
                        "responsible and pay",
                        "me for the food you lost.",
                        "There, I took 100 zeny",
                        "from you. That's fair, so",
                        "please don't lose it this time."
                    ],
                )?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
                ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "W-wait... You don't",
                        "have any money to pay",
                        "me for the soup you lost?",
                        "Well, I don't think I can",
                        "trust somebody like you",
                        "with any deliveries..."
                    ],
                )?;
                ctx.var("hg_odeng").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if ctx.var("hg_odeng").get()? == 2 {
            if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "Whoa, whoa, whoa!",
                        "You should have left",
                        "already to deliver that",
                        "soup to Maewan! Hurry,",
                        "before it gets too cold!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Cellette",
                    args![
                        "What's that? You lost",
                        "the Fish Cake Soup you're",
                        "supposed to give to Maewan?",
                        "Oh well, I suppose I'll have",
                        "to give you this fresh bowl",
                        "of soup that I just made..."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("Zeny").get()?.number()? > 99 {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "But you need to be",
                            "responsible and pay",
                            "me for the food you lost.",
                            "There, I took 100 zeny",
                            "from you. That's fair, so",
                            "please don't lose it this time."
                        ],
                    )?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "W-wait... You don't",
                            "have any money to pay",
                            "me for the soup you lost?",
                            "Well, I don't think I can",
                            "trust somebody like you",
                            "with any deliveries..."
                        ],
                    )?;
                    ctx.var("hg_odeng").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("hg_odeng").get()? == 3 {
                if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "I thought you left to",
                            "deliver that soup to",
                            "Layoma. You should hurry",
                            "and bring it to her, before",
                            "it doesn't taste good after",
                            "it gets cold, you know."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Cellette",
                        args![
                            "What's that? You lost",
                            "the Fish Cake Soup you're",
                            "supposed to give to Layoma?",
                            "Oh well, I suppose I'll have",
                            "to give you this fresh bowl",
                            "of soup that I just made..."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("Zeny").get()?.number()? > 99 {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "But you need to be",
                                "responsible and pay",
                                "me for the food you lost.",
                                "There, I took 100 zeny",
                                "from you. That's fair, so",
                                "please don't lose it this time."
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
                        ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "W-wait... You don't",
                                "have any money to pay",
                                "me for the soup you lost?",
                                "Well, I don't think I can",
                                "trust somebody like you",
                                "with any deliveries..."
                            ],
                        )?;
                        ctx.var("hg_odeng").set(Val::from(5))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else {
                if ctx.var("hg_odeng").get()? == 4 {
                    if ctx.call(Function::CountItem, vec![Val::from(584)])?.is_true() {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "Oh, dear, you better",
                                "leave now and deliver",
                                "that Fish Cake Soup to",
                                "Erjan before he can think",
                                "of something to complain about. Although it's probably too late..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "What's that? You lost",
                                "the Fish Cake Soup you're",
                                "supposed to deliver to Erjan?",
                                "Oh well, I suppose I'll have",
                                "to give you this fresh bowl",
                                "of soup that I just made..."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("Zeny").get()?.number()? > 99 {
                            ctx.lines_as(
                                "Cellette",
                                args![
                                    "But you need to be",
                                    "responsible and pay",
                                    "me for the food you lost.",
                                    "There, I took 100 zeny",
                                    "from you. That's fair, so",
                                    "please don't lose it this time."
                                ],
                            )?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(584), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Cellette",
                                args![
                                    "W-wait... You don't",
                                    "have any money to pay",
                                    "me for the soup you lost?",
                                    "Well, I don't think I can",
                                    "trust somebody like you",
                                    "with any deliveries..."
                                ],
                            )?;
                            ctx.var("hg_odeng").set(Val::from(5))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    if ctx.var("hg_odeng").get()? == 5 {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "Ah, I'm sorry, but",
                                "I can't do any business",
                                "with somebody that's proven",
                                "to be irresponsible. Nothing",
                                "personal, it's just my policy."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("hg_odeng").get()? == 10 {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "Good work! I hear that",
                                "you successfully delivered",
                                "the soup. I knew I could trust",
                                "you! Ah, before I forget, here",
                                "is your Fish Cake Soup. Thank",
                                "you so much for your help~"
                            ],
                        )?;
                        ctx.var("hg_odeng").set(Val::from(0))?;
                        l_i = Val::from(8072);
                        'l4: loop {
                            if !(l_i.clone().number()? <= 8075) {
                                break 'l4;
                            }
                            'b4: {
                                if ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? > -1 {
                                    ctx.call(Function::EraseQuest, vec![l_i.clone()])?;
                                }
                            }
                            l_i = (l_i.clone() + Val::from(1));
                        }
                        ctx.call(Function::GetItem, vec![Val::from(584), Val::from(3)])?;
                        ctx.call(Function::GetExperience, vec![Val::from(1000), Val::from(0)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "It was really nice working",
                                "with you, and I hope you come",
                                "by and help me again sometime.",
                                "Oh, and please tell all your",
                                "friends about my delicious",
                                "Fish Cake Soup. See you later~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Cellette",
                            args![
                                "Come and taste some",
                                "authentic Hugel cuisine!",
                                "Enjoy a big steaming bowl",
                                "of delicious Fish Cake Soup~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn cellette_lavit(ctx: &Ctx) -> Script {
    cellette_lavit_body(ctx, Vec::new()).map(|_| ())
}
