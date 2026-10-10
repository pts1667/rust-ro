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

#[derive(Clone, Copy, Debug)]
enum HypnotistTeacherStep {
    Start,
    Info,
    Reset,
}

fn hypnotist_teacher_run(ctx: &Ctx, mut step: HypnotistTeacherStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HypnotistTeacherStep::Start => {
                ctx.var("@npcname$").set(Val::from("[^D5A500Hypnotist^000000]"))?;
                ctx.lines(args![ctx.var("@npcname$").get()?])?;
                if (ctx.var("misc_quest").get()?.number()? & 1024) != 0 {
                    ctx.mes("I already told you that you may only complete this event once.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.player().base_level()? < 60 {
                    ctx.mes("Please return when you reach BaseLv 60 or higher.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("SkillPoint").get()? != 0 {
                    ctx.lines(args![
                        "You will need to use up all of your skill points if you want me to continue.",
                        "Please come again soon!"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("Weight").get()?.is_true()
                    || ctx.call(Function::CheckFalcon, args![])?.is_true()
                    || ctx.call(Function::CheckCart, args![])?.is_true()
                    || ctx.call(Function::CheckRiding, args![])?.is_true()
                {
                    ctx.lines(args!["Would you like to reset skills?", "I'm sorry, but..."])?;
                    ctx.next()?;
                    ctx.lines(args![ctx.var("@npcname$").get()?])?;
                    if ctx.var("Weight").get()?.is_true() {
                        ctx.lines(args!["You cannot reset skills", "when you keep", "any items."])?;
                    } else if ctx.call(Function::CheckCart, args![])?.is_true() {
                        ctx.mes("Please, drop your cart and we'll continue.")?;
                    } else if ctx.call(Function::CheckFalcon, args![])?.is_true() {
                        ctx.mes("Please, free your Falcon and we'll continue.")?;
                    } else if ctx.call(Function::CheckRiding, args![])?.is_true() {
                        ctx.mes("Please, free your PecoPeco and we'll continue.")?;
                    }
                    ctx.next()?;
                    ctx.lines(args![ctx.var("@npcname$").get()?, "Come back soon!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "Hello, I am the Skill Resetter.",
                    format!("Your name is ^0080FF{}^000000.", ctx.player().name()?),
                    "How can I help you?"
                ])?;
                ctx.next()?;
                let choice = runtime::select(
                    ctx,
                    &[
                        "^009500Information about Reset skills.^000000",
                        "^00B6FFReset skills.^000000",
                        "^000088Nevermind^000000",
                    ],
                )?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {
                        step = HypnotistTeacherStep::Info;
                        continue 'machine;
                    }
                    2 => {
                        step = HypnotistTeacherStep::Reset;
                        continue 'machine;
                    }
                    3 => {}
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    "You know where to find me,",
                    "if you ever want a reset!!"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            HypnotistTeacherStep::Info => {
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    "This skill reset is not FREE OF CHARGE!!",
                    "Expense for the reset of skill is ^D5A50020000 Zeny x BaseLv^000000.",
                    "Yeah ...each One BaseLv costs 20000 Zeny to reset skill."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    "Oh yeah, one more thing!",
                    "Any carts, falcons or pecos you have equiped",
                    "will be removed if you reset your skills."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    "Just one time does again to shake the skill point",
                    "Careful with your skills from here on."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            HypnotistTeacherStep::Reset => {
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    "Before skill reset in starting.",
                    "You shall have to first tell me your Base Level."
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    format!("^D5A500[{}]^000000", ctx.player().name()?),
                    format!("My Base level is ^AA00AALevel {}BaseLv.^000000", ctx.player().base_level()?),
                ])?;
                ctx.next()?;
                ctx.var("@zeny").set(ctx.player().base_level()? * 20000)?;
                ctx.lines(args![
                    ctx.var("@npcname$").get()?,
                    format!(
                        "Total zeny to the reset of skill amount ^529DFF{}Zeny^000000  for the skill reset service.",
                        ctx.var("@zeny").get()?.number()?
                    ),
                ])?;
                ctx.next()?;
                if ctx.player().zeny()? < ctx.var("@zeny").get()?.number()? {
                    ctx.lines(args![
                        ctx.var("@npcname$").get()?,
                        "It seems that you don't have enough money.",
                        "In addition we wait for the opportunity."
                    ])?;
                    ctx.npc().emotion(constants::ET_SCRATCH)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.player().set_zeny(ctx.player().zeny()? - ctx.var("@zeny").get()?.number()?)?;
                ctx.var("misc_quest").set(ctx.var("misc_quest").get()?.number()? | 1024)?;
                ctx.call(Function::ResetSkills, args![])?;
                ctx.lines(args![ctx.var("@npcname$").get()?, "Thank you."])?;
                ctx.npc().emotion(constants::ET_THANKS)?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hypnotist_teacher(ctx: &Ctx) -> Script {
    hypnotist_teacher_run(ctx, HypnotistTeacherStep::Start, Vec::new()).map(|_| ())
}
