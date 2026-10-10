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

fn guild_skill_level(ctx: &Ctx, skill_id: i32) -> Result<Val, Stop> {
    ctx.call(
        Function::GetGuildSkillLevel,
        args![ctx.call(Function::GetCharacterId, args![2])?, skill_id],
    )
}

fn guild_blocked_from_novice_castles(ctx: &Ctx) -> Result<bool, Stop> {
    if guild_skill_level(ctx, 10013)?.is_true() {
        return Ok(true);
    }
    let mut total = guild_skill_level(ctx, 10000)?;
    for skill_id in 10001..=10014 {
        total = total + guild_skill_level(ctx, skill_id)?;
    }
    Ok(total.number()? > 9)
}

pub fn novice_castles(ctx: &Ctx) -> Script {
    ctx.mes("[Cita]")?;
    if !ctx.call(Function::GetCharacterId, args![2])?.is_true() {
        ctx.mes("^FF0000You have to enter a guild to be able to hit Emperium!^000000")?;
    } else if guild_blocked_from_novice_castles(ctx)? {
        ctx.lines(args![
            "I see... your guild has Emergency Call mastered.",
            "You cannot enter the Novice Castle area."
        ])?;
        ctx.npc().emotion(constants::ET_SCRATCH)?;
        ctx.call(
            Function::Emotion,
            args![
                constants::ET_KEK,
                Val::from(ctx.call(Function::GetCharacterId, args![0])?.is_true())
            ],
        )?;
    } else {
        ctx.mes("I'm a new usher of Novice Castles.")?;
        ctx.next()?;
        ctx.mes("[Cita]")?;
        if (ctx.ea_class(None)? & (constants::EAJL_2 | constants::EAJL_UPPER)) != 0 || ctx.player().base_level()? >= 60 {
            ctx.mes("I'm sorry, you can't enter the sacred Novice Castles place.")?;
            ctx.npc().emotion(constants::ET_SORRY)?;
        } else if ctx.menu(&["Warp me to Novice Castles", "Cancel"])? == 0 {
            for status in [
                "SC_ASSUMPTIO",
                "SC_IMPOSITIO",
                "SC_SUFFRAGIUM",
                "SC_MAGNIFICAT",
                "SC_WEAPONPERFECTION",
                "SC_GOSPEL",
                "SC_BASILICA",
                "SC_MAGICPOWER",
                "SC_MARIONETTE",
                "SC_MARIONETTE2",
                "SC_DEVOTION",
                "SC_SACRIFICE",
                "SC_MAXOVERTHRUST",
                "SC_SPIRIT",
            ] {
                ctx.call(Function::EndStatus, args![ctx.constant(status)?])?;
            }
            ctx.call(
                Function::Warp,
                args!["n_castle", 102, Val::from(93) + ctx.call(Function::Rand, args![14])?],
            )?;
        }
    }
    ctx.close()
}

pub fn cita(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Cita",
        args![Val::from("Hello, ") + ctx.player().name()? + Val::from(". Can I help you?")],
    )?;
    ctx.next()?;
    if ctx.menu(&["Warp me to Prontera!", "Cancel"])? == 0 {
        ctx.call(
            Function::Warp,
            args!["prontera", 155, Val::from(177) + ctx.call(Function::Rand, args![5])?],
        )?;
    } else {
        ctx.lines_as("Cita", args!["Ok."])?;
    }
    ctx.close()
}
