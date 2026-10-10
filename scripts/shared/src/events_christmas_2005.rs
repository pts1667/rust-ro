#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn f_carol_devi(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("christ_carol05").get()? != 1 {
        deviruchi_carry(ctx, &args, 2)?;
        return Err(Stop::End);
    }
    deviruchi_grab(ctx, "ET_CHUP")?;
    match ctx.rand_range(1, 4)? {
        1 => {
            deviruchi_struggle(ctx)?;
            ctx.lines(args![
                "-Deviruchi quickly ran away-",
                "-^4d4dffWhere Deviruchi is gone,-",
                "-there is a worn out paper.^000000-",
                "-Let's read the paper.-"
            ])?;
            ctx.call(Function::EnableNpc, args![runtime::arg(&args, 1, Val::from(0))])?;
            ctx.call(Function::DisableNpc, args![runtime::arg(&args, 0, Val::from(0))])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Rudolph... If... Foggy...",
                    "Sledge... Reindeer...",
                    "Hmm, I think I got what I need.",
                    "Let's go back to Ms.Oholy."
                ],
            )?;
            ctx.var("christ_carol05").set(Val::from(2))?;
            ctx.items().give(1097, 1)?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            deviruchi_struggle(ctx)?;
            deviruchi_status(ctx, &args, "SC_CURSE", "-You are now cursed!!!-")?;
            return Err(Stop::End);
        }
        3 => {
            deviruchi_struggle(ctx)?;
            deviruchi_status(ctx, &args, "SC_BLIND", "-You are blinded!!!-")?;
            return Err(Stop::End);
        }
        4 => {
            deviruchi_struggle(ctx)?;
            deviruchi_status(ctx, &args, "SC_POISON", "-You are poisoned!!!-")?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn f_carol_devi2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    deviruchi_carry(ctx, &args, 0)?;
    Err(Stop::End)
}

fn deviruchi_grab(ctx: &Ctx, emote: &str) -> Result<(), Stop> {
    ctx.lines_as(
        "Deviruchi",
        args![
            "Heyhey, human!!",
            "Don't ya wanna sell your",
            "soul and be bound in a beneficial contract with me?"
        ],
    )?;
    ctx.call(Function::Emotion, args![ctx.constant(emote)?])?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["Hey, you! Deviruchi!!!", "What a brat!!! Gotcha!"])?;
    ctx.next()?;
    ctx.lines(args!["-You quickly snatched-", "-the nape of Deviruchi's neck-"])?;
    ctx.next()
}

fn deviruchi_struggle(ctx: &Ctx) -> Result<(), Stop> {
    ctx.lines_as(
        "Deviruchi",
        args!["What are you doing!?", "Human?", "Let go of me...right now!"],
    )?;
    ctx.call(Function::Emotion, args![ctx.constant("ET_HUK")?])?;
    ctx.call(
        Function::Emotion,
        args![ctx.constant("ET_HUK")?, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.next()
}

fn deviruchi_status(ctx: &Ctx, args: &[Val], status: &str, text: &str) -> Result<(), Stop> {
    ctx.lines(args!["-Deviruchi ran away quickly-", "-and mumbled something.-"])?;
    ctx.call(Function::EnableNpc, args![runtime::arg(args, 1, Val::from(0))])?;
    ctx.call(Function::DisableNpc, args![runtime::arg(args, 0, Val::from(0))])?;
    ctx.next()?;
    ctx.mes(text)?;
    ctx.call(Function::StartStatus, args![ctx.constant(status)?, 5000, 0])?;
    ctx.call(
        Function::Emotion,
        args![ctx.constant("ET_HUK")?, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.close_window()
}

// Warp target is map, x, y at args[first..first + 3].
fn deviruchi_carry(ctx: &Ctx, args: &[Val], first: i32) -> Result<(), Stop> {
    deviruchi_grab(ctx, "ET_HUK")?;
    deviruchi_struggle(ctx)?;
    ctx.lines(args![
        "-Deviruchi ran away quickly-",
        "-and mumbled something.-",
        "-Your body is suddenly floating.-"
    ])?;
    ctx.close_window()?;
    ctx.call(
        Function::Warp,
        args![
            runtime::arg(args, first, Val::from(0)),
            runtime::arg(args, first + 1, Val::from(0)),
            runtime::arg(args, first + 2, Val::from(0))
        ],
    )?;
    Ok(())
}
