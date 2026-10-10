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

pub fn stephen(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Stephen",
        args!["Guess what I've got?", "A tasty treat not easily found in Midgard...."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Stephen",
        args![
            "Chocolate!",
            "That's right, don't you love chocolate.... I do.",
            "And you are in luck, because I'm selling them for only 5,000 zeny a piece!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["I want some chocolate!", "No thanks."])? == 1 {
        ctx.lines_as(
            "Stephen",
            args![
                "You don't want any chocolate?",
                "I'm telling you! You'll regret it!",
                "You better get some now... you won't come across Chocolate like this ever again!",
                "Think it over and visit me again sometime."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Stephen",
        args![
            "Hah!",
            "I knew it!",
            "But I can't sell you more than 5 at once... but, if you really need more....",
            "you can come back again.",
            "So how many do you want?"
        ],
    )?;
    ctx.next()?;
    let (input, _status) = runtime::input_number(ctx, None, None)?;
    let amount = input.number()?;
    if amount <= 0 {
        return ctx.close();
    }
    ctx.mes("[Stephen]")?;
    if amount > 5 {
        ctx.mes("I'm sorry, but I can't give you that many.")?;
        return ctx.close();
    }
    if ctx.player().zeny()? < amount * 5000 {
        ctx.mes("I'm sorry, but it seems you can't afford to buy these off me.")?;
        return ctx.close();
    }
    ctx.player().set_zeny(ctx.player().zeny()? - amount * 5000)?;
    ctx.call(Function::GetItem, args![558, amount])?;
    ctx.lines(args![
        "There you go!",
        "You can give that to someone as a gift, or enjoy it yourself!",
        "Mmm... sweet chocolate...",
        "Visit me anytime...!"
    ])?;
    ctx.close()
}

pub fn jainie(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Jainie",
        args![
            "You know what? The chocolate that my boyfriend sells are from me!",
            "I made them by myself."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jainie",
        args![
            "You know ... In cetain countries, there's a tradition of presenting chocolates to a person that you love...",
            "They call it, ^3355FFValentine's Day^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jainie",
        args![
            "So I gave him my delicious chocolate...",
            "And then he made me cook a lot more...",
            "And now he is selling them to everyone.",
            "I guess he really enjoyed it.",
            "But, I do feel good when people buy something I have made."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jainie",
        args![
            "It would be great if you bought some too...",
            "I will be making chocolates for a while so..."
        ],
    )?;
    ctx.close()
}

pub fn carl_orleans(ctx: &Ctx) -> Script {
    ctx.lines_as("Carl Orleans", args!["Yes?"])?;
    ctx.next()?;
    if ctx.menu(&["I want some hand made chocolate...", "I'm lost, sorry to bother you."])? == 1 {
        ctx.lines_as(
            "Carl Orleans",
            args![
                "Oh... well, if you want me to make some of my special Hand Made Chocolate....",
                "You will need to give me at least ^0000FF 3 Chocolates^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Carl Orleans",
            args![
                "That's right, only ^0000FF 3 Chocolates^000000",
                "Bring them to me and you'll get what you came for."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Carl Orleans", args!["See You."])?;
        return ctx.close();
    }
    ctx.lines_as("Carl Orleans", args!["Well, I just might be able to fulfill your needs..."])?;
    ctx.next()?;
    ctx.mes("[Carl Orleans]")?;
    if ctx.items().count(558)? < 3 {
        ctx.mes("I'm sorry, you don't have enough Chocolate Bars to do this.")?;
        return ctx.close();
    }
    ctx.items().take(558, 3)?;
    ctx.lines(args!["You got 3 pieces of pure chocolate, I see.", "Give them to me..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Carl Orleans",
        args!["Ok, now I will only create my special hand made chocolates if you promise to use it wisely."],
    )?;
    ctx.next()?;
    ctx.lines_as("Carl Orleans", args!["....Hmmmmmm.....", "Well..."])?;
    ctx.next()?;
    ctx.mes("[Carl Orleans]")?;
    ctx.items().give(559, 1)?;
    ctx.lines(args![
        "Here.",
        "I hope you give it to someone special, because its a special chocolate.",
        "As you know... only I can create this."
    ])?;
    ctx.next()?;
    ctx.lines_as("Carl Orleans", args!["Enjoy."])?;
    ctx.close()
}
