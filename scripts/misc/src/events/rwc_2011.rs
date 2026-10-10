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

pub fn rwc2011_agent_2(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0
        || ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 500
    {
        ctx.lines_as(
            "New Agent",
            args!["You have too many items. Please make room in your inventory and come back."],
        )?;
        return ctx.close();
    }
    if ctx.var("oversea_event").get()? == 0 {
        ctx.lines_as("New Agent", args!["Wow~ Finally!! It's the season we've all been waiting for!"])?;
        ctx.npc().emotion(constants::ET_BIGTHROB)?;
        ctx.next()?;
        ctx.lines_as("New Agent", args!["It's RWC time!"])?;
        ctx.next()?;
        loop {
            match ctx.menu(&["What is RWC?", "Are you a New Agent?", "I don't care."])? {
                0 => {
                    ctx.mes("[New Agent]")?;
                    if ctx.player().base_level()? < 70 {
                        ctx.mes("You must be a new adventurer. I will kindly explain it to you.")?;
                    } else {
                        ctx.mes("It seems you've traveled quite enough but you're not good with hearing the news around the world.")?;
                    }
                    ctx.next()?;
                    ctx.lines_as("New Agent", args!["There are few adventurers in Midgard whose lives are dedicated to battle, who never skip their training, live in seclusion, and are experts in their art."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "New Agent",
                        args!["The RWC is the festival where those adventurers can compete with each other and find out who is the best!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "New Agent",
                        args!["But this cannot be achieved all alone. Your friends will have a huge role in the competition."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "New Agent",
                        args!["If you're interested in this festival, start looking for friends you can trust!"],
                    )?;
                    ctx.next()?;
                }
                1 => {
                    ctx.lines_as("New Agent", args!["Many events are organized for RWC promotion every year."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "New Agent",
                        args!["And this year, a special mission has been entrusted to a new agent... myself!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("New Agent", args!["I am supposed to attach blue cards with the letters RWC2011 all over the world to promote the event. But... but..."])?;
                    ctx.next()?;
                    ctx.mes("- He thought of something and then looked at your eyes. -")?;
                    ctx.next()?;
                    ctx.lines_as("New Agent", args!["Would you like to listen to my story?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Sure, let me hear it.", "No, thank you."])? == 1 {
                        ctx.lines_as(
                            "New Agent",
                            args!["Arrgg!! Are you ignoring me because I'm new? Usually people listen..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("- New Agent is looking at you with pitiable eyes. -")?;
                        ctx.next()?;
                        ctx.mes("- His brimming small eyes seem to tell you something. ^FF0000Please talk to me again...^000000 -")?;
                        return ctx.close();
                    }
                    break;
                }
                2 => {
                    ctx.lines_as(
                        "New Agent",
                        args!["This is my first mission but people's reaction is not good."],
                    )?;
                    ctx.next()?;
                    ctx.mes("- New Agent is looking at you with pitiable eyes. -")?;
                    ctx.next()?;
                    ctx.mes("- His twinkling small eyes seem to tell you something. ^FF0000Please talk to me again...^000000 -")?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        ctx.lines_as("New Agent", args!["I used to be a very normal boy in Morocc.", "I am not an adventurer and I don't even know how to buy/sell things. Sometimes I'm just very happy to see adventurers. You know... I am just an ordinary boy."])?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args![
                "People used to tell me that since I was very shy.",
                "I would spend my life in a suburb."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args![
                "But eventually, my patience and restrained personality motivated me to become an agent for RWC2011!",
                "I was so happy that I couldn't sleep! I would finally have a chance to meet lots of adventurers and get to know them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args![
                "This mission was so important to me,",
                "I was afraid of being attacked by monsters when attaching the blue cards for RWC2011 in the streets."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("New Agent", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("New Agent", args!["And you know what?", "^FF0000It really happened!^000000"])?;
        ctx.next()?;
        ctx.var("@menu")
            .set(runtime::select_values(ctx, &[Val::from("Are you serious!?")])?)?;
        ctx.lines_as(
            "New Agent",
            args![
                "I had turned off the light and was trying to get some sleep, then I saw this bright light through the window.",
                "Suddenly, the window was broken and it appeared!"
            ],
        )?;
        ctx.next()?;
        ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("It?")])?)?;
        ctx.lines_as(
            "New Agent",
            args!["It was a shining poring! And it ate all blue cards I had prepared in less than a second!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["I was so shocked, I tried to calm down and think... I had neither heard about nor seen any shining poring!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["^FF0000Yes! It must be a dream!^000000 That's the conclusion I came up with and went back to sleep."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["But it wasn't a dream. When I woke up, I realized all the blue cards were gone..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["If I ruin this promotion, I might lose the job and have to go back to where I used to live alone..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["If you see the Golden Poring, could you please find the precious blue cards for me?"],
        )?;
        ctx.next()?;
        ctx.npc().emotion(constants::ET_CRY)?;
        ctx.mes("- He looks at you with imploring eyes. -")?;
        ctx.next()?;
        if ctx.menu(&["Ok, I will help you!", "How much money can you offer me?"])? == 1 {
            ctx.lines_as(
                "New Agent",
                args!["Have you decided to help me? Well, I will prepare some gifts. I'm sure you'll like them."],
            )?;
            ctx.next()?;
        }
        ctx.lines_as("New Agent", args!["What you are looking for are large cards with the letters ^FF0000RWC2011^000000. Some of them might already be digested though."])?;
        ctx.next()?;
        ctx.lines_as(
            "New Agent",
            args!["Try to gather remaining cards until you make the word, 'RWC2011'."],
        )?;
        ctx.next()?;
        ctx.lines_as("New Agent", args!["Please help me out!! My life is in your hands!"])?;
        ctx.var("oversea_event").set(1)?;
        ctx.quests().start(13000)?;
        return ctx.close();
    }

    if ctx.var("oversea_event").get()? == 1 {
        if !(ctx.call(Function::CountItem, args![6485])?.is_true()
            && ctx.call(Function::CountItem, args![6486])?.is_true()
            && ctx.items().count(6487)? > 1
            && ctx.call(Function::CountItem, args![7602])?.is_true()
            && ctx.call(Function::CountItem, args![7470])?.is_true()
            && ctx.call(Function::CountItem, args![6012])?.is_true())
        {
            ctx.lines_as("RWC2011 Agent", args!["Mmmm~ not yet? We're short-handed... Please help me."])?;
            ctx.next()?;
            ctx.lines_as(
                "RWC2011 Agent",
                args!["I'm just saying that out of concern for you. You need TWO ^FF0000Blue 1 Card^000000. It's \"2011\"... Got it?"],
            )?;
            return ctx.close();
        }
        ctx.lines_as("RWC2011 Agent", args!["Have you gathered all the cards?"])?;
        ctx.next()?;
        ctx.lines_as("RWC2011 Agent", args!["Wow~ You bring light in my life again!"])?;
        ctx.next()?;
        ctx.lines_as("RWC2011 Agent", args!["Here are the gifts I promised... Let's see..."])?;
        ctx.next()?;
        ctx.items().take(6485, 1)?;
        ctx.items().take(6486, 1)?;
        ctx.items().take(6487, 2)?;
        ctx.items().take(7602, 1)?;
        ctx.items().take(7470, 1)?;
        ctx.items().take(6012, 1)?;
        ctx.var("oversea_event").set(2)?;
        ctx.quests().erase(13000)?;
        ctx.quests().start(13001)?;
        match ctx.rand_range(1, 100)? {
            1 => ctx.items().give(12690, 1)?,
            2 => ctx.items().give(12691, 1)?,
            3 => ctx.items().give(12693, 1)?,
            4 => ctx.items().give(12694, 1)?,
            5 => ctx.items().give(12698, 1)?,
            6 => ctx.items().give(12695, 1)?,
            7 => ctx.items().give(12692, 1)?,
            ..=27 => ctx.items().give(547, 10)?,
            28..=37 => ctx.items().give(607, 1)?,
            38..=50 => ctx.items().give(608, 1)?,
            _ => {
                ctx.items().give(12696, 5)?;
                ctx.items().give(12697, 5)?;
            }
        }
        ctx.lines_as(
            "RWC2011 Agent",
            args![
                "It's not much but please keep it! Haha, we still have lots of cards to be found. I hope you can help me tomorrow as well."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "RWC2011 Agent",
            args!["Thank you!! I can continue working now. Nothing is impossible when we have great adventurers like you in this world!"],
        )?;
        ctx.next()?;
        ctx.lines_as("RWC2011 Agent", args!["ADIOS!"])?;
        ctx.next()?;
        ctx.mes("- Agent muttered something to himself. -")?;
        return ctx.close();
    }
    if ctx.var("oversea_event").get()? == 2 {
        let playtime = ctx.call(Function::CheckQuest, args![13001, constants::PLAYTIME])?.number()?;
        if playtime == 0 || playtime == 1 {
            ctx.lines_as(
                "RWC2011 Agent",
                args![
                    "First, I will start attaching cards you found... I'll take the rest of cards tomorrow.",
                    "Please come back tomorrow."
                ],
            )?;
            return ctx.close();
        }
        if playtime == 2 {
            ctx.quests().erase(13001)?;
        }
        ctx.lines_as(
            "RWC2011 Agent",
            args![
                "Thanks for the last time.",
                "I must keep on promoting the event so I want to ask you again~!"
            ],
        )?;
        ctx.var("oversea_event").set(3)?;
        return ctx.close();
    }
    if ctx.var("oversea_event").get()? == 3 {
        ctx.lines_as(
            "RWC2011 Agent",
            args![
                "You've come again~",
                "Thanks for the last time! We still have lots of cards to find. Could you help me out?"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Ok!", "I'm busy now."])? {
            0 => {
                ctx.lines_as(
                    "RWC2011 Agent",
                    args![
                        "It's the same mission as before.",
                        "Try to gather blue cards and make the word, ^FF0000R W C 2 0 1 1 ^000000.",
                        "Good Luck!"
                    ],
                )?;
                ctx.var("oversea_event").set(1)?;
                ctx.quests().start(13000)?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "RWC2011 Agent",
                    args![
                        "...I see...",
                        "I still have lots of cards to find so if you have time, please come back and help again."
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    } else {
        ctx.lines_as("RWC2011 Agent", args!["... huh...?", "I am... a new agent."])?;
        return ctx.close();
    }
    Ok(())
}
