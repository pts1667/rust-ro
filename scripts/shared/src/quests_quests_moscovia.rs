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

pub fn f_mos_1(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gallina",
        args!["Oh, where the heck is he?", "I'll teach him a lesson.", "He's timid like Dad."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Anna",
        args!["Mikhail, he's a coward, a crybaby.", "Mikhail, he's a coward, a crybaby."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args!["It's because you didn't look after your brother. So you clean the house, Anna?"],
    )?;
    ctx.next()?;
    ctx.call(
        Function::Emotion,
        args![constants::ET_CRY, ctx.call(Function::GetNpcId, args![0, "Anna#mos"])?],
    )?;
    ctx.lines_as("Anna", args!["Oh, my................"])?;
    ctx.next()?;
    if ctx.menu(&["Talk to her.", "Just pass by her"])? == 1 {
        ctx.lines_as("Gallina", args!["I'm worried that he's making trouble somewhere..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(ctx.player().name()?, args!["Hello, there?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args![
            "Oh, God!",
            "I didn't see you there. Sorry!",
            "You want to buy a hotcake, don't you?",
            "I'm sorry but we're not ready to open the store.."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args!["No, that's ok.", "Is there something that I can help with? What's the matter?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args![
            "Oh, well...",
            "My son, Mikhail broke our Matrushka while I was away from home.",
            "He's afraid that I would punish him. So he ran away."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args![
            "How timid the boy is!",
            "I doubt that he'd be able to be a great general in the future"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args![
            "He used to come home at this time.",
            "I'm worried that something bad has happened to him."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.player().name()?,
        args!["You look anxious. I'd like to help you to find your son."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args![
            "Did you say I'm anxious?",
            "I'm just anxious for him to get home.",
            "...So I can punish him for what he did."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gallina",
        args!["But it's not good that I refuse your kindness", "Will you find Mikhail for me?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["I was not serious.", "Yes, I will!"])? == 0 {
        ctx.lines_as("Gallina", args!["You meanie! I'm not in the mood for jokes."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Gallina",
        args![
            "Oh, God, You're so kind",
            "Mikhail is such a timid boy. I guess he didn't leave this village.",
            "Please bring him to me, then~"
        ],
    )?;
    ctx.var("mos_swan").set(1)?;
    ctx.quests().start(18060)?;
    ctx.close_window()?;
    return Err(Stop::End);
}
