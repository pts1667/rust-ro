use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn sick_old_man_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_einbech = Val::from(0);
    let mut l_input_s = Val::from("");
    if ctx.var("shinokas_quest").get()?.number()? < 2 {
        ctx.lines_as("Sick Old Man", args!["...!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Sick Old Man",
            args![
                "Awwwk~",
                "It's killing me!",
                "Arrrgh! Awwwrgh!",
                "W-when will my son",
                "come back from",
                "the factory...?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou feel really awkward",
            "just staring at this old",
            "man violently rolling",
            "around in his bed.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("shinokas_quest").get()? == 2 {
            ctx.lines_as("Sick Old Man", args!["...!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Sick Old Man",
                args![
                    "Awwwk~",
                    "It's killing me!",
                    "Arrrgh! Awwwrgh!",
                    "W-when will my son",
                    "come back from",
                    "the factory...?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Shi...", "Shinokas?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sick Old Man",
                args!["Huh...?", "Noooo! M-my name is", "Shinotarous. Y-you've", "got the wrong person!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "^333333This has to be the",
                    "Shinokas that Hikeman",
                    "was talking about in Einbech.",
                    "Hmmm, but how can I get",
                    "him to admit it?^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["^333333*Ahem!*^000000", "Excuse me..."],
            )?;
            ctx.next()?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    if l_einbech.clone().number()? > 6 {
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Have you ever heard of ^3131FFHikeman^000000 before?:Weren't you living in ^3131FFEinbech^000000?",
                            )],
                        )?) == 1
                        {
                            break 'l1;
                        }
                        ctx.lines_as("Sick Old Man", args!["Einbech...?", "No! I've never", "lived there before!"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFIt doesn't seem",
                            "like this old man",
                            "is telling the truth...^000000"
                        ])?;
                        l_einbech = Val::from(0);
                        ctx.next()?;
                    } else {
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Remember the mine tunnel collapse?:Didn't you used to be a miner?:Have you lived in Einbroch all your life?",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Sick Old Man",
                                    args!["Noooo!", "I don't know", "what the hell", "you're talking", "about!"],
                                )?;
                                l_einbech = (l_einbech.clone() + Val::from(1));
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Sick Old Man",
                                    args!["Nooo...!", "W-why would you", "even ask me that", "kind of question?!"],
                                )?;
                                l_einbech = (l_einbech.clone() + Val::from(1));
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Sick Old Man",
                                    args!["Y-yes!", "Born and raised", "raised here in", "Einbe--Einbroch!"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFIt doesn't seem",
                                    "like this old man",
                                    "is telling the truth...^000000"
                                ])?;
                                if l_einbech.clone().number()? > 0 {
                                    l_einbech = Val::from(0);
                                }
                                ctx.next()?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Are you sure", "that you don't", "know anything", "about ^3131FFHikeman^000000?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shinokas",
                args!["No...!", "I've never met", "Buender Hikeman", "in my entire li--"],
            )?;
            ctx.next()?;
            ctx.lines_as("Shinokas", args!["...", "......."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I was right.", "You're Shinokas!"],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_KIK")?,
                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shinokas",
                args!["Curses!", "I've blown", "my cover!", "W-wait! How much", "do you know?!"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Shinokas",
                args![
                    "Did ^3131FFthey^000000 send you?",
                    "^333333*Sigh*^000000 I think that this",
                    "is it. I'll never be",
                    "able to solve the",
                    "secret before I die."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shinokas", args!["Okay.", "Get on with it.", "I'm ready now..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Tell me everything",
                    "you know related to",
                    "that incident where",
                    "the mine tunnel",
                    "collapsed around",
                    "you and Hikeman."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shinokas", args!["...?", "Errr....", "You're not", "here to kill me?"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["No...!", "I'm here to find", "out the truth!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shinokas",
                args![
                    "That means...",
                    "I still have some time.",
                    "This must be destiny!",
                    "Alright, I'll tell you what",
                    "happened. But it's a",
                    "long story..."
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2072), Val::from(2073)])?;
            ctx.var("shinokas_quest").set(Val::from(3))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("shinokas_quest").get()? == 3 {
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "How much do you know",
                        "about the accident? No,",
                        "wait. Don't answer that.",
                        "I don't want to hear it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "I admit it. Yes.",
                        "I stabbed my friends",
                        "in the back. It was an",
                        "unforgivable sin that will",
                        "haunt me until the day I die."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "I sold out my friends for",
                        "money. I destroyed that",
                        "tunnel and killed them. But",
                        "I suppose I was tricked as well. ^3131FFThey^000000 never intended to keep their end of our agreement."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shinokas", args!["After I destroyed the tunnel,", "they targeted me and I ended", "spending the rest of my life being pursued and running from place to place. What the hell was that ^3131FFore^000000 and why was it so important?"])?;
                ctx.next()?;
                ctx.lines_as("Shinokas", args!["I need to know more about", "that ore if it's worth killing for. That's why I've risked sneaking into Einbroch. Supposedly, an ore similar to the one we found has", "been transported here recently."])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Did you find it?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "No, not yet.",
                        "I've been searching",
                        "for that ore every night.",
                        "During the day, this kind",
                        "blacksmith has managed",
                        "to hide me from those men."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "But I won't be safe",
                        "for very long. Look, I'm",
                        "no saint, but before I die,",
                        "I wanna do this one last",
                        "thing and see what's so",
                        "great about this ore..."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Wait, who's trying to get you?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "Who's trying to kill me?",
                        "The people who hired me and",
                        "my friends to dig up that ore in the first place. We thought they were ordinary businessmen,",
                        "but... They're dangerous."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "So I told you everything",
                        "and now you know that my",
                        "days might be numbered.",
                        "Please do an old man a favor",
                        "and search Einbroch for that",
                        "strange, mysterious ore."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shinokas", args!["Now, the first thing I learned", "in this town is that the richest family is the Kapellthaines. Only the rich and powerful can possibly be involved in something so big."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "Would you search",
                        "Kappellthaine Manor",
                        "for that ore? It's on the way",
                        "to the Airport and it shouldn't be hard to miss. They're the richest people in Einbroch, after all."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Shinokas", args!["I'm too old and weak to leave", "this house, and there's the chance that those men will find me. If you can sympathize with my situation, please find out if the Kapellthaine family has any unique ores..."])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2073), Val::from(2074)])?;
                ctx.var("shinokas_quest").set(Val::from(4))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("shinokas_quest").get()? == 4 {
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "Please...",
                        "Find out if the Kapellthaines",
                        "are keeping some kind of unique",
                        "ore. Their manor is on the road",
                        "that leads to the Airport."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("shinokas_quest").get()? == 5 {
                ctx.lines_as("Shinokas", args!["So, did you find", "anything from the", "Kapellthaines?"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, Mr. Kapellthaine",
                        "is kind of violent, but",
                        "I didn't find anything",
                        "really suspicious."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args!["Huh...", "I must have", "been wrong, then...", "Where else could it be?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "Oh, right! The huge",
                        "factory in Einbroch!",
                        "What could be more",
                        "suspicious? There's a ton",
                        "of workers, but no one really",
                        "knows what they do there..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shinokas",
                    args![
                        "If they're so secretive,",
                        "it's possible that they're",
                        "keeping the ore over there.",
                        "Please go and search the",
                        "Factory for that ore as",
                        "soon as you can."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2074), Val::from(2075)])?;
                ctx.var("shinokas_quest").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("shinokas_quest").get()? == 6 {
                if ctx.var("einfactory").get()?.number()? > 12 {
                    ctx.lines_as("Shinokas", args!["So...?", "Did you learn", "anything new", "in the Factory?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well...",
                            "There's a lot of",
                            "suspicious business",
                            "in the factory, but I don't",
                            "think any of it is related",
                            "to that ore you mentioned."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "Damn it...",
                            "Then where did",
                            "they hide it? Where",
                            "do you think that",
                            "ore might be?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Airport:Factory:Train Station:Airship Repairshop:Laboratory")])? {
                        1 => {
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "No...",
                                    "The Airport is always",
                                    "crowded with people.",
                                    "It'd be a bad idea to hide",
                                    "something so important",
                                    "in that kind of place."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "Didn't you just",
                                    "check the factory?",
                                    "You couldn't find",
                                    "any clues to the",
                                    "ore over there..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "Hmm...",
                                    "People are always going",
                                    "in and out of the Train Station. It's not the best place to hide something as important as the ore."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "The Airship? Hm, it does",
                                    "fly through some mysterious",
                                    "power... But I already checked",
                                    "the Airship Repairshop myself.",
                                    "I haven't found any trace of",
                                    "the ore over there."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        5 => {
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "...!",
                                    "Yes. Yes...!",
                                    "That could be it!",
                                    "Why didn't I think",
                                    "about the Laboratory?",
                                    "It makes so much sense!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "I have a good feeling",
                                    "about this. Please sneak",
                                    "into that Laboratory and",
                                    "see if you can find the ore!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Shinokas",
                                args![
                                    "Remember...",
                                    "Not just anybody",
                                    "can enter that kind",
                                    "of place. But I'm sure",
                                    "you'll figure something out."
                                ],
                            )?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(2075), Val::from(2076)])?;
                            ctx.var("shinokas_quest").set(Val::from(7))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "I'm really in no position",
                            "to ask and I'm not trying",
                            "to give you a hard time on",
                            "purpose, but would you please",
                            "check for any traces of the ore",
                            "in the Factory once again?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("shinokas_quest").get()? == 9 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Shinokas,", "I found something!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "You...",
                            "Came... back...",
                            "Even... if... it's",
                            "already... Too late.",
                            "^333333*Cough Cough*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou help Shinokas",
                        "sit up, but find that",
                        "your hands have been",
                        "stained with his blood.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Blood...?!", "Shinokas!", "Who did this to you?", "W-we need to call for help!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "No...",
                            "It's already",
                            "too late for me.",
                            "I should have died",
                            "a long time ago..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "^333333*Cough*^000000",
                            "I only regret that",
                            "I've never been able",
                            "to apologize to my",
                            "friends... ^333333*Cough!*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Shinokas",
                        args![
                            "But did you",
                            "find out? D-did",
                            "you find out what's",
                            "so special about",
                            "that ore? W-what",
                            "is it... Really?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["It was..."])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["It was", ((Val::from("^3131FF") + l_input_s.clone()) + Val::from("^000000!"))],
                    )?;
                    ctx.next()?;
                    if l_input_s.clone() == "Ymir" {
                        ctx.lines_as(
                            "Shinokas",
                            args![
                                "Y-Ymir...?",
                                "The h-heart of Ymir?",
                                "The power to control",
                                "the world... We really",
                                "did find something",
                                "great, didn't we?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shinokas",
                            args![
                                "I remember...",
                                "The first time",
                                "we saw it. We...",
                                "We were so excited.",
                                "So amazed and proud..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shinokas",
                            args![
                                "But digging up...",
                                "Something so great...",
                                "Was only possible",
                                "with-- ^333333*Cough*^000000 --",
                                "M-my buddies..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Shinokas",
                            args![
                                "^333333*Cough*^000000",
                                "It's getting...",
                                "Darker... I...",
                                "Don't have to",
                                "run anymore."
                            ],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as(
                            "Shinokas",
                            args!["W-wait...!", "What did...", "I can't underst--", "^333333*Cough cough!*^000000"],
                        )?;
                        ctx.next()?;
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "No...!",
                            "Tell me who",
                            "did this to you!",
                            "Where are they?",
                            "Speak to me, please!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Shinokas", args!["W-wha...?", "I told you.", "Th-they we--"])?;
                    ctx.next()?;
                    ctx.lines_as("Shinokas", args!["...", "......"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["No!", "Shinokas!", "Why, God?", "Why..."],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2077), Val::from(2078)])?;
                    ctx.var("shinokas_quest").set(Val::from(10))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("shinokas_quest").get()?.number()? > 9 {
                    ctx.lines(args!["^3355FFShinokas's", "body has grown", "cold to the touch.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Shinokas", args!["...", "......"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn sick_old_man_ein(ctx: &Ctx) -> Script {
    sick_old_man_ein_body(ctx, Vec::new()).map(|_| ())
}

fn maid_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("shinokas_quest").get()? == 4 {
        ctx.lines_as(
            "Maid",
            args!["Did you ask me", "if I saw some kind", "of ore around here?", "That's strange..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Maid",
            args![
                "Well, I've been with",
                "this family for a long",
                "time. Let me assure you",
                "that there's no secrets",
                "from me in this household!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Maid",
            args![
                "I'm sorry, but I don't",
                "think we have any ores,",
                "special or otherwise, here",
                "in the manor. What exactly",
                "do you need them for?"
            ],
        )?;
        ctx.var("shinokas_quest").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Maid", args!["There's no end", "to all these plates", "I have to clean...!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn maid_ein(ctx: &Ctx) -> Script {
    maid_ein_body(ctx, Vec::new()).map(|_| ())
}

fn scientist_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Scientist", args!["^333333*Grumble grumble*^000000"])?;
    ctx.next()?;
    ctx.lines_as("Scientist", args!["Huh...?", "How did you", "get in here?"])?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Oh! Ah....", "I'm the new...", "Guard. Nice", "to meet you."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Scientist",
        args!["Oh. Yeah.", "Nice-meet-you.", "..............", "^333333*Grumble grumble*^000000"],
    )?;
    if ctx.var("shinokas_quest").get()? == 7 {
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("What's wrong?:Catch you later.")])? {
            1 => {
                ctx.lines_as(
                    "Scientist",
                    args![
                        "What's wrong...?!",
                        "Oh, don't get me started!",
                        "I'm stuck here doing all the",
                        "work while the Lab Department Head goes out every freakin' day!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Scientist",
                    args![
                        "While I'm slaving",
                        "away here, he's in",
                        "that Airship, busy",
                        "flirting with that",
                        "woman. God...!",
                        "I'm like, so teed off!"
                    ],
                )?;
                ctx.var("shinokas_quest").set(Val::from(8))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Scientist", args!["Right.", "Yeah.", "Later, man."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("shinokas_quest").get()? == 8 {
        ctx.next()?;
        ctx.lines_as(
            "Scientist",
            args![
                "Man alive!",
                "Would it kill the",
                "Department Head",
                "to come in here and do",
                "some work for a change?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Scientist",
            args!["I mean, come on!", "I shouldn't have to", "carry his workload!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn scientist_ein(ctx: &Ctx) -> Script {
    scientist_ein_body(ctx, Vec::new()).map(|_| ())
}

fn unknown_stuff_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThere's something",
        "attached to a huge",
        "machine with many cords",
        "and folds of barbed wire.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn unknown_stuff_ein(ctx: &Ctx) -> Script {
    unknown_stuff_ein_body(ctx, Vec::new()).map(|_| ())
}

fn laboratory_soldier_ein_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Laboratory Soldier",
        args!["This area", "is off limits.", "Please leave", "immediately."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn laboratory_soldier_ein_1(ctx: &Ctx) -> Script {
    laboratory_soldier_ein_1_body(ctx, Vec::new()).map(|_| ())
}

fn laboratory_soldier_ein_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Laboratory Soldier",
        args![
            "Yuck...!",
            "There's this",
            "nasty dusty taste",
            "right inside my mouth",
            "that I can't get rid of!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laboratory Soldier",
        args![
            "^333333*Sigh*^000000",
            "I want to go home.",
            "Get some mouthwash.",
            "You know. Something."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn laboratory_soldier_ein_2(ctx: &Ctx) -> Script {
    laboratory_soldier_ein_2_body(ctx, Vec::new()).map(|_| ())
}

fn drunken_man_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("shinokas_quest").get()? == 8 {
        ctx.lines_as(
            "Drunken Man",
            args!["Okay okay...", "Daddy's gonna", "win some Apples", "this time for sure!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args!["Let's do it!", "^3131FFYmir's Heart^000000 is", "on my side! GO!"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_GO")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "What did he just say?", "It seemed important!"],
        )?;
        ctx.next()?;
        ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
        ctx.lines_as(
            "Kaci",
            args![
                "I have a total of ^FF000011^000000",
                "and you have total ^FF00005^000000.",
                "You lose this game. I'm",
                "sorry, but I hope we play",
                "again sometime."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args!["Waaaaaahhhhhhhhh!", "Apples! My apples!", "Apples, I need more...!"],
        )?;
        if ctx.call(Function::CountItem, vec![Val::from(512)])?.number()? < 11 {
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.call(Function::CountItem, vec![Val::from(512)])?.number()? > 10
            && ctx.call(Function::CountItem, vec![Val::from(512)])?.number()? < 100)
        {
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Do you want", "some of mine?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Drunken Man",
                args![
                    "What...?",
                    "I can't do anything",
                    "with so few Apples!",
                    "I'm a high roller and",
                    "this is a high stakes game!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.call(
                Function::DelItem,
                vec![Val::from(512), ctx.call(Function::CountItem, vec![Val::from(512)])?],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Do you want", "some of mine?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Drunken Man",
                args![
                    "Wha--? Yes...",
                    "Hell yes! Gimme",
                    "some of your Apples!",
                    "Yeeeeeeeeeeehaw!",
                    "I'm back, baby!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Drunken Man",
                args!["Now, to win!", "Daddy needs love...", "Time to go from", "crappy to classy!"],
            )?;
            ctx.next()?;
            ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 7 {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
                ctx.lines_as(
                    "Kaci",
                    args![
                        "I got a total of ^FF00008^000000,",
                        "and you have total ^FF000011^000000.",
                        "Congratulations, you won!",
                        "Let me give you your winnings",
                        "and we'll play again some time~"
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.lines_as(
                    "Kaci",
                    args![
                        "Oooh...",
                        "I got a total of ^FF000010^000000,",
                        "and you have a total of ^FF00007^000000.",
                        "I'm sorry, but you lose",
                        "again. Better luck next time..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Drunken Man", args!["Waaaaahhhhhhhhhh!", "Waaaaaaahhhhhhh!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        ctx.lines_as(
            "Drunken Man",
            args![
                "Mwahhahahahaha!",
                "Whahahahahahahaha!",
                "That's why they call me",
                "the ''Resurrection Kid!''",
                "I always come back!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "Oh yes, right!",
                "You...! You lent",
                "me those lucky",
                "Apples. ^333333*Hiccup*^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args![
                "I'm Kurshenburg!",
                "Thanks to you, I'm",
                "on a winning streak!",
                "Hahaha! Th-thank you~",
                "^333333*Hic-hic-hiccup!*^000000"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("So what was that about Ymir's Heart?:Hehe, you're welcome.")])? {
            1 => {
                ctx.lines_as(
                    "Drunken Man",
                    args!["What...?", "Ymir's Heart?", "How do you know", "about that? ^333333*Hiccup*^000000"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Y-you...!",
                        "You were just yelling,",
                        "''Ymir's Heart is on my",
                        "side,'' while you were",
                        "gambling with those Apples!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Drunken Man", args!["What...?!", "No way~", "Err...? Did I...?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "Heh heh...",
                        "I'm not really",
                        "supposed to be talking",
                        "about this. As head of",
                        "the Laboratory, I'm sworn",
                        "to secrecy about Ymir's Heart."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "But what do I care?!",
                        "All they want are the",
                        "results of my research!",
                        "They don't appreciate",
                        "my work at all! My title",
                        "is completely worthless!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Drunken Man", args!["You don't know how long I've", "been imprisoned in the lab and", "that the work conditions just get worse and worse. Screw them! I'll keep getting paid as long as I show them some progress in our project!"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Project?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "Yeah, we're researching",
                        "Ymir's Heart. It was found",
                        "a long time ago in ^3131FFEinbech^000000",
                        "and it's in our lab now."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "Everyone knows it's supposed",
                        "to hold some legendary power,",
                        "but even I was surprised to see",
                        "what it was capable of. It's both terrible and miraculous, scary",
                        "and wondrous..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "So...",
                        "I figure...",
                        "It may even",
                        "have the power to",
                        "win me Dice games!",
                        "Bwahaha--^333333*Hiccup!*^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "^333333(That thing I saw",
                        "hooked up to all those",
                        "wires in the Laboratory",
                        "must have been a piece",
                        "of ^3131FFYmir's Heart^333333. I better",
                        "tell Shinokas about this.)^000000"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2076), Val::from(2077)])?;
                ctx.var("shinokas_quest").set(Val::from(9))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Drunken Man",
                    args![
                        "Hahahaha!",
                        "You're great!",
                        "And I feel great!",
                        "Everything's great!",
                        "Bwahahahaahahah!",
                        "^333333*Hiccup*^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("shinokas_quest").get()? == 9 {
        ctx.lines_as(
            "Drunken Man",
            args![
                "Hahahaha!",
                "You're great!",
                "I feel great!",
                "Everything's great!",
                "Bwahahahaahahah!",
                "^333333*Hiccup*^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Drunken Man",
            args!["Okay okay...", "Daddy's gonna", "win some Apples", "this time for sure!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Drunken Man",
            args!["Let's do it!", "^3131FFYmir's Heart^000000 is", "on my side! GO!"],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_GO")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh...?", "What did he just say?", "It seemed important!"],
        )?;
        ctx.next()?;
        ctx.mes("^3355FF*Rolling and rumbling*^000000")?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
        ctx.lines_as(
            "Kaci",
            args![
                "I have a total of ^FF00003^000000",
                "and you have total ^FF00002^000000.",
                "You lose this game. I'm",
                "sorry, but I hope we play",
                "again sometime."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Drunken Man", args!["Waaaaaahhhhhhhhh!", "Apples! My apples!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn drunken_man_ein(ctx: &Ctx) -> Script {
    drunken_man_ein_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KenkaStep {
    Start,
    OnTouch,
}

fn kenka_run(ctx: &Ctx, mut step: KenkaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KenkaStep::Start => {
                step = KenkaStep::OnTouch;
                continue 'machine;
            }
            KenkaStep::OnTouch => {
                if ctx.var("shinokas_quest").get()?.number()? > 8 {
                    ctx.lines(args![
                        "^3355FFThe open window rattles",
                        "as you enter the room and",
                        "are welcomed by a sudden",
                        "chill. A trail of red footprints",
                        "lies near your feet.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFA grey sheet lies",
                        "rumpled on the bed,",
                        "but you can see dark red",
                        "stains in between the folds.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kenka(ctx: &Ctx) -> Script {
    kenka_run(ctx, KenkaStep::Start, Vec::new()).map(|_| ())
}

pub fn kenka_ontouch(ctx: &Ctx) -> Script {
    kenka_run(ctx, KenkaStep::OnTouch, Vec::new()).map(|_| ())
}

fn young_man_shinokas_quest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Young Man",
        args![
            "Behind the pub,",
            "you'll see this old man",
            "who's always mumbling",
            "something to himself."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "Sometimes he seems really",
            "angry, but other times he looks",
            "awfully depressed. He must have",
            "lived through some really horrible experience. I can't help but feel really sorry for the old guy."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "He says and does",
            "a lot of strange things.",
            "It's sad to see someone",
            "that old act that way, but",
            "it makes me wonder what",
            "could have happened to him."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_man_shinokas_quest(ctx: &Ctx) -> Script {
    young_man_shinokas_quest_body(ctx, Vec::new()).map(|_| ())
}

fn security_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn security_ein(ctx: &Ctx) -> Script {
    security_ein_body(ctx, Vec::new()).map(|_| ())
}

fn security_ein_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_endtime = Val::from(0);
    let mut l_input1_s = Val::from("");
    let mut l_input2_s = Val::from("");
    let mut l_startseconds = Val::from(0);
    let mut l_time = Val::from(0);
    let mut l_word1_s = Val::from("");
    let mut l_word2_s = Val::from("");
    if ((ctx.var("shinokas_quest").get()? == 7 || ctx.var("lhz_heart").get()? == 9) || ctx.var("lhz_heart").get()? == 10) {
        ctx.lines_as(
            "Security System",
            args![
                "^8B0000*Beep Boop*^000000",
                "Restricted Access Area.",
                "Please identify yourself",
                "through the system."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Identify.:Information:Cancel")])? {
            1 => {
                ctx.lines_as(
                    "Security System",
                    args![
                        "Enter the following",
                        "password in 60 seconds.",
                        "Failure to do so will result",
                        "in lockout. Please wait."
                    ],
                )?;
                ctx.next()?;
                l_startseconds = ctx.call(Function::GetTimeTick, vec![Val::from(1)])?;
                ctx.mes("[Security System]")?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                if subject2 == 1 {
                    l_word1_s = Val::from("burrdingdingdilidingdingphoohudaambandoorabambarambambamburanbamding");
                    l_word2_s = Val::from("burapaphurarlandreamduranbatuhiwooikabamturubamdingding");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^FF1493") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 2 {
                    l_word1_s = Val::from("...silence. quiet benevolence... soul mate... wonder. enigma... cloud.");
                    l_word2_s = Val::from("opeN,Open!op3n.openOpen0p3nOpEn0pen`open'0Pen open?open!111OPENSESAME");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^FF1493") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 3 {
                    l_word1_s = Val::from("Coboman no chikara-yumei na chikara-daiookii na chikara da ze! COBO ON");
                    l_word2_s = Val::from("hfjdkeldjsieldjshfjdjeiskdlefvbd");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^FF1493") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 4 {
                    l_word1_s = Val::from("belief love luck grimace sweat rush folktale rodimus optimus bumblebee");
                    l_word2_s = Val::from("LiGhTsPeEd RiGhT SPEed LeFT TURn RiGhT BuRn OrIGInAL GaNgSteR SmACk");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^FF1493") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 5 {
                    l_word1_s = Val::from("By the power of p-po-poi-po-poi-poin-poing GOD-POING. I NEVER LOSE!");
                    l_word2_s = Val::from("uNflAPPaBLe LoVaBLe SeCreTs AnD BoWLiNg aGaINST tHe KarMA of YoUtH");
                    ctx.lines(args![
                        "^3CBCBCBy the power of p-po-poi-po-poi-poin-poing",
                        "GOD-POING. I NEVER LOSE!^000000"
                    ])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^FF1493") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 6 {
                    l_word1_s = Val::from("You give me no choice. I guess it's time for me to reveal my secret...");
                    l_word2_s = Val::from("fReeDoM eCstAcy JoUrnaLiSm ArMpIt DisCoverY hEaDaChE MoonbeAmS jUsTiCE");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^800080") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                } else if subject2 == 7 {
                    l_word1_s = Val::from("I'm the King of All Weirdos! Now you know of my true power. Obey~!");
                    l_word2_s = Val::from("uNflAPPaBLe LoVaBLe SeCreTs AnD BoWLiNg aGaINST tHe KarMA of YoUtH");
                    ctx.lines(args![((Val::from("^3CBCBC") + l_word1_s.clone()) + Val::from("^000000"))])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input1_s = input;
                    ctx.lines_as(
                        "Security System",
                        args![((Val::from("^800080") + l_word2_s.clone()) + Val::from("^000000"))],
                    )?;
                }
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input2_s = input;
                l_endtime = ctx.call(Function::GetTimeTick, vec![Val::from(1)])?;
                l_time = (l_endtime.clone().try_sub(l_startseconds.clone())?);
                ctx.mes("[Security System]")?;
                if (l_input1_s.clone().loosely_equals(&l_word1_s.clone()) && l_input2_s.clone().loosely_equals(&l_word2_s.clone())) {
                    if l_time.clone().number()? > 60 {
                        ctx.lines(args![
                            "Time over.",
                            ((Val::from("It took ^ff0000") + l_time.clone()) + Val::from(" seconds^000000")),
                            "for you to enter the",
                            "password. Initiating",
                            "lockout. Access denied."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            ((Val::from("It took ^ff0000") + l_time.clone()) + Val::from(" seconds^000000")),
                            "for you to enter the",
                            "password. Initiating",
                            "override. Access granted."
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("ein_in01"), Val::from(283), Val::from(25)])?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args!["You have failed", "the identification", "check. Access denied."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Security System",
                    args![
                        "You must use the",
                        "security system in order",
                        "to gain access into the",
                        "Einbroch Laboratory."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Security System",
                    args![
                        "You will be given",
                        "a password that you",
                        "must input correctly",
                        "within 60 seconds.",
                        "Otherwise, you will",
                        "fail the security check."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Security System",
                    args![
                        "If you take longer",
                        "than 3 minutes to",
                        "enter the password,",
                        "the security system",
                        "will initiate lockout."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as("Security System", args!["You have canceled", "the ID security check."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args!["^3355FFThe door is locked.", "You cannot enter.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn security_ein_ontouch(ctx: &Ctx) -> Script {
    security_ein_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn calla_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(70)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? < 60 {
        ctx.lines_as(
            "Calla",
            args![
                "Hello adventurer.",
                "Our city must just be",
                "another place where",
                "you'll stay no longer",
                "than a few days."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "You must have so much",
                "freedom. I envy you. I can't",
                "do what I want to do. I don't",
                "even have the courage to tell",
                "my family what I really want,",
                "much less change things here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "How is it like?",
                "Going wherever you",
                "please, following your",
                "heart's true desire?",
                "What I would give to",
                "be able to do that..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 17 {
        ctx.lines_as(
            "Calla",
            args![
                "Thank you so much!",
                "I'll try my best to convince",
                "my parents to accept our",
                "relationship. It'll be hard,",
                "but it's a good first step~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "I hope that we can all",
                "work together to improve",
                "relations between Einbech",
                "and Einbroch. The hatred",
                "between our towns must end..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "I really appreciate",
                "what you've done for",
                "all of us. I'll be praying",
                "for your safety, adventurer."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 16 {
        ctx.lines_as(
            "Calla",
            args![
                "I just heard from my mother",
                "that she's planning to have",
                "tea with Klitzer! I'm sure that",
                "I have you to thank for this~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "I never dreamed that",
                "something as wonderful",
                "as this could happen.",
                "I'm so happy, I could cry...",
                "I'll always be grateful",
                "for what you've done."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "I feel like such a fool,",
                "thinking it was all hopeless.",
                "I'll be doing my best to have",
                "my parents accept Klitzer and",
                "someday we'll be married~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "There isn't much that I can",
                "give you, but I can show you",
                "one of my family's secrets.",
                "It's an invigorating massage",
                "technique that makes you a lot",
                "healthier in only ten seconds."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Well, please take",
                "off your equipment",
                "and stand still while",
                "I give the massage. It",
                "might hurt a bit at first..."
            ],
        )?;
        ctx.call(Function::Nude, vec![])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FF*Rub Rub Rub*",
            "*Knead Knead Knead*",
            "*Crrack C-c-c--c-crack*",
            "*Crack Crack Crrrrrrack*",
            "*Rub Crrraaaaaaaaaack*^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Ooooooooh...",
                "I feel sooo",
                "sore and yet",
                "soooooo good.",
                "Wait. Now I just",
                "feel goooood~"
            ],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
        ctx.var("ein_loverq").set(Val::from(17))?;
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
            "Calla",
            args![
                "So how was it?",
                "I hope it was refreshing.",
                "Please understand that",
                "it's the best thing I can",
                "give you to show my gratitude."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Once again,",
                "thank you so",
                ((Val::from("much, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "I'll always pray for",
                "your safety on your",
                "your adventures~"
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(8088)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 5 && ctx.call(Function::CountItem, vec![Val::from(712)])?.number()? > 0) {
        ctx.lines_as("Calla", args!["You've spoken", "with Klitzer? How", "is he? What did he say?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Oh...?",
                "He asked you to",
                "deliver this flower",
                "to me? How sweet~",
                "Thank you very much,",
                "kind adventurer~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Ah, I'm so rude!",
                "I've been calling you",
                "''adventurer'' this whole",
                "time you've been helping",
                "me! Would you please",
                "tell me your name?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s
            .clone()
            .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
        {
            ctx.lines_as(
                "Calla",
                args![
                    ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                    "Such a lovely name~",
                    "I promise that I won't ever",
                    "forget it. Oh, and if you pass by Einbech, would you thank Klitzer for the flower for me please?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Calla",
                args![
                    "A-and... And...",
                    "Please tell him that",
                    "I really miss him a lot.",
                    "^333333*Sob Sob...*^000000"
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(712), Val::from(1)])?;
            ctx.var("ein_loverq").set(Val::from(6))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8079), Val::from(8080)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Calla",
                args![
                    "I'm sorry...",
                    "I didn't catch that.",
                    "Would you please tell",
                    "me your name again?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("ein_loverq").get()? == 4 {
        ctx.lines_as(
            "Calla",
            args![
                "Oh my god...",
                "Are you alright?",
                "I just found out that",
                "you ran into my father!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "By all means,",
                "try to avoid my dad!",
                "He doesn't trust anyone",
                "who's not considered part",
                "of the upper class, even",
                "adventurers like you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Would you please",
                "take this Violin and",
                "try to make it to Klitzer",
                "this time? Thank you",
                "for your help~"
            ],
        )?;
        ctx.var("ein_loverq").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8078), Val::from(8077)])?;
        ctx.call(Function::GetItem, vec![Val::from(1901), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("ein_loverq").get()? == 3 || ctx.var("ein_loverq").get()? == 5) {
        ctx.lines_as(
            "Calla",
            args![
                "Oh, please send my",
                "regards to Klitzer for me.",
                "I wish I could comfort",
                "him in person, but this",
                "is the best I can do for now."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ein_loverq").get()? == 2 {
        ctx.lines_as(
            "Calla",
            args![
                "You're the adventurer",
                "from before, aren't you?",
                "Sadly, there isn't much",
                "to do around here. This",
                "place is basically like",
                "a prison to me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args!["Oh, you've met Klitzer?", "Isn't he so kind, and such", "a perfect gentleman?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "I really wish I could",
                "see him, but it's almost",
                "impossible. My parents think",
                "he's not good enough for me,",
                "but they're wrong! What am",
                "I going to do? Oh, Klitzer..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "Well, maybe I can't see",
                "him, but would you give",
                "my violin to Klitzer for me?",
                "I used to play this for him",
                "all the time..."
            ],
        )?;
        ctx.next()?;
        ctx.var("ein_loverq").set(Val::from(3))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8076), Val::from(8077)])?;
        ctx.call(Function::GetItem, vec![Val::from(1901), Val::from(1)])?;
        ctx.lines_as(
            "Calla",
            args![
                "I'm sorry to trouble you,",
                "but please understand",
                "that I want to comfort my",
                "Klitzer in any way that",
                "I possibly can. Thank",
                "you so much, adventurer..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Calla",
            args![
                "Hello adventurer.",
                "Our city must just be",
                "another place where",
                "you'll stay no longer",
                "than a few days."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Calla",
            args![
                "You must have so much",
                "freedom. I envy you. I can't",
                "do what I want to do. I don't",
                "even have the courage to tell",
                "my family what I really want,",
                "much less change things here..."
            ],
        )?;
        ctx.next()?;
        if ctx.var("ein_loverq").get()? == 0 {
            ctx.var("ein_loverq").set(Val::from(1))?;
        }
        ctx.lines_as(
            "Calla",
            args![
                "How is it like?",
                "Going wherever you",
                "please, following your",
                "heart's true desire?",
                "What I would give to",
                "be able to do that..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn calla_ein(ctx: &Ctx) -> Script {
    calla_ein_body(ctx, Vec::new()).map(|_| ())
}
