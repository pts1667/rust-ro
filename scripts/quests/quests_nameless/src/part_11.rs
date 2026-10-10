use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn rogue_guild_agent_nd2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rumour_nd").get()?.number()? < 3 {
        ctx.lines_as("Agent", args!["What? You want somethin'?"])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["?????????????????"])?;
        ctx.next()?;
        ctx.lines_as("Agent", args!["Heh. Thought so."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("rumour_nd").get()?.number()? > 2 && ctx.var("rumour_nd").get()?.number()? < 6) {
            ctx.lines_as("Agent", args!["What's up? I got", "my hands full, so", "make it quick."])?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "Nothin', huh?",
                    "Guess you should",
                    "be talking a Rogue",
                    "agent in one of the",
                    "other towns."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rumour_nd").get()? == 6 {
                ctx.lines_as(
                    "Agent",
                    args![
                        "Hey. What are you",
                        "doin', just loitering",
                        "around here? You lookin'",
                        "for somebody? Huh?"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("I'm here to help a Rogue Agent!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Agent", args!["Oh, yeah?"])?;
                ctx.next()?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "Heya guy. Always good",
                            "to see a fellow Rogue.",
                            "You know I'm just messin'",
                            "around with you, right?"
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "I heard o' you guys,",
                            "Rogues workin' undercover.",
                            "Don't try to fool me: under",
                            "those clothes, you are one",
                            "hundred percent Rogue. Heh!"
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines(args!["^3355FFYou deliver the", "folded note to him.^000000"])?;
                ctx.next()?;
                ctx.lines_as("Agent", args!["Awwww, man!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Agent",
                    args![
                        "I keep telling that",
                        "guy to just send me",
                        "pictures! I've told him",
                        "so many times that I've",
                        "got trouble reading!",
                        "That guy's a moron!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Agent", args!["Uh...", "You mind reading", "this note to me?"])?;
                ctx.var("rumour_nd").set(Val::from(7))?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Let's see,", "the note says..."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Cooperate with this adventurer.:......")])? {
                    1 => {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "So... We're partners?",
                                "Cool. Nice to meetcha.",
                                "I'm the Payon agent.",
                                "I guess we gotta work",
                                "together from now on."
                            ],
                        )?;
                        ctx.var("rumour_nd").set(Val::from(8))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Agent", args!["You...", "You can't read either?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Hahaha! Right on, right on.",
                                "Well, I'm pretty sure they",
                                "sent you to work with me.",
                                "When you're ready to get",
                                "to business, come back",
                                "and we'll talk, okay?"
                            ],
                        )?;
                        ctx.var("rumour_nd").set(Val::from(8))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("rumour_nd").get()? == 7 {
                    ctx.lines_as(
                        "Agent",
                        args!["Hey, I asked you", "to read the note,", "not run away! Now...", "What's it say?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Let's see,", "the note says..."],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Cooperate with this adventurer.:......")])? {
                        1 => {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "So... We're partners?",
                                    "Cool. Nice to meetcha.",
                                    "I'm the Payon agent.",
                                    "I guess we gotta work",
                                    "together from now on."
                                ],
                            )?;
                            ctx.var("rumour_nd").set(Val::from(8))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Agent", args!["You...", "You can't read either?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Hahaha! Right on, right on.",
                                    "Well, I'm pretty sure they",
                                    "sent you to work with me.",
                                    "When you're ready to get",
                                    "to business, come back",
                                    "and we'll talk, okay?"
                                ],
                            )?;
                            ctx.var("rumour_nd").set(Val::from(8))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if ctx.var("rumour_nd").get()? == 8 {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Good, you're back.",
                                "We got a pretty big",
                                "job to take care of.",
                                "Lives are at stake,",
                                "all that jazz. Ready?"
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.var("zdan_edq").get()?.number()? > 12 {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "The Z Gang poisoned",
                                    "one of the water wells",
                                    "here in Payon. All the",
                                    "water's black now. We gotta",
                                    "use Red Herbs to counter",
                                    "that poison, okay?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "So remember to",
                                    "collect some ^FF0000Red Herbs^000000.",
                                    "Don't forget: ^FF0000Red herbs^000000."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Wait, that doesn't",
                                    "sound right. Did I say",
                                    "Red Herbs? I'm sorry,",
                                    "I meant to say ^0000FFGreen",
                                    "Herbs^000000. Bring me ^0000FF1 Green",
                                    "Herb^000000, okay? *Phew~*"
                                ],
                            )?;
                            ctx.var("rumour_nd").set(Val::from(9))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Someone's poisoned",
                                    "Payon's well water...",
                                    "It's all black and",
                                    "deadly now. I want you",
                                    "to bring me a ^FF0000Red Herb^000000",
                                    "so that we can fix this!"
                                ],
                            )?;
                            ctx.var("rumour_nd").set(Val::from(10))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("rumour_nd").get()? == 9 {
                            ctx.lines_as(
                                "Agent",
                                args!["Hey, you're back!", "You're much quicker", "that I thought you'd be~"],
                            )?;
                            ctx.next()?;
                            if ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0 {
                                ctx.call(Function::DelItem, vec![Val::from(511), Val::from(1)])?;
                                ctx.var("rumour_nd").set(Val::from(11))?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Alright, it's gonna take",
                                        "a while to neutralize all",
                                        "this poison, but I think",
                                        "we did it. Hey, gimmie",
                                        "some time to draw a report,",
                                        "and then come back later, 'kay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Agent", args!["Err?", "Where's that", "Green Herb?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("rumour_nd").get()? == 10 {
                                ctx.lines_as(
                                    "Agent",
                                    args!["Hey, you're back!", "You're much quicker", "that I thought you'd be~"],
                                )?;
                                ctx.next()?;
                                if ctx.call(Function::CountItem, vec![Val::from(507)])?.number()? > 0 {
                                    if ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0 {
                                        ctx.lines_as(
                                            "Agent",
                                            args![
                                                "Ah, a Green Herb!",
                                                "Is that what I told",
                                                "you to bring? 'Cause",
                                                "I'm sure this is the",
                                                "right one. Absolutely~"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Agent",
                                            args![
                                                "Alright, it's gonna take",
                                                "a while to neutralize all",
                                                "this poison, but I think",
                                                "we did it. Hey, gimmie",
                                                "some time to draw a report,",
                                                "and then come back later, 'kay?"
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(511), Val::from(1)])?;
                                        ctx.var("rumour_nd").set(Val::from(11))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Agent", args!["Hey, what are you", "doin' with the Red Herb?"])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("You said a Red Herb...")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Agent",
                                            args![
                                                "Oh, what?! Huh...",
                                                "I guess I did. Eh, no",
                                                "biggie. Hey, bring me",
                                                "a Green Herb, will you?",
                                                "That's the one I wanted."
                                            ],
                                        )?;
                                        ctx.var("rumour_nd").set(Val::from(9))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0 {
                                        ctx.lines_as(
                                            "Agent",
                                            args![
                                                "Alright, it's gonna take",
                                                "a while to neutralize all",
                                                "this poison, but I think",
                                                "we did it. Hey, gimmie",
                                                "some time to draw a report,",
                                                "and then come back later, 'kay?"
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(511), Val::from(1)])?;
                                        ctx.var("rumour_nd").set(Val::from(11))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Agent", args!["Err?", "Where's that", "Green Herb?"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            } else {
                                if ctx.var("rumour_nd").get()? == 11 {
                                    ctx.lines_as(
                                        "Agent",
                                        args![
                                            "Hey, you know what?",
                                            "The well water is still",
                                            "black. That means...",
                                            "If the herb we used is",
                                            "good... Then it's just",
                                            "dyed black to scare people."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Agent",
                                        args![
                                            "So to clean up the",
                                            "water, we gotta use",
                                            "a Counteragent, right?",
                                            "Will you help me out?",
                                            "I got a Karvodailnirol..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Agent",
                                        args![
                                            "So if you bring me",
                                            "^0000FF1 Alcohol^000000, we should",
                                            "be totally good to",
                                            "make a Counteragent."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) {
                                        let choice = runtime::select_values(ctx, &[Val::from("Wait, Karvodailnirol?!")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Agent", args!["Yeah. Karvodailnirol.", "What about it?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["You need Detrimindexta", "to make a Counteragent,", "not Karvodailnirol."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Agent", args!["Oh, really? Heh!", "You're a lifesaver!", "Detrimindexta, huh?"])?;
                                        ctx.next()?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                                        ctx.lines_as(
                                            "Agent",
                                            args!["Don't worry, I got", "that one too! All you", "gotta do is bring Alcohol~"],
                                        )?;
                                        ctx.var("rumour_nd").set(Val::from(12))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Agent",
                                            args!["Alright, just bring", "me an Alcohol, and", "I'll take care of the rest."],
                                        )?;
                                        ctx.var("rumour_nd").set(Val::from(13))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("rumour_nd").get()? == 12 {
                                        ctx.lines_as(
                                            "Agent",
                                            args!["Everything's ready", "except for the Alcohol.", "Did you bring it?"],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::CountItem, vec![Val::from(970)])?.number()? > 0 {
                                            ctx.lines_as(
                                                "Agent",
                                                args!["Great, now all we", "have to do is wait.", "Nice work, partner."],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(970), Val::from(1)])?;
                                            ctx.var("rumour_nd").set(Val::from(15))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Agent",
                                                args!["You don't have it?", "You didn't drink", "the alcohol, did you?"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.var("rumour_nd").get()? == 13 {
                                            ctx.lines_as(
                                                "Agent",
                                                args!["Everything's ready", "except for the Alcohol.", "Did you bring it?"],
                                            )?;
                                            ctx.next()?;
                                            if ctx.call(Function::CountItem, vec![Val::from(970)])?.number()? > 0 {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args!["Great, now all we", "have to do is wait.", "Nice work, partner."],
                                                )?;
                                                ctx.call(Function::DelItem, vec![Val::from(970), Val::from(1)])?;
                                                ctx.var("rumour_nd").set(Val::from(14))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args!["You don't have it?", "You didn't drink", "the alcohol, did you?"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            if ctx.var("rumour_nd").get()? == 14 {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Um, I made a mistake",
                                                        "making the Counteragent...",
                                                        "I was supposed to use this",
                                                        "Detrimindexta, not the other",
                                                        "thing, Karvodailnirol, to",
                                                        "make the Counteragent."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Sorry, they both look",
                                                        "the same to me. And, well,",
                                                        "so do all the letters of",
                                                        "the alphabet. You mind",
                                                        "trying this out again?",
                                                        "Just 1 more Alcohol..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Hey, I promise!",
                                                        "I won't screw it up",
                                                        "this time. We'll",
                                                        "do this for sure~"
                                                    ],
                                                )?;
                                                ctx.var("rumour_nd").set(Val::from(12))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("rumour_nd").get()? == 15 {
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Heh heh! Looks like",
                                                        "we solved the case!",
                                                        "The water wasn't really",
                                                        "poisoned... Just colored",
                                                        "to look suspicious. See?",
                                                        "Completely safe to drink."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Anyway, looks like",
                                                        "we're done here. You",
                                                        "mind taking my report over",
                                                        "to the next Rogue agent?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
                                                    ctx.lines_as(
                                                        "Agent",
                                                        args!["Make sure to take", "this to the guy in", "Comodo, okay?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou received a note that",
                                                        "communicates solely",
                                                        "through doodles.^000000"
                                                    ])?;
                                                    ctx.var("rumour_nd").set(Val::from(18))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as(
                                                        "Agent",
                                                        args!["Make sure to take", "this to the guy in", "Al De Baran, okay?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou received a note that",
                                                        "communicates solely",
                                                        "through doodles.^000000"
                                                    ])?;
                                                    ctx.var("rumour_nd").set(Val::from(16))?;
                                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else if ctx.var("rumour_nd").get()? == 16 {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Be sure that the",
                                                        "Rogue agent over",
                                                        "in Al De Baran gets",
                                                        "my report, okay?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("rumour_nd").get()? == 17 {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Huh, the guy in",
                                                        "Al De Baran wouldn't",
                                                        "take it? Oh! That's why!",
                                                        "I meant Comodo! You gotta",
                                                        "go to Comodo! Heh, my bad~"
                                                    ],
                                                )?;
                                                ctx.var("rumour_nd").set(Val::from(18))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("rumour_nd").get()? == 18 {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args!["Be sure that the", "Rogue agent over", "in Comodo gets", "my report, okay?"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Agent",
                                                    args![
                                                        "Hey, partner, how's",
                                                        "it going? I'm sorry",
                                                        "if I'm a bit of a hassle",
                                                        "to work with. I might lack",
                                                        "book smarts, but I make",
                                                        "up for it some with my saavy."
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
    Ok(Val::from(0))
}

pub fn rogue_guild_agent_nd2(ctx: &Ctx) -> Script {
    rogue_guild_agent_nd2_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_guild_agent_nd3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sorry_item = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Agent",
            args![
                "You've got too many",
                "things on you. Why",
                "don't you put some",
                "of it in Kafra Storage",
                "before talking to me, eh?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rumour_nd").get()?.number()? < 1 {
        ctx.lines_as("Agent", args!["......", "......"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("rumour_nd").get()?.number()? > 0 && ctx.var("rumour_nd").get()?.number()? < 16) {
            if ctx.var("zdan_edq").get()?.number()? > 12 {
                ctx.lines_as(
                    "Agent",
                    args!["You're the one", "that helped take", "down the Z Gang?", "Heh! Good work~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Agent", args!["Who are you? Hm."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("rumour_nd").get()? == 16 || ctx.var("rumour_nd").get()? == 18) {
                ctx.lines_as(
                    "Agent",
                    args![
                        "You know, Rogues are",
                        "tricky and hard to catch,",
                        "so I'm gonna teach you to",
                        "carry yourself through some",
                        "self-discipline. Got it?",
                        "Discipline! You need it!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Agent",
                    args![
                        "From here on out,",
                        "I want you to yell",
                        "''Yes, sir!'' when you",
                        "acknowledge my instructions.",
                        "You got that! Loud and clear!"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes, Sir!:...")])? {
                    1 => {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Right on, right on.",
                                "Hm. Let's try that again...",
                                "Let me hear you say it louder!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Yes, Sir!")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Agent", args!["Nice!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Okay, now for your",
                                "training instructions...",
                                "There's a rumor that a monster",
                                "has invaded the village, but",
                                "I'm sure it's just someone",
                                "trying to scare the public."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Agent",
                            args![
                                "I'm pretty sure the, uh,",
                                "perp is wearing a mask to",
                                "fool people into thinkin'",
                                "fhe's monster. This means...",
                                "Your mission is to bring",
                                "me some monsters masks."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Agent", args!["Heh...", "What's your mission?!"])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("Bring monster masks!")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as("Agent", args!["Groove."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Alright, bring me",
                                "^FF00002 scary lookin'",
                                "monster masks^000000.",
                                "Just 2, and make",
                                "sure they're scary."
                            ],
                        )?;
                        ctx.var("rumour_nd").set(Val::from(19))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "What do you think",
                                "you're doin'? I'm",
                                "pretty insulted, seeing",
                                "as you're all defiant like",
                                "this. How do you expect",
                                "to become a good Rogue?!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("rumour_nd").get()? == 19 {
                    ctx.lines_as("Agent", args!["So... You bring", "the masks like", "a good little soldier?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Uh huh.:Yes, Sir!")])? {
                        1 => {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Hey, remember our",
                                    "little rule? I even",
                                    "clued you off when",
                                    "I said ''soldier!'' Heh~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Agent", args!["Okay, let me check."])?;
                            ctx.next()?;
                            if (((((((((((((((((((ctx.call(Function::CountItem, vec![Val::from(2278)])?.number()? > 0
                                || ctx.call(Function::CountItem, vec![Val::from(2281)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5043)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(2288)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(2292)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(2297)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5005)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5086)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5087)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5088)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5089)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5090)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5176)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5203)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5098)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5121)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5130)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5177)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5169)])?.number()? > 0)
                                || ctx.call(Function::CountItem, vec![Val::from(5143)])?.number()? > 0)
                            {
                                ctx.lines_as("Agent", args!["Excellent!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Look, I know I gave you",
                                        "a hard time. And don't worry,",
                                        "I'm not gonna take your masks.",
                                        "I just wanted to train you",
                                        "to be the best Rogue that",
                                        "you can be. Understand?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args!["Eh? So what's this", "note you keep carrying", "around in your hand?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args!["Hmmm... Mmmhmmm...", "Hmmm... Mmmhmmm...", "Hmmm... Mmmhmmm... Oh!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Aw, nuts! Why didn't",
                                        "you say something?",
                                        "I thought you were",
                                        "here to take the Job",
                                        "change test for Rogues!"
                                    ],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SORRY")?])?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Aw man, you came all this",
                                        "way to help me out, and",
                                        "I thought I--I'm really,",
                                        "really sorry! It's just",
                                        "that I got you mixed up",
                                        "with a trainee I'm expecting..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Do me a favor, and",
                                        "not tell anyone, yeah?",
                                        "Consider this... I dunno,",
                                        "an insider look into the",
                                        "Rogue world if you would."
                                    ],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Yes, sir!:I won't tell anyone.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Agent",
                                            args![
                                                "Whoa! Whoa!",
                                                "You can knock",
                                                "that off now.",
                                                "Thanks, I really",
                                                "appreciate it."
                                            ],
                                        )?;
                                        ctx.next()?;
                                    }
                                    2 => {
                                        ctx.lines_as("Agent", args!["Thanks. I knew I could", "trust you somehow!"])?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Anyway, you've got one",
                                        "last place to go, Morocc.",
                                        "Check in with the Rogue",
                                        "agent over there, will you?",
                                        "Oh, and take this food",
                                        "with my apologies, 'kay?"
                                    ],
                                )?;
                                l_sorry_item = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                                if l_sorry_item.clone() == 1 {
                                    ctx.call(Function::GetItem, vec![Val::from(12044), Val::from(1)])?;
                                } else {
                                    if l_sorry_item.clone() == 2 {
                                        ctx.call(Function::GetItem, vec![Val::from(12059), Val::from(1)])?;
                                    } else if l_sorry_item.clone() == 3 {
                                        ctx.call(Function::GetItem, vec![Val::from(12064), Val::from(1)])?;
                                    } else if l_sorry_item.clone() == 4 {
                                        ctx.call(Function::GetItem, vec![Val::from(12049), Val::from(1)])?;
                                    } else if l_sorry_item.clone() == 5 {
                                        ctx.call(Function::GetItem, vec![Val::from(12054), Val::from(1)])?;
                                    } else if l_sorry_item.clone() == 6 {
                                        ctx.call(Function::GetItem, vec![Val::from(12069), Val::from(1)])?;
                                    } else {
                                        ctx.lines_as(
                                            "Agent",
                                            args!["Wait, something", "isn't quite right. You", "mind coming back", "a little later?"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Hey, be safe on your",
                                        "way to Morocc, alright?",
                                        "Good luck finding that",
                                        "Rogue agent over there.",
                                        "Oh, and mum's the word",
                                        "on my little mistake~"
                                    ],
                                )?;
                                ctx.var("rumour_nd").set(Val::from(20))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Agent",
                                    args!["Hey, didja even bother", "to bring the masks?", "Go back and find them!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx.var("rumour_nd").get()? == 20 {
                    ctx.lines_as(
                        "Agent",
                        args!["Oh, hey, you're back?", "Weren't you already on", "your way to Morocc?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("rumour_nd").get()? == 22 {
                    ctx.lines_as("Agent", args!["Heh, looks like we", "solved this case~"])?;
                    ctx.next()?;
                    if ctx.var("zdan_edq").get()?.number()? > 12 {
                        ctx.lines_as(
                            "Agent",
                            args!["Thanks to you,", "we don't have to", "worry about that", "crummy ol' Z Gang."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Until the world is",
                                "free of evil and all",
                                "that, the Rogues'll",
                                "always have work to do."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "Why are you still",
                            "sticking around?",
                            "Don't you have",
                            "something more",
                            "important to do?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn rogue_guild_agent_nd3(ctx: &Ctx) -> Script {
    rogue_guild_agent_nd3_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_guild_agent_nd4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rumour_nd").get()? == 1 {
        ctx.lines_as(
            "Agent",
            args![
                "A case? What are you...?",
                "Ask somebody else. I don't",
                "know what you're saying.",
                "Leave me alone, and",
                "just let me relax~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("rumour_nd").get()?.number()? > 1 && ctx.var("rumour_nd").get()?.number()? < 20) {
        ctx.lines_as(
            "Agent",
            args!["Ahhh, what'd I do", "for a nice jug of", "frosty beer now~", "...What?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rumour_nd").get()? == 20 {
        ctx.lines_as("Agent", args!["Oh, yeah, I'm almost", "done with my investigation."])?;
        ctx.next()?;
        if ctx.var("zdan_edq").get()?.number()? > 12 {
            ctx.lines_as(
                "Agent",
                args![
                    "The Z Gang has been",
                    "causing trouble everywhere.",
                    "It's just mischief, but...",
                    "If I got my hands on them..."
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "Anyway, that gang is",
                    "drawing a lot of flak",
                    "to Morocc. I mean, they",
                    "kind of live here, so..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "God, I want a drink.",
                    "Listen, would you just",
                    "go report to the Rogue",
                    "Guild for me? I know it's",
                    "my responsibility, but...",
                    "It's something for you to do."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Agent", args!["Heh! Keep up", "the good work~"])?;
            ctx.var("rumour_nd").set(Val::from(21))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Agent",
                args![
                    "I don't know who these",
                    "pranksters are, but once",
                    "I get my hands on them...",
                    "I'm gonna--! I'm gonna--!"
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "God, I want a drink.",
                    "Listen, would you just",
                    "go report to the Rogue",
                    "Guild for me? I know it's",
                    "my responsibility, but...",
                    "It's something for you to do."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Agent", args!["Heh! Keep up", "the good work~"])?;
            ctx.var("rumour_nd").set(Val::from(21))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("rumour_nd").get()? == 21 {
        ctx.lines_as(
            "Agent",
            args![
                "Hurry up, and report",
                "for me to the Rogue",
                "Guild. While you do that,",
                "I think I'll enjoy a drink~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Agent", args!["*Hiccup*", "Ahahaha,", "this feels good!", "Good ol' beeeer~"])?;
        ctx.next()?;
        ctx.lines_as("Agent", args!["*Hiccup*"])?;
        ctx.next()?;
        ctx.lines_as("Agent", args!["What are you?", "Get lost!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn rogue_guild_agent_nd4(ctx: &Ctx) -> Script {
    rogue_guild_agent_nd4_body(ctx, Vec::new()).map(|_| ())
}
