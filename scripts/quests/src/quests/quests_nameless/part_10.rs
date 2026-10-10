use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn the_wanderer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("jewel_nd").get()?.number()? < 21 {
        ctx.lines_as(
            "The Wanderer",
            args![
                "I'm just a wanderer",
                "drifting through this",
                "world. Please just",
                "leave me in peace."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("jewel_nd").get()? == 21 {
            if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "That... That looks",
                        "just like the emerald",
                        "I used to own. Hmmm...",
                        "Did you come looking for me?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "Get rid of that thing right",
                        "now. It's an evil jewel that",
                        "causes tragedy wherever",
                        "it goes. I remember... I got",
                        "that jewel from monsters..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "I was just a simple man,",
                        "dreaming of becoming the",
                        "world's greatest warrior.",
                        "I had that emerald on me",
                        "when we encountered a huge",
                        "group of vicious Cobolds."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "No matter how many",
                        "we killed, there was",
                        "just no end to them...",
                        "And then... We... We...",
                        "I... It hurts too much",
                        "to recall what happened..."
                    ],
                )?;
                ctx.var("jewel_nd").set(Val::from(22))?;
                ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "Why have you summoned me?",
                        "I'm just the soul of a fallen",
                        "warrior, seeking peace.",
                        "Please don't bother me",
                        "if you can avoid it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("jewel_nd").get()? == 22 {
                ctx.lines_as(
                    "The Wanderer",
                    args![
                        "You again? Why do you still",
                        "hold the jewel?! Didn't I just",
                        "what happened to me and",
                        "my comrades! We all died",
                        "that day, that day the",
                        "Cobolds overpowered us!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("The Wanderer", args!["Just let me return", "to my slumber."])?;
                ctx.var("jewel_nd").set(Val::from(23))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("jewel_nd").get()? == 25 {
                    ctx.lines_as(
                        "The Wanderer",
                        args![
                            "Why have you called",
                            "me here again? What",
                            "is it you want from me?",
                            "Can't you leave me alone?"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Were you really killed by Cobolds?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("The Wanderer", args!["Argh! What is it you're", "trying to say?! Leave now!"])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("jewel_nd").get()? == 26 {
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "Still you pester me.",
                                "Unfortunately, the power",
                                "of the jewel compels me",
                                "to return to this plane...",
                                "What is it this time?"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("You weren't killed by Cobolds, were you?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "How many times must I tell",
                                "you? Are you saying that",
                                "I'm lying about the deaths",
                                "of my comrades? Forget it.",
                                "The dead shouldn't talk.",
                                "It's a waste of my time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "Take care, young",
                                "adventurer, and don't",
                                "reopen the wounds of",
                                "the past anymore."
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(27))?;
                        ctx.next()?;
                        'l1: loop {
                            if !(true) {
                                break 'l1;
                            }
                            'b1: {
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("You killed your comrades!:Do you remember your last moment?")],
                                )?) == 1
                                {
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "Did I...? Ha. Haha!",
                                            "adventurer, but that sounds",
                                            "like something from a trashy",
                                            "novel. Hah hah hah hah!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                } else {
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "It's a little hazy...",
                                            "I... We tried to kill as",
                                            "many Cobolds as we ",
                                            "could... Everyone was...",
                                            "Everyone was dead.",
                                            "Everyone but me."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "I was all alone.",
                                            "We killed all the",
                                            "Cobolds, but I was...",
                                            "Then... The next thing...",
                                            "I was covered in blood.",
                                            "Then, something appeared?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("The Wanderer", args!["No, it can't have been.", "A human soldier like me?"])?;
                                    ctx.var("jewel_nd").set(Val::from(28))?;
                                    ctx.next()?;
                                    break 'l1;
                                }
                            }
                        }
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I was killed... No, wait.",
                                "I survived the battle with",
                                "the Cobolds. Then, some",
                                "coward killed me from",
                                "behind, and stole my",
                                "emerald. That must be it!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("How did your comrades die?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "The Cobolds killed them!",
                                "I was too weak to protect",
                                "my comrades. Too weak...",
                                "When I came back to my",
                                "senses, they were all..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I'm not sure what",
                                "happened. There was",
                                "blood coming into my eyes...",
                                "I remember slashing, and",
                                "stabbing, and... And slashing."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("You slashed everything...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["You slashed everything...", "Even the comrades you", "were supposed to protect."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I... I did...?",
                                "I... No! Shut up!",
                                "Stop it! I couldn't...",
                                "Get out of here now!",
                                "You're trying to trick me!"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(29))?;
                        ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("jewel_nd").get()? == 27 {
                        ctx.lines_as(
                            "The Wanderer",
                            args!["What is it?", "What could you", "possibly have to", "say to me this time?"],
                        )?;
                        ctx.next()?;
                        'l2: loop {
                            if !(true) {
                                break 'l2;
                            }
                            'b2: {
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("You killed your comrades!:Do you remember your last moment?")],
                                )?) == 1
                                {
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "Did I...? Ha. Haha!",
                                            "adventurer, but that sounds",
                                            "like something from a trashy",
                                            "novel. Hah hah hah hah!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                } else {
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "It's a little hazy...",
                                            "I... We tried to kill as",
                                            "many Cobolds as we ",
                                            "could... Everyone was...",
                                            "Everyone was dead.",
                                            "Everyone but me."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "The Wanderer",
                                        args![
                                            "I was all alone.",
                                            "We killed all the",
                                            "Cobolds, but I was...",
                                            "Then... The next thing...",
                                            "I was covered in blood.",
                                            "Then, something appeared?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("The Wanderer", args!["No, it can't have been.", "A human soldier like me?"])?;
                                    ctx.var("jewel_nd").set(Val::from(28))?;
                                    ctx.next()?;
                                    break 'l2;
                                }
                            }
                        }
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I was killed... No, wait.",
                                "I survived the battle with",
                                "the Cobolds. Then, some",
                                "coward killed me from",
                                "behind, and stole my",
                                "emerald. That must be it!"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("How did your comrades die?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "The Cobolds killed them!",
                                "I was too weak to protect",
                                "my comrades. Too weak...",
                                "When I came back to my",
                                "senses, they were all..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I'm not sure what",
                                "happened. There was",
                                "blood coming into my eyes...",
                                "I remember slashing, and",
                                "stabbing, and... And slashing."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("You slashed everything...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["You slashed everything...", "Even the comrades you", "were supposed to protect."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I... I did...?",
                                "I... No! Shut up!",
                                "Stop it! I couldn't...",
                                "Get out of here now!",
                                "You're trying to trick me!"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(29))?;
                        ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("jewel_nd").get()? == 28 {
                        ctx.lines_as("The Wanderer", args!["Why you..."])?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("How did your comrades die?")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "The Cobolds killed them!",
                                "I was too weak to protect",
                                "my comrades. Too weak...",
                                "When I came back to my",
                                "senses, they were all..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I'm not sure what",
                                "happened. There was",
                                "blood coming into my eyes...",
                                "I remember slashing, and",
                                "stabbing, and... And slashing."
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("You slashed everything...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["You slashed everything...", "Even the comrades you", "were supposed to protect."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "The Wanderer",
                            args![
                                "I... I did...?",
                                "I... No! Shut up!",
                                "Stop it! I couldn't...",
                                "Get out of here now!",
                                "You're trying to trick me!"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(29))?;
                        ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("jewel_nd").get()? == 29 {
                        ctx.lines_as("The Wanderer", args!["Leave me alone!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("jewel_nd").get()?.number()? > 29 {
                        ctx.lines_as("The Wanderer", args!["Shut up! Stop harassing", "me! I can't... I don't...!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("The Wanderer", args!["........."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn the_wanderer(ctx: &Ctx) -> Script {
    the_wanderer_body(ctx, Vec::new()).map(|_| ())
}

fn the_wanderer_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
    return Err(Stop::End);
}

pub fn the_wanderer_oninit(ctx: &Ctx) -> Script {
    the_wanderer_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn the_wanderer_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("The Wanderer")])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn the_wanderer_onenable(ctx: &Ctx) -> Script {
    the_wanderer_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn the_wanderer_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("The Wanderer")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn the_wanderer_ontimer60000(ctx: &Ctx) -> Script {
    the_wanderer_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn callghost_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn callghost(ctx: &Ctx) -> Script {
    callghost_body(ctx, Vec::new()).map(|_| ())
}

fn callghost_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("jewel_nd").get()? == 21 {
        if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["This must be the place", "that shaman was talking", "about. Let's see now..."],
            )?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("The Wanderer::OnEnable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "This must be the place",
                    "that shaman was talking",
                    "about. Let's see now...",
                    "Wait. I need that emerald..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (ctx.var("jewel_nd").get()?.number()? > 21 && ctx.var("jewel_nd").get()?.number()? < 29) {
        ctx.call(Function::DoNpcEvent, vec![Val::from("The Wanderer::OnEnable")])?;
    }
    return Err(Stop::End);
}

pub fn callghost_ontouch(ctx: &Ctx) -> Script {
    callghost_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn man_from_morocc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rumour_nd").get()? == 0 {
        ctx.lines_as(
            "Morocc Traveler",
            args![
                "Ah, it's very nice and",
                "refreshing to be near this",
                "fountain. It's much cooler",
                "here than back in Morocc."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Nice day today, huh?:How's Morocc lately?")])? {
            1 => {
                ctx.lines_as(
                    "Morocc Traveler",
                    args![
                        "Oh, yes. It's a wonderful",
                        "day! Prontera is so sunny",
                        "and cool. No wonder so",
                        "many people live over here!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Morocc Traveler",
                    args![
                        "You're an adventurer, and",
                        "you haven't heard the crazy",
                        "rumors spreading all over",
                        "the Rune-Midgarts kingdom?",
                        "I thought you guys traveled",
                        "around and heard things."
                    ],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
        let choice = runtime::select_values(ctx, &[Val::from("What crazy rumors?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Morocc Traveler",
            args![
                "Uh... Rogue agents!",
                "They're all over the place!",
                "Why don't you talk to some",
                "of the Rogue agents dispatched",
                "in the other towns? Then you'll",
                "know what I'm talking about!"
            ],
        )?;
        ctx.var("rumour_nd").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rumour_nd").get()? == 1 {
        ctx.lines_as(
            "Morocc Traveler",
            args![
                "Shouldn't you be",
                "looking for a Rogue",
                "agent to talk to?",
                "I mean, maybe you",
                "can pitch in and",
                "help them out."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rumour_nd").get()?.number()? > 1 {
        if ctx.var("zdan_edq").get()?.number()? > 12 {
            ctx.lines_as(
                "Morocc Traveler",
                args![
                    "So the Z Gang was what",
                    "those crazy rumors were",
                    "all about. Good to hear",
                    "that they're all in jail."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Morocc Traveler",
                args!["Hey, uh...", "I'm just trying to", "relax on my vacation."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn man_from_morocc(ctx: &Ctx) -> Script {
    man_from_morocc_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_agent_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Guildsman",
            args![
                "You're carrying too",
                "much stuff on you.",
                "Take a load off, put",
                "your extra stuff in",
                "Kafra Storage: that's",
                "what's it's for~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rumour_nd").get()? == 0 {
        if ctx.var("zdan_edq").get()? == 0 {
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Man, it feels much safer",
                        "hanging out in an alley",
                        "than some crowded joint.",
                        "Us Rogues... I guess you",
                        "can say we got a bit of",
                        "agoraphobia, naturally."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Hey, I don't think",
                        "you're supposed to be",
                        "here. You better get",
                        "going on your way.",
                        "It's not a threat, just",
                        "a friendly word of advice."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("zdan_edq").get()?.number()? > 0 && ctx.var("zdan_edq").get()?.number()? < 13) {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Hey, I don't think",
                        "I can help you out.",
                        "You're pretty much",
                        "on your own from here."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Huh. I hate clean-up",
                        "duty. It's always better",
                        "to make messes than to",
                        "hang around afterwards."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if ctx.var("rumour_nd").get()? == 1 {
            ctx.lines_as(
                "Guildsman",
                args![
                    "Yeah, we're investigating",
                    "the strange crimes that are",
                    "going on around the kingdom.",
                    "How'd you know about that?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I was just...:Can I help?")])? {
                1 => {
                    ctx.lines_as("Guildsman", args!["Just what...?", "Speak up, pal."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    if ctx.var("zdan_edq").get()?.number()? > 10 {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "So you wanted to help",
                                "us out? Heck, we could",
                                "use the help, I guess.",
                                "I need you to go on a bit",
                                "of a trip for me, so get",
                                "some provisions ready, yeah?"
                            ],
                        )?;
                        ctx.var("rumour_nd").set(Val::from(2))?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("rumour_nd").get()? == 2 {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "You all ready and",
                        "everything? Good.",
                        "Now, I need you to",
                        "deliver some intel to",
                        "the other Rogues for me.",
                        "Think you can do that?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "First, I want you to",
                        "go to Al De Baran.",
                        "Find our guy, and help",
                        "him out any way you can.",
                        "Us Rogues are hard to",
                        "find, but you'll figure it out."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "I know that's probably",
                        "not enough instruction,",
                        "but that's just the first",
                        "step, anyway. Good luck."
                    ],
                )?;
                ctx.var("rumour_nd").set(Val::from(3))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rumour_nd").get()? == 3 {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Hey, you gotta get",
                        "goin' to Al De Baran,",
                        "and find one of us Rogues",
                        "working on the investigation.",
                        "I thought you wanted to help?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Here's a bit of a tip~",
                        "Us Rogue agents usually",
                        "try to hide out in the",
                        "south side of towns.",
                        "Bit of a tradition."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rumour_nd").get()? == 24 {
                ctx.lines_as(
                    "Guildsman",
                    args!["What's this, a report", "from Al De Baran? 'Kay,", "let me give it a read..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Huh, okay. Looks like",
                        "your next stop is gonna",
                        "be in Geffen. Look for our",
                        "Geffen agent somewhere in",
                        "the South Geffen area, okay?",
                        "Ah, and take this note too."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3355FFYou received", "another folded note.^000000"])?;
                ctx.var("rumour_nd").set(Val::from(25))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rumour_nd").get()? == 25 {
                ctx.lines_as(
                    "Guildsman",
                    args![
                        "Hey, why haven't you",
                        "left for Geffen? Our",
                        "agent oughta be in the",
                        "south side of town.",
                        "It's not far, so don't",
                        "throw a hissy fit."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rumour_nd").get()? == 21 {
                if ctx.var("zdan_edq").get()?.number()? > 12 {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Finally, the Z Gang",
                            "case is closed. Those",
                            "guys really gave us some",
                            "trouble. Why the hell did",
                            "they stir up so much ruckus",
                            "in the kingdom, anyhow?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Listen, I wanna thank",
                            "you for pitching in and",
                            "helping out. It's a small",
                            "gift, but maybe you'll",
                            "like it. If not, don't",
                            "lemme know. Got it?"
                        ],
                    )?;
                    ctx.var("rumour_nd").set(Val::from(22))?;
                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Listen, I wanna thank",
                            "you for pitching in and",
                            "helping out. It's a small",
                            "gift, but maybe you'll",
                            "like it. If not, don't",
                            "lemme know. Got it?"
                        ],
                    )?;
                    ctx.var("rumour_nd").set(Val::from(22))?;
                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("rumour_nd").get()? == 22 {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Hey, you were really",
                            "a big help earlier.",
                            "Can we count on you",
                            "again sometime? The",
                            "Rogues can always use",
                            "another friend, you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "I don't know who you're",
                            "lookin' for, but you",
                            "got the wrong guy.",
                            "Listen, I got work",
                            "to do, so..."
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

pub fn rogue_agent(ctx: &Ctx) -> Script {
    rogue_agent_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_agent_nd0_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rumour_nd").get()?.number()? < 3 {
        ctx.lines_as(
            "Agent",
            args![
                "Listen, I'm working on",
                "an important assignment",
                "for the Rogue Guild.",
                "Would let lemme alone",
                "so I can do what I gotta?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rumour_nd").get()? == 3 {
        if ctx.var("zdan_edq").get()?.number()? > 12 {
            ctx.lines_as(
                "Agent",
                args![
                    "They sent you to help me?",
                    "Eh, thanks for comin' so",
                    "far. I've been collecting",
                    "intel here in Al De Baran.",
                    "I'm pretty sure the Z Gang's",
                    "spreading rumors around here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "They've been telling",
                    "people there's gonna be",
                    "a war comin', but... It",
                    "seems like a load of huey.",
                    "The Z Gang's captured so",
                    "the rumor'll just die out..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "Do me a favor, and",
                    "get this report over",
                    "the agent in Geffen?",
                    "Thanks, I'd appreciate it."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou received a", "tightly folded note.^000000"])?;
            ctx.var("rumour_nd").set(Val::from(4))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Agent",
                args![
                    "They sent you to help me?",
                    "Eh, thanks for comin' so",
                    "far. I've been collecting",
                    "intel here in Al De Baran."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "There's this rumor goin'",
                    "around that war's comin'",
                    "but... I don't see any real",
                    "signs of it. Trains, teleports,",
                    "airships, they still keep",
                    "on doin' what they do."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "I'm not sure who, but",
                    "someone's probably just",
                    "spreading this rumor to",
                    "get people to panick.",
                    "It's kinda workin', though..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Agent",
                args![
                    "Do me a favor, and",
                    "get this report over",
                    "the agent in Geffen?",
                    "Thanks, I'd appreciate it."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou received a", "tightly folded note.^000000"])?;
            ctx.var("rumour_nd").set(Val::from(24))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if (ctx.var("rumour_nd").get()? == 4 || ctx.var("rumour_nd").get()? == 24) {
            ctx.lines_as("Agent", args!["Hey..."])?;
            if ctx.var("rumour_nd").get()? == 4 {
                ctx.lines(args![
                    "Would you please get",
                    "my report over to the",
                    "Rogue agent in Geffen?",
                    "You'd better hurry since",
                    "we gotta work quick."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "Would you please get",
                    "my report over to the",
                    "Rogue Guild? They",
                    "need to read it A.S.A.P.",
                    "You know, pronto."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("rumour_nd").get()? == 16 {
            ctx.lines_as(
                "Agent",
                args![
                    "What's this, a report",
                    "from the Payon agent?",
                    "Um, listen, I don't think",
                    "this is for me. You might",
                    "wanna try one of the other",
                    "Rogue agents around, okay?"
                ],
            )?;
            ctx.var("rumour_nd").set(Val::from(17))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rumour_nd").get()?.number()? > 21 {
            if ctx.var("zdan_edq").get()?.number()? > 12 {
                ctx.lines_as(
                    "Agent",
                    args![
                        "Hey, thanks for",
                        "helping out. I sure",
                        "hope the kingdom can",
                        "enjoy a little peace",
                        "with the Z Gang out",
                        "of the picture."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Agent",
                    args!["Well, we've tracked", "down the Z Gang. All we", "gotta do now is get them!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Agent",
                args!["I still got some", "things to do around", "here. Them's the breaks."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn rogue_agent_nd0(ctx: &Ctx) -> Script {
    rogue_agent_nd0_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_guild_agent_nd1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rumour_nd").get()?.number()? < 4 {
        if ctx.var("zdan_edq").get()?.number()? > 12 {
            ctx.lines_as(
                "Rogue Guild Agent",
                args!["Are you the one that's", "been helping to take", "down the Z Gang? Nice!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Rogue Guild Agent",
                args![
                    "If you got nothing",
                    "to do with me, then...",
                    "I got nothing to do",
                    "with you. Later!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("rumour_nd").get()? == 4 {
            if ctx.var("zdan_edq").get()?.number()? > 12 {
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "This is from the",
                        "Al De Baran agent?",
                        "Thanks! It musta been",
                        "kinda hard to find me, eh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "The Z Gang's been",
                        "spreading rumors about",
                        "ghosts pouring out of",
                        "Geffen Tower. Just to be",
                        "sure, I want you to check",
                        "it out. Will you do that?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "I want you to capture",
                        "some monsters, and get",
                        "some proof that shows that",
                        "the ghosts stay inside the",
                        "tower, and they're not",
                        "coming out. Yeah, bring..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "^FF00001 Fabric^000000,",
                        "^FF00001 Golden Hair^000000,",
                        "^FF00001 Little Evil Horn^000000,",
                        "^FF00001 Horseshoe^000000, and",
                        "^FF00001 Jack O' Pumpkin^000000.",
                        "That oughta do it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "We just need to get",
                        "a little concrete proof",
                        "so that the people can",
                        "calm down, and not panick.",
                        "Hurry it up before a riot",
                        "breaks out or somethin'."
                    ],
                )?;
                ctx.var("rumour_nd").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "This report is from the",
                        "Rogue agent in Al De Baran?",
                        "Hey, this says that this is",
                        "for the Rogue Guild. You",
                        "better take this over there."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("rumour_nd").get()? == 5 {
                ctx.lines_as(
                    "Rogue Guild Agent",
                    args![
                        "Hey, you done hunting",
                        "down the monsters in",
                        "Geffen Tower yet?",
                        "I hope you brought",
                        "the stuff..."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give Items:I forgot what you need.")])? {
                    1 => {
                        if ((((ctx.call(Function::CountItem, vec![Val::from(1062)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(944)])?.number()? > 0)
                            && ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 0)
                            && ctx.call(Function::CountItem, vec![Val::from(1060)])?.number()? > 0)
                            && ctx.call(Function::CountItem, vec![Val::from(1059)])?.number()? > 0)
                        {
                            ctx.call(Function::DelItem, vec![Val::from(1062), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(944), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(1060), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(1059), Val::from(1)])?;
                            ctx.var("rumour_nd").set(Val::from(6))?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Thanks. This stuff",
                                    "should help the people",
                                    "in Geffen calm down a bit.",
                                    "The Z Gang's been doin'",
                                    "all these things to make them",
                                    "believe the monsters are loose."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "They've been leavin'",
                                    "horseprints that look like",
                                    "they belong to Nightmares,",
                                    "dressing up in Fabric and",
                                    "pretending they're Whispers...",
                                    "They're freakin' hooligans."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "I think the Rogue agent",
                                    "in Payon should know about",
                                    "all this. Take this report",
                                    "over to him, will you?",
                                    "Thanks a bundle, pal."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "Hold on, you're",
                                    "missing some stuff.",
                                    "You mind checking if",
                                    "you left some of it",
                                    "behind somewhere?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "^FF00001 Fabric^000000,",
                                "^FF00001 Golden Hair^000000,",
                                "^FF00001 Little Evil Horn^000000,",
                                "^FF00001 Horseshoe^000000, and",
                                "^FF00001 Jack O' Pumpkin^000000.",
                                "Don't forget this time!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else if ctx.var("rumour_nd").get()? == 6 {
                ctx.lines_as(
                    "Agent",
                    args!["Would you deliver", "my report over to", "the Rogue agent", "in Payon? Thanks."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("rumour_nd").get()? == 25 {
                ctx.lines_as(
                    "Agent",
                    args![
                        "Hey, the guild sent",
                        "you over to see me?",
                        "Oh, hey, you brought",
                        "me a message. Let's",
                        "take a look-see~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Agent", args!["Umm..."])?;
                ctx.next()?;
                if ctx.var("zdan_edq").get()?.number()? > 12 {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "So... The Z Gang's",
                            "responsible for those",
                            "rumors about the ghosts",
                            "running loose in Geffen.",
                            "Huh. Those rascally guys.",
                            "Okay, this is what we do."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "I want you to capture",
                            "some monsters, and get",
                            "some proof that shows that",
                            "the ghosts stay inside the",
                            "tower, and they're not",
                            "coming out. Yeah, bring..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "^FF00001 Fabric^000000,",
                            "^FF00001 Golden Hair^000000,",
                            "^FF00001 Little Evil Horn^000000,",
                            "^FF00001 Horseshoe^000000, and",
                            "^FF00001 Jack O' Pumpkin^000000.",
                            "That oughta do it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "We just need to get",
                            "a little concrete proof",
                            "so that the people can",
                            "calm down, and not panick.",
                            "Hurry it up before a riot",
                            "breaks out or somethin'."
                        ],
                    )?;
                    ctx.var("rumour_nd").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "The Z Gang's been",
                            "spreading rumors about",
                            "ghosts pouring out of",
                            "Geffen Tower. Just to be",
                            "sure, I want you to check",
                            "it out. Will you do that?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "I want you to capture",
                            "some monsters, and get",
                            "some proof that shows that",
                            "the ghosts stay inside the",
                            "tower, and they're not",
                            "coming out. Yeah, bring..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "^FF00001 Fabric^000000,",
                            "^FF00001 Golden Hair^000000,",
                            "^FF00001 Little Evil Horn^000000,",
                            "^FF00001 Horseshoe^000000, and",
                            "^FF00001 Jack O' Pumpkin^000000.",
                            "That oughta do it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Rogue Guild Agent",
                        args![
                            "We just need to get",
                            "a little concrete proof",
                            "so that the people can",
                            "calm down, and not panick.",
                            "Hurry it up before a riot",
                            "breaks out or somethin'."
                        ],
                    )?;
                    ctx.var("rumour_nd").set(Val::from(26))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else if ctx.var("rumour_nd").get()? == 26 {
                if ctx.var("zdan_edq").get()?.number()? > 12 {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "Hey, while you were",
                            "gone, I heard the Z Gang",
                            "was finally captured.",
                            "Big relief to hear it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Agent",
                        args![
                            "But I still want to",
                            "reassure the people in",
                            "Geffen that the monsters",
                            "in Geffen Tower are safely",
                            "locked up inside. You",
                            "bring the stuff?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Give Items:I forgot what you need.")])? {
                        1 => {
                            if ((((ctx.call(Function::CountItem, vec![Val::from(1062)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(944)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 1)
                                && ctx.call(Function::CountItem, vec![Val::from(1060)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(1059)])?.number()? > 0)
                            {
                                ctx.call(Function::DelItem, vec![Val::from(1062), Val::from(2)])?;
                                ctx.call(Function::DelItem, vec![Val::from(944), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(2)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1060), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1059), Val::from(1)])?;
                                ctx.var("rumour_nd").set(Val::from(6))?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Thanks. This stuff",
                                        "should help the people",
                                        "in Geffen calm down a bit.",
                                        "The Z Gang's been doin'",
                                        "all these things to make them",
                                        "believe the monsters are loose."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "They've been leavin'",
                                        "horseprints that look like",
                                        "they belong to Nightmares,",
                                        "dressing up in Fabric and",
                                        "pretending they're Whispers...",
                                        "They're freakin' hooligans."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "I think the Rogue agent",
                                        "in Payon should know about",
                                        "all this. Take this report",
                                        "over to him, will you?",
                                        "Thanks a bundle, pal."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Hold on, you're",
                                        "missing some stuff.",
                                        "You mind checking if",
                                        "you left some of it",
                                        "behind somewhere?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Rogue Guild Agent",
                                args![
                                    "^FF00001 Fabric^000000,",
                                    "^FF00001 Golden Hair^000000,",
                                    "^FF00001 Little Evil Horn^000000,",
                                    "^FF00001 Horseshoe^000000, and",
                                    "^FF00001 Jack O' Pumpkin^000000.",
                                    "Don't forget this time!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Agent",
                        args![
                            "I asked you to bring",
                            "me items that prove that",
                            "the monsters in Geffen",
                            "Tower are safely locked",
                            "up in there. You do that yet?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Give Items:I forgot what you need.")])? {
                        1 => {
                            if ((((ctx.call(Function::CountItem, vec![Val::from(1062)])?.number()? > 1
                                && ctx.call(Function::CountItem, vec![Val::from(944)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(1038)])?.number()? > 1)
                                && ctx.call(Function::CountItem, vec![Val::from(1060)])?.number()? > 2)
                                && ctx.call(Function::CountItem, vec![Val::from(1059)])?.number()? > 0)
                            {
                                ctx.call(Function::DelItem, vec![Val::from(1062), Val::from(2)])?;
                                ctx.call(Function::DelItem, vec![Val::from(944), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1038), Val::from(2)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1060), Val::from(3)])?;
                                ctx.call(Function::DelItem, vec![Val::from(1059), Val::from(1)])?;
                                ctx.var("rumour_nd").set(Val::from(6))?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Thanks. This stuff",
                                        "should help the people",
                                        "in Geffen calm down a bit.",
                                        "The Z Gang's been doin'",
                                        "all these things to make them",
                                        "believe the monsters are loose."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "They've been leavin'",
                                        "horseprints that look like",
                                        "they belong to Nightmares,",
                                        "dressing up in Fabric and",
                                        "pretending they're Whispers...",
                                        "They're freakin' hooligans."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "I think the Rogue agent",
                                        "in Payon should know about",
                                        "all this. Take this report",
                                        "over to him, will you?",
                                        "Thanks a bundle, pal."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Agent",
                                    args![
                                        "Hold on, you're",
                                        "missing some stuff.",
                                        "You mind checking if",
                                        "you left some of it",
                                        "behind somewhere?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Agent",
                                args![
                                    "^FF00001 Fabric^000000,",
                                    "^FF00001 Golden Hair^000000,",
                                    "^FF00001 Little Evil Horn^000000,",
                                    "^FF00001 Horseshoe^000000, and",
                                    "^FF00001 Jack O' Pumpkin^000000.",
                                    "Don't forget this time!"
                                ],
                            )?;
                        }
                        _ => {}
                    }
                }
            } else {
                if ctx.var("rumour_nd").get()? == 22 {
                    if ctx.var("zdan_edq").get()?.number()? > 12 {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Looks like there'll",
                                "be some peace and quiet",
                                "with the Z Gang all locked up.",
                                "Those guys were a double",
                                "heaping of trouble if",
                                "you ask me. Huh."
                            ],
                        )?;
                    } else {
                        ctx.lines_as(
                            "Agent",
                            args![
                                "Keep up the good work!",
                                "With guys like you on",
                                "our side, the Z Gang'll",
                                "caught sooner than later."
                            ],
                        )?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Agent", args!["So busy!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn rogue_guild_agent_nd1(ctx: &Ctx) -> Script {
    rogue_guild_agent_nd1_body(ctx, Vec::new()).map(|_| ())
}
