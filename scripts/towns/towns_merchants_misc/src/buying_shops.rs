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

pub fn black_marketeer_buying(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0
        || ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2400
    {
        ctx.lines_as(
            "Mr. Jass",
            args![
                "You're too greedy, even compared to someone like me!",
                "Why don't you go lighten your bag first?"
            ],
        )?;
        return ctx.close();
    }
    if ctx.call(Function::GetSkillLv, args!["ALL_BUYING_STORE"])? == 1 {
        ctx.lines_as(
            "Mr. Jass",
            args!["Hey, you already made a contract with Hugh.", "I don't have any business with you."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Mr. Jass",
        args!["You must need something badly to come to find me.", "What do you want?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Bulk Buyer Shop License", "Who are you?", "Nothing, nothing!"])? {
        0 => {
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "I knew it!",
                    "Sure, I can make it for you.",
                    "Mine looks just like the authentic one that Merchants get from that bastard Mr. Hugh!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "And my license is better 'cuz you don't need ta' learn any skills.",
                    "How many do you want?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "Just so you know, I can only make them in small quantities, up to 10 at a time.",
                    "It'll cost 500 zeny for each one."
                ],
            )?;
            ctx.next()?;
            loop {
                let (input, _) = runtime::input_number(ctx, None, None)?;
                let count = input.number()?;
                ctx.mes("[Mr. Jass]")?;
                if count == 0 {
                    ctx.mes("Don't you need those licenses?")?;
                    return ctx.close();
                } else if count > 10 {
                    ctx.mes("I can only make up to 10 at a time, you know.")?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![Val::from("It'll cost ") + Val::from(count * 500) + Val::from(" zeny.")])?;
                    if ctx.player().zeny()? < count * 500 {
                        ctx.mes("but you don't have enough money.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Jass",
                            args![
                                "Don't you know the basics of business? Everything has a price.",
                                "If you want something, you gotta pay for it."
                            ],
                        )?;
                    } else {
                        ctx.lines(args![
                            "Ha... Ha ha ha!",
                            "Mr. Hugh, I'll take over your license business. You'll see!",
                            "*Giggle Giggle*"
                        ])?;
                        ctx.call(Function::GetItem, args![12548, count])?;
                        ctx.player().set_zeny(ctx.player().zeny()? - count * 500)?;
                    }
                    return ctx.close();
                }
            }
        }
        1 => {
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "I left my hometown a long time ago.",
                    "It's meaningless to ask who I am because all I've got left now is my hatred."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "...",
                    "Hugh is a corrupt merchant with no sense of business ethics.",
                    "My sole purpose in life is to destroy Hugh."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "Aw, I drank too much... (*Hic*)",
                    "That's just the alohol talking.",
                    "Please forget anything I said."
                ],
            )?;
            ctx.close()
        }
        _ => {
            ctx.lines_as(
                "Mr. Jass",
                args![
                    "Alright, alright! You don't have to yell.",
                    "Just leave me alone if you've got no business with me."
                ],
            )?;
            ctx.close()
        }
    }
}

