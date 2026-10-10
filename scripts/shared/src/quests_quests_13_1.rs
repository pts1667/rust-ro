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

pub fn guard_13_1(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Guard]")?;
    if ctx.var("ep13_ryu").get()?.number()? > 19 {
        ctx.lines(args![
            "You're not allowed to enter.",
            "All of you are instructed to go to the Time-Gap of Dimension on Rune-Midgarts!"
        ])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Just go there.", "This is a restricted area."])?;
        ctx.close_window()?;
        ctx.call(
            Function::Warp,
            args![
                "lhz_in01",
                runtime::arg(&args, 1, Val::from(0)),
                runtime::arg(&args, 2, Val::from(0))
            ],
        )?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_ryu").get()?.number()? > 8 {
        ctx.lines(args!["You've come here", "to register as a member of the three kingdoms?"])?;
        ctx.next()?;
        ctx.lines_as("Guard", args!["Enter."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["You are not allowed", "to enter."])?;
    if !runtime::arg(&args, 0, Val::from(0)).is_true() {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    ctx.call(
        Function::Warp,
        args![
            "lhz_in01",
            runtime::arg(&args, 1, Val::from(0)),
            runtime::arg(&args, 2, Val::from(0))
        ],
    )?;
    Err(Stop::End)
}

pub fn promotional_staff(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Promotional Staff",
        args![
            "We are looking for adventurers who are super curious and extremely brave.",
            "Join us for a wonderful adventure!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Promotional Staff",
        args!["Hey, you're a knowledgeable person, right? Are you interested in my story?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes.", "No, thanks."])? == 0 {
        ctx.lines_as(
            "Promotional Staff",
            args!["You're a real adventurer.", "Good for you!", "You won't regret it."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "I usually send adventurers to",
                "newly found places for research.",
                "It's quite challeging, as nobody's ever been to these places."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "The missions are quite dangerous,",
                "so only those who are courageous",
                "are qualified for the challenge",
                "of this mission."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Promotional Staff", args!["I'm not sure that you're strong enough, but you seem brave. How about going to the kingdom receptionist? He should be in the first room of Prontera Castle."])?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "From what I've heard about this place... It's not a new continent.",
                "I don't know where it is.",
                "Hmm... Inside of the sky?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args!["Could just be a rumor,", "but I don't know exactly.", "It's not my business."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Promotional Staff",
            args![
                "Anyway, I'm supposed to inform many adventurers about it.",
                "There's no time to waste!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Promotional Staff",
        args!["Huh, I thought you were a real adventurer. You're missing a big opportunity. You're definitely not brave. Absolutely not!"],
    )?;
    ctx.close_window()?;
    Err(Stop::End)
}
