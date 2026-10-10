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

pub fn f_auctionwarper(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Auction Hall Guide",
        args!["Hello, would you", "like to enter the", "Auction Hall?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        let l_num = runtime::arg(&args, 0, Val::from(0));
        if l_num == 1 || l_num == 4 {
            ctx.lines_as(
                "Auction Hall Guide",
                args!["Great! Well then,", "I hope you have fun", "and enjoy the auction~"],
            )?;
        } else {
            ctx.lines_as("Auction Hall Guide", args!["Enjoy your auction."])?;
        }
        ctx.close_window()?;
        if l_num == 1 {
            ctx.warp("auction_01", 179, 53)?;
            return Err(Stop::End);
        } else if l_num == 2 {
            ctx.warp("auction_01", 21, 43)?;
            return Err(Stop::End);
        } else if l_num == 3 {
            ctx.warp("auction_02", 151, 23)?;
            return Err(Stop::End);
        } else if l_num == 4 {
            ctx.warp("auction_02", 43, 24)?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Auction Hall Guide",
        args![
            "Alright then,",
            "see you later.",
            "If you change your",
            "mind, please come",
            "and enjoy the auctions~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}
