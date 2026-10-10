use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn praying_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(
            "Praying Man",
            args!["Let everyone live a life", "of happiness. Let there be", "peace in the world..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Praying Man",
            args![
                "May I hit it rich with this",
                "Old Blue Box that I spend this",
                "month's paycheck on..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as(
            "Praying Man",
            args!["Now is the time to reflect upon the past to prepare for the future."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Praying Man",
            args!["It is in stillness that we find the wisdom to know which steps we must take to move forward. Let's pray."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("god_brising").get()?.number()? > 34 {
        ctx.mes("^3355FFHermite Charles continues to fervently pray. His eyes are closed and his hands are clasped to his chest. There seems to be an air of faint sadness about him.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("god_brising").get()? == 34 {
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "What did you just say?",
                    "Did she really become an",
                    "Einherjar? That's the greatest honor for any warrior..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "She must",
                    "be so happy.",
                    "I guess in the end,",
                    "I never really had",
                    "any place in her heart.",
                    "Lowen..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "But I know one thing",
                    "for sure. She should have blamed me for what happened. Why did she take everything upon herself?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args!["I guess she just saw me as a little brother. She obviously didn't take me very seriously."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args!["Haha... Hahaha...", "Now I feel better!", "So, what did you say?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "Yes, right.",
                    "Please tell Enrico this.",
                    "The object he's interested in was given by His Majesty to a guild that possessed a castle."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hermite Charles", args!["It can never be made through just human technology. But I guess that His Majesty will let scholars study it and let men possess it if they prove capable."])?;
            ctx.next()?;
            ctx.lines_as("Hermite Charles", args!["I wasn't powerful enough to", "earn it. Frankly, I stole it. But soon I began to fear its presense and my heart would pound with trepidation when I saw it."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "One day, I think it actually possessed me and I ended up",
                    "going on a journey. I'm not really sure exactly where though..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args!["All I remember was that I was in Lutie, the town of snow. I went over a bridge to a hill to the West..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args!["But once I arrived there, I was seized with this feeling of terror I can't explain."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args![
                    "I ended up running away.",
                    "That's why I gave it to Mr. Enrico. You can laugh at me for being a coward if you want."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hermite Charles",
                args!["You know everything now.", "Please leave me alone.", "^333333*Sigh...*^000000"],
            )?;
            ctx.var("god_brising").set(Val::from(35))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("god_brising").get()?.number()? > 19 && ctx.var("god_brising").get()?.number()? < 34) {
                ctx.lines_as(
                    "Hermite Charles",
                    args![
                        "I'll be waiting.",
                        "For some reason, I believe you'll be able to find some sort of clue about what happened these past",
                        "two years one of these days."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_brising").get()? == 12 {
                    ctx.lines_as(
                        "Hermite Charles",
                        args![
                            "Lohen Phelicia?",
                            "The person I asked you to find is Lowen, not Lohen. Lowen Ellenen."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args!["I'm sorry", "for the trouble,", "but I think you", "better try again."],
                    )?;
                    ctx.var("god_brising").set(Val::from(10))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("god_brising").get()? == 11 {
                    ctx.lines_as("Hermite Charles", args!["Yes...", "Lowen is no", "longer with us."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args!["I've heard different rumors, but she definitely passed away. I was even at her funeral."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args![
                            "The last time I saw her alive",
                            "was in Geffen. Two years after that, I saw her buried in her grave. Didn't I tell you this?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hermite Charles", args!["I wanted you to find out what happened in those two years before she died. I wanted to know what led to her death."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args!["What the hell", "killed her?!", "Don't you understand", "the way I feel?"],
                    )?;
                    ctx.var("god_brising").set(Val::from(20))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("god_brising").get()? == 10 {
                    ctx.lines_as(
                        "Hermite Charles",
                        args!["Lowen was a Crusader, a knight in the service of holiness preparing for the Holy War."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args![
                            "I always expected her to",
                            "become a Knight in the Prontera Chivalry, but suddenly she changed her mind and became a Crusader.",
                            "I saw her for the last time in Geffen."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hermite Charles",
                        args![
                            "I beg you...",
                            "Please learn",
                            "anything you can",
                            "about what happened",
                            "to Lowen."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("god_brising").get()? == 5 {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 1 {
                        ctx.lines_as(
                            "Hermite Charles",
                            args!["You're still here.", "Does that mean you", "want to hear my", "story after all?"],
                        )?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes, I would like to.:No, thanks.")])?);
                            let mut matched1 = false;
                            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args![
                                        "Thank you...",
                                        "Perhaps you're doing this",
                                        "merely out of consideration,",
                                        "but I still appreciate your kindness. The story I will",
                                        "tell you is very old."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args![
                                        "Long ago, there was a little boy who never knew his parents. Just",
                                        "to survive, he became a Thief and eventually joined the Rogue Guild."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["Without any goals or dreams, his life was pretty aimless. He pretty much only lived so that he could see tomorrow."])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["However, the little boy met someone who was full of hope and kindess. He admired her and she became his reason for living."])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["She will never know that the little boy loved her more than anything else. Unlike the boy, her dream was devote her life to God."])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["Although he never understood that, he knew that he would be happy to just be near her, watching from a distance."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args!["But one day, because of one fatal mistake, he lost her and she never came back."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args![
                                        "Ever since, that boy has been coming to the Sanctuary everyday",
                                        "to pray for her safety, even into adulthood."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args!["...", "^333333*Sob*^000000", "Please find", "her for me.", "Find Lowen."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args!["I know that Enrico has sent you. I'm guessing he needs something from me. Am I wrong?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["I'll do whatever you want if you do what I want you to do. It's simple: find out anything you can about Lowen."])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["It's doesn't matter how insignificant the clues may be, anything will do. I want to know everything related to her!"])?;
                                ctx.next()?;
                                'b2: {
                                    let subject2 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Uhhhhh...:Sorry, I can't.:When was the last time you saw her?")],
                                    )?);
                                    let mut matched2 = false;
                                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                                        && !subject2.loosely_equals(&Val::from(2))
                                        && !subject2.loosely_equals(&Val::from(3));
                                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Please consider my",
                                                "proposal. If you can",
                                                "find Lowen for me, I'll",
                                                "give you what you want."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Please, I beg you.",
                                                "You are a strong and well-experienced adventurer. Isn't this a simple thing for you to do?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["If you've ever dearly loved someone, then you'd know how desperate I am. Please find Lowen for me. Please..."])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("...Alright.:I'm sorry, I don't think I can do it.")],
                                        )? {
                                            1 => {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["Alright.", "I'll try my best to find her. Would you tell me more about Lowen?"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "She is a Crusader.",
                                                        "The last time I saw her was",
                                                        "deep inside Geffen Dungeon."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["Long ago, some Crusaders entered the Geffen Dungeon on a monster subjugation expedition."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args!["Please find", "anything that", "you can. I beg", "of you..."],
                                                )?;
                                                ctx.var("god_brising").set(Val::from(10))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "^333333*Sigh*^000000",
                                                        "I understand.",
                                                        "But if you don't help me, then Kaili won't get the help he needs..."
                                                    ],
                                                )?;
                                                ctx.var("god_brising").set(Val::from(9))?;
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
                                        ctx.lines_as("Hermite Charles", args!["You don't understand!", "If I weren't this much of a coward, I would already have gone out to find out what I could for myself."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["I'm not afraid of monsters or dying. It's the fact that she might hate me now. That's what I fear: Lowen's reproach."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["All I can do for her is just earnestly pray. I had no idea it'd be so horrible not to be able to see her anymore..."])?;
                                        ctx.var("god_brising").set(Val::from(9))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["The last time", "I saw her was deep", "inside the Geffen Dungeon."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["Although it was created by humans, that dungeon is now cursed and inhabited with horrific monsters."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["I'm guessing the Crusaders saw that it was necessary to exterminate the monsters there to keep it from getting even worse."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Lowen accompanied",
                                                "a group of other Crusaders",
                                                "for the good of the people,",
                                                "but I haven't heard anything",
                                                "about her since..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["That was the last time...", "Please find out anything", "you can. I beg you..."],
                                        )?;
                                        ctx.var("god_brising").set(Val::from(10))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args![
                                        "I misunderstood you.",
                                        "I should have realized that people never really listen unless it's of some benefit to them."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args![
                                        "I won't waste any more of",
                                        "your time. I'll accept Kaili's letter and wait until I find someone who'll listen to",
                                        "my story and help me."
                                    ],
                                )?;
                                ctx.var("god_brising").set(Val::from(5))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    } else {
                        ctx.lines_as(
                            "Hermite Charles",
                            args![
                                "...",
                                "^333333*Sob...*^000000",
                                "I'm sorry.",
                                "I'm so sorry, Lowen.",
                                "It was all my fault..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("god_brising").get()? == 4 {
                        ctx.lines_as(
                            "Hermite Charles",
                            args!["What do you", "want from me?", "Were you sent by", "the Rogue Guild?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Hermite Charles", args!["Tell them", "I quit already.", "Leave me alone!"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Leave him alone.:Give him Kaili's Letter.")])? {
                            1 => {
                                ctx.mes("^3355FFYou're not sure why he's so upset, but it doesn't seem to be the best time to try to speak with him. Perhaps later would be better...^000000")?;
                                ctx.var("god_brising").set(Val::from(4))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args!["Huh...", "Enrico Kaili.", "Yeah, I remember", "him. So what..?"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFHermite nonchalantly", "tosses the letter back to you.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["...", "^333333*Sigh...*^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["You...", "Would you like to", "listen to my story?"])?;
                                ctx.next()?;
                                'b5: {
                                    let subject5 = Val::from(runtime::select_values(ctx, &[Val::from("Sure!:I'm busy, actually.")])?);
                                    let mut matched5 = false;
                                    let no_case5 = !subject5.loosely_equals(&Val::from(1)) && !subject5.loosely_equals(&Val::from(2));
                                    if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Thank you...",
                                                "Perhaps you're doing this",
                                                "merely out of consideration,",
                                                "but I still appreciate your kindness. The story I will",
                                                "tell you is very old."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Long ago, there was a little boy who never knew his parents. Just",
                                                "to survive, he became a Thief and eventually joined the Rogue Guild."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["Without any goals or dreams, his life was pretty aimless. He pretty much only lived so that he could see tomorrow."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["However, the little boy met someone who was full of hope and kindess. He admired her and she became his reason for living."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["She will never know that the little boy loved her more than anything else. Unlike the boy, her dream was devote her life to God."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["Although he never understood that, he knew that he would be happy to just be near her, watching from a distance."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["But one day, because of one fatal mistake, he lost her and she never came back."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Ever since, that boy has been coming to the Sanctuary everyday",
                                                "to pray for her safety, even into adulthood."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["...", "^333333*Sob*^000000", "Please find", "her for me.", "Find Lowen."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["I know that Enrico has sent you. I'm guessing he needs something from me. Am I wrong?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["I'll do whatever you want if you do what I want you to do. It's simple: find out anything you can about Lowen."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["It's doesn't matter how insignificant the clues may be, anything will do. I want to know everything related to her!"])?;
                                        ctx.next()?;
                                        'b6: {
                                            let subject6 = Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("Uhhhhh...:Sorry, I can't.:When was the last time you saw her?")],
                                            )?);
                                            let mut matched6 = false;
                                            let no_case6 = !subject6.loosely_equals(&Val::from(1))
                                                && !subject6.loosely_equals(&Val::from(2))
                                                && !subject6.loosely_equals(&Val::from(3));
                                            if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                                matched6 = true;
                                            }
                                            if matched6 {
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "Please consider my",
                                                        "proposal. If you can",
                                                        "find Lowen for me, I'll",
                                                        "give you what you want."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["Please, I beg you.", "You are a strong and well-experienced adventurer. Isn't this a simple thing for you to do?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["If you've ever dearly loved someone, then you'd know how desperate I am. Please find Lowen for me. Please..."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("...Alright.:I'm sorry, I don't think I can do it.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Alright.",
                                                                "I'll try my best to find her. Would you tell me more about Lowen?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args![
                                                                "She is a Crusader.",
                                                                "The last time I saw her was",
                                                                "deep inside Geffen Dungeon."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hermite Charles", args!["Long ago, some Crusaders entered the Geffen Dungeon on a monster subjugation expedition."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args!["Please find", "anything that", "you can. I beg", "of you..."],
                                                        )?;
                                                        ctx.var("god_brising").set(Val::from(10))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args![
                                                                "^333333*Sigh*^000000",
                                                                "I understand.",
                                                                "But if you don't help me, then Kaili won't get the help he needs..."
                                                            ],
                                                        )?;
                                                        ctx.var("god_brising").set(Val::from(9))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                                matched6 = true;
                                            }
                                            if matched6 {
                                                ctx.lines_as("Hermite Charles", args!["You don't understand!", "If I weren't this much of a coward, I would already have gone out to find out what I could for myself."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["I'm not afraid of monsters or dying. It's the fact that she might hate me now. That's what I fear: Lowen's reproach."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["All I can do for her is just earnestly pray. I had no idea it'd be so horrible not to be able to see her anymore..."])?;
                                                ctx.var("god_brising").set(Val::from(9))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            if !matched6 && subject6.loosely_equals(&Val::from(3)) {
                                                matched6 = true;
                                            }
                                            if matched6 {
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args!["The last time", "I saw her was deep", "inside the Geffen Dungeon."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["Although it was created by humans, that dungeon is now cursed and inhabited with horrific monsters."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["I'm guessing the Crusaders saw that it was necessary to exterminate the monsters there to keep it from getting even worse."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "Lowen accompanied",
                                                        "a group of other Crusaders",
                                                        "for the good of the people,",
                                                        "but I haven't heard anything",
                                                        "about her since..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args!["That was the last time...", "Please find out anything", "you can. I beg you..."],
                                                )?;
                                                ctx.var("god_brising").set(Val::from(10))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    }
                                    if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as("Hermite Charles", args!["I misunderstood you.", "I should have realized that people never really listen unless it's of some benefit to them."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "I won't waste any more of",
                                                "your time. I'll accept Kaili's letter and wait until I find someone who'll listen to",
                                                "my story and help me."
                                            ],
                                        )?;
                                        ctx.var("god_brising").set(Val::from(5))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if ctx.var("god_brising").get()? == 3 {
                        ctx.lines_as(
                            "Sad-looking Man",
                            args!["...", "^333333*Sob...*^000000", "I am so", "sorry, Lowen.", "It's all fault..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Excuse me.", "I am looking", "for someone named...", "Hermite?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hermite Charles",
                            args!["Hm...?", "Hermite? That's me.", "But if you don't mind,", "I want to be alone..."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Leave him alone.:Give him Kaili's Letter.")])? {
                            1 => {
                                ctx.mes("^3355FFYou're not sure why he's so upset, but it doesn't seem to be the best time to try to speak with him. Perhaps later would be better...^000000")?;
                                ctx.var("god_brising").set(Val::from(4))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Hermite Charles",
                                    args!["Huh...", "Enrico Kaili.", "Yeah, I remember", "him. So what..?"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFHermite nonchalantly", "tosses the letter back to you.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["...", "^333333*Sigh...*^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Hermite Charles", args!["You...", "Would you like to", "listen to my story?"])?;
                                ctx.next()?;
                                'b9: {
                                    let subject9 = Val::from(runtime::select_values(ctx, &[Val::from("Sure!:I'm busy, actually.")])?);
                                    let mut matched9 = false;
                                    let no_case9 = !subject9.loosely_equals(&Val::from(1)) && !subject9.loosely_equals(&Val::from(2));
                                    if !matched9 && subject9.loosely_equals(&Val::from(1)) {
                                        matched9 = true;
                                    }
                                    if matched9 {
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Thank you...",
                                                "Perhaps you're doing this",
                                                "merely out of consideration,",
                                                "but I still appreciate your kindness. The story I will",
                                                "tell you is very old."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Long ago, there was a little boy who never knew his parents. Just",
                                                "to survive, he became a Thief and eventually joined the Rogue Guild."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["Without any goals or dreams, his life was pretty aimless. He pretty much only lived so that he could see tomorrow."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["However, the little boy met someone who was full of hope and kindess. He admired her and she became his reason for living."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["She will never know that the little boy loved her more than anything else. Unlike the boy, her dream was devote her life to God."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["Although he never understood that, he knew that he would be happy to just be near her, watching from a distance."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["But one day, because of one fatal mistake, he lost her and she never came back."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "Ever since, that boy has been coming to the Sanctuary everyday",
                                                "to pray for her safety, even into adulthood."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["...", "^333333*Sob*^000000", "Please find", "her for me.", "Find Lowen."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args!["I know that Enrico has sent you. I'm guessing he needs something from me. Am I wrong?"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["I'll do whatever you want if you do what I want you to do. It's simple: find out anything you can about Lowen."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Hermite Charles", args!["It's doesn't matter how insignificant the clues may be, anything will do. I want to know everything related to her!"])?;
                                        ctx.next()?;
                                        'b10: {
                                            let subject10 = Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("Uhhhhh...:Sorry, I can't.:When was the last time you saw her?")],
                                            )?);
                                            let mut matched10 = false;
                                            let no_case10 = !subject10.loosely_equals(&Val::from(1))
                                                && !subject10.loosely_equals(&Val::from(2))
                                                && !subject10.loosely_equals(&Val::from(3));
                                            if !matched10 && subject10.loosely_equals(&Val::from(1)) {
                                                matched10 = true;
                                            }
                                            if matched10 {
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "Please consider my",
                                                        "proposal. If you can",
                                                        "find Lowen for me, I'll",
                                                        "give you what you want."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["Please, I beg you.", "You are a strong and well-experienced adventurer. Isn't this a simple thing for you to do?"])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["If you've ever dearly loved someone, then you'd know how desperate I am. Please find Lowen for me. Please..."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("...Alright.:I'm sorry, I don't think I can do it.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args![
                                                                "Alright.",
                                                                "I'll try my best to find her. Would you tell me more about Lowen?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args![
                                                                "She is a Crusader.",
                                                                "The last time I saw her was",
                                                                "deep inside Geffen Dungeon."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Hermite Charles", args!["Long ago, some Crusaders entered the Geffen Dungeon on a monster subjugation expedition."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args!["Please find", "anything that", "you can. I beg", "of you..."],
                                                        )?;
                                                        ctx.var("god_brising").set(Val::from(10))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Hermite Charles",
                                                            args![
                                                                "^333333*Sigh*^000000",
                                                                "I understand.",
                                                                "But if you don't help me, then Kaili won't get the help he needs..."
                                                            ],
                                                        )?;
                                                        ctx.var("god_brising").set(Val::from(9))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            if !matched10 && subject10.loosely_equals(&Val::from(2)) {
                                                matched10 = true;
                                            }
                                            if matched10 {
                                                ctx.lines_as("Hermite Charles", args!["You don't understand!", "If I weren't this much of a coward, I would already have gone out to find out what I could for myself."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["I'm not afraid of monsters or dying. It's the fact that she might hate me now. That's what I fear: Lowen's reproach."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["All I can do for her is just earnestly pray. I had no idea it'd be so horrible not to be able to see her anymore..."])?;
                                                ctx.var("god_brising").set(Val::from(9))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                                                matched10 = true;
                                            }
                                            if matched10 {
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args!["The last time", "I saw her was deep", "inside the Geffen Dungeon."],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["Although it was created by humans, that dungeon is now cursed and inhabited with horrific monsters."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Hermite Charles", args!["I'm guessing the Crusaders saw that it was necessary to exterminate the monsters there to keep it from getting even worse."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args![
                                                        "Lowen accompanied",
                                                        "a group of other Crusaders",
                                                        "for the good of the people,",
                                                        "but I haven't heard anything",
                                                        "about her since..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Hermite Charles",
                                                    args!["That was the last time...", "Please find out anything", "you can. I beg you..."],
                                                )?;
                                                ctx.var("god_brising").set(Val::from(10))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    }
                                    if !matched9 && subject9.loosely_equals(&Val::from(2)) {
                                        matched9 = true;
                                    }
                                    if matched9 {
                                        ctx.lines_as("Hermite Charles", args!["I misunderstood you.", "I should have realized that people never really listen unless it's of some benefit to them."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Hermite Charles",
                                            args![
                                                "I won't waste any more of",
                                                "your time. I'll accept Kaili's letter and wait until I find someone who'll listen to",
                                                "my story and help me."
                                            ],
                                        )?;
                                        ctx.var("god_brising").set(Val::from(5))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Sad-looking Man",
                            args![
                                "...",
                                "^333333*Sob...*",
                                "Lowen...",
                                "I'm so sorry.",
                                "It's all my fault.",
                                "Please be okay..."
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

pub fn praying_man(ctx: &Ctx) -> Script {
    praying_man_body(ctx, Vec::new()).map(|_| ())
}

fn librarian_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as("Librarian", args!["What are you doing here?", "Don't touch anything!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as("Librarian", args!["What are you doing here?", "Don't touch anything!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("god_brising").get()?.number()? > 24 && ctx.var("god_brising").get()?.number()? < 27) {
        ctx.lines_as("Librarian", args!["Zzzzz...", "...Zzzzz...", "......Zzzzzz..."])?;
        ctx.next()?;
        ctx.mes("^3355FFThis librarian seems to be deeply asleep. It'd be smarter not to wake him if you want to check the Crusader Personnel Records.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("god_brising").get()? == 10 {
        ctx.lines_as(
            "Librarian",
            args![
                "All confidential personnel records are kept here in the royal library. However, you need authorization",
                "for full access."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Librarian",
            args![
                "Did you say that you",
                "need to find a person?",
                "Please give me that",
                "person's name, as well",
                "as your relationship."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Lowen, Sibling:Lowen, Spouse:Lowen, Enemy:Lowen, a Friend")])? {
            1 => {
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines_as("Librarian", args!["How dare you", "lie to the royal", "librarian!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args!["It says here", "in the records", "that Lowen did not", "have any male siblings!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args![
                            "^333333(Crap...!",
                            "I just disclosed",
                            "^666666classified^000000 ^333333info!)^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args!["The Royal Library won't", "tolerate identity fraud!", "Please leave!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Librarian",
                        args!["You're related to Miss Lowen? How have you not received any news about her before?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args!["I see...", "Separated at birth,", "that's truly tragic.", "Okay, let me check."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args!["I feel terrible telling you this, but it's too late to find Lowen Ellenen, according to the records."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Librarian", args!["During the monster subjugation mission in Geffen dungeon she reportedly disappeared and... was later pronounced dead."])?;
                    ctx.next()?;
                    ctx.lines_as("Librarian", args!["I'm sorry, but", "that's all I know.", "Thank you..."])?;
                    ctx.var("god_brising").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines_as(
                        "Librarian",
                        args![
                            "Spouse...?",
                            "Well, I guess you look like a husband. Heh, I've got a girlfriend myself. Well, at least I think so. Anyway..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args![
                            "Lowen Ellenen",
                            "the Crusader, right?",
                            "I'm so... sorry.",
                            "You might want",
                            "to have a seat."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args![
                            "During the monster",
                            "subjugation in Geffen",
                            "Dungeon, she reportedly",
                            "disappeared. And later,",
                            "she was pronounced dead."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Librarian",
                        args!["That's all the", "information I have,", "sir. I'm truly sorry", "for your loss."],
                    )?;
                    ctx.var("god_brising").set(Val::from(11))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Librarian",
                        args!["Hmmm...?", "Spouse?", "Or ^333333*Ahem*^000000", "cohabitational partners?"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe librarian",
                        "now seems awfully",
                        "distracted, as if",
                        "he were lost in",
                        "vivid daydream...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I'm looking", "for Ellenen.", "Lowen Ellenen.", "My sworn arch-enemy."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Librarian",
                    args![
                        "Arch-enemy...?",
                        "Um, uh ^666666*Ahem!*^000000",
                        "All I can tell you is that there was a Crusader by that name."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Librarian", args!["I'm sorry, but", "I can't tell you", "more than that."])?;
                ctx.next()?;
                ctx.lines_as("Librarian", args!["^333333*Cough*^000000", "^666666Nutcase!^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines_as(
                    "Librarian",
                    args!["A friend...?", "Well, we have a", "record of someone", "named Lohen Phelica."],
                )?;
                ctx.next()?;
                ctx.lines_as("Librarian", args!["He retired from the service a few years ago. I'd like to give you his address, but we don't have any of that information."])?;
                ctx.var("god_brising").set(Val::from(12))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("god_brising").get()?.number()? > 10 && ctx.var("god_brising").get()?.number()? < 13) {
        ctx.lines_as("Librarian", args!["Now, may", "I excuse myself?", "Thank you!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Librarian",
            args![
                "All confidential personnel records are kept here in the royal library. However, you need authorization",
                "for full access."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn librarian_2(ctx: &Ctx) -> Script {
    librarian_2_body(ctx, Vec::new()).map(|_| ())
}

fn woman_rosa_ellenen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(
            "Woman",
            args![
                "Hm?",
                "What brings",
                "you here? It's",
                "kind of strange",
                "to come just to",
                "take a walk."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as(
            "Woman",
            args![
                "Hm?",
                "What brings",
                "you here? It's",
                "kind of strange",
                "to come just to",
                "take a walk."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("god_brising").get()?.number()? > 33 {
        ctx.lines_as(
            "Rosa Ellenen",
            args!["Are you the one", "who visited me before?", "Ah, of course."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "I'm not sure how",
                "to tell you this, but...",
                "This is Lowen's grave.",
                "And thank you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("god_brising").get()? == 21 {
        ctx.lines_as("Rosa Ellenen", args!["What are you", "talking about?", "A message from Lowen?"])?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["What?!", "Why hasn't she", "gone to heaven yet?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "I hope you remember",
                "everything I'm going",
                "to tell you. She should",
                "go to heaven right now!",
                "I still don't understand..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["And please tell that praying fool that he shouldn't waste his time blaming himself. ^333333*Sigh*^000000 I still don't know why she chose to", "speak to you."])?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["Let me tell you something. Lowen was a fencing prodigy. However, she always had trouble enduring the rigorous training and bearing the weight of the armor..."])?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["But she loved fencing and always tried twice as hard to develop her physical strength so that she could serve God in battle."])?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "Finally, she became a Crusader.",
                "For a while things were great, but then she suddenly disappeared while on a mission to Geffen Dungeon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["Two years later, she was buried in this grave after her body was found by some adventurers. I was given the broken shards of armor and a blood stained ribbon she left behind."])?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["I never really learned what exactly happened. And Hermite was always following her like she was his real sister. Ever since she died, he's changed. I hope he doesn't hurt himself..."])?;
        ctx.next()?;
        ctx.lines_as("Rosa Ellenen", args!["Something suspicious seems to have happened in the military, but they won't even talk to me about it. What could have happened?!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args!["In the end, I gave up trying to find the truth. After all, nothing can bring her back..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "Please tell her I'm fine,",
                "but I want to know why her",
                "soul hasn't moved on to",
                "heaven! Maybe..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rose Ellenen",
            args![
                "Maybe there's",
                "something in the",
                "Crusader Personnel",
                "Records that might",
                "explain something?"
            ],
        )?;
        ctx.var("god_brising").set(Val::from(25))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("god_brising").get()?.number()? > 9 && ctx.var("god_brising").get()?.number()? < 20) {
        ctx.lines_as(
            "Rosa Ellenen",
            args!["This is my little", "sister's grave...", "Her name was Lowen."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "Are you a friend",
                "of Lowen's? Hmmm?",
                "The man in the Sanctuary?",
                "That must be Hermite."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "He was shocked when Lowen",
                "died but he doesn't have to blame himself for her death. I don't want him to suffer from the guilt."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rosa Ellenen",
            args![
                "Anyway, thank you for visiting",
                "my little sister. It's good to know she has friends, even after she's passed on."
            ],
        )?;
        ctx.var("god_brising").set(Val::from(11))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Rosa Ellenen",
            args!["Oh hello...", "Have you come", "to pay your respects", "to someone here as well?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn woman_rosa_ellenen(ctx: &Ctx) -> Script {
    woman_rosa_ellenen_body(ctx, Vec::new()).map(|_| ())
}

fn gravestone_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$god2").get()?, ">", &ctx.var("$@god_check1").get()?)?.is_true() {
        if ctx.var("god_brising").get()?.number()? > 33 {
            ctx.lines(args![
                "Lowen Ellenen",
                " ",
                " XXXX. XX. XX.",
                "Her noble spirit",
                "was sent to the holy",
                "place by Valkyrie's will."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("^3355FFYou couldn't read the rest of the epitaph since the tombstone has been eroded with time.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args!["^3355FFIt's just an", "ordinary gravestone.^000000"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFNothing really",
            "sets it apart from the rest of the tombstones here in the Prontera cemetary.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn gravestone(ctx: &Ctx) -> Script {
    gravestone_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LowentraceStep {
    Start,
    OnTouch,
}

fn lowentrace_run(ctx: &Ctx, mut step: LowentraceStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LowentraceStep::Start => {
                if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an old,", "dry piece of wood.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an old,", "dry piece of wood.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("god_brising").get()?.number()? > 20 && ctx.var("god_brising").get()?.number()? < 34) {
                    ctx.lines(args!["^3355FFWill you", "summon her?^000000"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                        1 => {
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            ctx.var("@lowenstring$").set(input)?;
                            if ctx.var("@lowenstring$").get()? == "Lowen" {
                                if ctx.var("god_brising").get()? == 30 {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args![
                                            "^6E7B8B...Y-you're...",
                                            "I-I can't stay long,",
                                            "I'm feeling too weak.",
                                            "Come closer, I want",
                                            "to show you something...^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFHer voice grew",
                                        "softer and softer",
                                        "until you could no",
                                        "longer hear her.^000000"
                                    ])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("god_brising").get()? == 21 {
                                    ctx.lines_as("Lowen Ellenen", args!["^6E7B8BPlease tell Rosa", "not to worry about me anymore. You might be able to find her beside my grave. I don't want her to suffer anymore...^000000"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("god_brising").get()? == 25 {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args![
                                            "^6E7B8BYou've come back.",
                                            "Have you met Rosa?",
                                            "Hm, yes, it sounds like she'd say that. But thank you for sending her my message.^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Lowen Ellenen", args!["^6E7B8BAh, it's too dangero--", "This place is too dangerous for you. I hate myself for not being able to help protect you from", "these evil creatures. Be careful...^000000"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args![
                                            "^6E7B8BMy spirit is",
                                            "growing weaker...",
                                            "Please be careful...",
                                            "This place is too dangerous.^000000"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("god_brising").get()? == 20 {
                    ctx.lines_as(
                        "The voice of a female",
                        args!["^6E7B8BI-It's dangerous...", "Be careful...^000000"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore it.:What are you talking about?")])? {
                        1 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "The voice of a female",
                                args![
                                    "^6E7B8B...No way...",
                                    "You can hear me?",
                                    "I can't believe this.",
                                    "It's impossible...",
                                    "But if you don't mind,",
                                    "may I talk to you...?^000000"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Holy...! Run!:What are you?!")])? {
                                1 => {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("The voice of a female", args!["^6E7B8BI...", "I don't have my body anymore, so... I think I'm a ghost. Yes, I've been wandering in this place ever since I got here.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BI'm still not sure why I'm",
                                            "bound to this realm. If you",
                                            "don't mind, would you visit",
                                            "my sister for me?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BPlease tell her...",
                                            "Please tell her that her",
                                            "little sister Lowen is fine.^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BUm...",
                                            "If you want to meet me again, just say my name out loud. '^000000Lowen^6E7B8B.'^000000"
                                        ],
                                    )?;
                                    ctx.var("god_brising").set(Val::from(21))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("^3355FFYou find a piece of twisted, dry wood. Looking at it seems to bring out a feeling of sadness within you for some reason.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = LowentraceStep::OnTouch;
                continue 'machine;
            }
            LowentraceStep::OnTouch => {
                if (ctx.var("god_brising").get()?.number()? > 9 && ctx.var("god_brising").get()?.number()? < 34) {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn lowentrace(ctx: &Ctx) -> Script {
    lowentrace_run(ctx, LowentraceStep::Start, Vec::new()).map(|_| ())
}

pub fn lowentrace_ontouch(ctx: &Ctx) -> Script {
    lowentrace_run(ctx, LowentraceStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Lowentrace1Step {
    Start,
    OnTouch,
}

fn lowentrace1_run(ctx: &Ctx, mut step: Lowentrace1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Lowentrace1Step::Start => {
                if runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an old,", "dry piece of wood.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("$god3").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                    ctx.lines(args!["^3355FFIt's just an old,", "dry piece of wood.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("god_brising").get()?.number()? > 20 && ctx.var("god_brising").get()?.number()? < 34) {
                    ctx.lines(args!["Would you", "like to summon her? "])?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            ctx.var("@lowenstring$").set(input)?;
                            if ctx.var("@lowenstring$").get()? == "Lowen" {
                                if ctx.var("god_brising").get()? == 31 {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args!["^6E7B8BYou came back!", "You'll be with me", "...Won't you?^000000"],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Of course.:Sorry, I don't think I can...")])? {
                                        1 => {
                                            ctx.lines_as("Lowen Ellenen", args!["^6E7B8BThank you,", "thank you so much...^000000"])?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(47), Val::from(53)])?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Lowen Ellenen",
                                                args!["^6E7B8BI understand.", "Please take", "care of yourself...^000000"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if ctx.var("god_brising").get()? == 30 {
                                    ctx.lines_as("Lowen Ellenen", args!["^6E7B8BAh yes. This is it.", "You look very curious about me, yet I am amazed that you can hear my voice. Please let me tell you an old story.^000000"])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(47), Val::from(53)])?;
                                    return Err(Stop::End);
                                } else if ctx.var("god_brising").get()? == 25 {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args![
                                            "^6E7B8BYou've come back.",
                                            "Have you met Rosa?",
                                            "Hm, yes, it sounds like she'd say that. But thank you for sending her my message.^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Lowen Ellenen", args!["^6E7B8BAh, it's too dangero--", "This place is too dangerous for you. I hate myself for not being able to help protect you from", "these evil creatures. Be careful...^000000"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Lowen Ellenen",
                                        args![
                                            "^6E7B8BMy spirit is",
                                            "growing weaker...",
                                            "Please be careful...",
                                            "This place is too dangerous.^000000"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else if ctx.var("god_brising").get()? == 20 {
                    ctx.lines_as(
                        "The voice of a female",
                        args!["^6E7B8BI-It's dangerous...", "Be careful...^000000"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore it.:What are you talking about?")])? {
                        1 => {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "The voice of a female",
                                args![
                                    "^6E7B8B...No way...",
                                    "You can hear me?",
                                    "I can't believe this.",
                                    "It's impossible...",
                                    "But if you don't mind,",
                                    "may I talk to you...?^000000"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Holy...! Run!:What are you?!")])? {
                                1 => {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("The voice of a female", args!["^6E7B8BI...", "I don't have my body anymore, so... I think I'm a ghost. Yes, I've been wandering in this place ever since I got here.^000000"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BI'm still not sure why I'm",
                                            "bound to this realm. If you",
                                            "don't mind, would you visit",
                                            "my sister for me?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BPlease tell her...",
                                            "Please tell her that her",
                                            "little sister Lowen is fine.^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The voice of a female",
                                        args![
                                            "^6E7B8BUm...",
                                            "If you want to meet me again, just say my name out loud. '^000000Lowen^6E7B8B.'^000000"
                                        ],
                                    )?;
                                    ctx.var("god_brising").set(Val::from(21))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("^3355FFYou find a piece of twisted, dry wood. Looking at it seems to bring out a feeling of sadness within you for some reason.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Lowentrace1Step::OnTouch;
                continue 'machine;
            }
            Lowentrace1Step::OnTouch => {
                if (ctx.var("god_brising").get()?.number()? > 9 && ctx.var("god_brising").get()?.number()? < 34) {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn lowentrace1(ctx: &Ctx) -> Script {
    lowentrace1_run(ctx, Lowentrace1Step::Start, Vec::new()).map(|_| ())
}

pub fn lowentrace1_ontouch(ctx: &Ctx) -> Script {
    lowentrace1_run(ctx, Lowentrace1Step::OnTouch, Vec::new()).map(|_| ())
}

fn lowen_ellenen_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_brising").get()? == 30 {
        ctx.lines_as(
            "Lowen Ellenen",
            args![
                "Can you see me now?",
                "You must be surprised.",
                "This is the story I will",
                "share with you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lowen Ellenen",
            args![
                "Look around.",
                "Recognize it?",
                "We're still in the same place, except now we're in my past."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Lowen Ellenen", args!["All of us came here to Geffen to cleanse this area of evil spirits. You asked for the reason I remained in this awful place?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lowen Ellenen",
            args![
                "I couldn't forgive myself for what I've done to my comrades. They",
                "were killed because of me."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("You were cursed...?:You still have something to do.")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["You're saying...", "The anger of your", "comrades manifested", "into a curse?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["Cursed by my fallen comrades... Yes, that's what I thought at first. So I tried to redeem myself as well as avenge their deaths."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args!["I have been trying my best to comfort their spirits for 2 years."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Ever since you guys failed on a mission, you've spent two years here killing monsters?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args![
                        "Yes, that's right.",
                        "I thought it would bring comfort to the spirits of my comrades."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args![
                        "But once I died, I met all of my old friends in Niflheim, the town of the dead. You've heard of it, haven't you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args![
                        "No one seemed to",
                        "blame me for what",
                        "had happened. In fact, they all seemed worried about me."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Then why?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Lowen Ellenen", args!["They waited for me in Niflheim for two years, since they didn't want me to misunderstand them. They never blamed me for what happened."])?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["But I still feel like something is wrong. Out of all of us, I'm the only one unable to rest in peace. Everyone else was selected to go to Valhalla."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lowen Ellenen",
                    args!["But Valhalla is where all honorable Crusaders are supposed to go. What have I done wrong?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Lowen Ellenen",
                    args!["I don't think I have any unfinished business. But who knows, maybe there is something I have to do."],
                )?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["All of my late comrades that I met in Niflheim were selected by Valkyrie and sent to Valhalla. That's how honorable Crusaders spend the afterlife. But it looks like I don't deserve it..."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("I can help you!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Lowen Ellenen", args!["You can", "help me?", "How...?"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["You said this is the place where the subjugation mission happened, right? I'll fight with you!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Now come on, you don't deserve to feel guilty like this. Let's think about how we're gonna fight these monsters!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Lowen Ellenen", args!["...", "Thank you.", "Thank you so much."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Now,", "shall we go in?"],
                )?;
                ctx.var("god_brising").set(Val::from(31))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("god_brising").get()? == 31 {
        ctx.lines_as(
            "Lowen Ellenen",
            args![
                "Walk down the stairs",
                "ahead. From this point",
                "I can only travel with",
                "you in your mind..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Lowen Ellenen", args!["How come...", "You're here?"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(101)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lowen_ellenen(ctx: &Ctx) -> Script {
    lowen_ellenen_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Soldier1BrisingStep {
    Start,
    OnTouch,
}

fn soldier_1_brising_run(ctx: &Ctx, mut step: Soldier1BrisingStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Soldier1BrisingStep::Start => {
                if ctx.var("god_brising").get()? == 31 {
                    ctx.lines_as(
                        "Soldier",
                        args!["Have you volunteered for monster subjugation? Would you like to start the mission now?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes.:I need more time.")])? {
                        1 => {
                            ctx.lines_as("Soldier", args!["We're you're ready,", "come and stand in", "front of me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Soldier",
                                args!["No problem, just come back when your preparations are completed."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as("Soldier", args!["Please back away.", "You are not permitted", "to enter!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Soldier1BrisingStep::OnTouch;
                continue 'machine;
            }
            Soldier1BrisingStep::OnTouch => {
                if ctx.var("god_brising").get()? == 31 {
                    ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(174), Val::from(49)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsold::OnSold1Off")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsold2::OnSold2On")])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Soldier", args!["Please back away.", "You are not permitted", "to enter!"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_god02"), Val::from(51), Val::from(59)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn soldier_1_brising(ctx: &Ctx) -> Script {
    soldier_1_brising_run(ctx, Soldier1BrisingStep::Start, Vec::new()).map(|_| ())
}

pub fn soldier_1_brising_ontouch(ctx: &Ctx) -> Script {
    soldier_1_brising_run(ctx, Soldier1BrisingStep::OnTouch, Vec::new()).map(|_| ())
}

fn soldier_2_brising_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Soldier", args!["Please back away.", "You are not permitted", "to enter!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_2_brising(ctx: &Ctx) -> Script {
    soldier_2_brising_body(ctx, Vec::new()).map(|_| ())
}

pub fn brisinsold2(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::Start, Vec::new()).map(|_| ())
}

pub fn brisinsold2_oninit(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn brisinsold2_onsold2on(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnSold2On, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer420000(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer480000(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer480000, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer540000(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer540000, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer542000(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer542000, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer550000(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer550000, Vec::new()).map(|_| ())
}

pub fn brisinsold2_ontimer550500(ctx: &Ctx) -> Script {
    brisinsold2_run(ctx, Brisinsold2Step::OnTimer550500, Vec::new()).map(|_| ())
}

pub fn brisinsummon(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::Start, Vec::new()).map(|_| ())
}

pub fn brisinsummon_oninit(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnInit, Vec::new()).map(|_| ())
}

pub fn brisinsummon_ondoppel1on(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnDoppel1On, Vec::new()).map(|_| ())
}

pub fn brisinsummon_ondoppel1off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnDoppel1Off, Vec::new()).map(|_| ())
}

pub fn brisinsummon_ondoppel2on(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnDoppel2On, Vec::new()).map(|_| ())
}

pub fn brisinsummon_ondoppel2off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnDoppel2Off, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight1on(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight1On, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight2on(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight2On, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight3on(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight3On, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight1off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight1Off, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight2off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight2Off, Vec::new()).map(|_| ())
}

pub fn brisinsummon_onknight3off(ctx: &Ctx) -> Script {
    brisinsummon_run(ctx, BrisinsummonStep::OnKnight3Off, Vec::new()).map(|_| ())
}
