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

fn play_training_effects(ctx: &Ctx) -> Result<(), Stop> {
    for effect in [
        constants::EF_EARTHSPIKE,
        constants::EF_DEVIL,
        constants::EF_COIN,
        constants::EF_SIGHTRASHER,
    ] {
        ctx.fx().special_effect(effect)?;
        ctx.next()?;
    }
    Ok(())
}

pub fn ripped_cabus_gympass(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ripped",
        args![
            "Hey, there. People aren't",
            "as physically active as they",
            "used to be. Even if you fight",
            "for a living, your body might",
            "be weak and flabby in some",
            "areas. Know what I mean?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ripped",
        args![
            "Hey, train with me, and I can",
            "guarantee that you'll be able",
            "to lift and carry more of your",
            "stuff. Just gimme your",
            "^FF0000Gym Pass^000000 each time,",
            "and we'll be good to go."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ripped",
        args![
            "But don't get too excited:",
            "no matter how much training",
            "I take you through, you can",
            "overdo it. You ever hear of",
            "anyone that got too buff?",
            "That's cuz they're dead. See?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ripped",
        args![
            "I'd say that it'd be safe",
            "for you to seriously train",
            "with me and increase your",
            "item carrying capacity ^FF000010 times^000000.",
            "So... Are you ready to sweat?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "Um, my workouts wore off."])? {
        0 => {
            if ctx.var("gympassmemory").get()?.number()? >= 10 {
                ctx.lines_as(
                    "Ripped",
                    args![
                        "Dude, I don't think we can",
                        "build up your item carrying",
                        "muscles anymore than that.",
                        "It's too dangerous for your",
                        "body if we even tried! C'mon,",
                        "I told you about the limits."
                    ],
                )?;
                return ctx.close();
            }
            let add_carry = ctx.var("gympassmemory").get()?.number()? + 1;
            let remain_carry = 10 - add_carry;
            if ctx.items().count(7776)? <= 0 {
                ctx.lines_as(
                    "Ripped",
                    args![
                        "Dude, what'd I tell you?",
                        "You gotta bring me your",
                        "^FF0000Gym Pass^000000 if you wanna",
                        "work out, and build up your",
                        "item carrying muscles."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Ripped",
                args![
                    "Oh, awesome, I see you",
                    "brought your Gym Pass.",
                    "Alright, just do what I do,",
                    "and try to feel the burn.",
                    "Ready? Let's do this."
                ],
            )?;
            ctx.next()?;
            play_training_effects(ctx)?;
            ctx.lines_as(
                "Ripped",
                args![
                    "There, you should be able",
                    "to carry more stuff with you.",
                    "Let's see, we can increase",
                    "your item carrying capacity",
                    Val::from("^FF00000") + Val::from(remain_carry) + Val::from("^000000 more times if we continue"),
                    "training together like this."
                ],
            )?;
            ctx.items().take(7776, 1)?;
            ctx.var("gympassmemory").set(Val::from(add_carry))?;
            ctx.call(Function::Skill, args!["ALL_INCCARRY", add_carry, constants::SKILL_PERM_GRANT])?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Ripped",
                args![
                    "Aw, that's too bad.",
                    "Well, come back if you",
                    "change your mind. Tell",
                    "your friends about me:",
                    "if they're flabby, I'll help",
                    "get them in shape."
                ],
            )?;
            ctx.close()
        }
        2 => {
            if ctx.var("gympassmemory").get()?.number()? <= 0 {
                ctx.lines_as(
                    "Ripped",
                    args!["Uhh...", "We didn't work out", "together before.", "I'm sure about that."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Ripped",
                args![
                    "What happened?",
                    "You let your item carrying",
                    "muscles just atrophy? Lucky",
                    "for you, there's such a thing",
                    "as muscle memory. It's won't take",
                    "as long to build 'em back up..."
                ],
            )?;
            ctx.next()?;
            play_training_effects(ctx)?;
            ctx.lines_as(
                "Ripped",
                args![
                    "How about that?",
                    "Your item carrying",
                    "muscles grew back,",
                    "just like that! Try not to",
                    "wimp out again, okay?"
                ],
            )?;
            ctx.call(
                Function::Skill,
                args!["ALL_INCCARRY", ctx.var("gympassmemory").get()?, constants::SKILL_PERM_GRANT],
            )?;
            ctx.close()
        }
        _ => ctx.end(),
    }
}
