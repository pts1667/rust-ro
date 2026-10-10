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

pub fn dollshoi(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mage Guildsman",
        args!["You want a Solution? Hmm, give me 50 Zeny and at least 1 Empty Test Tube."],
    )?;
    ctx.next()?;
    if ctx.menu(&["Alright, Deal.", "Nah, forget it."])? == 0 {
        ctx.mes("[Mage Guildsman]")?;
        if ctx.player().zeny()? < 50 {
            ctx.mes("Hey! You don't have enough money to cover my 50 Zeny charge.")?;
            return ctx.close();
        }
        if ctx.call(Function::CountItem, args![1092])? == 0 {
            ctx.mes("You can't carry solutions without a bottle! Bring me an Empty Test Tube.")?;
            return ctx.close();
        }
        ctx.items().take(1092, 1)?;
        ctx.player().set_zeny(ctx.player().zeny()? - 50)?;
        ctx.items().give(1089, 1)?;
    }
    ctx.close()
}

pub fn ponka_hontas(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mage Guildsman",
        args!["Would you like a Solution? Then please give me 50 Zeny and at least 1 Empty Test Tube."],
    )?;
    ctx.next()?;
    if ctx.menu(&["Alright, Deal.", "Nah, forget it."])? == 0 {
        ctx.mes("[Mage Guildsman]")?;
        if ctx.player().zeny()? < 50 {
            ctx.mes("I'm sorry, but you don't have enough money to cover the 50 Zeny fee.")?;
            return ctx.close();
        }
        if ctx.call(Function::CountItem, args![1092])? == 0 {
            ctx.mes("You can't carry liquids without using a bottle. Bring an Empty Test Tube the next time you see me.")?;
            return ctx.close();
        }
        ctx.items().take(1092, 1)?;
        ctx.player().set_zeny(ctx.player().zeny()? - 50)?;
        ctx.items().give(1088, 1)?;
    }
    ctx.close()
}
