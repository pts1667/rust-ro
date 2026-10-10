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

pub fn alreg_nif(ctx: &Ctx) -> Script {
    if ctx.var("nif_piano").get()?.is_true() {
        ctx.var("nif_piano").set(Val::from(0))?;
    }
    ctx.lines_as("Alreg", args!["Me like meat, muheh.", "You! Look tempting."])?;
    ctx.next()?;
    ctx.lines_as("Alreg", args!["Me want to taste you."])?;
    ctx.next()?;
    ctx.mes("^FF3355*Crunch* Crunch*^000000")?;
    if ctx.var("nif_esc").get()? == 0 && (ctx.var("misc_quest").get()?.number()? & 32) == 0 {
        ctx.call(Function::PercentHeal, args![-60, 0])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["NooOOoOOoOoO~!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Alreg",
            args![
                "It long time since I had meat",
                "so... so good. You taste",
                "good. Alreg give you this",
                "for payback. Present!",
                "*Burrrp*"
            ],
        )?;
        ctx.var("nif_esc").set(ctx.call(Function::Rand, args![1, 2])?)?;
        ctx.items().give(7184, 1)?;
        return ctx.close();
    }
    ctx.call(Function::PercentHeal, args![-30, 0])?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["Ow~! What are you", "trying to do, kill me?!"])?;
    ctx.next()?;
    ctx.lines_as("Alreg", args!["Hm? Oh, no no no.", "Me am trying to eat you."])?;
    ctx.close()
}

