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

pub fn edgar_izlude(ctx: &Ctx) -> Script {
    if (ctx.var("misc_quest").get()?.number()? & 16) != 0 {
        ctx.lines_as(
            "Edgar",
            args![
                "So are you heading to Alberta again? Let me give you the same discount and only charge 250 Zeny, just like the last time.",
                "How's that sound?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Alrighty~!", "Why are you being so nice to me?!"])? == 0 {
            if ctx.player().zeny()? < 250 {
                ctx.lines_as(
                    "Edgar",
                    args!["Um...", "This isn't", "enough money.", "Why don't you go", "get some more cash?"],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 250)?;
            ctx.warp("alberta", 195, 164)?;
            return ctx.end();
        }
        ctx.lines_as("Edgar", args!["It's just the way I am. That, and your devilish smile reminds me of my beloved blond haired son who left home years ago to become a Sailor on his own ship. Bless his soul, wherever he is."])?;
        return ctx.close();
    }
    ctx.lines_as("Edgar", args!["My town, Izlude, is connected to Alberta by the harbor in the West. There is so much traffic between us, I almost become an Albertian.  Hehehe~"])?;
    ctx.next()?;
    ctx.lines_as(
        "Edgar",
        args!["There's this guy I know pretty well, Phelix, who lives in Alberta. That guy is really stingy... He charges for everything!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Edgar", args!["But he's really a nice guy and likes helping other people. He has a good heart and will give you his support if you meet his price."])?;
    ctx.next()?;
    ctx.lines_as("Edgar", args!["Lately, people in Alberta say that he is really trying to help folks and that his demand for Jellopies is just a coverup.  Well, you should take a look at what he has to offer."])?;
    ctx.next()?;
    if ctx.menu(&["Can you tell me how to get to Alberta?", "End Conversation"])? == 0 {
        ctx.lines_as(
            "Edgar",
            args!["Huh? Well, you can use your feet and just walk.  But if you have money, I'd like to suggest that you take a ship."],
        )?;
        ctx.next()?;
        if ctx.menu(&["Okay, gotcha.", "But I'm sick of walking and I'm broke!"])? == 0 {
            ctx.lines_as("Edgar", args!["Alrighty, take care~"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Edgar",
            args![
                "Okay...",
                "You don't want to walk AND you've got no cash, but you still want to go there? Oh geez..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Edgar",
            args!["Fine fine. Me, being the captain of a ship, can afford to bring you there at a lower price. How does 250 Zeny sound?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Alrighty~!", "Bah, what a rip off!!"])? == 0 {
            ctx.var("misc_quest").set(Val::from(ctx.var("misc_quest").get()?.number()? | 16))?;
            if ctx.player().zeny()? < 250 {
                ctx.lines_as(
                    "Edgar",
                    args!["Um...", "This isn't", "enough money.", "Go and get", "some more."],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 250)?;
            ctx.warp("alberta", 195, 164)?;
            return ctx.end();
        }
        ctx.lines_as("Edgar", args!["Boy oh boy,", "if you think", "that's a rip off..."])?;
        return ctx.close();
    }
    ctx.lines_as("Edgar", args!["Yeah, alright.", "See you later~"])?;
    ctx.close()
}
