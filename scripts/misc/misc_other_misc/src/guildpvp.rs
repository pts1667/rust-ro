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

pub fn guild_battle_guide(ctx: &Ctx) -> Script {
    let mut map1_x: Vec<Val> = Vec::new();
    let mut map2_x: Vec<Val> = Vec::new();
    let mut map_y: Vec<Val> = Vec::new();
    runtime::local_set(&mut map1_x, &Val::from(1), Val::from(7), false);
    runtime::local_set(&mut map1_x, &Val::from(2), Val::from(9), false);
    runtime::local_set(&mut map1_x, &Val::from(3), Val::from(16), false);
    runtime::local_set(&mut map1_x, &Val::from(4), Val::from(8), false);
    runtime::local_set(&mut map1_x, &Val::from(5), Val::from(20), false);
    runtime::local_set(&mut map2_x, &Val::from(1), Val::from(91), false);
    runtime::local_set(&mut map2_x, &Val::from(2), Val::from(90), false);
    runtime::local_set(&mut map2_x, &Val::from(3), Val::from(83), false);
    runtime::local_set(&mut map2_x, &Val::from(4), Val::from(91), false);
    runtime::local_set(&mut map2_x, &Val::from(5), Val::from(79), false);
    runtime::local_set(&mut map_y, &Val::from(1), Val::from(49), false);
    runtime::local_set(&mut map_y, &Val::from(2), Val::from(49), false);
    runtime::local_set(&mut map_y, &Val::from(3), Val::from(50), false);
    runtime::local_set(&mut map_y, &Val::from(4), Val::from(49), false);
    runtime::local_set(&mut map_y, &Val::from(5), Val::from(50), false);
    ctx.lines_as(
        "Guild Battle Guide",
        args![
            "How are you doing?",
            "I'm the Guild Battle Guide",
            "for the new PvP maps.",
            "Let me know to which map",
            "you want me to move you."
        ],
    )?;
    ctx.next()?;
    let map_no = Val::from(runtime::select_values(ctx, &[Val::from("Map 1:Map 2:Map 3:Map 4:Map 5")])?);
    ctx.lines_as(
        "Guild Battle Guide",
        args![
            Val::from("You've chosen Map ") + map_no.clone() + Val::from("."),
            "Now, which team are",
            "you on? You can choose",
            "either Team 1 or Team 2."
        ],
    )?;
    ctx.next()?;
    let team = Val::from(runtime::select_values(ctx, &[Val::from("Team 1:Team 2")])?);
    ctx.lines_as(
        "Guild Battle Guide",
        args![
            "Alright, I'll move you",
            Val::from("to Map ") + map_no.clone() + Val::from(" as a member"),
            Val::from("of Team ") + team.clone() + Val::from(". Are you ready"),
            "to be transported there now?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 1 {
        ctx.lines_as(
            "Guild Battle Guide",
            args!["Alright, I hope to", "see you again on", "the PvP fields!"],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Guild Battle Guide", args!["Great! Have a good time~"])?;
    ctx.close_window()?;
    ctx.call(
        Function::Warp,
        args![
            Val::from("guild_vs") + map_no.clone(),
            runtime::getd(
                ctx,
                &(Val::from(".@map") + team.clone() + Val::from("X[") + map_no.clone() + Val::from("]")),
                &[
                    (".@i", runtime::Local::Scalar(&map_no)),
                    (".@j", runtime::Local::Scalar(&team)),
                    (".@map1x", runtime::Local::Array(&map1_x)),
                    (".@map2x", runtime::Local::Array(&map2_x)),
                    (".@mapy", runtime::Local::Array(&map_y)),
                ],
            )?,
            runtime::local_get(&map_y, &map_no, false),
        ],
    )?;
    ctx.end()
}
