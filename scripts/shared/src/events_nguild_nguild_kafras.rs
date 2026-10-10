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

#[derive(Clone, Copy, Debug)]
enum FGkafraStep {
    CheckGuild,
    OpenMenu,
}

fn f_gkafra_run(ctx: &Ctx, mut step: FGkafraStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FGkafraStep::CheckGuild => {
                ctx.fx().cutin("kafra_01", 2)?;
                ctx.var("@gid")
                    .set(ctx.call(Function::GetCastleData, args![runtime::arg(&args, 0, Val::from(0)), 1])?)?;
                if ctx
                    .call(Function::GetCharacterId, args![2])?
                    .loosely_equals(&ctx.var("@gid").get()?)
                    && ctx
                        .call(Function::GetGuildSkillLevel, args![ctx.var("@gid").get()?, 10001])?
                        .is_true()
                {
                    step = FGkafraStep::OpenMenu;
                    continue 'machine;
                }
                ctx.lines_as(
                    "Kafra Service",
                    args![
                        Val::from("I am contracted to provide service only for the ^ff0000")
                            + ctx.call(Function::GetGuildInfo, args![ctx.var("@gid").get()?, 0])?
                            + "^000000 Guild. Please use another Kafra Corporation staff member around here. I am Sorry for your inconvenience."
                    ],
                )?;
                ctx.fx().cutin("", 255)?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            FGkafraStep::OpenMenu => {
                ctx.var("@wrpp").set_at(0, Val::from(200))?;
                ctx.var("@wrpd$").set_at(0, runtime::arg(&args, 1, Val::from(0)))?;
                ctx.var("@wrpc$").set_at(
                    0,
                    ctx.var("@wrpd$").get_at(0)? + " ^880000" + ctx.var("@wrpp").get_at(0)? + "^000000 z",
                )?;
                ctx.var("@wrpc$").set_at(1, Val::from("Cancel"))?;
                for i in 2..=5 {
                    ctx.var("@wrpc$").set_at(i, Val::from(""))?;
                }
                crate::kafras_functions_kafras::f_kafra(ctx, args![2, 0, 0, 0, 800])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn f_gkafra(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_gkafra_run(ctx, FGkafraStep::CheckGuild, args)
}
