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

pub fn auction_hall_guide_moc(ctx: &Ctx) -> Script {
    shared::other_auction::f_auctionwarper(ctx, args![1])?;
    Ok(())
}

pub fn auction_hall_guide_prt(ctx: &Ctx) -> Script {
    shared::other_auction::f_auctionwarper(ctx, args![2])?;
    Ok(())
}

pub fn auction_hall_guide_yuno(ctx: &Ctx) -> Script {
    shared::other_auction::f_auctionwarper(ctx, args![3])?;
    Ok(())
}

pub fn auction_hall_guide_lhz(ctx: &Ctx) -> Script {
    shared::other_auction::f_auctionwarper(ctx, args![4])?;
    Ok(())
}

pub fn information_post_dum(ctx: &Ctx) -> Script {
    ctx.lines_as("Information", args!["Auction Warp Guide"])?;
    ctx.close()
}

pub fn auction_broker_dum(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Auction Broker",
        args!["Welcome to the Auction Hall.", "Would you like to view the goods?"],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        ctx.lines_as(
            "Auction Broker",
            args!["Very well.", "Please take", "a look, and see", "What's being offered~"],
        )?;
        ctx.call(Function::OpenAuction, vec![])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Auction Broker",
        args![
            "Very well, then.",
            "If you change your",
            "mind, then please",
            "come and check",
            "out the auctions~"
        ],
    )?;
    ctx.close()
}
