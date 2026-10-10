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

fn view_point(ctx: &Ctx, action: i32, x: i32, y: i32, id: i32, color: i32) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn adventurer_um(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Adventurer",
        args![
            "This is a very strange place...",
            "It's underdeveloped, and there",
            "are a number of complex, winding paths..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Adventurer",
        args![
            "However, since I have been here",
            "for months, I am familiar with",
            "this area's geography and points",
            "of interest in this village.",
            "You're welcome to ask me about the",
            "locations of buildings."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Locations of buildings.", "Remove marks on the mini map.", "Quit."])? {
        0 => {
            ctx.lines_as("Adventurer", args!["So, which one do you want to check?"])?;
            ctx.next()?;
            match ctx.menu(&[
                "Chief's House",
                "Shaman's House",
                "Weapon Shop",
                "Tool Shop",
                "Bungee Jump Place",
                "Cancel",
            ])? {
                0 => {
                    ctx.lines_as("Adventurer", args!["I have made a ^FF3355+^000000 mark", "on your mini map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "Only the chief knows the language",
                            "of the outside world. So you'd",
                            "better visit him before anything else."
                        ],
                    )?;
                    view_point(ctx, 1, 66, 250, 2, 16724821)?;
                }
                1 => {
                    ctx.lines_as("Adventurer", args!["I have made a ^CE6300+^000000 mark", "on your mini map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "The Utan Shaman has some",
                            "sort of mystic power...",
                            "People say she can create rough",
                            "enchanted stones, and divide a",
                            "pure enchanted stone into rough ones."
                        ],
                    )?;
                    view_point(ctx, 1, 217, 186, 3, 13525760)?;
                }
                2 => {
                    ctx.lines_as("Adventurer", args!["I have made a ^55FF33+^000000 mark", "on your mini map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "The Utans are usually well armed",
                            "in preparation for attacks from",
                            "their enemies. Apparently, they",
                            "have been attacked from the outside many times in the past."
                        ],
                    )?;
                    view_point(ctx, 1, 126, 154, 4, 5635891)?;
                }
                3 => {
                    ctx.lines_as("Adventurer", args!["I have made a ^3355FF+^000000 mark", "on your mini map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "There are many useful things for",
                            "traveling in the Tool Shop, so why don't you go look around?"
                        ],
                    )?;
                    view_point(ctx, 1, 136, 127, 5, 3364351)?;
                }
                4 => {
                    ctx.lines_as("Adventurer", args!["I have made a ^00FF00+^000000 mark", "on your mini map."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "Umbala has a unique locale called",
                            "the 'Bungee Jump Place'.",
                            "If you're interested in testing",
                            "your courage, why don't you go",
                            "and partake in this Utan",
                            "ritual yourself?"
                        ],
                    )?;
                    view_point(ctx, 1, 139, 198, 6, 65280)?;
                }
                5 => {
                    ctx.lines_as(
                        "Adventurer",
                        args![
                            "If you want to remove the location",
                            "marks on your mini map, please",
                            "choose 'Remove marks on the mini map' menu."
                        ],
                    )?;
                }
                _ => {}
            }
        }
        1 => {
            view_point(ctx, 2, 66, 250, 2, 16724821)?;
            view_point(ctx, 2, 217, 186, 3, 13525760)?;
            view_point(ctx, 2, 126, 154, 4, 5635891)?;
            view_point(ctx, 2, 136, 127, 5, 3364351)?;
            view_point(ctx, 2, 139, 198, 6, 65280)?;
            ctx.lines_as(
                "Adventurer",
                args![
                    "I removed all the marks from your",
                    "mini map. Feel free to ask me",
                    "again if you want me to mark building locations."
                ],
            )?;
        }
        2 => {
            ctx.lines_as("Adventurer", args!["It's fun to learn Utan culture on your own. Take care."])?;
        }
        _ => {}
    }
    ctx.close()
}
