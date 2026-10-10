use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn han_ran_jiao_gon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("nakha").get()? == 0 {
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "Oh no! What will I do?!",
                "I can't believe I dropped",
                "my irreplaceable knife! Ahhhhhh!"
            ],
        )?;
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? >= 20 {
            ctx.var("nakha").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(10034)])?;
            ctx.lines_as(
                "Han Ran Jiao",
                args![
                    "I need to go down to get it",
                    "but...the monsters...",
                    "I'm so scared... What should I do?!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("nakha").get()? == 1 {
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "Oh no! What will I do?",
                "I can't believe I dropped",
                "my irreplaceable knife! Ahhhhhh!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "I need to go down to get it",
                "but...the monsters...",
                "I'm so scared... What should I do?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("nakha").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(1201)])?.number()? > 0) {
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "Ehhhh... what should I do...",
                "Oh~! I didn't notice you there.",
                "Can I help you with anything?",
                "Hmm?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "That...that knife!",
                "That's my ancestor's sacred knife",
                "that I accidentally dropped from",
                "here! Where did you get it!?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "I dropped that knife from being",
                "careless. It belongs to one of my",
                "ancestors. I know it looks like",
                "a cheap knife..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "But it has been in the",
                "family for a very long time.",
                "Would you please return that",
                "knife to me?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Return the knife.:Refuse.")])?) == 1 {
            ctx.var("nakha").set(Val::from(3))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(10035)])?;
            ctx.call(Function::DelItem, vec![Val::from(1201), Val::from(1)])?;
            ctx.lines_as(
                "Han Ran Jiao",
                args![
                    "Thank you! Thank you so much!",
                    "Please take these potions.",
                    "They may not be much, but they",
                    "are the best I can give you."
                ],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(505), Val::from(2)])?;
            ctx.next()?;
            ctx.lines_as(
                "Han Ran Jiao",
                args![
                    "You kept my family heirloom safe!",
                    "I give you my deepest gratitude",
                    "for returning this."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "No~! My family heirloom!!",
                "You scoundrel! May the spirits",
                "forever haunt you! My curse shall be upon your head!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nakha").get()? == 2 {
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "Oh no! What will I do?",
                "I can't believe I dropped",
                "my irreplaceable knife! Ahhhhhh!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "I need to go down to get it",
                "but...the monsters...",
                "I'm so scared... What should I do?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("nakha").get()? == 3 {
        ctx.lines_as(
            "Han Ran Jiao",
            args![
                "My neighbor seemed looked pretty bothered about something...",
                "Oh well, I have better things to worry about than his fancy tea."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn han_ran_jiao_gon(ctx: &Ctx) -> Script {
    han_ran_jiao_gon_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GonknifeStep {
    Start,
    OnTouch,
}

fn gonknife_run(ctx: &Ctx, mut step: GonknifeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GonknifeStep::Start => {
                step = GonknifeStep::OnTouch;
                continue 'machine;
            }
            GonknifeStep::OnTouch => {
                if ctx.var("nakha").get()? == 1 {
                    ctx.var("nakha").set(Val::from(2))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(10034), Val::from(10035)])?;
                    ctx.lines(args![
                        "^3355FFHm? What's this?",
                        " ",
                        "Something was hidden beneath the leaves...^000000",
                        " ",
                        "- You have found an old knife -"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::GetItem, vec![Val::from(1201), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn gonknife(ctx: &Ctx) -> Script {
    gonknife_run(ctx, GonknifeStep::Start, Vec::new()).map(|_| ())
}

pub fn gonknife_ontouch(ctx: &Ctx) -> Script {
    gonknife_run(ctx, GonknifeStep::OnTouch, Vec::new()).map(|_| ())
}
