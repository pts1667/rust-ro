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

pub fn acolyte_prtclear(ctx: &Ctx) -> Script {
    ctx.lines_as("Keiki", args!["Hello there, adventurer.", "I've been studying magic from all over Rune-Midgarts to upgrade what I believe to be one of the greatest skills available to the acolyte class."])?;
    ctx.next()?;
    ctx.lines_as(
        "Keiki",
        args!["I am the one and only Acolyte that has attained the Level 10 Warp Portal skill!"],
    )?;
    ctx.call(
        Function::Emotion,
        args![constants::ET_THINK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Keiki",
        args![
            "That's right! And...",
            "I promise you that I don't forget locations that I have already memorized.",
            "One day I will level up my skills to warp to wherever I please~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Keiki",
        args![
            "I am willing to warp you to the many locations that I have memorized for a small fee.",
            "Would you like to use this service?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
        1 => {
            ctx.lines_as(
                "Keiki",
                args!["Where would you like to go to?", "I wish you goodluck on your journey."],
            )?;
            ctx.next()?;
            let mut towns: Vec<Val> = Vec::new();
            runtime::local_set(&mut towns, &Val::from(0), Val::from("Izlude"), true);
            runtime::local_set(&mut towns, &Val::from(1), Val::from("Geffen"), true);
            runtime::local_set(&mut towns, &Val::from(2), Val::from("Payon"), true);
            runtime::local_set(&mut towns, &Val::from(3), Val::from("Morocc"), true);
            runtime::local_set(&mut towns, &Val::from(4), Val::from("Alberta"), true);
            runtime::local_set(&mut towns, &Val::from(5), Val::from("Al De Baran"), true);
            runtime::local_set(&mut towns, &Val::from(6), Val::from("Comodo"), true);
            runtime::local_set(&mut towns, &Val::from(7), Val::from("Umbala"), true);
            runtime::local_set(&mut towns, &Val::from(8), Val::from("Juno"), true);
            let mut costs: Vec<Val> = Vec::new();
            runtime::local_set(&mut costs, &Val::from(0), Val::from(600), false);
            runtime::local_set(&mut costs, &Val::from(1), Val::from(1200), false);
            runtime::local_set(&mut costs, &Val::from(2), Val::from(1200), false);
            runtime::local_set(&mut costs, &Val::from(3), Val::from(1200), false);
            runtime::local_set(&mut costs, &Val::from(4), Val::from(1800), false);
            runtime::local_set(&mut costs, &Val::from(5), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(6), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(7), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(8), Val::from(1800), false);
            let size = towns.len() as i32;
            let mut menu = Val::from("");
            for i in 0..size {
                menu = menu
                    + runtime::local_get(&towns, &Val::from(i), true)
                    + Val::from(" -> ")
                    + runtime::local_get(&costs, &Val::from(i), false)
                    + Val::from("z:");
            }
            let index = runtime::select_values(ctx, &[menu + Val::from("Cancel")])? - 1;
            if index == size {
                return ctx.close();
            }
            let cost = runtime::local_get(&costs, &Val::from(index), false).number()?;
            if ctx.player().zeny()? < cost {
                let town = runtime::local_get(&towns, &Val::from(index), true);
                ctx.lines_as(
                    "Keiki",
                    args![
                        "I'm sorry, but you don't have",
                        "enough zeny for the Teleport",
                        "Service. The fee to teleport",
                        Val::from("to ") + town + Val::from(" is ") + Val::from(cost) + Val::from(" zeny.")
                    ],
                )?;
                return ctx.close();
            }
            ctx.fx().special_effect(constants::EF_READYPORTAL)?;
            ctx.fx().special_effect(constants::EF_TELEPORTATION)?;
            ctx.fx().special_effect(constants::EF_PORTAL)?;
            ctx.next()?;
            ctx.player().set_zeny(ctx.player().zeny()? - cost)?;
            match index {
                0 => ctx.warp("izlude", 91, 105)?,
                1 => ctx.warp("geffen", 120, 39)?,
                2 => ctx.warp("payon", 161, 58)?,
                3 => ctx.warp("morocc", 156, 46)?,
                4 => ctx.warp("alberta", 117, 56)?,
                5 => ctx.warp("aldebaran", 168, 112)?,
                6 => ctx.warp("comodo", 209, 143)?,
                7 => ctx.warp("umbala", 100, 154)?,
                8 => ctx.warp("yuno", 158, 125)?,
                _ => {}
            }
            return ctx.close();
        }
        2 => return ctx.close(),
        _ => {}
    }
    Ok(())
}

pub fn acolyte_junoclear(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Isalei",
        args![
            "Hello, adventurer.",
            "My companion Keiki and I have discovered a way to increase our warp portal abilities."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Isalei",
        args!["Though I have not mastered up to the level that she has, I have been able to attain Level 5."],
    )?;
    ctx.call(
        Function::Emotion,
        args![constants::ET_THINK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Isalei",
        args!["Maybe one day I can level up my skills enough so that I can use Warp portal to more saved locations."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Isalei",
        args![
            "I am willing to warp you to the many locations that I have memorized for a small fee.",
            "Would you like to use this service?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
        1 => {
            ctx.lines_as(
                "Isalei",
                args!["Where would you like to go to?", "I wish you goodluck on your journey."],
            )?;
            ctx.next()?;
            let mut towns: Vec<Val> = Vec::new();
            runtime::local_set(&mut towns, &Val::from(0), Val::from("Einbroch"), true);
            runtime::local_set(&mut towns, &Val::from(1), Val::from("Lighthalzen"), true);
            runtime::local_set(&mut towns, &Val::from(2), Val::from("Hugel"), true);
            runtime::local_set(&mut towns, &Val::from(3), Val::from("Rachel"), true);
            runtime::local_set(&mut towns, &Val::from(4), Val::from("Prontera"), true);
            let mut costs: Vec<Val> = Vec::new();
            runtime::local_set(&mut costs, &Val::from(0), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(1), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(2), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(3), Val::from(2200), false);
            runtime::local_set(&mut costs, &Val::from(4), Val::from(1800), false);
            let size = towns.len() as i32;
            let mut menu = Val::from("");
            for i in 0..size {
                menu = menu
                    + runtime::local_get(&towns, &Val::from(i), true)
                    + Val::from(" -> ")
                    + runtime::local_get(&costs, &Val::from(i), false)
                    + Val::from("z:");
            }
            let index = runtime::select_values(ctx, &[menu + Val::from("Cancel")])? - 1;
            if index == size {
                return ctx.close();
            }
            let cost = runtime::local_get(&costs, &Val::from(index), false).number()?;
            if ctx.player().zeny()? < cost {
                let town = runtime::local_get(&towns, &Val::from(index), true);
                ctx.lines_as(
                    "Isalei",
                    args![
                        "I'm sorry, but you don't have",
                        "enough zeny for the Teleport",
                        "Service. The fee to teleport",
                        Val::from("to ") + town + Val::from(" is ") + Val::from(cost) + Val::from(" zeny.")
                    ],
                )?;
                return ctx.close();
            }
            ctx.fx().special_effect(constants::EF_READYPORTAL)?;
            ctx.fx().special_effect(constants::EF_TELEPORTATION)?;
            ctx.fx().special_effect(constants::EF_PORTAL)?;
            ctx.next()?;
            ctx.player().set_zeny(ctx.player().zeny()? - cost)?;
            match index {
                0 => ctx.warp("einbroch", 67, 195)?,
                1 => ctx.warp("lighthalzen", 159, 90)?,
                2 => ctx.warp("hugel", 98, 150)?,
                3 => ctx.warp("rachel", 119, 135)?,
                4 => ctx.warp("prontera", 116, 72)?,
                _ => {}
            }
            return ctx.close();
        }
        2 => return ctx.close(),
        _ => {}
    }
    Ok(())
}
