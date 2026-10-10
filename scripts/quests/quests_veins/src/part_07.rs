use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn male_customer_ve2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Male Customer",
        args![
            "How can that ugly old",
            "man have girls hanging",
            "off his arms when I just",
            "got dumped by my girlfriend",
            "and ditched by all my friends?!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Male Customer",
        args![
            "Oh, alcohol...",
            "Right now, you're",
            "my only friend in",
            "all the world...",
            "A toast... To drinking!"
        ],
    )?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ASPERSIO")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn male_customer_ve2(ctx: &Ctx) -> Script {
    male_customer_ve2_body(ctx, Vec::new()).map(|_| ())
}
