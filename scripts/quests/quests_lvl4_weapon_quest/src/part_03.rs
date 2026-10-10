use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hein_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_aekddam = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_myhand1 = Val::from(0);
    let mut l_myhand2 = Val::from(0);
    let mut l_myhand3 = Val::from(0);
    let mut l_npchand1 = Val::from(0);
    let mut l_npchand2 = Val::from(0);
    let mut l_npchand3 = Val::from(0);
    let mut l_shobu = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as("Hein", args!["It looks like you want to ask me something. Heh, my customers used to have that look on that faces too when I worked as a weaponsmith..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Back during those days,",
                "I never accepted money for my work. Father always used to say, 'Never accept payment to forge a good weapon. It... It...'"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
            1 => {
                ctx.lines_as(
                    "Hein",
                    args!["What was the rest of it?", "Why can't I remember the", "rest of that saying?!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hein",
                    args![
                        "It's so weird.",
                        "How could I forget",
                        "something I used to",
                        "say all the time? I...",
                        "I hate being dead!"
                    ],
                )?;
                ctx.var("lv4_weapon").set(Val::from(40))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Hein", args!["Nuts! Why can't I remember", "the rest of that saying?!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 40 || ctx.var("lv4_weapon").get()?.number()? > 48) {
        ctx.lines_as("Hein", args!["Mm? I can tell you've here because you've got something to do. It's weird enough to see a living being here, and weirder still to see one who's here for a good reason."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Do me a favor",
                "and stay alive.",
                "Take it from me,",
                "there's no pleasure",
                "at all in being dead."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lv4_weapon").get()? == 47 || ctx.var("lv4_weapon").get()? == 48) {
        ctx.lines_as(
            "Hein",
            args![
                "You're here!",
                "I just finished my work!",
                "You want to see it, don't you?",
                "Now, take a deep breath..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as(
                "Hein",
                args![
                    "Haha~!",
                    "This is great!",
                    "Finally, my life's",
                    "work is complete!",
                    "This weapon's name"
                ],
            )?;
            if ctx.var("lv4_weapon").get()? == 47 {
                ctx.mes("is Edge!")?;
                ctx.call(Function::GetItem, vec![Val::from(1132), Val::from(1)])?;
            } else {
                ctx.mes("is Dragon Slayer!")?;
                ctx.call(Function::GetItem, vec![Val::from(1166), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Hein",
                args![
                    "Haha~!",
                    "This is great!",
                    "Finally, my life's",
                    "work is complete!",
                    "This weapon's name"
                ],
            )?;
            if ctx.var("lv4_weapon").get()? == 47 {
                ctx.mes("is Excalibur!")?;
                ctx.call(Function::GetItem, vec![Val::from(1137), Val::from(1)])?;
            } else {
                ctx.mes("is Schweizersabel!")?;
                ctx.call(Function::GetItem, vec![Val::from(1167), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
        }
        ctx.lines_as("Hein", args!["Well...", "It looks like this is goodbye for now. I can already feel my memory getting hazier. Haha, being dead is definitely not good."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Thank you for your help. If you come back and help me retrieve",
                "my memories again, I'll try to help you as well. So long, adventurer..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("lv4_weapon").get()? == 45 && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
        || (((ctx.var("lv4_weapon").get()? == 46 && ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29))
    {
        ctx.lines_as(
            "Hein",
            args![
                "Mm...?",
                "The stuff you brought seems to be just what I need to finish my work. Let me have a look at them..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Wow...",
                "How did you know the materials",
                "I needed? It's almost too good to be true! Alright, we're that much closer to finishing my life's work!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["Oh, that's right. Skill alone isn't enough to create this. We'll also need your luck. First of all, we need to test if your luck is good today."])?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["We're going to play '^660000Rock, Paper, Scissors^000000.' If you win at least 2 out of 3, we'll go ahead and start crafting this thing!"])?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["But if you fail in this game, I'll have to expel your negative luck by throwing out a lot of one of the ores you've brought along."])?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["Here's a piece of paper.", "When I tell you to, write down 'Rock,' 'Paper' or 'Scissors.' I'll do the same thing, and we'll compare our results at the end, okay? Let's begin!"])?;
        ctx.next()?;
        l_npchand1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand1 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand1.clone() == 1 && l_npchand1.clone() == 3) || (l_myhand1.clone() == 2 && l_npchand1.clone() == 1))
            || (l_myhand1.clone() == 3 && l_npchand1.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as("Hein", args!["Okay, let's play", "the second round!"])?;
        ctx.next()?;
        l_npchand2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand2 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand2.clone() == 1 && l_npchand2.clone() == 3) || (l_myhand2.clone() == 2 && l_npchand2.clone() == 1))
            || (l_myhand2.clone() == 3 && l_npchand2.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as(
            "Hein",
            args!["Okay...", "Last round.", "After this, we", "compare our results."],
        )?;
        ctx.next()?;
        l_npchand3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand3 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        l_aekddam = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if (((l_myhand3.clone() == 1 && l_npchand3.clone() == 3) || (l_myhand3.clone() == 2 && l_npchand3.clone() == 1))
            || (l_myhand3.clone() == 3 && l_npchand3.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        if l_shobu.clone().number()? > 1 {
            if ctx.var("lv4_weapon").get()? == 45 {
                ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7297), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
            } else {
                ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
            }
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(2)))?;
        } else if l_shobu.clone().number()? < 2 {
            if l_aekddam.clone() == 1 {
                if ctx.var("lv4_weapon").get()? == 45 {
                    ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                } else {
                    ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
                }
            } else {
                if l_aekddam.clone() == 2 {
                    if ctx.var("lv4_weapon").get()? == 45 {
                        ctx.call(Function::DelItem, vec![Val::from(7297), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                    }
                } else if l_aekddam.clone() == 3 {
                    if ctx.var("lv4_weapon").get()? == 45 {
                        ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
                    }
                }
            }
        }
        ctx.lines_as("Hein", args!["Ready? This is", "what I wrote down..."])?;
        if l_npchand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        ctx.lines_as("Hein", args!["You wrote down", "these answers in", "this order..."])?;
        if l_myhand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        if l_shobu.clone().number()? > 1 {
            ctx.lines_as(
                "Hein",
                args![
                    "Let's see...",
                    ((Val::from("You won ") + ctx.var("shobu").get()?) + Val::from(" times.")),
                    "You're really good at game!",
                    "Yes, your luck is at its highest!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args!["As promised, I will craft a weapon for you. Give me the materials now, and come back when I'm finished preparing."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_shobu.clone().number()? < 2 {
            ctx.lines_as(
                "Hein",
                args![
                    "Well, you lost.",
                    "I'm sorry, but we",
                    "need to expel the",
                    "misfortune around you",
                    "with the ore you've brought..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args![
                    "Okay, I'll wait here.",
                    "Come back with the materials",
                    "I need and we'll try this again."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 44 || ctx.var("lv4_weapon").get()? == 45) || ctx.var("lv4_weapon").get()? == 46) {
        ctx.lines_as("Hein", args!["Would you give me more time to remember the materials? I can't seem to focus very well. It might have to do with the fact that I'm dead..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args!["I had a much better memory when I was alive I'm sure. My father would quiz me on this stuff all the time..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("lv4_weapon").get()? == 43 && ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 1)
        && ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 19)
    {
        ctx.lines_as(
            "Hein",
            args!["Hey, you brought everything I asked for already. You must be ready to get started!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Sadly, we can't begin until",
                "I remember the rest of the things we need. I can't for the life of me remember the most important materials..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["Well, for now, let me keep the things you've brought to me. I promise that I'll use this stuff to make you a good weapon once everything is ready."])?;
        ctx.call(Function::DelItem, vec![Val::from(1005), Val::from(2)])?;
        ctx.call(Function::DelItem, vec![Val::from(989), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(710), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(20)])?;
        ctx.var("lv4_weapon").set(Val::from(44))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 43 {
        ctx.lines_as(
            "Hein",
            args![
                "For sure I'll need",
                "2 Hammer of Blacksmith,",
                "1 Emperium Anvil,",
                "1 Illusion Flower and",
                "20 Gold..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args!["So for now,", "go ahead and", "bring that stuff", "to me, please."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 42 {
        ctx.lines_as(
            "Hein",
            args![
                "I take it that you",
                "really want me to make",
                "you a weapon. For a former",
                "weaponsmith like me, that in",
                "itself is a great compliment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Alright, I'll need...",
                "1 Emperium Anvil,",
                "2 Hammer of Blacksmith,",
                "1 Illusion Flower and",
                "20 Gold..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "There were 3 other things,",
                "but I'm not too sure what they were. I guess a bunch of memories from my life are still missing."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hein", args!["^333333*Sigh*^000000 I'm sorry to ask this of you, but would you please go out and figure out what those things are? I'm sure you can find some clue in the land of the living."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args!["While you do that, I'll try my best to remember what else I need to make this weapon."],
        )?;
        ctx.var("lv4_weapon").set(Val::from(43))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 41 {
        ctx.lines_as("Hein", args!["It looks like you want to ask me something. Heh, my customers used to have that look on that faces too when I worked as a weaponsmith..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Back during those days,",
                "I never accepted money for my work. Father always used to say, 'Never accept payment to forge a good weapon. It... It...'"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "It brings bad luck." {
            ctx.lines_as(
                "Hein",
                args!["That's right!", "And you need luck when you're creating high quality weapons!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args![
                    "Oh... crap.",
                    "I'm a dead man.",
                    "I just realized.",
                    "Or remembered.",
                    "Perhaps I'm brain",
                    "dead too, huh?"
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(42))?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args!["Well, I must be a lucky dead man for remembering some of my past, right? Hahahaha!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Hein", args!["Right, I died while trying to finish my weapon research. Maybe this is fate's way of allowing me to finish my uncompleted work?"])?;
            ctx.next()?;
            ctx.lines_as("Hein", args!["Hey, let me ask you a favor. Would you bring the materials I need to finish my research? If I succeed, the finished product is yours to keep. All yours!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args![
                    "Take your time",
                    "and think about it.",
                    "It can't be that bad",
                    "of a deal... Right?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Hein",
                args![
                    "Wha...?",
                    "What are you talking about?",
                    "What was I going to say?",
                    "Oww... My head hurts!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 40 {
        ctx.lines_as("Hein", args!["It looks like you want to ask me something. Heh, my customers used to have that look on that faces too when I worked as a weaponsmith..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "Back during those days,",
                "I never accepted money for my work. Father always used to say, 'Never accept payment to forge a good weapon. It... It...'"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args!["What was the rest of it?", "Why can't I remember the", "rest of that saying?!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hein",
            args![
                "It's so weird.",
                "How could I forget",
                "something I used to",
                "say all the time? I...",
                "I hate being dead!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Hein", args!["It looks like you want to ask me something. Heh, my customers used to have that look on that faces too when I worked as a weaponsmith..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Hein",
        args![
            "Back during those days,",
            "I never accepted money for my work. Father always used to say, 'Never accept payment to forge a good weapon. It... It...'"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
        1 => {
            ctx.lines_as(
                "Hein",
                args!["What was the rest of it?", "Why can't I remember the", "rest of that saying?!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hein",
                args![
                    "It's so weird.",
                    "How could I forget",
                    "something I used to",
                    "say all the time? I...",
                    "I hate being dead!"
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(40))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Hein", args!["Nuts! Why can't I remember", "the rest of that saying?!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn hein_lv4(ctx: &Ctx) -> Script {
    hein_lv4_body(ctx, Vec::new()).map(|_| ())
}

fn waltboughst_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_aekddam = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_myhand1 = Val::from(0);
    let mut l_myhand2 = Val::from(0);
    let mut l_myhand3 = Val::from(0);
    let mut l_npchand1 = Val::from(0);
    let mut l_npchand2 = Val::from(0);
    let mut l_npchand3 = Val::from(0);
    let mut l_shobu = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? < 80 {
        ctx.lines_as("Waltboughst", args!["Hah...!", "A living person!", "But not for long!"])?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["I hope you came here with some strong allies. Judging from your strength, there's no way you'll survive in this kind of place on", "your own, you know."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as(
            "Waltboughst",
            args![
                "It's so frustrating being dead.",
                "I can't do anything without a body, no matter how hard I try."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["It's funny, you know. My father always used to say, 'Trying your best won't always result in success...' Um, that's ... Huh? There was more, I think."])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
            1 => {
                ctx.lines_as(
                    "Waltboughst",
                    args![
                        "Let me try it again.",
                        "'Trying your best won't",
                        "always result in success...'",
                        "I'm sure that's not all of it..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Waltboughst",
                    args!["I...", "I can't think!", "What was supposed", "to come after that part?"],
                )?;
                ctx.var("lv4_weapon").set(Val::from(49))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Waltboughst",
                    args![
                        "Let me try it again.",
                        "'Trying your best won't",
                        "always result in success...'",
                        "I'm sure that's not all of it..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 49 || ctx.var("lv4_weapon").get()?.number()? > 57) {
        ctx.lines_as("Waltboughst", args!["It doesn't look like you're just wandering around. You're here for something specific, aren't you? But what does the realm of the dead have to offer the living?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lv4_weapon").get()? == 56 || ctx.var("lv4_weapon").get()? == 57) {
        ctx.lines_as(
            "Waltboughst",
            args![
                "Take a look at this!",
                "In just a short time, I made something with the materials you've given me. You're curious what the result is, aren't you?"
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as("Waltboughst", args!["Waltboughst never fails!"])?;
            if ctx.var("lv4_weapon").get()? == 56 {
                ctx.mes("With my skill and your luck, there was zero probability for failure. Behold... Byeollungum!")?;
                ctx.call(Function::GetItem, vec![Val::from(1140), Val::from(1)])?;
            } else {
                ctx.mes("With my skill and your luck, there was zero probability for failure. Behold... Exorcise!")?;
                ctx.call(Function::GetItem, vec![Val::from(1233), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "I'm a genius!",
                    "Hahahahahahahaha!",
                    "I succeeded where my brothers",
                    "could not! Mwahahahaha ha ha ...ha?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["Damn, I can feel that fuzziness in my head again. This can't be good. But still, I was able to prove my genius even after I died."])?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["I'm sure that if you come back and help me retrieve my memories, I can be of service to you again. But for now, I must thank you, even if I do end up forgetting everything..."])?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["Yes...", "It will be your job to remember the accomplishments of Waltboughst, and that he was a weaponsmith ahead of his time. Good bye for now, adventurer..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Waltboughst", args!["Waltboughst never fails!"])?;
            if ctx.var("lv4_weapon").get()? == 56 {
                ctx.mes("With my skill and your luck, there was zero probability for failure. Behold... Combat Knife!")?;
                ctx.call(Function::GetItem, vec![Val::from(1228), Val::from(1)])?;
            } else {
                ctx.mes("With my skill and your luck, there was zero probability for failure. Behold...Grand Cross!!")?;
                ctx.call(Function::GetItem, vec![Val::from(1528), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "I'm a genius!",
                    "Hahahahahahahaha!",
                    "I succeeded where my brothers",
                    "could not! Mwahahahaha ha ha ...ha?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["Damn, I can feel that fuzziness in my head again. This can't be good. But still, I was able to prove my genius even after I died."])?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["I'm sure that if you come back and help me retrieve my memories, I can be of service to you again. But for now, I must thank you, even if I do end up forgetting everything..."])?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["Yes...", "It will be your job to remember the accomplishments of Waltboughst, and that he was a weaponsmith ahead of his time. Good bye for now, adventurer..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((((ctx.var("lv4_weapon").get()? == 54 && ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29)
        || (((ctx.var("lv4_weapon").get()? == 55 && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29))
    {
        ctx.lines_as(
            "Waltboughst",
            args![
                "Wait, what's that you're holding?",
                "I think those are the exact things we need for me to make you a weapon..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args![
                "Yes...!",
                "This is perfect.",
                "It looks like I'll",
                "be able to start work",
                "on this really soon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["Now we need to test your luck. And to do that, we need to play '^660000Rock, Paper, Scissors^000000.' I'm serious about this..."])?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["If your luck is strong today, there'll be less risk of failing the weapon creation process. You need to win 2 out of 3 times for us to begin."])?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args![
                "But if you lose...",
                "We'll need to expel your bad luck by getting rid of a good amount of one of the ores you brought."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args![
                "Take this piece of paper.",
                "Now, when I tell you to, write down 'Rock,' 'Paper' or 'Scissors.' Let's begin...!"
            ],
        )?;
        ctx.next()?;
        l_npchand1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand1 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand1.clone() == 1 && l_npchand1.clone() == 3) || (l_myhand1.clone() == 2 && l_npchand1.clone() == 1))
            || (l_myhand1.clone() == 3 && l_npchand1.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as("Waltboughst", args!["Alright...", "Now, let's play", "a second time."])?;
        ctx.next()?;
        l_npchand2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand2 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand2.clone() == 1 && l_npchand2.clone() == 3) || (l_myhand2.clone() == 2 && l_npchand2.clone() == 1))
            || (l_myhand2.clone() == 3 && l_npchand2.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as(
            "Waltboughst",
            args!["One last time.", "After this, we", "compare results", "and check your luck."],
        )?;
        ctx.next()?;
        l_npchand3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand3 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand3.clone() == 1 && l_npchand3.clone() == 3) || (l_myhand3.clone() == 2 && l_npchand3.clone() == 1))
            || (l_myhand3.clone() == 3 && l_npchand3.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        l_aekddam = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if l_shobu.clone().number()? > 1 {
            if ctx.var("lv4_weapon").get()? == 54 {
                ctx.call(Function::DelItem, vec![Val::from(7292), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
            } else {
                ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
            }
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(2)))?;
        } else if l_shobu.clone().number()? < 2 {
            if l_aekddam.clone() == 1 {
                if ctx.var("lv4_weapon").get()? == 54 {
                    ctx.call(Function::DelItem, vec![Val::from(7292), Val::from(30)])?;
                } else {
                    ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                }
            } else {
                if l_aekddam.clone() == 2 {
                    if ctx.var("lv4_weapon").get()? == 54 {
                        ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                    }
                } else if l_aekddam.clone() == 3 {
                    if ctx.var("lv4_weapon").get()? == 54 {
                        ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
                    }
                }
            }
        }
        ctx.lines_as("Waltboughst", args!["Okay...", "I wrote down..."])?;
        if l_npchand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["Now, you wrote..."])?;
        if l_myhand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        if l_shobu.clone().number()? > 1 {
            ctx.lines_as(
                "Waltboughst",
                args![
                    ((Val::from("Excellent. You won ") + ctx.var("shobu").get()?) + Val::from(" times,")),
                    "so your luck must be really high.",
                    "That means we can begin~!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Leave the ores here and give",
                    "me some time to complete my preparations. Come back later, and I should be finished."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_shobu.clone().number()? < 2 {
            ctx.lines_as(
                "Waltboughst",
                args![
                    "You lost...",
                    "I'm sorry, but we",
                    "need to repel the bad",
                    "luck by throwing away",
                    "some of your ores..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Gather the required ores again,",
                    "and come back to me so we can test your luck. I'll be waiting right here."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 53 || ctx.var("lv4_weapon").get()? == 54) || ctx.var("lv4_weapon").get()? == 55) {
        ctx.lines_as(
            "Waltboughst",
            args![
                "Why can't I remember...?!",
                "A good weaponsmith is supposed",
                "to know these things by heart!",
                "When I was alive, I had no such recall problems..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args!["If Father were", "to see me now, I'm", "sure he'd be ashamed..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("lv4_weapon").get()? == 52 && ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 1)
        && ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 19)
    {
        ctx.lines_as(
            "Waltboughst",
            args![
                "You brought everything I needed already? That's great, but somehow, I can't still remember the other materials I need..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["Tell you what. Let me hold on to these things, and I promise you that I'll use them to create a weapon for you. In the meantime, we need to figure out the rest of the necessary materials..."])?;
        ctx.call(Function::DelItem, vec![Val::from(1005), Val::from(2)])?;
        ctx.call(Function::DelItem, vec![Val::from(989), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(710), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(20)])?;
        ctx.var("lv4_weapon").set(Val::from(53))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 52 {
        ctx.lines_as(
            "Waltboughst",
            args![
                "For now, bring",
                "the basic things...",
                "1 Emperium Anvil,",
                "2 Hammer of Blacksmith,",
                "1 Illusion Flower and",
                "20 Gold..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 51 {
        ctx.lines_as(
            "Waltboughst",
            args!["I just remembered some of the most basic materials we need for me to make a weapon for you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args![
                "For now, bring",
                "the basic things...",
                "1 Emperium Anvil,",
                "2 Hammer of Blacksmith,",
                "1 Illusion Flower and",
                "20 Gold..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args!["There were a few other necessary materials, but I need some time to focus and remember what they were."],
        )?;
        ctx.var("lv4_weapon").set(Val::from(52))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 50 {
        ctx.lines_as(
            "Waltboughst",
            args![
                "It's so frustrating being dead.",
                "I can't do anything without a body, no matter how hard I try."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["It's funny, you know. My father always used to say, 'Trying your best won't always result in success...' Um, that's ... Huh? There was more, I think."])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "But that's no excuse!" {
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Yes! That was it!",
                    "But that's no excuse!",
                    "Every successful man in",
                    "history has tried his best!"
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(51))?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "I remember now...",
                    "I got myself killed trying to make some kind of special weapon.",
                    "Heh heh, but not this time!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "We must have been fated to meet. Yes, adventurers of great strength and men of genius naturally attract each other."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Lend me your power.",
                    "Help me complete the experiment",
                    "I left unfinished in life, and I will let you keep the finished product. All I wish to do is complete my life's work."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Waltboughst", args!["It seems that being deceased has impaired my rententive faculties. Give me some time to remember the things I'll need to conduct my experiment. We shall talk later."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Waltboughst", args!["What...?", "No, no. That", "wasn't it at all...", "Hmmm."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 49 {
        ctx.lines_as(
            "Waltboughst",
            args![
                "It's so frustrating being dead.",
                "I can't do anything without a body, no matter how hard I try."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Waltboughst", args!["It's funny, you know. My father always used to say, 'Trying your best won't always result in success...' Um, that's ... Huh? There was more, I think."])?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args![
                "Let me try it again.",
                "'Trying your best won't",
                "always result in success...'",
                "I'm sure that's not all of it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Waltboughst",
            args!["I...", "I can't think!", "What was supposed", "to come after that part?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Waltboughst",
        args![
            "It's so frustrating being dead.",
            "I can't do anything without a body, no matter how hard I try."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Waltboughst", args!["It's funny, you know. My father always used to say, 'Trying your best won't always result in success...' Um, that's ... Huh? There was more, I think."])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
        1 => {
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Let me try it again.",
                    "'Trying your best won't",
                    "always result in success...'",
                    "I'm sure that's not all of it..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Waltboughst",
                args!["I...", "I can't think!", "What was supposed", "to come after that part?"],
            )?;
            ctx.var("lv4_weapon").set(Val::from(49))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Waltboughst",
                args![
                    "Let me try it again.",
                    "'Trying your best won't",
                    "always result in success...'",
                    "I'm sure that's not all of it..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn waltboughst_lv4(ctx: &Ctx) -> Script {
    waltboughst_lv4_body(ctx, Vec::new()).map(|_| ())
}
