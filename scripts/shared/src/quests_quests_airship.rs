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

fn emote(ctx: &Ctx, emotion: i32, npc: &str) -> Result<(), Stop> {
    ctx.call(Function::Emotion, args![emotion, ctx.call(Function::GetNpcId, args![0, npc])?])?;
    Ok(())
}

pub fn f_cherno(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_COMBOATTACK1, constants::AREA, "#exp_ein"],
    )?;
    ctx.mes("^33355F*Bang*^000000")?;
    ctx.next()?;
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_COMBOATTACK2, constants::AREA, "#exp_ein"],
    )?;
    ctx.mes("^33355F*Bang*^000000")?;
    ctx.next()?;
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_COMBOATTACK3, constants::AREA, "#exp_ein"],
    )?;
    ctx.mes("^33355F*Bang*^000000")?;
    ctx.next()?;
    ctx.call(
        Function::NpcSpecialEffect,
        args![constants::EF_POISONATTACK, constants::AREA, "#exp_ein"],
    )?;
    ctx.lines(args!["^33355F*Crash!*^000000", "....."])?;
    emote(ctx, constants::ET_SWEAT, "Theo Cherno")?;
    emote(ctx, constants::ET_SWEAT, "Tarsha Cherno")?;
    ctx.next()?;
    ctx.lines_as(
        "Theo Cherno",
        args!["Honey...", "I think there's", "a critical structural", "problem with the joint."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tarsha Cherno",
        args!["They must not", "be connected right.", "Maybe if we... Hmmm."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Theo Cherno",
        args!["Well, let's call", "it a day and finish", "this tomorrow, yeah?"],
    )?;
    ctx.next()?;
    emote(ctx, constants::ET_QUESTION, "Tarsha Cherno")?;
    ctx.lines_as(
        "Tarsha Cherno",
        args![
            "Oh~",
            "I didn't know we",
            "had a guest. Hello,",
            "how are you doing?",
            "Are you interested in",
            "any of our inventions?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Nothing.", "Your daughter asked me to visit you."])? {
        0 => {
            ctx.lines_as(
                "Theo Cherno",
                args![
                    "Oh yes, my wife are",
                    "I are keeping ourselves",
                    "busy by conducting research.",
                    "We're trying to invent new",
                    "things for better living.",
                    "It's what we do..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            emote(ctx, constants::ET_SWEAT, "Tarsha Cherno")?;
            emote(ctx, constants::ET_SWEAT, "Theo Cherno")?;
            ctx.lines_as(
                "Tarsha Cherno",
                args!["Hahaha, Elle?", "Why would our", "daughter send", "you to visit us?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Well, uh...",
                    "She wanted me to",
                    "check if her mom",
                    "was sick or sad",
                    "...Or something?"
                ],
            )?;
            ctx.next()?;
            emote(ctx, constants::ET_HUK, "Tarsha Cherno")?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Oh my god...!",
                    "What is she thinking?",
                    "I apologize for troubling",
                    "you over this kind of silly",
                    "mistake, adventurer!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Theo Cherno",
                args![
                    "But wait, darling.",
                    "I agree that sometimes",
                    "you look like the loneliest",
                    "person in the world, gazing",
                    "out that window with those",
                    "impossibly wistful eyes."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Oh, I must be",
                    "making my family",
                    "very anxious. But",
                    "don't you worry, love.",
                    "I'm very happy with",
                    "you and Elle."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Tarsha Cherno", args!["And...", "Kind adventurer,", "may I ask your name?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args!["I am called,", format!("{}.", ctx.player().name()?)],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    format!("{},", ctx.player().name()?),
                    "I appreciate your",
                    "concern for us. Would",
                    "you stay for a cup of tea?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Now, please pardon the",
                    "mess. My husband and I are",
                    "mechanical engineers focused",
                    "on creating machines that would",
                    "reduce the risk of mining ores."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tarsha Cherno",
                args![
                    "Since we work at home, we",
                    "can't avoid having machine",
                    "parts lying here and there",
                    "sometimes. In any case, we're",
                    "hoping we create a machine that could make miners' lives safer..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Ah...",
                    "So that's why",
                    "there's so many",
                    "interesting things",
                    "in your house..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou begin to",
                "take a look around",
                "the Cherno household.^000000"
            ])?;
            return Ok(Val::from(0));
        }
        _ => {}
    }
    Ok(Val::from(0))
}