pub fn crayu_nif(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Crayu",
        args![
            "Living one...",
            "Listen to this song...",
            "It's been sung by many for a",
            "long time, but nobody knows",
            "when it was made or who wrote it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Crayu",
        args![
            "^FF00001. Mountain sunset to the west",
            "^FF00002. Where the purple dusk falls ",
            "^FF00003. Surrounded by beautiful melody",
            "^FF00004. You become the key that ignores its master^000000"
        ],
    )?;
    ctx.next()?;
    if ctx.var("nif_esc").get()? == 1 || ctx.var("nif_esc").get()? == 4 {
        ctx.lines_as("Crayu", args!["Okay, wait! Here's a test for you!"])?;
        let songline = match ctx.rand_range(1, 4)? {
            1 => {
                ctx.mes("Write down the first line of the song.")?;
                "Mountain sunset to the west"
            }
            2 => {
                ctx.mes("Write down the second line of the song.")?;
                "Where the purple dusk falls"
            }
            3 => {
                ctx.mes("Write down the third line of the song.")?;
                "Surrounded by beautiful melody"
            }
            4 => {
                ctx.mes("Write down the fourth line of the song.")?;
                "You become the key that ignores its master"
            }
            _ => "",
        };
        ctx.next()?;
        let (input, _) = runtime::input_text(ctx, None, None)?;
        if input.loosely_equals(&Val::from(songline)) {
            ctx.lines_as(
                "Crayu",
                args![
                    "Excellent...!",
                    "So you've been listening~!",
                    "That makes you one of my",
                    "favorite guests. Here's a",
                    "little reward for you."
                ],
            )?;
            ctx.next()?;
            if ctx.var("nif_esc").get()? == 1 {
                ctx.var("nif_esc").set(Val::from(3))?;
            } else {
                ctx.var("nif_esc").set(Val::from(6))?;
            }
            ctx.items().give(7184, 1)?;
            ctx.lines_as(
                "Crayu",
                args![
                    "Lastly, I hope you will show your",
                    "respect to other Bards, just as",
                    "you have done for me."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Crayu",
            args![
                "*Sigh*... If you had only",
                "listened to me, it would",
                "have been easy to answer.",
                "Try to remember the line",
                "and try again!"
            ],
        )?;
        return ctx.close();
    }
    ctx.next()?;
    ctx.lines_as(
        "Crayu",
        args![
            "I wonder what the lyrics mean...",
            "They seem quite profound...",
            "I have the feeling that this song",
            "carries some secret."
        ],
    )?;
    ctx.close()
}

pub fn kuzkahina_nif(ctx: &Ctx) -> Script {
    if ctx.var("nif_esc").get()? == 2 || ctx.var("nif_esc").get()? == 3 {
        ctx.lines_as(
            "Kuzkahina",
            args![
                "I don't understand",
                "why I can't make any",
                "money with this store...",
                "Even when I was alive",
                "I could never make any",
                "money with my businesses..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kuzkahina",
            args!["Hey you! Make yourself useful", "and throw this away when you leave."],
        )?;
        ctx.var("nif_esc").set(ctx.var("nif_esc").get()?.number()? + 2)?;
        ctx.items().give(7184, 1)?;
        ctx.next()?;
        ctx.lines_as(
            "Kuzkahina",
            args![
                "Gosh, what's with this store?",
                "How did I end up hiring such useless employees...?",
                "*mumble mumble*",
                "*mumble mumble*"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Kuzkahina",
        args![
            "I don't understand",
            "why I can't make any",
            "money with this store...",
            "Even when I was alive",
            "I could never make any",
            "money with my businesses..."
        ],
    )?;
    ctx.close()
}

pub fn graveyard1(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn graveyard1_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("nif_esc").get()? == 5 || ctx.var("nif_esc").get()? == 7 || ctx.var("nif_esc").get()? == 8 || ctx.var("nif_esc").get()? == 10
    {
        ctx.lines(args![
            "^3355FFYou found something",
            "half-buried near a grave.",
            "It looks very well-shaped.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FFWould you like to pick it up?^000000")?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.var("nif_esc").get()?.number()? < 10 {
                ctx.var("nif_esc").set(ctx.var("nif_esc").get()?.number()? + 2)?;
            } else if ctx.var("nif_esc").get()? == 10 {
                ctx.var("nif_esc").set(11)?;
            }
            ctx.items().give(7184, 1)?;
        }
        return ctx.close();
    }
    ctx.end()
}

pub fn graveyard2(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn graveyard2_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("nif_esc").get()? == 6 || ctx.var("nif_esc").get()? == 9 {
        ctx.lines(args![
            "^3355FFYou found something",
            "half-buried near a grave.",
            "It looks very well-shaped.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^3355FFWould you like to pick it up?^000000")?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            ctx.var("nif_esc").set(ctx.var("nif_esc").get()?.number()? + 2)?;
            ctx.items().give(7184, 1)?;
        }
        return ctx.close();
    }
    ctx.end()
}

pub fn piano(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn piano_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("nif_esc").get()? == 0 && (ctx.var("misc_quest").get()?.number()? & 32) == 0 {
        ctx.lines(args![
            "^3355FFYou see a big, heavy piano.",
            "You get the feeling that",
            "its music would sound magnificient.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAs you examine the piano,",
            "you find that 7 keys",
            "are missing in the",
            "middle of the keyboard.",
            "If you had all the keys,",
            "you could play music...^000000"
        ])?;
        return ctx.close();
    }
    if ctx.var("nif_esc").get()? == 11 && ctx.items().count(7184)? > 5 {
        ctx.var("nif_esc").set(Val::from(12))?;
        ctx.items().take(7184, 6)?;
        ctx.lines(args![
            "^3355FFYou took the keys you've found",
            "and inserted them into",
            "each empty space.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFBut then you realize",
            "that you're missing the last",
            "key for the space on the far",
            "left of the keyboard...^000000"
        ])?;
        return ctx.close();
    }
    ctx.end()
}

pub fn piano3(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn piano3_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("nif_esc").get()? == 12 || ctx.var("nif_esc").get()? == 13 || (ctx.var("misc_quest").get()?.number()? & 32) != 0 {
        ctx.lines(args![
            "^3355FFAs you bend towards",
            "the final empty key space",
            "to take a better look at it...^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::SoundEffect, args!["complete.wav", 0])?;
        ctx.next()?;
        if ctx.var("nif_esc").get()? == 12 {
            ctx.var("misc_quest").set(ctx.var("misc_quest").get()?.number()? | 32)?;
            ctx.var("nif_piano").set(Val::from(0))?;
        }
        ctx.lines(args![
            "^3355FFYou begin to feel dizzy and",
            "your body feels as if it were",
            "floating in air. Then, your",
            "vision starts to blur...^000000"
        ])?;
        ctx.close_window()?;
        ctx.warp("nif_in", 179, 163)?;
        return ctx.end();
    }
    ctx.end()
}

pub fn witch_nif(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Kirkena",
        args![
            "Huh? ...You're alive, aren't you?",
            "I'm not sure how more of you are",
            "able to get here and I don't",
            "know your reasons for coming,",
            "but this place is dangerous for",
            "the living."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kirkena",
        args![
            "I am sending you back",
            "outside of this town.",
            "If you can help it, you",
            "better not come back to",
            "this place."
        ],
    )?;
    ctx.close_window()?;
    ctx.warp("umbala", 138, 208)?;
    ctx.end()
}

pub fn erious_nif(ctx: &Ctx) -> Script {
    if (ctx.var("misc_quest").get()?.number()? & 16384) != 0 {
        ctx.lines_as("Erious", args!["I wish you safety from harm", "in your journeys, adventurer."])?;
        return ctx.close();
    }
    if ctx.var("nif_revive").get()? == 0 {
        ctx.lines_as(
            "Erious",
            args!["*Sob*Sob*", "*Cries* Wahhh...!", "Elly, how could you", "leave me this way..."],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_CRY])?;
        ctx.next()?;
        if ctx.menu(&["Comfort him.", "Neglect him."])? == 0 {
            ctx.lines_as(
                "Erious",
                args![
                    "Ah, thank you for your kindness.",
                    "*Sob* My wife died while I was on",
                    "a trip. I'm so sorry that I",
                    "couldn't be with her when",
                    "the end came..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Erious",
                args![
                    "I wish I could talk to my Elly,",
                    "although I know it's too late now.",
                    "*Cries* But... there is one last",
                    "hope that I have. My absolute",
                    "final chance..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Erious",
                args![
                    "I was told that somewhere",
                    "in this world, there exists a town",
                    "where the dead reside. I might be",
                    "able to meet my wife there, but",
                    "I cannot leave my children here",
                    "without me. *Cries*"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["I can deliver your message to her.", "I am so sorry to hear that."])? == 0 {
                ctx.lines_as(
                    "Erious",
                    args![
                        "Oh, can you?",
                        "Thank you so much for your",
                        "generosity. If you can do",
                        "this for me, I would be",
                        "greatly indebted to you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "You're an adventurer, aren't you?",
                        "If your journeys take you to that",
                        "town of the dead, and if you",
                        "happen to meet her by chance..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "Please send Elly a message for me,",
                        "let her know that what happened",
                        "a month ago was all my fault.",
                        "Thank you for your help,",
                        "and please take care."
                    ],
                )?;
                ctx.var("nif_revive").set(Val::from(1))?;
                ctx.quests().start(11038)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Erious",
                args![
                    "*Cries* Is there no way that I",
                    "can see her again? If I could",
                    "talk to her at least once more,",
                    "even if it's the last time,",
                    "I would do anything..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Erious",
            args![
                "*Cries* Eeeeelllllyyyy~~~~!",
                "I never thought that would be",
                "the last time we would see",
                "each other...",
                "*Sob*..."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("nif_revive").get()? == 1 {
        ctx.lines_as(
            "Erious",
            args![
                "This place I was told about...",
                "The town of the dead...",
                "From what I remember, it was",
                "located near some rest area."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Erious",
            args![
                "All day and night, it's filled",
                "with never ending screams of",
                "agony and despair. I hope that",
                "this will be helpful in your search."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("nif_revive").get()? == 5 || ctx.var("nif_revive").get()? == 6 {
        ctx.lines_as(
            "Erious",
            args![
                "Oh my! You really found the town,",
                "have you? I really appreciate that",
                "you've endured all this hardship",
                "for me... Um...So in the end...",
                "Were you able to meet my Elly?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes.", "Sorry, I couldn't."])? == 0 {
            if ctx.var("nif_revive").get()? == 6 && ctx.items().count(934)? > 0 {
                ctx.lines_as(
                    "Erious",
                    args!["Are you serious? What did she say?", "Did she say she will forgive me?"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Yes. And she gave me...this.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.items().take(934, 1)?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "Ah...In this box were the gifts",
                        "I gave her when we started",
                        "seeing each other. She had a",
                        "serious illness even before we",
                        "met, so I promised her that I",
                        "would cure her illness..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "That's why I went on that trip...",
                        "to find a cure or medicine",
                        "for her... But my efforts turned",
                        "out to be a good for nothing.",
                        "...................",
                        "........."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "Anyway, I appreciate your",
                        "generousity. I want to pay",
                        "you back for all the trouble",
                        "I caused you, but I only have",
                        "the medicine I found for her",
                        "illness during this trip."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Erious",
                    args!["I hope you will take this medicine", "as a token of my gratitute."],
                )?;
                ctx.next()?;
                if ctx.call(Function::CheckWeight, args![610, 26])?.is_true() {
                    ctx.var("misc_quest").set(ctx.var("misc_quest").get()?.number()? | 16384)?;
                    ctx.var("nif_revive").set(Val::from(0))?;
                    ctx.quests().complete(11043)?;
                    ctx.items().give(607, 2)?;
                    ctx.items().give(608, 2)?;
                    ctx.items().give(610, 10)?;
                    ctx.lines_as(
                        "Erious",
                        args!["Thank you for everything.", "I wish you luck and safety", "in your adventures."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Erious",
                    args![
                        "I have something to give you,",
                        "but you're carrying too much",
                        "stuff for me to give it to",
                        "right now. Why don't you",
                        "store your items somewhere?"
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("nif_revive").get()? == 6 && ctx.items().count(934)? < 1 {
                ctx.lines_as("Erious", args!["Hmm...this is odd. I'm sure she would give you something as a token of meeting her. By any chance, is there anything that you've forgetten to bring me?"])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Erious",
                args!["I guess you misunderstood", "something or you're trying", "to scam a man in grief."],
            )?;
            return ctx.close();
        }
        if ctx.var("nif_revive").get()? == 5 {
            ctx.lines_as(
                "Erious",
                args![
                    "I see...you have failed to find",
                    "her... However, I also understand",
                    "that fulfilling my request may",
                    "be impossible..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Erious",
                args![
                    "You've really gone out of your way",
                    "to help me, so I want to give you",
                    "something as a token of my",
                    "gratitute."
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::CheckWeight, args![610, 10])?.is_true() {
                ctx.var("misc_quest").set(ctx.var("misc_quest").get()?.number()? | 16384)?;
                ctx.var("nif_revive").set(Val::from(0))?;
                ctx.quests().complete(11042)?;
                ctx.items().give(610, 10)?;
                ctx.lines_as(
                    "Erious",
                    args![
                        "Thank you for everything.",
                        "I wish you luck and safety",
                        "from harm in your journeys."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Erious",
                args![
                    "I have something to give you,",
                    "but you're carrying too much",
                    "stuff for me to give it to",
                    "right now. Why don't you",
                    "store your items somewhere?"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Erious",
            args![
                "I see...",
                "...Wait! Doesn't that belong",
                "to my wife, Elly? Why are you",
                "trying to cheat a grieving man...?"
            ],
        )?;
        return ctx.close();
    }
    ctx.end()
}

pub fn billik(ctx: &Ctx) -> Script {
    if ctx.var("nif_revive").get()?.number()? > 1 || (ctx.var("misc_quest").get()?.number()? & 16384) != 0 {
        ctx.lines_as(
            "Billik",
            args![
                "Well, long time no see.",
                "If you have a chance in the",
                "future, let's meet in the",
                "town of the dead later.",
                "Hahahaha~"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_BEST])?;
        return ctx.close();
    }
    if ctx.var("nif_revive").get()? == 1 {
        ctx.lines_as(
            "Billik",
            args![
                "The Town of the dead...",
                "It's a very dangerous place.",
                "Why would you want to go there?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["I have to meet somebody there.", "It's a part of my adventure."])? == 0 {
            ctx.lines_as(
                "Billik",
                args![
                    "Someone you have to meet there...",
                    "Hmm... I suppose you wish to",
                    "contact the dead...",
                    "I can also see that you're",
                    "determined to go."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Billik",
                args![
                    "*Sigh* Okay, if you really want to",
                    "go, you must find a man named",
                    "'Feylin.' It's very important to",
                    "let him know that I introduced",
                    "him to you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Billik",
                args![
                    "He's not very trusting, but he",
                    "likes roses a lot, so bring a rose",
                    "for him. That way, he'll know for",
                    "sure that I've sent you."
                ],
            )?;
            ctx.var("nif_revive").set(Val::from(2))?;
            ctx.quests().change(11038, 11039)?;
            ctx.next()?;
            ctx.lines_as(
                "Billik",
                args![
                    "The town of the dead is located",
                    "north of this place. Remember, it",
                    "is very dangerous to go there,",
                    "even for well experienced",
                    "adventurers, so...",
                    "Be careful."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Billik",
            args![
                "Hahaha~ I guess you like adventure",
                "as much as I do. Although it's a",
                "very dangrous place, you will be",
                "paid back for your effort after",
                "you get there. But be careful."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("nif_revive").get()? == 0 {
        ctx.lines_as(
            "Billik",
            args!["What brings you here? I'm busy,", "leave if you don't have any business."],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn feylin(ctx: &Ctx) -> Script {
    if ctx.var("nif_revive").get()? == 2 {
        ctx.lines_as(
            "Feylin",
            args![
                format!(
                    "Poor {}...",
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        "guy"
                    } else {
                        "girl"
                    }
                ),
                "Dying at such a young age...",
                "I am always sorry to see the",
                "young pass away."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Feylin",
            args![
                "............??",
                "Oh, I see you're not dead yet,",
                "are you? It's unbelievable to",
                "see another living person",
                "in this town."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Feylin",
            args![
                "Are you coming to rescue someone,",
                "or do you have another purpose?",
                "If you came here to meet a",
                "deceased friend of yours, you've",
                "come to the wrong person."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Feylin", args!["Nobody in Niflheim can", "perform that kind of miracle..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Feylin",
            args![
                "I can tell you have been through",
                "much hardship, but unfortunately,",
                "you came here for nothing. I hope",
                "you go back to where you came",
                "from before it's too late."
            ],
        )?;
        ctx.var("nif_revive").set(Val::from(3))?;
        ctx.quests().change(11039, 11040)?;
        return ctx.close();
    } else if ctx.var("nif_revive").get()? == 3 {
        ctx.lines_as(
            "Feylin",
            args![
                "I've asked you to leave this",
                "town immediately. I'm sorry",
                "but I know nothing about what",
                "you are asking."
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_FRET])?;
        ctx.next()?;
        if ctx.menu(&["But...", "I am sorry."])? == 0 {
            ctx.lines_as(
                "Feylin",
                args![
                    "You must know something...",
                    "Otherwise, you wouldn't be",
                    "so stubborn. I am not sure",
                    "who told you contacting the",
                    "dead might be possible..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Feylin",
                args![
                    "But I surely wouldn't have told",
                    "you. Besides, speaking about",
                    "such things is taboo."
                ],
            )?;
            ctx.var("nif_revive").set(Val::from(4))?;
            ctx.quests().change(11040, 11041)?;
            return ctx.close();
        }
        ctx.lines_as(
            "Feylin",
            args![
                "Please consider leaving this",
                "place as soon as possible.",
                "Nobody should stay in",
                "Niflheim for too long."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("nif_revive").get()? == 4 {
        if ctx.items().count(748)? > 0 {
            ctx.lines_as(
                "Feylin",
                args![
                    "Huh, Billik must have sent you.",
                    "Since I'm indebted to him, I",
                    "suppose I will help you.",
                    "But let me warn you.",
                    "The price that I demand may be",
                    "more that you expect..."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["I'm willing to do this.", "I don't think I can do this."])? == 0 {
                if ctx.player().base_level()? > 79 {
                    ctx.lines_as(
                        "Feylin",
                        args![
                            "Alright, but you must promise",
                            "me that you will not let other",
                            "people know what I am about",
                            "to tell you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Feylin",
                        args![
                            "This sorcery requires great",
                            "sacrifice. Needless to say, there",
                            "is a chance that this spell may",
                            "fail. I must also have",
                            "10 Yggdrasil Leaf to cast this",
                            "spell. Do you wish to continue?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 0 {
                        if ctx.items().count(610)? > 9 {
                            ctx.lines_as(
                                "Feylin",
                                args![
                                    "I see. Then we shall begin.",
                                    "Woombahsokasabahah!",
                                    "Woombah woombah hoombabah!",
                                    "Yeeeeep!"
                                ],
                            )?;
                            ctx.next()?;
                            let roll = ctx.rand_range(1, 100)?;
                            if roll > 0 && roll < 88 {
                                ctx.lines_as(
                                    "Feylin",
                                    args![
                                        "...I am sorry.",
                                        "I have failed to summon",
                                        "who you wish to see. I am",
                                        "really sorry for this result."
                                    ],
                                )?;
                                ctx.call(Function::Emotion, args![constants::ET_HUK])?;
                                ctx.items().take(748, 1)?;
                                ctx.items().take(610, 10)?;
                                ctx.var("nif_revive").set(Val::from(5))?;
                                ctx.quests().change(11041, 11042)?;
                                ctx.call(Function::PercentHeal, args![-99, 0])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Feylin",
                                    args![
                                        "Unfortunately, it may be",
                                        "impossible to summon the",
                                        "spirit of the person with",
                                        "whom you wished to speak..."
                                    ],
                                )?;
                                return ctx.close();
                            } else if roll > 87 && roll < 101 {
                                ctx.items().take(748, 1)?;
                                ctx.items().take(610, 10)?;
                                ctx.call(Function::PercentHeal, args![-50, 0])?;
                                ctx.lines_as("Feylin", args!["..........", ".........."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args!["..........", ".....Umm...", "...Wasn't I just dead?", "Um, do I know you?"],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("I brought a message from your husband.")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Oh...I see...I'm sorry that I could not meet him before I died...",
                                        "So what is his message for me?"
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Forgive him for what happened a month ago.")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "*Sigh* He's such a nice husband...",
                                        "He pays such attention, even to",
                                        "the little things. Though, I",
                                        "should be the one apologizing",
                                        "to him..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Please give him this box, and tell",
                                        "him to forget about me. He should",
                                        "live the rest of his life happily",
                                        "with someone else. Oh, also let",
                                        "him know that I forgive him."
                                    ],
                                )?;
                                ctx.var("nif_revive").set(Val::from(6))?;
                                ctx.quests().change(11040, 11043)?;
                                ctx.items().give(934, 1)?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "I am glad to hear from my",
                                        "husband one more time. But I",
                                        "think I have to go now.",
                                        "Thank you for your trouble.",
                                        "Farewell..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elly", args![".....................", "...................."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Feylin",
                                    args![
                                        "I guess you have accomplished what",
                                        "you wanted. Well then, be careful",
                                        "when you go back..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                        } else {
                            ctx.lines_as(
                                "Feylin",
                                args![
                                    "If you don't have the",
                                    "items, we cannot proceed.",
                                    "Remember, I need 10 Yggdrasil",
                                    "Leaves to perform this spell.",
                                    "Please bring them as soon",
                                    "as possible."
                                ],
                            )?;
                            return ctx.close();
                        }
                    }
                    ctx.lines_as(
                        "Feylin",
                        args![
                            "I hope you will bring the items",
                            "first. But... I don't have the",
                            "confidence to promise that",
                            "this spell will cast successfully."
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Feylin",
                    args![
                        "Hmm... You don't seem to have",
                        "the strength to endure the",
                        "casting of this spell. At",
                        "your current strength, this",
                        "spell will kill you.",
                        "I cannot take that risk."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Feylin",
                args![
                    "You made a good decision.",
                    "It's not a good idea to perform",
                    "this kind of sorcery in the",
                    "first place because of the",
                    "risks involved..."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Feylin",
            args![
                "No matter how many times you ask",
                "me, I cannot help you. Please",
                "leave this place as soon as you can."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Feylin",
        args![
            "What makes you to come to such",
            "a dangerous town? Please leave",
            "this place as soon as you can."
        ],
    )?;
    ctx.close()
}