pub fn purchasing_team_buying(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0
        || ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2400
    {
        ctx.mes("- You cannot converse or perform the quest because you are carrying too many items. -")?;
        return ctx.close();
    }
    ctx.mes("[Mr. Hugh]")?;
    if ctx.var("BaseClass").get()? == constants::JOB_MERCHANT && ctx.call(Function::GetSkillLv, args!["MC_VENDING"])?.number()? >= 1 {
        if ctx.call(Function::GetSkillLv, args!["ALL_BUYING_STORE"])? == 1 {
            ctx.lines(args!["I'm Hugh from the Purchasing Team.", "How may I help you today?"])?;
            ctx.next()?;
            if ctx.menu(&["Purchase Bulk Buyer Shop License", "Quit"])? == 1 {
                ctx.lines_as(
                    "Mr. Hugh",
                    args![
                        "Please feel free to ask me if you need any Bulk Buyer Shop Licenses.",
                        "Come again~"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "It's 200 zeny for each Bulk Buyer Shop License, and you may purchase up to 50 at a time.",
                    "How many licenses do you need?"
                ],
            )?;
            ctx.next()?;
            loop {
                let (input, _) = runtime::input_number(ctx, None, None)?;
                let count = input.number()?;
                ctx.mes("[Mr. Hugh]")?;
                if count == 0 {
                    ctx.lines(args!["You have cancelled the trade.", "Have a good day."])?;
                    return ctx.close();
                } else if count > 50 {
                    ctx.mes("Please enter a value of 50 or less.")?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        Val::from("It'll cost ")
                            + Val::from(count * 200)
                            + Val::from(" zeny for ")
                            + Val::from(count)
                            + Val::from(" licenses.")
                    ])?;
                    if ctx.player().zeny()? < count * 200 {
                        ctx.mes("but you don't seem to have enough money.")?;
                    } else {
                        ctx.mes("Thank you for your patronage.")?;
                        ctx.call(Function::GetItem, args![6377, count])?;
                        ctx.player().set_zeny(ctx.player().zeny()? - count * 200)?;
                    }
                    return ctx.close();
                }
            }
        } else {
            ctx.lines(args![
                "I'm Hugh from the Purchasing Team at the Alberta Merchant Guild.",
                "You're..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args!["Did you know? Our guild has issued a license to allow individuals to buy goods from others."],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("I've never had problems buying items...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "You're right, but think about it:",
                    "haven't you had a hard time buying in bulk?",
                    "You'd have to find and talk to everyone that has an item you want.",
                    "Pretty inconvenient, isn't it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "Since buying in bulk is an important issue to us Merchants,",
                    "I've proposed an innovative plan to our guild, based on my 10 years of experience in making purchases."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "'Let Individuals Open",
                    "a Bulk Buyer Shop!'",
                    "That's the title of my proposal.",
                    "You'll see, when you read it..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args!["...", "...(Mr. Hugh yammers on and on with all the details.)"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Alright, what's your point?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Mr. Hugh",
                args!["Oh, yes. In summary,", "you can buy certain items in bulk through Vending."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args!["In order to open a Bulk Buyer Shop, you need a license issued from the Merchant Guild."],
            )?;
            ctx.next()?;
            ctx.lines_as("Mr. Hugh", args!["You need it every time you open the shop. We're expecting a significant increase in profits through this new kind of licensing."])?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "That's the point of my proposal!",
                    "Our president was so happy to hear that we're going to make big bucks!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.player().name()?, args!["Please get to the point already!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "Don't be so impatient, alright?",
                    "My point is, we can let you open the Bulk Buyer Shop if you've learned Vending."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "Of course, we charge 10,000 zeny as a one-time registration fee.",
                    "You will need the ^4A4AFFBulk Buyer Shop License^000000 every time you open the shop."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    "Say, would you like to register now?",
                    "If you do, I'll teach you how to open the Bulk Buyer Shop."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Learn how to open Bulk Buyer Shop", "Cancel"])? == 1 {
                ctx.lines_as(
                    "Mr. Hugh",
                    args![
                        "Man, that's disappointing!",
                        "Using this bulk buyer option can benefit your business in many ways, but it's your call.",
                        "I'm always open for consultation!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.mes("[Mr. Hugh]")?;
            if ctx.player().zeny()? < 10000 {
                ctx.lines(args![
                    "The registration fee is 10,000 zeny.",
                    "Please have the fee ready first."
                ])?;
                return ctx.close();
            }
            ctx.lines(args![
                "You've made a good decision.",
                "Please give me the registration fee, and sign right here...."
            ])?;
            ctx.next()?;
            let (name, _) = runtime::input_text(ctx, None, None)?;
            ctx.lines_as(
                "Mr. Hugh",
                args![
                    name + Val::from("...."),
                    "I like your handwriting.",
                    "Okay, you're now approved to open the Bulk Buyer Shop."
                ],
            )?;
            ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
            ctx.items().give(6377, 5)?;
            ctx.call(Function::Skill, args!["ALL_BUYING_STORE", 1, constants::SKILL_PERM_GRANT])?;
            ctx.next()?;
            ctx.lines_as("Mr. Hugh", args!["Currently, only normal items ^8C2121EXCEPT^000000 equipment, certain potions, and hand-crafted items can be purchased in bulk, but this can still be very beneficial to you, depending on how you use it."])?;
            ctx.next()?;
            ctx.lines_as("Mr. Hugh", args!["Oh, and you need at least one of the item that you want to buy in your inventory because you have to show it to other through your shop."])?;
            ctx.next()?;
            ctx.lines_as("Mr. Hugh", args!["Your skill should now be registered in your skill window. If you can't see it you probably have to minimize your Skill List and check the 3rd Job Tab."])?;
            ctx.next()?;
            ctx.lines_as(
                "Mr. Hugh",
                args!["I've given you 5 Bulk Buyer Shop Licenses for your trial.", "Enjoy shopping!"],
            )?;
            return ctx.close();
        }
    }
    ctx.lines(args![
        "I'm Hugh from the Purchasing Team at the Alberta Merchant Guild.",
        "I'd love to chat, but I'm too busy at the moment."
    ])?;
    ctx.next()?;
    ctx.lines_as(
        "Mr. Hugh",
        args!["My time is solely dedicated to our customers in the Merchant industry."],
    )?;
    ctx.close()
}
