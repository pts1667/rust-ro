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

pub fn mailbox_dummy(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mailbox",
        args![
            "To use the mailbox service,",
            "you are required to pay 130 zeny.",
            "Would you like to use the service?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes.", "No."])? {
        0 => {
            ctx.mes("[Mailbox]")?;
            if ctx.player().zeny()? < 130 {
                ctx.lines(args![
                    "I am sorry, but you do not have enough money.",
                    "To use the mailbox service,",
                    "you are required to pay 130 zeny."
                ])?;
                return ctx.close();
            }
            ctx.mes("Thank you, please come again.")?;
            ctx.player().set_zeny(ctx.player().zeny()? - 130)?;
            ctx.close_window()?;
            ctx.call(Function::OpenMail, args![])?;
            return ctx.end();
        }
        1 => {
            ctx.lines_as("Mailbox", args!["Thank you, please come again."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn rodexmailboxinit(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rodexmailboxinit_oninit(ctx: &Ctx) -> Script {
    if ctx.constant("PACKETVER")?.number()? >= 20150513 {
        ctx.call(Function::UnloadNpc, args!["MailBox"])?;
    }
    ctx.end()
}
