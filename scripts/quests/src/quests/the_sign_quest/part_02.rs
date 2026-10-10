use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn rogue_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second! Right now,",
            "you have too many items in your inventory. Please come back after you've freed up more inventory space.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sign_q").get()?.number()? < 3 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 3 {
            ctx.lines_as("Arian", args!["...", "Who the hell", "are you, jerkface?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[
                    ((Val::from("Metz sent me here.:^0000FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from("^000000.:Who are you then?")),
                ],
            )? {
                1 => {
                    ctx.lines_as(
                        "Arian",
                        args![
                            "Oh yeah...?",
                            "Well, I'm sending you",
                            "back! No way I'm fallin'",
                            "for that trick, chump!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(279), Val::from(173)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Arian",
                        args![
                            "Wha...?",
                            ((Val::from("^0000FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000?")),
                            "Yeah, okay. Metz did",
                            "mention something about",
                            "about you. You're here",
                            "for the test, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arian",
                        args![
                            "Alright, this",
                            "test is simple.",
                            "I tell you to bring me",
                            "a bunch of items and",
                            "you go get them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Arian", args!["I know you jerkward adventurers", "are always talking and you share the answers for any test some dude is givin' out. 'Course, it doesn't help that I use the same test", "every frickin' time..."])?;
                    ctx.next()?;
                    ctx.lines_as("Arian", args!["Now, you gotta bring and only", "have the item I ask you for when you come to me. If you know that I'll be asking for something later and you happen to have it, I'll straight up ^FF0000jack it^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arian",
                        args![
                            "The first item?",
                            "Gimme ^FF0000100 Fluff^000000.",
                            "Bring that and I'll",
                            "tell you what to",
                            "bring next."
                        ],
                    )?;
                    ctx.var("sign_q").set(Val::from(4))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Arian", args!["...", "......"])?;
                    ctx.next()?;
                    ctx.lines_as("Arian", args!["...", "......", "Your mom.", "Now get the", "hell outta here!"])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FIREHIT")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("sign_q").get()?.number()? < 15 {
            let subject2 = ctx.var("sign_q").get()?;
            if subject2 == 4 {
                ctx.lines_as("Arian", args!["..."])?;
                if ctx.call(Function::CountItem, vec![Val::from(914)])?.number()? > 99 {
                    ctx.call(Function::DelItem, vec![Val::from(914), Val::from(100)])?;
                    if ctx.call(Function::CountItem, vec![Val::from(7033)])?.number()? > 49 {
                        ctx.call(Function::DelItem, vec![Val::from(7033), Val::from(50)])?;
                        if ctx.call(Function::CountItem, vec![Val::from(904)])?.number()? > 29 {
                            ctx.call(Function::DelItem, vec![Val::from(904), Val::from(30)])?;
                            if ctx.call(Function::CountItem, vec![Val::from(930)])?.number()? > 19 {
                                ctx.call(Function::DelItem, vec![Val::from(930), Val::from(20)])?;
                                if ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 14 {
                                    ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(15)])?;
                                    if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                                        ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    ctx.lines(args![
                        "Hey. What the hell's wrong",
                        "with you? Hurry and get me",
                        "^FF0000100 Fluff^000000, ya slacker."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "Alright, that looks like",
                    "enough Fluff. Now, go and",
                    "get me ^FF000050 Poison Spores^000000."
                ])?;
                ctx.var("sign_q").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 5 {
                ctx.lines_as("Arian", args!["..."])?;
                if ctx.call(Function::CountItem, vec![Val::from(7033)])?.number()? > 49 {
                    ctx.call(Function::DelItem, vec![Val::from(7033), Val::from(50)])?;
                    if ctx.call(Function::CountItem, vec![Val::from(904)])?.number()? > 29 {
                        ctx.call(Function::DelItem, vec![Val::from(904), Val::from(30)])?;
                        if ctx.call(Function::CountItem, vec![Val::from(930)])?.number()? > 19 {
                            ctx.call(Function::DelItem, vec![Val::from(930), Val::from(20)])?;
                            if ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 14 {
                                ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(15)])?;
                                if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                                    ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                                }
                            }
                        }
                    }
                } else {
                    ctx.lines(args![
                        "Hey. What part of 'Get me",
                        "^FF000050 Poison Spores^000000 or I'll",
                        "kick your ass,' don't",
                        "you understand?"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "It's about time you",
                    "got here with those",
                    "Poison Spores. Now, go",
                    "and get ^FF000030 Scorpion Tails^000000."
                ])?;
                ctx.var("sign_q").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 6 {
                ctx.lines_as("Arian", args!["..."])?;
                if ctx.call(Function::CountItem, vec![Val::from(904)])?.number()? > 29 {
                    ctx.call(Function::DelItem, vec![Val::from(904), Val::from(30)])?;
                    if ctx.call(Function::CountItem, vec![Val::from(930)])?.number()? > 19 {
                        ctx.call(Function::DelItem, vec![Val::from(930), Val::from(20)])?;
                        if ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 14 {
                            ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(15)])?;
                            if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                                ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                            }
                        }
                    }
                } else {
                    ctx.lines(args![
                        "What, your mom drop you on the",
                        "head right after you were born?",
                        "Stop bein' stupid and get me",
                        "^FF000030 Scorpion Tails^000000, nimrod."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "Damn, you sure took your",
                    "sweet time bringing over this",
                    "over this crap. Okay, now bring",
                    "^FF000020 Rotten Bandages^000000."
                ])?;
                ctx.var("sign_q").set(Val::from(7))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 7 {
                ctx.lines_as("Arian", args!["..."])?;
                if ctx.call(Function::CountItem, vec![Val::from(930)])?.number()? > 19 {
                    ctx.call(Function::DelItem, vec![Val::from(930), Val::from(20)])?;
                    if ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 14 {
                        ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(15)])?;
                        if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                            ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                        }
                    }
                } else {
                    ctx.lines(args![
                        "Is it really that hard to get",
                        "^FF000020 Rotten Bandages^000000? Cuz if it",
                        "is, then you must really blow.",
                        "Now hustle it up, punk!"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "You brought the bandages.",
                    "Okay short stuff, go and get",
                    "me ^FF000015 Little Evil Horn^000000. What",
                    "are you waiting for, a memo?",
                    "Get outta here~!"
                ])?;
                ctx.var("sign_q").set(Val::from(8))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 8 {
                ctx.lines_as("Arian", args!["..."])?;
                if ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 14 {
                    ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(15)])?;
                    if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                        ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                    }
                } else {
                    ctx.lines(args![
                        "I don't get it.",
                        "You don't have the",
                        "^FF000015 Little Evil Horn^000000",
                        "I told you to get."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Arian",
                        args![
                            "I see, so it's my",
                            "fault for not telling you",
                            "not to bring your crap face",
                            "back without 'em. ^333333*A-hem*^000000",
                            "Don't bring your crap face here without 15 Little Evil Horns!!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "My grandma coulda grown",
                    "out her beard in the time",
                    "it tookyou to come back.",
                    "Now hurry it up and get",
                    "me ^FF000010 Coral Reefs^000000!"
                ])?;
                ctx.var("sign_q").set(Val::from(9))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 9 {
                ctx.mes("[Arian]")?;
                if ctx.call(Function::CountItem, vec![Val::from(7013)])?.number()? > 9 {
                    ctx.call(Function::DelItem, vec![Val::from(7013), Val::from(10)])?;
                } else {
                    ctx.lines(args![
                        "I don't care how pretty",
                        "they are or if you're ruining",
                        "the ecosystem! When I say",
                        "''bring ^FF000010 Coral Reefs^000000,'' you",
                        "better have them! All the",
                        "fish can die for all I care."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "Finally you're back.",
                    "Gimme a sec to count",
                    "all this stuff so I can make",
                    "sure you're not trying to",
                    "cheat. God help you if you",
                    "try to pull a fast one on me...!"
                ])?;
                ctx.var("sign_q").set(Val::from(10))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 10 {
                ctx.lines_as(
                    "Arian",
                    args![
                        "...",
                        "Alright. Go talk",
                        "to the guy to my left.",
                        "The dude at the counter.",
                        "What's-his-face, Ganaan."
                    ],
                )?;
                ctx.var("sign_q").set(Val::from(11))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 11 {
                ctx.lines_as(
                    "Arian",
                    args![
                        "...",
                        "Alright. Go talk",
                        "to the guy to my left.",
                        "The dude at the counter.",
                        "What's-his-face, Ganaan."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arian",
                    args![
                        "You wanna leave me",
                        "alone now and gimme",
                        "a little personal space?!",
                        "I need a break from looking",
                        "at your ugly mug, you know?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 12 {
                ctx.lines_as(
                    "Arian",
                    args![
                        "Let's see...",
                        "I'm looking at your",
                        "answers and they totally",
                        "suck. Take the test again",
                        "and do it right this time!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Arian",
                    args!["Geez...", "You're givin' Ganaan", "a hard time. You hear", "me? That's my job!"],
                )?;
                ctx.var("sign_q").set(Val::from(11))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 13 {
                ctx.lines_as(
                    "Arian",
                    args![
                        "Alright. The answers",
                        "you gave Ganaan tell me",
                        "you're not a total dumbass.",
                        "Now you're supposed to go see Daewoon in Payon. And don't forget to show him this Star thingee."
                    ],
                )?;
                {
                    if ctx.var("BaseLevel").get()?.number()? < 60 {
                        ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                        ctx.call(Function::GetExperience, vec![Val::from(7500), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                        ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                        ctx.call(Function::GetExperience, vec![Val::from(13000), Val::from(0)])?;
                    } else {
                        ctx.call(Function::GetExperience, vec![Val::from(18000), Val::from(0)])?;
                    }
                }
                ctx.var("sign_q").set(Val::from(14))?;
                ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 14 {
                ctx.lines_as(
                    "Arian",
                    args![
                        "...........",
                        "Payon...?",
                        "Daewoon?",
                        "Any of it ring a bell?",
                        "Cuz it really oughtta!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Arian",
                args![
                    "...",
                    "Shaddup and",
                    "lemme alone!",
                    "I'm thinking about",
                    "solving world hunger",
                    "here! Or somethin' like that."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn rogue_sign(ctx: &Ctx) -> Script {
    rogue_sign_body(ctx, Vec::new()).map(|_| ())
}

fn young_man_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_point_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Gaanan]")?;
    if ctx.var("sign_q").get()?.number()? < 11 {
        ctx.lines(args![
            "The weather here in",
            "Morocc is too hot for me.",
            "I'm having a hard time just",
            "trying to live here. Do you",
            "know a nice and cool place",
            "where I can work?"
        ])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 11 {
        ctx.lines(args![
            "Oh, Arian sent you to me,",
            "right? I'm sorry, but he takes",
            "a little getting used to. Even",
            "though I still have to get used",
            "to his... mannerisms."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gaanan",
            args![
                "Anyway, since he thinks it's",
                "boring, Arian put me in charge",
                "of the quiz portion of your test. Please carefully choose an",
                "answer when I ask you a",
                "question. Are you ready?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gaanan",
            args![
                "Now, the first question.",
                "Let's say that you just",
                "found yourself on a deserted",
                "island. What is the very first",
                "thing that you should do?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Look for fresh water.:Just wait for rescue.:Forage for food.:Explore the island.",
            )],
        )? {
            1 => {
                l_point_s = (l_point_s.clone() + Val::from(7));
            }
            2 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            3 => {
                l_point_s = (l_point_s.clone() + Val::from(4));
            }
            4 => {
                l_point_s = (l_point_s.clone() + Val::from(10));
            }
            _ => {}
        }
        ctx.lines_as(
            "Gaanan",
            args![
                "The second question is...",
                "You happen to be stuck in",
                "a narrow place inside some",
                "collapsed building. What",
                "do you plan to do first?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Scream for help.:Stay quiet.:Wait for death to come.:Find a way out.")],
        )? {
            1 => {
                l_point_s = (l_point_s.clone() + Val::from(3));
            }
            2 => {
                l_point_s = (l_point_s.clone() + Val::from(7));
            }
            3 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            4 => {
                l_point_s = (l_point_s.clone() + Val::from(7));
            }
            _ => {}
        }
        ctx.lines_as(
            "Gaanan",
            args![
                "Now, the third question.",
                "You're on some dungeon",
                "expedition with your friends,",
                "but you got lost somehow.",
                "How do you handle it?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Try to find my friends.:Find a way out.:Stay put.:Continue exploring the dungeon.",
            )],
        )? {
            1 => {}
            2 => {
                l_point_s = (l_point_s.clone() + Val::from(5));
            }
            3 => {
                l_point_s = (l_point_s.clone() + Val::from(10));
            }
            4 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            _ => {}
        }
        ctx.lines_as(
            "Ganaan",
            args![
                "Here's the fourth question.",
                "You're with your friends inside",
                "a mansion with no exit. What do",
                "you do first when a murder happens inside the mansion?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Wait for the case to get solved.:Try to find my friends first.:Find a way out.:Solve the murder case on my own.:Kill the others before they kill me.",
            )],
        )? {
            1 => {
                l_point_s = (l_point_s.clone() + Val::from(5));
            }
            2 => {
                l_point_s = (l_point_s.clone() + Val::from(7));
            }
            3 => {
                l_point_s = (l_point_s.clone() + Val::from(3));
            }
            4 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            5 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            _ => {}
        }
        ctx.lines_as(
            "Gaanan",
            args![
                "The fifth question is...",
                "You hear that the end of",
                "the world is in one week.",
                "So what do you do during",
                "this final week?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Wait to see the end.:Plant an apple tree.:Look to move to a different world.:What else? Save the world.:What else? Go on a crime spree.",
            )],
        )? {
            1 => {
                l_point_s = (l_point_s.clone() + Val::from(5));
            }
            2 => {
                l_point_s = (l_point_s.clone() + Val::from(3));
            }
            3 => {
                l_point_s = (l_point_s.clone() + Val::from(7));
            }
            4 => {
                l_point_s = (l_point_s.clone() + Val::from(10));
            }
            5 => {
                l_point_s = (l_point_s.clone() + Val::from(1));
            }
            _ => {}
        }
        ctx.lines_as(
            "Gaanan",
            args![
                "Oh! We're done!",
                "Okay, let me give",
                "these answers to Arian.",
                "Sooo... Talk to him and",
                "he'll let you know how",
                "you did."
            ],
        )?;
        if l_point_s.clone().number()? > 33 {
            ctx.var("sign_q").set(Val::from(13))?;
        } else {
            ctx.var("sign_q").set(Val::from(12))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()?.number()? < 14 {
        ctx.lines(args![
            "Hm...?",
            "Arian is the one",
            "who's grading your",
            "test. But I understand.",
            "If you failed, I'd want",
            "to avoid him too..."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^333333*Sigh...*^000000",
            "I wish the weather'd",
            "cool down, even just a little.",
            "While I'm asking for miracles,",
            "I may as well wish for a billion kajillion zeny. And maybe a yacht."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_man_sign(ctx: &Ctx) -> Script {
    young_man_sign_body(ctx, Vec::new()).map(|_| ())
}

fn hagin_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hagin",
        args!["Have you heard?", "Payon's most eligible", "bachelor is back home!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hagin",
        args![
            "He was a little brat",
            "back when he was a kid,",
            "but now he's grown up to",
            "be the manliest of men!",
            "No wonder all the ladies",
            "just can't resist him!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hagin",
        args![
            "Heh heh...",
            "With this beautiful",
            "face and charming figure...",
            "I'm gonna seduce him."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Who are you talking about?:You're freaking me out!")])? {
        1 => {
            ctx.lines_as(
                "Hagin",
                args![
                    "You don't know",
                    "Daewoon, the most",
                    "beautiful man in all",
                    "of Payon and maybe",
                    "even the world?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hagin",
                args![
                    "He left home to become stronger",
                    "when he was just a boy. I had no idea he would come back as such ",
                    "a fine specimen of a man..."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("I'd like to meet this Daewoon.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as("Hagin", args!["Oh, I see~"])?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.lines(args![
                    "You plan to ask him",
                    "to show you some fighting",
                    "pointers, do you? You men",
                    "are all the same: not as",
                    "cool or tough as Daewoon!"
                ])?;
            } else {
                ctx.lines(args![
                    "Interested, are you?",
                    "Well, you'll have to get",
                    "in line. So many women",
                    "and only one Daewoon...",
                    "^333333*Sigh*^000000"
                ])?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Hagin",
                args![
                    "Anyway, Daewoon",
                    "is staying in the annex",
                    "to the Payon Castle. If you",
                    "want to see him for yourself,",
                    "why don't you go there?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Hagin",
                args![
                    "Freaking you out?",
                    "Excuuuuse me~!",
                    "I'll have you know that",
                    "a flower's life is brief.",
                    "I better snag Daewoon",
                    "while I still can~"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn hagin_sign(ctx: &Ctx) -> Script {
    hagin_sign_body(ctx, Vec::new()).map(|_| ())
}

fn maid_a1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_a1(ctx: &Ctx) -> Script {
    maid_a1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_a2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Maid",
        args!["Welcome to the", "Payon Castle annex", "where Master Daewoon", "is staying."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_a2(ctx: &Ctx) -> Script {
    maid_a2_body(ctx, Vec::new()).map(|_| ())
}

fn maid_b1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_b1(ctx: &Ctx) -> Script {
    maid_b1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_b2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Maid", args!["Welcome.", "Are you another", "visitor for Master", "Daewoon?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_b2(ctx: &Ctx) -> Script {
    maid_b2_body(ctx, Vec::new()).map(|_| ())
}

fn maid_c1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_c1(ctx: &Ctx) -> Script {
    maid_c1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_c2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Maid",
        args![
            "Greetings.",
            "Welcome to the",
            "Annex to Payon Castle.",
            "Have you come here to",
            "see Master Daewoon?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_c2(ctx: &Ctx) -> Script {
    maid_c2_body(ctx, Vec::new()).map(|_| ())
}

fn maid_d1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_d1(ctx: &Ctx) -> Script {
    maid_d1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_d2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Maid",
        args![
            "Welcome to the",
            "Payon Castle Annex.",
            "Master Daewoon is",
            "currently lodging here.",
            "Have you come to see him?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_d2(ctx: &Ctx) -> Script {
    maid_d2_body(ctx, Vec::new()).map(|_| ())
}

fn maid_e1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_e1(ctx: &Ctx) -> Script {
    maid_e1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_e2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Chamberlain", args!["Please refrain", "from attacking", "the servants."])?;
    ctx.next()?;
    ctx.lines_as(
        "Chamberlain",
        args![
            "Although you may recognize",
            "us as monsters, we have",
            "all been properly hired and",
            "faithfully serve our master,",
            "Daewoon. So there's no",
            "need to feel uneasy."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_e2(ctx: &Ctx) -> Script {
    maid_e2_body(ctx, Vec::new()).map(|_| ())
}

fn maid_f1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maid_f1(ctx: &Ctx) -> Script {
    maid_f1_body(ctx, Vec::new()).map(|_| ())
}

fn maid_f2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Chamberlain",
        args![
            "Master Daewoon~!",
            "Please try this too!",
            "It's Tiger Foot Skin soup,",
            "boiled with nine kinds of",
            "exotic, potent herbs."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chamberlain",
        args![
            "It's famous for easing",
            "sores and fatigure, preventing",
            "cold and ^333333*Ahem*^000000 enhancing male vigor. Now, open wide, master~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maid_f2(ctx: &Ctx) -> Script {
    maid_f2_body(ctx, Vec::new()).map(|_| ())
}

fn daewoon_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pay_point = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.lines_as(
        "Daewoon",
        args![
            "Mwah ah hah!",
            "Feasting and merriment,",
            "wine, women and song!",
            "I could ask for nothing more!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Daewoon",
        args![
            "The most scrumptious",
            "delicacies are all mine to",
            "taste! And I'm not just talking about the food. Mwah ah hah!",
            "I'm the king of the world!"
        ],
    )?;
    ctx.next()?;
    if ctx.var("sign_q").get()? == 14 {
        ctx.lines_as(
            "Daewoon",
            args!["Oh, a visitor?", "I'm sorry, but I believe", "you're an uninvited guest~"],
        )?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
            ctx.lines(args![
                "You'll have to forgive me",
                "if I wish to spend my time with maidens as opposed to men."
            ])?;
        } else {
            ctx.lines(args![
                "No matter, a beauty is a beauty, whether or not I know her name.",
                "Come and drink with me~"
            ])?;
        }
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(7177)])?.number()? > 0 {
            l_pay_point = Val::from(1);
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Sure...", "But first, would you", "take a look at this?"],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou carefully take",
                "out the small, lucid",
                "jewel that Arian gave",
                "you and reveal it to",
                "Daewoon's roving eyes.^000000"
            ])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "Eh? Why that's...",
                    "I see now, Arian must",
                    "have sent you! Oh you",
                    "should have said so",
                    "at the very beginning~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Daewoon", args!["My apologies~", "Lately I've been attracting", "all sorts of strange attention like some kind of teen celebrity. I've gotten used to being too careful in screening out the dangerous sort."])?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "Now then.",
                    "Do you have any",
                    "idea what that jewel",
                    "you're holding actually is?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Kind of?:How the hell would I know?:No, but would you tell me?")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Oh...!",
                            "You must be much",
                            "smarter than I expected.",
                            "So tell me, what do you",
                            "understand about this jewel?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("It's handy.:I actually don't know...")])? {
                        1 => {
                            l_pay_point = (l_pay_point.clone() + Val::from(2));
                            ctx.mes("[Daewoon]")?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                            ctx.lines(args![
                                "Mmm...?",
                                "Handy? If you truly",
                                "understood, I don't",
                                "think you'd describe this",
                                "jewel as merely 'handy.'"
                            ])?;
                            ctx.next()?;
                        }
                        2 => {
                            l_pay_point = (l_pay_point.clone() + Val::from(4));
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                            ctx.lines_as("Daewoon", args!["Really?", "Mm. At least you", "admit it. *Sigh*"])?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.mes("[Daewoon]")?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines(args![
                        "...",
                        "A rather crass way",
                        "of speaking, but perhaps",
                        "you picked it up from Arian.",
                        "In any case, let me explain."
                    ])?;
                    ctx.next()?;
                }
                3 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Didn't Arian tell you",
                            "anything? I appreciate",
                            "your honesty. And I don't",
                            "mind chatting a while, I much",
                            "prefer speaking to honest people rather than foolish know-it-alls."
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Daewoon",
                args![
                    "That ^31009CSobbing Starlight^000000",
                    "is no mere jewel. It is a key",
                    "item for unlocking some incredible power. I believe Metz happened to obtain a piece, though I am",
                    "unsure how..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "Metz has asked me and",
                    "some other friends who",
                    "hold the fragments of the",
                    "Sobbing Starlight to entrust",
                    "them to someone worthy of",
                    "finding the power it leads to."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "Of course, we all agreed",
                    "and now you're here for me",
                    "to judge whether or not you're",
                    "qualified for this task. Now,",
                    "are you ready for my test?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "I'M READY! YEEAH!:I'll do my best!:To hell with this!:Fine. Let's get it over with.",
                )],
            )? {
                1 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.lines_as(
                        "Daewoon",
                        args!["Ooh~", "Such unbridled", "enthusiasm usually", "does more good than harm."],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(7));
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_OK")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Mwah ah hah!",
                            "That's exactly what",
                            "I wanted to hear! That",
                            "kind of quiet and careful",
                            "confidence will help you",
                            "in the future, you'll see~"
                        ],
                    )?;
                    ctx.next()?;
                }
                3 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    l_pay_point = (l_pay_point.clone() + Val::from(2));
                    ctx.lines_as("Daewoon", args!["E...", "Excuse me?"])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "How can you be",
                            "so ridiculous at",
                            "at time like this?",
                            "Either mind your manners",
                            "or don't take this test at",
                            "all! ^333333*SLAP--!!*^000000"
                        ],
                    )?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                    ctx.next()?;
                }
                4 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(4));
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as("Daewoon", args!["Hmm. That kind of half-hearted", "attitude won't get you very far on your adventures. Still, so long as you don't get overly negative, you should have a decent chance of surviving your challenges."])?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Daewoon",
                args![
                    "Now, Metz expects me to ask",
                    "some rather serious questions,",
                    "but that really isn't my style. For this test, why don't we just talk?",
                    "Just answer me honestly and",
                    "light heartedly, alright?"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SCRATCH")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args!["So...", "Do you have a lot", "of friends that you can", "constantly party with?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes, I do.:I prefer soloing.:I am lonely.")])? {
                1 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Oh, that's great~!",
                            "To have many friends",
                            "is a priceless blessing.",
                            "Friends bring us joy and",
                            "aid when we find ourselves",
                            "suffering from difficulties."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Wouldn't you",
                            "agree that being",
                            "really close friends",
                            "with someone can be",
                            "a life long benefit?"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(4));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Well, everybody",
                            "needs to be alone once",
                            "in a while. And there are",
                            "some battles you must",
                            "fight all on your own."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Also, if you never",
                            "deal with other people,",
                            "you may grow selfish or",
                            "needy. It's better to go",
                            "out and meet people,",
                            "don't you think?"
                        ],
                    )?;
                    ctx.next()?;
                }
                3 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(3));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "You don't have any",
                            "friends at all? Well,",
                            "you better learn how",
                            "to get along with others",
                            "as soon as you can...!"
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Daewoon",
                args![
                    "Now, what's your opinion",
                    "on purposely getting lots",
                    "of monsters to follow you",
                    "around. I believe this is",
                    "called ''Mob Training...''"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Awesome~!:I hate people who do that.:I do it sometimes...")])? {
                1 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "But...",
                            "Wouldn't that be really",
                            "rude to anyone else hunting",
                            "on that same map? I think",
                            "it would even interfere with",
                            "someone else's gameplay..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Perhaps you should try",
                            "to consider other people's",
                            "feelings. Mob Training seems",
                            "to only be good at getting other",
                            "people angry with you..."
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.lines_as("Daewoon", args!["Really?", "I do too!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Just the other day, I've",
                            "heard some ruffians boasting",
                            "of their mob training activities. But personally, I feel they were compensating for their own shortcomings. "
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Such behavior truly",
                            "bothers me. Although I have",
                            "spent years in developing an",
                            "unflappable personality, I find",
                            "myself irked when encountering",
                            "such troublemakers."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                    ctx.lines_as("Daewoon", args!["When you face obstacles", "in your own training, never give in to weakness. Assert yourself and find the determination to overcome your tribulations with honor!"])?;
                    ctx.next()?;
                }
                3 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(3));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "At least you're honest.",
                            "But let me say that I cannot",
                            "condone that sort of weak willed behavior. True strength can only",
                            "be found through honor.",
                            "Remember that."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Daewoon", args!["In the face of overwhelming", "odds, do not despair. After all, what is achievement if it is not earned without difficulty? The greater the challenge, the", "greater the glory."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Even if you are having hard time,",
                            "try to take a firm stand.",
                            "One day, you will realise how strong you have become."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Daewoon", args!["And...", "Don't ever partake in", "mob training again, okay?"])?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Daewoon",
                args![
                    "Anyway, I'm sure you're",
                    "aware of the War of Emperium",
                    "in which mighty guilds all across Midgard battle for guild castle dominion. It's actually quite popular, really."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Daewoon", args!["Now imagine that both of us", "are in the midst of a heated guild war. The sounds of explosions and earth shaking magic spells are all we can hear. At any time, we can", "be lost in that mindless chaos."])?;
            ctx.next()?;
            ctx.lines_as("Daewoon", args!["Finally, through incredible", "good fortune, we manage to", "infiltrate the enemy guild castle and reach their Emperium. If we destroy that Emperium, that castle will belong to our guild."])?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "However...! This is",
                    "no ordinary Emperium!",
                    "It is a masterful sculpture",
                    "of a gorgeous Priestess!",
                    "Answer me, adventurer!",
                    "Would you still destroy it?!"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Destroy it!:I can't destroy such beauty...:I'd close my eyes, then destroy it.",
                )],
            )? {
                1 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(3));
                    ctx.lines_as("Daewoon", args!["Hmm. You have a truly", "strong will. Then again,"])?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines(args![
                            "perhaps you lack a true",
                            "appreciation for beauty.",
                            "Even in Emperium form, how",
                            "could you harm a Priestess?"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "perhaps you cannot appreciate",
                            "a Priestess's beauty in the",
                            "way that a man would."
                        ])?;
                    }
                }
                2 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.lines_as("Daewoon", args!["Ah yes! I felt you would answer that way! We must cherish and protect what is beautiful in this world. I would never be able to harm a Priestess, even in", "statue form..."])?;
                }
                3 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(6));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Interesting...!",
                            "Although you cherish",
                            "the beauty of the Priestess,",
                            "your loyalty to your guild",
                            "proves stronger. A most",
                            "admirable attitude!"
                        ],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "But yes, if it were",
                    "me, I would protect that",
                    "Priestess shaped Emperium",
                    "until the end of the guild war.",
                    "Mwah ah hah~!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "While we're on the topic",
                    "of guilds, let me present",
                    "another guild related scenario. Let's say that you joined a very popular guild with many allies,",
                    "as well as formidable enemies."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Daewoon", args!["Now, during one of your.", "hunts, you happen to meet", "a member of one of your enemy guilds. The two of you are the only people on that map. Suddenly, he finds himself in mortal danger!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "This enemy guild member",
                    "begins to yell for help. Now,",
                    "would you give your enemy",
                    "the help that he needs?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Yes, of course!:I'd pretend not to hear anything.:I'd make fun of him, then run off.",
                )],
            )? {
                1 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(6));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Ah, you would help him!",
                            "Good, good. I'm glad to see",
                            "that you understand that such pettiness should not get in the way of doing what is good and right."
                        ],
                    )?;
                }
                2 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(5));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "True, you're not really",
                            "obligated to help your",
                            "enemy. Besides, you may",
                            "hurt his feelings once he",
                            "realizes that he's had to",
                            "depend on his rival for help."
                        ],
                    )?;
                }
                3 => {
                    l_pay_point = (l_pay_point.clone() + Val::from(4));
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Even though he is",
                            "your enemy, I still",
                            "believe it's important",
                            "that you treat him with",
                            "respect. Where is your honor?"
                        ],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as(
                "Daewoon",
                args![
                    "Well, there's one",
                    "last thing I want to",
                    "know about you. It's the",
                    "most important question",
                    "in the world once you",
                    "think about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Daewoon", args!["...", "......", "Do you enjoy life?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                1 => {
                    ctx.mes("[Daewoon]")?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.lines(args![
                        "I'm glad.",
                        "When you don't live",
                        "with zeal, it's easy to",
                        "forget your goals and your",
                        "purpose for living. Don't have",
                        "any? Then make some up."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "I hope you always enjoy",
                            "life as much as you can.",
                            "As for me, I'm always happy",
                            "with my wine, women and song~",
                            "Mwah ah hah~!"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "I understand.",
                            "People cannot be happy",
                            "all the time. Sometimes",
                            "it's easy to forget your",
                            "goals and purpose in life."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Daewoon", args!["But you know what? If you", "have just one dream, one goal", "to strive towards, things shall get better. If you think you don't have dreams, look back to your past. What you remember may surprise you."])?;
                    ctx.next()?;
                    ctx.lines_as("Daewoon", args!["It's important to look forward,", "but first you must find what is truly precious to you before you can define your happiness. That's why I think it's good to experience new things as well as reflect."])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Now, what's precious to me?",
                            "Three things, actually...",
                            "Wine. Women. And song!",
                            "Though, if I had to rank them,",
                            "women would top that list.",
                            "Mwah ah hah~!"
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            if l_pay_point.clone().number()? > 29 {
                ctx.var("sign_q").set(Val::from(15))?;
                ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                {
                    if ctx.var("BaseLevel").get()?.number()? < 60 {
                        ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                        ctx.call(Function::GetExperience, vec![Val::from(7500), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                        ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                    } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                        ctx.call(Function::GetExperience, vec![Val::from(13000), Val::from(0)])?;
                    } else {
                        ctx.call(Function::GetExperience, vec![Val::from(18000), Val::from(0)])?;
                    }
                }
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.lines_as(
                    "Daewoon",
                    args![
                        "Well, I must say that I've",
                        "grown quite fond of you. Of",
                        "course, it helps that we have so much in common. Mwah ah hah~!",
                        "It was an honor to meet you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Daewoon",
                    args![
                        "Oh yes! Your next test",
                        "examiner is ^CE0000Sir Jore^000000, also",
                        "known as the Ghost of Al de Baran. Although he's always sick, he has great passion for his research."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Daewoon", args!["Alright, then.", "Good luck in", "Al de Baran~!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if l_pay_point.clone().number()? < 20 {
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "It pains me to say this,",
                            "but I do not think you're",
                            "worthy of holding my fragment",
                            "of the Sobbing Starlight. But",
                            "I am willing to give you",
                            "another chance~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (l_pay_point.clone().number()? > 26 && l_pay_point.clone().number()? < 30) {
                    ctx.var("sign_q").set(Val::from(15))?;
                    ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                    {
                        if ctx.var("BaseLevel").get()?.number()? < 60 {
                            ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                            ctx.call(Function::GetExperience, vec![Val::from(7500), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                            ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                        } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                            ctx.call(Function::GetExperience, vec![Val::from(13000), Val::from(0)])?;
                        } else {
                            ctx.call(Function::GetExperience, vec![Val::from(18000), Val::from(0)])?;
                        }
                    }
                    ctx.lines_as("Daewoon", args!["You know, after talking with", "you for a while, I now feel fairly comfortable with leaving you this piece of the Sobbing Starlight. Somehow, I think you're strong enough to get all the pieces."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args!["I hope you will pass the rest of the test", "and acquire the power."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "Oh yes! Your next test",
                            "examiner is ^CE0000Sir Jore^000000, also",
                            "known as the Ghost of Al de Baran. Although he's always sick, he has great passion for his research."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Daewoon", args!["Alright, then.", "Good luck in", "Al de Baran~!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Daewoon",
                        args![
                            "It pains me to say this,",
                            "but I do not think you're",
                            "worthy of holding my fragment",
                            "of the Sobbing Starlight. But",
                            "I am willing to give you",
                            "another chance~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Daewoon",
                args![
                    "I wonder...",
                    "Someone should have",
                    "passed Arian's test by now.",
                    "I can't wait to see who was",
                    "actually able to impress him..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("sign_q").get()?.number()? < 14 {
        ctx.lines_as(
            "Daewoon",
            args![
                "Mwah ah hah~!",
                "I couldn't be happier!",
                "What more do I need?",
                "Gourmet food, fine wine,",
                "nubile women...!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 98 {
        ctx.lines_as(
            "Daewoon",
            args![
                "I'm sorry, but ",
                "no price can make",
                "me change my mind.",
                "You... You've failed.",
                "I'm sorry, friend."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 99 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.lines_as(
            "Daewoon",
            args![
                "Oh~! It's you!",
                "It has been a while since",
                "we last conversed. Now that",
                "you've accessed the power",
                "behind the Sobbing Starlight,",
                "do you like it?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
            1 => {
                ctx.lines_as(
                    "Daewoon",
                    args![
                        "Ah, how great~!",
                        "Just remember that",
                        "power can be addictive.",
                        "Even if you don't agree,",
                        "it's still wise to avoid",
                        "greedy tendencies, yes?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Daewoon",
                    args![
                        "Please make good use of",
                        "the power you have obtained.",
                        "I am counting on you to be",
                        "responsbile and to never lose",
                        "that smile on your face, okay?",
                        "Mwah ah hah~!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SCRATCH")?])?;
                ctx.lines_as(
                    "Daewoon",
                    args![
                        "Eh...?",
                        "That's quite a surprise.",
                        "Perhaps Metz didn't know",
                        "enough about it? Still, I'm",
                        "sorry to hear you don't like it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Daewoon", args!["But in the end, keep in mind", "that you must have learned some great lesson from this experience. Anyway, please do your best to use the power responsibly, okay?", "Mwah ah hah~!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()? == 15 {
        ctx.lines_as(
            "Daewoon",
            args![
                "Hm...?",
                "Weren't you on",
                "your way to find",
                "^CE0000Sir Jore^000000 in Al de Baran?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Daewoon",
            args![
                "Oh, having trouble",
                "finding him, are you?",
                "Well, he's fairly shy, but",
                "I'm sure he's hiding some",
                "place in that town."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Daewoon",
            args![
                "Ah, I feel so...",
                "Satiated. The food was ",
                "stupendous and the women",
                "have been even more so.",
                "Servants! Fan me please~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Daewoon", args!["^333333*Yawn...*", "...z...z...Z....ZZzZZ^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn daewoon_sign(ctx: &Ctx) -> Script {
    daewoon_sign_body(ctx, Vec::new()).map(|_| ())
}

fn monograph_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Sir Jore",
        args!["Uwah~!", "Stop reading my", "research monograph!", "Put that back now!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn monograph_sign(ctx: &Ctx) -> Script {
    monograph_sign_body(ctx, Vec::new()).map(|_| ())
}
