use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn klitzer_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("ein_loverq").get()? == 16 {
        ctx.lines_as(
            "Klitzer",
            args![
                "Look out, world!",
                "I'm gonna become",
                "worthy of Calla's love!",
                "Someday, maybe even her",
                "parents will approve me!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 15 && ctx.call(Function::CountItem, vec![Val::from(7170)])?.number()? > 0) {
        ctx.lines_as(
            "Klitzer",
            args![
                "Waaaah!",
                "What should I wear?!",
                "I can't for the life of me",
                "figure this out! Something,",
                "um, formal? I've never worn",
                "anything like that before!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                ((Val::from("Wait, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "What's that you've got",
                "there? I've seen something",
                "like that before. It's called",
                "a Tuxedo, right? Something",
                "like that would be perfect!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Give it to him.:Ignore Him.")])? {
            1 => {
                ctx.lines_as(
                    "Klitzer",
                    args![
                        "I can have this?",
                        "Oh, thank you so much!",
                        "Finally, I have something",
                        "nice enough to wear to meet",
                        "Calla's mom! What a relief!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Klitzer",
                    args![
                        "You've been helping me all",
                        "this time and I haven't properly expressed my gratitude. I'm sorry",
                        "if I've been too absorbed in my own problems. I may be poor, but I need to repay you somehow..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Klitzer",
                    args![
                        "Wait...",
                        "Why don't you have this",
                        "ore? I don't know how",
                        "valuable it is, but I know",
                        "that it's pretty rare. It may",
                        "even be useful to you later~"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7170), Val::from(1)])?;
                ctx.var("ein_loverq").set(Val::from(16))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8087), Val::from(8088)])?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                if subject2 == 1 {
                    ctx.call(Function::GetItem, vec![Val::from(7289), Val::from(1)])?;
                } else if subject2 == 2 {
                    ctx.call(Function::GetItem, vec![Val::from(7290), Val::from(1)])?;
                } else if subject2 == 3 {
                    ctx.call(Function::GetItem, vec![Val::from(7291), Val::from(1)])?;
                } else if subject2 == 4 {
                    ctx.call(Function::GetItem, vec![Val::from(7293), Val::from(1)])?;
                } else if subject2 == 5 {
                    ctx.call(Function::GetItem, vec![Val::from(7294), Val::from(1)])?;
                } else if subject2 == 6 {
                    ctx.call(Function::GetItem, vec![Val::from(7295), Val::from(1)])?;
                } else if subject2 == 7 {
                    ctx.call(Function::GetItem, vec![Val::from(7296), Val::from(1)])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Klitzer",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                        "you've really opened",
                        "my eyes. From now on,",
                        "I'll do my best to earn the",
                        "approval of Calla's parents and",
                        "become worthy of Calla's love."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Klitzer",
                    args![
                        "But how can I get",
                        "a Tuxedo? Ooh, I hope",
                        "it doesn't cost too much",
                        "zeny or I won't be able to",
                        "get one of those soon..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("ein_loverq").get()? == 15 {
        ctx.lines_as(
            "Klitzer",
            args![
                "Noooooo!",
                "I've got to find",
                "the perfect thing",
                "to wear or Calla's",
                "mother might ^FF0000hate^000000 me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "What am I gonna do?!",
                "Okay, nothing too flashy",
                "or revealing. Can't let her",
                "think I'm wild, immature or",
                "unpredictable. Should I",
                "wear suspenders or a belt?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 14 {
        ctx.lines_as(
            "Klitzer",
            args![
                "Noooooo!",
                "I've got to find",
                "the perfect thing",
                "to wear or Calla's",
                "mother might ^FF0000hate^000000 me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Think, Klitzer, think!",
                "Okay, no fun colors.",
                "I don't want her to",
                "think I'm not serious",
                "about Calla. Stripes",
                "might be bad too..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 13 {
        ctx.lines_as(
            "Klitzer",
            args!["Eh...?", "What did you just say?", "You did something for", "the Kapellthaines?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "What...?",
                "Did you just say",
                "that Calla's mother",
                "wants me to have tea",
                "with her? Holy moley...!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Okay, okay.",
                "I-I I'll need something",
                "nice to wear, right? Oh.",
                "Wow. This is so sudden!",
                "W-what should I do?"
            ],
        )?;
        ctx.next()?;
        ctx.var("ein_loverq").set(Val::from(14))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8085), Val::from(8086)])?;
        ctx.lines_as(
            "Klitzer",
            args![
                "This monkey suit",
                "that I've got on just",
                "won't do! Arrrgh! But I've",
                "already outgrown all of my",
                "nice clothes already. Boy,",
                "am I in a pickle..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()?.number()? > 6 {
        ctx.lines_as(
            "Klitzer",
            args![
                "*Sigh...*",
                "What can I do to",
                "get Calla's parents",
                "to accept me as her",
                "boyfriend? I can't",
                "think of anything..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 6 {
        ctx.lines_as(
            "Klitzer",
            args!["You've given her", "the flower? That's", "great! Thank you,", "thank you so mu--"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "...Oh!",
                "Gosh! I was thinking so",
                "much about myself that",
                "I forgot to ask you for your",
                "name! I'm sorry for being",
                "so knuckle-headed..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Klitzer", args!["So...", "What's your name?"])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s
            .clone()
            .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
        {
            ctx.lines_as(
                "Klitzer",
                args![
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                    "That's very nice. Thank you,",
                    "I'm really grateful for your help. Although I can't see Calla in",
                    "person, I can at least send",
                    "my regards if you help me."
                ],
            )?;
            ctx.next()?;
            ctx.var("ein_loverq").set(Val::from(7))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8080), Val::from(8081)])?;
            ctx.lines_as(
                "Klitzer",
                args![
                    "I feel so much",
                    "better now. Oh!",
                    "When you have the",
                    "time, why don't you talk",
                    "to my mother? She always",
                    "likes meeting my friends."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Klitzer",
                args![
                    "Hm...?",
                    "Oh, don't be",
                    "so nervous~",
                    "But would you please",
                    "tell me your name again?",
                    "I couldn't hear you..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ein_loverq").get()? == 5 {
        ctx.lines_as(
            "Klitzer",
            args![
                "Have you left to",
                "see Calla for me yet?",
                "I'm sorry if I sound",
                "pretty demanding..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Anyway, please",
                "remember to bring",
                "Calla ^FF00001 Flower^000000 for me.",
                "It doesn't need to be",
                "fancy, an ordinary one",
                "should be just fine."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(1901)])? == 1) {
        ctx.lines_as(
            "Klitzer",
            args![
                "What brings you here?",
                "Aren't you tired of hearing",
                "me moan and whine about",
                "lost love? ^333333*Siiiiigh...*^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Hey, this violin...",
                "Calla used to play",
                "such beautiful music",
                "on this for me. Did",
                "she give this to you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "I see. Calla must have",
                "known that I'm all broken up",
                "right now. She's too good to",
                "to me. How can she consider",
                "my feelings before thinking",
                "about herself?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "I know I'm being",
                "shameless, but I have",
                "a favor to ask. Adventurer,",
                "would you please send",
                "Calla a present for me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "All you need to do is",
                "just give her ^FF00001 Flower^000000.",
                "I don't have the zeny and",
                "I don't think I'm welcome",
                "at Kapellthaine Manor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Here, in return, I'll",
                "give you this health",
                "massage. It's not a",
                "big deal, but when I'm",
                "done, your mind and body",
                "will be refreshed. Here goes!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FF*Knead Knead*",
            "*Rub Rub Rub Rub*",
            "*Press Press Press*",
            "*C-c-c-c-c-c-crack!*"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh--", "Hell yeah!", "That's the stuff!"],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
        ctx.call(Function::DelItem, vec![Val::from(1901), Val::from(1)])?;
        ctx.var("ein_loverq").set(Val::from(5))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8077), Val::from(8079)])?;
        {
            if ctx.var("BaseLevel").get()?.number()? < 41 {
                ctx.call(Function::GetExperience, vec![Val::from(610), Val::from(0)])?;
            } else if ctx.var("BaseLevel").get()?.number()? < 61 {
                ctx.call(Function::GetExperience, vec![Val::from(6000), Val::from(0)])?;
            } else if ctx.var("BaseLevel").get()?.number()? < 81 {
                ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
            } else {
                ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
            }
        }
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Would you please bring",
                "1 Flower to Calla for me?",
                "I'm sorry for troubling you... "
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 2 {
        ctx.lines_as(
            "Klitzer",
            args![
                "^333333*Sigh*^000000",
                "I really appreciate your",
                "sympathy, but I'm merely",
                "a coward and a fool."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Yeah...",
                "I'm a coward for doing",
                "nothing about my feelings",
                "and an idiot for falling in love with such a high class girl in",
                "the first place... Oh, Calla..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 1 {
        ctx.lines_as(
            "Klitzer",
            args![
                "That faintly sweet",
                "and pleasant scent...",
                "It's just like the fragrance",
                "they use in Calla's house."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Ah, so you visited",
                "Calla in Einbroch?",
                "I miss her sooo much!",
                "Is she doing alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Calla's so beautiful.",
                "And she's so lovely.",
                "Every time I close my",
                "eyes, I can still see",
                "her lovely smile."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Klitzer",
            args![
                "I'm sorry if I sound",
                "silly, but I can't help",
                "it. I know I'm acting like",
                "a complete idiot. But I'd",
                "give anything to see her..."
            ],
        )?;
        ctx.next()?;
        ctx.var("ein_loverq").set(Val::from(2))?;
        ctx.call(Function::SetQuest, vec![Val::from(8076)])?;
        ctx.lines_as(
            "Klitzer",
            args![
                "Just...",
                "Just don't listen",
                "to anything I say.",
                "I'm just a poor fool",
                "in love with the wrong",
                "person. That has to be it..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Klitzer",
        args![
            "Is there something",
            "that you really want",
            "in life, but it's just",
            "beyond your grasp?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Klitzer",
        args![
            "I wish I were more",
            "like you adventurers.",
            "People like you never",
            "seem to give up, no matter",
            "what the obstacles may be.",
            "But I'm so helpless..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Klitzer",
        args![
            "I can't even see",
            "the one person that",
            "I love. We've just so",
            "different that it's not",
            "even possible anymore..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn klitzer_ein(ctx: &Ctx) -> Script {
    klitzer_ein_body(ctx, Vec::new()).map(|_| ())
}

fn megass_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn megass_ein(ctx: &Ctx) -> Script {
    megass_ein_body(ctx, Vec::new()).map(|_| ())
}

fn megass_ein_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ein_loverq").get()? == 4 && ctx.var("ein_loverq").get()? == 5) {
        ctx.lines_as("Megass", args!["You again?!", "What do you", "want from me?!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Megass",
            args![
                "Guards...!",
                "Sweep the driveway with",
                "this guy's face and keep",
                "punching the stomach",
                "until there's nothing",
                "left to throw up!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(-90), Val::from(0)])?;
        ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(112), Val::from(245)])?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(1901)])?.number()? > 0) {
        ctx.lines_as(
            "Megass",
            args![
                "That's...",
                "That's my",
                "daughter's Violin.",
                "I gave that to her",
                "for her sweet sixteen..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Megass",
            args![
                "It's bad enough to",
                "intrude into my house,",
                "but to steal from my Calla?!",
                "You've crossed the line, punk!",
                "Guards! Get over here!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Megass",
            args![
                "Men, I want you to",
                "knock the wind out",
                "of this fool and anything",
                "else that might be inside!",
                "Make sure that if this punk",
                "wakes up, it won't be today!"
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(1901), Val::from(1)])?;
        ctx.var("ein_loverq").set(Val::from(4))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8077), Val::from(8078)])?;
        ctx.call(Function::PercentHeal, vec![Val::from(-90), Val::from(0)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(112), Val::from(245)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Megass",
        args![
            "How dare you...",
            "A vagabond like",
            "you setting foot",
            "into my home!?",
            "Unthinkable!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Megass",
        args!["Leave immediately", "before I report you", "to the authorities", "for trespassing!"],
    )?;
    ctx.next()?;
    ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(112), Val::from(245)])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn megass_ein_ontouch(ctx: &Ctx) -> Script {
    megass_ein_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn satra_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("ein_loverq").get()?.number()? > 12 {
        ctx.lines_as(
            "Satra",
            args![
                "I understand that my",
                "home is extravagantly",
                "splendid and to approach",
                "any Kappelthaine is an",
                "honor to most commoners."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "However, Klitzer",
                "has no reason to feel so",
                "intimidated. Tell the poor",
                "boy that he's earned the",
                "honor of speaking with me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 12 && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 9) {
        ctx.lines_as(
            "Satra",
            args![
                "Ho ho ho ho~",
                "Welcome adventurer,",
                "I so enjoy our little chats.",
                "Hors d'oeuvre?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh! You can't enjoy",
                "any food in that state!",
                "Your hands are atrociously",
                "grimy! May I ask why?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Ah~",
                "In my excitement,",
                "I nearly forgot that",
                "I asked you to bring",
                "Coals to me again!",
                "My apologies~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "You've brought so much Coal",
                "to me, I'm convinced that you're much more diligent than those",
                "languid peasants in Einbech.",
                "I appreciate that you've labored so much to win my favor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Hm, what was that?",
                "Someone else provided",
                "these Coals and you were",
                "only delivering them? Then",
                "who actually gathered these?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Klitzer" {
            ctx.lines_as(
                "Satra",
                args![
                    "Ah, Klitzer?",
                    "I must say, that's",
                    "a very humble name.",
                    "Yet it's so familiar..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satra",
                args![
                    "Ah, I recall there",
                    "was a hooligan that",
                    "has been pestering my",
                    "daughter named Klitzer.",
                    "Perhaps they are one",
                    "and the same. Hmm..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satra",
                args![
                    "I remember that he was",
                    "rather shabby looking and",
                    "lacked any semblance of",
                    "etiquette whatsoever. Clearly,",
                    "he is a fool and a coward, but now I see that he is sincere."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satra",
                args![
                    "It might not be possible",
                    "to let him go out with my",
                    "daughter straight away, but",
                    "I will invite him for a spot of",
                    "tea. And if Calla likes him,",
                    "well, he must be special."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Satra",
                args![
                    "For a humble peasant,",
                    "this must be like a dream",
                    "come true! And to have tea",
                    "with Klitzer. Oh, what would",
                    "the girls say? Ah, but I did",
                    "marry that oafish Megass~"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(10)])?;
            ctx.var("ein_loverq").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8084), Val::from(8085)])?;
            ctx.lines_as(
                "Satra",
                args![
                    "Anyway, when you next",
                    "meet Klitzer, please tell",
                    "him to pay me a visit soon.",
                    "Oh, and remind him to dress",
                    "appropriately for this special",
                    "occasion. Ho ho ho ho ho~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Satra",
                args![
                    "I beg your pardon?",
                    (l_input_s.clone() + Val::from("? Oh my...")),
                    "I believe I may have",
                    "misheard you. Ho ho ho ho~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ein_loverq").get()? == 12 {
        ctx.lines_as(
            "Satra",
            args![
                "Ho ho ho ho~",
                "Welcome adventurer,",
                "I so enjoy our little chats.",
                "Hors d'oeuvre?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Ah...",
                "It seems you've",
                "forgotten your ^FF0000Coals^000000.",
                "A silly mistake, but one",
                "I'm willing to overlook."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 11 && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 9) {
        ctx.lines_as(
            "Satra",
            args![
                "Why hello~",
                "Your visits have",
                "recently been quite",
                "delightful, fair adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Continue to show",
                "your appreciation",
                "and dedication to me",
                "and you'll soon be known",
                "to be my most favored",
                "commoner. Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "I'm sure you've noticed",
                "the strained relationship",
                "between Einbroch and Einbech",
                "by now. It's a shame, really."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "As Einbroch grew wealthier",
                "and Einbech became more",
                "destitude, the affluent began",
                "despising the impoverished.",
                "I suppose it follows that the poor started to resent the rich."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "It might be said that",
                "both towns have been",
                "trying to take advantage",
                "of each other, but it's",
                "clear that Einbroch has",
                "always had the upper hand."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "In fact, because of",
                "this rift between our",
                "cities, our families are",
                "taught not to associate",
                "with the people of Einbech."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Now, I believe that most",
                "people who live in Einbech",
                "are peons, but that does not",
                "mean I will not give them a",
                "chance to prove their worth."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(10)])?;
        ctx.var("ein_loverq").set(Val::from(12))?;
        ctx.lines_as(
            "Satra",
            args![
                "Why should I deprive",
                "the lowly of my gracious",
                "presense if they prove",
                "themselves meritable?",
                "Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, that was scrumptious!",
                "If you wish to pay me another",
                "visit, don't forget to bring some Coal with you. Tah tah~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 11 {
        ctx.lines_as(
            "Satra",
            args![
                "Why hello~",
                "Your visits have",
                "recently been quite",
                "delightful, fair adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, my apologies!",
                "I suppose you're just",
                "here for sight seeing,",
                "or perhaps you're running",
                "some sort of adventurer's",
                "errand. Am I right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "I know you well enough",
                "to know that you wouldn't",
                "be so rude as to stop by",
                "and chat without bringing",
                "any ^FF0000Coal^000000. Ho ho ho ho~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 10 && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 9) {
        ctx.lines_as(
            "Satra",
            args![
                "^333333*Titter~*^000000",
                "Why, if it isn't my",
                "intrepid adventurer.",
                "Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "My word!",
                "Why are you carrying",
                "all of that dirty Coal",
                "with you? You poor,",
                "impoverished thing."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh! Dear me,",
                "I've nearly forgotten~",
                "How divinely silly of me!",
                "Once again, I graciously",
                "accept your small gift on",
                "behalf of the Kappellthaines."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Let me share a little",
                "bit of history concerning",
                "our lovely Einbroch. At one",
                "time, there was only Einbech,",
                "the mining village. You can",
                "imagine how long ago that was."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "However, the minerals",
                "and ores mined in Einbech",
                "need to be processed and",
                "refined in factories that were",
                "all built in a nearby industrial complex which became Einbroch."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Now every citizen in",
                "Einbroch is wealthy and",
                "it's well known that there",
                "is a higher standard of",
                "living here than in Einbech."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(10)])?;
        ctx.var("ein_loverq").set(Val::from(11))?;
        ctx.lines_as(
            "Satra",
            args![
                "Goodness, I believe",
                "it's time for a spot of tea~",
                "The next time you wish to have",
                "an audience with me, it would",
                "behoove you to bring another",
                "gift of Coal. Toodles~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 10 {
        ctx.lines_as(
            "Satra",
            args![
                "^333333*Titter~*^000000",
                "Why, if it isn't my",
                "intrepid adventurer.",
                "Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "My apologies, but you",
                "must first prove to me",
                "that you are worthy of",
                "conversation. Why don't",
                "you deliver more of those",
                "^FF0000Coals^000000, mmm?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "After all, I have no other",
                "means of knowing whether",
                "or not you appreciate the time",
                "I sacrifice by socializing with",
                "someone of your status.",
                "Ho ho ho ho~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 9 && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 9) {
        ctx.lines_as(
            "Satra",
            args!["My, you've already", "brought the Coal?", "How charmingly", "prompt you are~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "On behalf of the",
                "Kappellthaine family,",
                "I shall ignore your lowly",
                "status and graciously",
                "accept your small gift.",
                "Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Do you hail from",
                "Einbech, adventurer?",
                "Ah, the Rune-Midgarts",
                "kingdom! I've visited your",
                "country. It's quite quaint",
                "and Jawaii is very lovely."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(10)])?;
        ctx.var("ein_loverq").set(Val::from(10))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8083), Val::from(8084)])?;
        ctx.lines_as(
            "Satra",
            args![
                "Well, I shall try to",
                "find some use for these.",
                "I'm afraid the gift I've asked",
                "from you isn't very practical.",
                "How is Coal usually used?",
                "Ah, I have a novel idea!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Wouldn't it be",
                "intoxicatingly wild if",
                "Megass were to hold one",
                "of those social functions that",
                "the lower classes are so fond",
                "of? A 'barbeque,' yes?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "In any case, if you",
                "bring me more of that",
                "Coal, you would be even",
                "more favored by me and",
                "you'll become a recipient",
                "of my good graces. Ho ho~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 9 {
        ctx.lines_as(
            "Satra",
            args![
                "A wandering adventurer...?",
                "How ravishingly delightful!",
                "You must have risked life and",
                "limb to sneak past my husband",
                "to enjoy the captivating sights",
                "of my home. Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Since you're a very",
                "unique guest, I shall",
                "give you a unique honor",
                "and deign to converse",
                "with you, adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Einbech exists to provide",
                "my family with coal and",
                "materials from their mines.",
                "It's a natural law: workers",
                "must be led by a chosen few."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "It's delightfully",
                "ludicrous to see those",
                "workers aspire to reach",
                "our heights of social",
                "prestige. Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, I haven't spoken to",
                "someone from a lower",
                "class in ages! It feels",
                "so forbiddenly exciting!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Well adventurer, that's ",
                "the end of our informal",
                "chat. I'll even grant you",
                "full permission to boast",
                "of the fact that you've",
                "spoken to Lady Satra."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "If you do wish for",
                "me to share words with",
                "you once more, prove to",
                "me that you're worthier",
                "than the other peons of",
                "my graceful presense."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, I have a marvelous",
                "idea! Why don't you bring",
                "me ^990000Coals^000000? It's not impossible",
                "for someone like yourself, but",
                "this kind of task will require",
                "some effort on your part."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Satra", args!["Tah tah,", "adventurer~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 8 {
        ctx.lines_as(
            "Satra",
            args![
                "A wandering adventurer...?",
                "How ravishingly delightful!",
                "You must have risked life and",
                "limb to sneak past my husband",
                "to enjoy the captivating sights",
                "of my home. Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Since you're a very",
                "unique guest, I shall",
                "give you a unique honor",
                "and deign to converse",
                "with you, adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Einbech exists to provide",
                "my family with coal and",
                "materials from their mines.",
                "It's a natural law: workers",
                "must be led by a chosen few."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "It's delightfully",
                "ludicrous to see those",
                "workers aspire to reach",
                "our heights of social",
                "prestige. Ho ho ho ho~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, I haven't spoken to",
                "someone from a lower",
                "class in ages! It feels",
                "so forbiddenly exciting!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Well adventurer, that's ",
                "the end of our informal",
                "chat. I'll even grant you",
                "full permission to boast",
                "of the fact that you've",
                "spoken to Lady Satra."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "If you do wish for",
                "me to share words with",
                "you once more, prove to",
                "me that you're worthier",
                "than the other peons of",
                "my graceful presense."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Satra",
            args![
                "Oh, I have a marvelous",
                "idea! Why don't you bring",
                "me ^990000Coals^000000? It's not impossible",
                "for someone like yourself, but",
                "this kind of task will require",
                "some effort on your part."
            ],
        )?;
        ctx.var("ein_loverq").set(Val::from(9))?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8081)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8081), Val::from(8083)])?;
        } else {
            ctx.call(Function::ChangeQuest, vec![Val::from(8082), Val::from(8083)])?;
        }
        ctx.next()?;
        ctx.lines_as("Satra", args!["Tah tah,", "adventurer~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Satra",
        args!["A wandering", "adventurer...?", "How sinfully", "intriguing!", "Ho ho ho ho~"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Satra",
        args![
            "Oh, do not worry.",
            "I'm sure the beauty of",
            "my home has captured your",
            "curiosity. My brutish excuse",
            "for a husband would have you",
            "beat if he found you..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Satra",
        args![
            "But I'm far more genteel",
            "than Megass. You're more",
            "than welcome to enjoy the",
            "furnishings. Ho ho ho ho~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn satra_ein(ctx: &Ctx) -> Script {
    satra_ein_body(ctx, Vec::new()).map(|_| ())
}

fn kaijeta_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ein_loverq").get()?.number()? > 15 {
        ctx.lines_as(
            "Kaijeta",
            args![
                "My son learned",
                "an awful lot from",
                "your good example.",
                "As a mother, I really",
                "appreciate everything",
                "you've done for him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "Well, adventurer,",
                "I will be praying for",
                "your safety wherever",
                "your journeys may",
                "take you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 15 {
        ctx.lines_as(
            "Kaijeta",
            args![
                "Thank you for helping",
                "my son Klitzer. Sadly, the",
                "little fool doesn't have any",
                "clue when it comes to certain",
                "things like choosing clothing.",
                "He gets so nervous about it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "A man should wear nice",
                "clothes, like a Formal Suit",
                "or a Tuxedo, for important",
                "meetings and special occasions.",
                "Hopefully, he'll learn that soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 14 {
        ctx.lines_as(
            "Kaijeta",
            args![
                "Thank you for helping",
                "my son Klitzer. Sadly, the",
                "little fool doesn't have any",
                "clue when it comes to certain",
                "things like choosing clothing.",
                "He gets so nervous about it!"
            ],
        )?;
        ctx.next()?;
        ctx.var("ein_loverq").set(Val::from(15))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8086), Val::from(8087)])?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "A man should wear nice",
                "clothes, like a Formal Suit",
                "or a Tuxedo, for important",
                "meetings and special occasions.",
                "Hopefully, he'll learn that soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 8 {
        ctx.lines_as(
            "Kaijeta",
            args![
                "I can't bear to see the",
                "petty hatred between our",
                "two towns stop my son from",
                "seeing the woman he loves..."
            ],
        )?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8081)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8081), Val::from(8082)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 7 {
        ctx.lines_as(
            "Kaijeta",
            args![
                "Thank you for helping my",
                "son. He may look like a fool",
                "for falling in love with someone from Einbroch, but he's an honest",
                "hard working man."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "Then again, it was bound to",
                "happen sometime. Einbech is",
                "a very poor town while Einbroch",
                "is a very rich town. I'd understand if you don't agree, but opposites attract sooner or later."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "^666666*Sigh...*^000000",
                "If my son and Calla can",
                "work things out, maybe it",
                "would improve relations",
                "between our two towns.",
                "I certainly hope so..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "Still, I'm quite baffled!",
                "I raised my son to have more",
                "guts than to wallow in misery",
                "when his heart's broken. And",
                "I still have no idea how he got together with such a rich woman."
            ],
        )?;
        ctx.next()?;
        ctx.var("ein_loverq").set(Val::from(8))?;
        ctx.lines_as(
            "Kaijeta",
            args![
                "Do you have any idea",
                "how we can put an end",
                "to the hate between our",
                "two towns? I don't want to",
                "see this couple separated",
                "because of such pettiness."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Kaijeta",
        args!["Welcome to my humble", "abode, adventurer. I'm", "sorry if I'm a poor host."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kaijeta",
        args![
            "As you can see, we have",
            "to share this house with",
            "other families so we don't",
            "have much open space or",
            "privacy. I'm afraid we can't",
            "afford even basic comfort."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kaijeta",
        args![
            "For now, this is the best",
            "we can do. We don't have",
            "the zeny to buy a house or",
            "land, so we have no choice",
            "but to endure through this..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kaijeta_ein(ctx: &Ctx) -> Script {
    kaijeta_ein_body(ctx, Vec::new()).map(|_| ())
}

fn keneshiotz_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Keneshiotz",
        args!["This city is full of sky", "high smokestacks and", "the droning hum of machines."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Keneshiotz",
        args![
            "Sure, the air is polluted,",
            "but I think it's a fair price",
            "to pay for wealth and a",
            "modern life of comfort.",
            "Screw the environment!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Keneshiotz",
        args![
            "I'd much rather live like",
            "this than end up like those",
            "backwards vagrants in that",
            "filthy Einbech. Don't they",
            "know that money makes",
            "the world go 'round?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn keneshiotz_ein(ctx: &Ctx) -> Script {
    keneshiotz_ein_body(ctx, Vec::new()).map(|_| ())
}

fn catzllanpu_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Catzllanpu",
        args![
            "^333333*Sigh...*^000000",
            "Simple pleasures.",
            "They're what make",
            "life worth living,",
            "you know?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Catzllanpu",
        args![
            "It's enough for me just to",
            "live a normal and happy life,",
            "but everyone around me wants",
            "to work harder and harder. If",
            "you never take a rest, you're",
            "killing yourself pretty slowly."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Catzllanpu",
        args![
            "I guess you can tell that",
            "I don't have the worries",
            "other people have about",
            "money. It's great, but it's",
            "not worth sacrificing the",
            "quality of your life, right?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn catzllanpu_ein(ctx: &Ctx) -> Script {
    catzllanpu_ein_body(ctx, Vec::new()).map(|_| ())
}

fn kesunboss_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kesunboss",
        args!["Lady Calla is the", "epitome of elegance,", "a veritable goddess", "of Einbroch."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kesunboss",
        args![
            "Her gentle voice,",
            "that angelic smile, her",
            "kindness and warmth",
            "towards other people",
            "and above all..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kesunboss",
        args![
            "Calla's family",
            "is wealthy beyond",
            "imagination! She's",
            "perfect! I don't know who",
            "she'll marry, but he'd be",
            "a lucky gentleman, I'm sure."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kesunboss",
        args![
            "Lady Calla lives in a magnificent mansion that makes other houses",
            "look like shacks in comparison.",
            "Head north and then west from",
            "here if you wish to marvel in its",
            "beauty and elegance."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kesunboss_ein(ctx: &Ctx) -> Script {
    kesunboss_ein_body(ctx, Vec::new()).map(|_| ())
}

fn ellhenje_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Ellhenje",
        args![
            "Things might be",
            "bad in this town",
            "with the pollution",
            "and the bullying",
            "from Einbroch..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ellhenje",
        args![
            "But somehow, people",
            "are able to get by. That's",
            "because there's a guy",
            "that everyone here likes..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ellhenje",
        args![
            "I'm talking about Klitzer!",
            "He's almost too honest and",
            "almost too diligent. But most",
            "of all, he's the nicest guy~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ellhenje",
        args![
            "Klitzer was born in one of",
            "Einbech's poorest families,",
            "but he's usually happy and always thinks about others. I guess that's why people like to think of him",
            "as representing all of Einbech."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ellhenje",
        args![
            "Recently, something's",
            "been bothering him. I'm",
            "not sure, but I think only",
            "woman troubles could make",
            "a guy feel so glum. I hope he",
            "feels better soon..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ellhenje_ein(ctx: &Ctx) -> Script {
    ellhenje_ein_body(ctx, Vec::new()).map(|_| ())
}

fn decii_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Decii",
        args!["This is so", "frustrating!", "I'm surrounded", "by all these ^FF0000people^000000!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Decii",
        args![
            "There's absolutely",
            "no privacy in a city",
            "this crowded! I guess",
            "I should try to move",
            "out as soon as I can."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn decii_ein(ctx: &Ctx) -> Script {
    decii_ein_body(ctx, Vec::new()).map(|_| ())
}

fn supineque_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Supineque", args!["Ugh...", "I'm starving!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Supineque",
        args![
            "I haven't had food for so",
            "long that my stomach is",
            "beginning to digest itself!",
            "This is horrible..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Supineque",
        args![
            "I mean, I have",
            "food that I can",
            "eat today. But if",
            "I finish it, what am",
            "I gonna eat tomorrow?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supineque_ein(ctx: &Ctx) -> Script {
    supineque_ein_body(ctx, Vec::new()).map(|_| ())
}
