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

pub fn f_clocktowergate(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let floor = runtime::arg(&args, 0, Val::from(0));
    let item_req = runtime::arg(&args, 1, Val::from(0));
    ctx.lines_as(
        "Gatekeeper Boy",
        args![
            "Welcome to",
            "Kinase - Blue Gallino",
            "The one of Local Speciality in Aldebaran.",
            Val::from("You can't go through from ") + floor.clone() + Val::from(" Floor,"),
            "Please go back."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("About Clock Tower:About the ")
            + floor.clone()
            + Val::from(" Floor:Move to the ")
            + floor
            + Val::from(" Floor:End Dialogue")],
    )? {
        1 => {
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "Homeland of Alchemy, Aldebaran!",
                    "Long Time ago, there were",
                    "3 Legendary Alchemists...They are",
                    "Bruke Seimer",
                    "Philip Warisez",
                    "And .."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "Romero Specialre!",
                    "This venerable architecture is",
                    "their masterpiece.",
                    "I assume you would feel something unusual",
                    "While on the way to this floor,",
                    "Every feature of This Clocktower "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "Consists of Mysterious Ancient Magics.",
                    "If you just wander around here,",
                    " without any intention"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "By any means,",
                    "You will meet with a mishap",
                    "by Gatekeeper Creatures.",
                    "Please be careful .."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "Ancient Alchemists",
                    "Sealed the Gate of 4th Floor using an Alchemistic Device ",
                    "To keep something",
                    "From Evil Creatures and Human Enemies.",
                    "To go through this door"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "It needs a Key.",
                    "That Key has rumored to be possessed by Gatekeeper Creatures",
                    "Prowling around here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "The Key is the Intensiveness of Ancient Alchemy,",
                    "By hearsay When used once,",
                    "It will be released from being spelled",
                    "And be disappeared."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "If that key",
                    "Comes into your possession,",
                    "Please show me.",
                    "The one who possesses the Key",
                    "Will have access to go through",
                    "This Gate with his own will!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Gatekeeper Boy", args!["I will give you a chance.", ". . . . ."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            if ctx.call(Function::CountItem, args![item_req.clone()])?.number()? > 0 {
                ctx.lines_as(
                    "Gatekeeper Boy",
                    args![
                        "Hmm! I already felt that you are not an Ordinary person,",
                        "Now it seems to be successful in Speculation.",
                        "Please, you may enter.",
                        "May God bless you .."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DelItem, args![item_req, 1])?;
                ctx.call(
                    Function::Warp,
                    args![
                        runtime::arg(&args, 2, Val::from(0)),
                        runtime::arg(&args, 3, Val::from(0)),
                        runtime::arg(&args, 4, Val::from(0)),
                    ],
                )?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    ". . . . . .",
                    "Unfortunately you don't have a privilege",
                    "To enter this Gate ..",
                    "You won't be able to go through",
                    "As long as Ancient Alchemists",
                    " Don't grant you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.lines_as(
                "Gatekeeper Boy",
                args![
                    "This Clock Tower",
                    "Is the place where the 3 Ancient Legendary Alchemists",
                    "Has left their Spirits and Skills.",
                    "Please Do not Scribble or Damage on the Interior."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}
