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

pub fn smile_assistance(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Smile Girl",
        args!["Hi ~ Hi ~", "This is Smile Assistance.", "How may I help you ?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["^3355FFMr. Smile^000000 ?", "Construct ^3355FFMr. Smile^000000 ", "Quit"])? {
        0 => {
            ctx.lines_as(
                "Smile Girl",
                args![
                    "National Event held by the command of ^5577FFHis majesty Tristram the 3rd^000000,",
                    "that intends to encourage the nation of the Rune-Midgarts Kingdom",
                    " to play in more enjoyable atmosphere!",
                    "I am ^3355FF' Smile Assistance '^000000,",
                    ".. who leads the national event under the name of ",
                    "^3355FFSmile throughout the Rune-Midgarts Kingdom~^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Smile Girl",
                args![
                    "With simple and easy-to-get items,",
                    "I can provide you",
                    "^3355FF' Mr. Smile '^000000.",
                    "The requirements are followings."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Smile Girl",
                args!["^3355FF10 Jellopy^000000", "^3355FF10 Fluff^000000", "^3355FF10 Clover^000000"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Smile Girl",
                args![
                    "With this event",
                    "Everybody will be happy and smile,",
                    "getting together with other people,",
                    "And will try to make Ragnarok the most enjoyable game in the world."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.items().count(909)? > 9 && ctx.items().count(914)? > 9 && ctx.items().count(705)? > 9 {
                ctx.lines_as("Smile Girl", args!["Congratulations !", "Now please take this Mr.Smile."])?;
                ctx.next()?;
                ctx.items().take(909, 10)?;
                ctx.items().take(914, 10)?;
                ctx.items().take(705, 10)?;
                ctx.items().give(2278, 1)?;
                ctx.lines_as(
                    "Smile Girl",
                    args![
                        "His majesty,Tristram the 3rd",
                        "has promised to try his best to make Ragnarok better and more enjoyable."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "The fact mentioned above",
                    "was announced by",
                    "the Public Information Bureau of the Rune-Midgarts Kingom."
                ])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Smile Girl",
                args![
                    "Oh - unfortunately",
                    "You have not brought",
                    "enough items for Mr. Smile.",
                    "^3355FF10 Jellopy^000000",
                    "^3355FF10 Fluff^000000",
                    "^3355FF10 Clover^000000",
                    "Please check the requirements above."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Smile Girl",
                args![
                    "Thank you for visiting us.",
                    "We ..",
                    "The Rune-Midgarts Kingdom",
                    "always try to make Ragnarok",
                    "better and more enjoyable game."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Smile Girl",
                args![
                    "We sincerely ask you",
                    "to cooperate.",
                    "The fact mentioned above",
                    "was announced by the Public Information Bureau of the Rune-Midgarts Kingom."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
