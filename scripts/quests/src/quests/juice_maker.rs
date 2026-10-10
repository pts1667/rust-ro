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

pub fn marianne_juice(ctx: &Ctx) -> Script {
    if (ctx.var("misc_quest").get()?.number()? & 1) != 0 || ctx.var("morison_meat").get()? == 15 {
        ctx.lines_as("Housewife Marianne", args!["Whew...!", "Still, he won't eat anything unless it's Meat. But maybe he will eat fruit if it was cut so that it was easy to eat. Like, if it was blended into juice..."])?;
        ctx.next()?;
        ctx.var("mother_marienu").set(0)?;
        ctx.var("morison_meat").set(0)?;
        ctx.var("misc_quest").set(ctx.var("misc_quest").get()?.number()? | 1)?;
        ctx.lines_as(
            "Housewife Marianne",
            args!["Ah! Come to think of it, I heard they were making fruit juice somewhere. Now where was it... Payon Village, or Morocc?"],
        )?;
        return ctx.close();
    }
    if ctx.var("mother_marienu").get()? == 1 {
        ctx.lines_as(
            "Housewife Marianne",
            args!["Morrison!! Eat some fruits!! You don't want to become a slobby fat pig when you grow up, do you?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Talk", "Cancel"])? == 0 {
            ctx.lines_as(
                "Housewife Marianne",
                args![
                    "Hm? ...You!",
                    "You're the one who gave, my little Morrison that Meat?! Did you come here thinking I wouldn't know about it?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Housewife Marianne", args!["Go away, get out of my house NOW!!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Housewife Marianne",
            args!["Morrison!! I'm going to be very very mad if you keep doing this!"],
        )?;
        return ctx.close();
    }
    if ctx.var("morison_meat").get()?.number()? > 0 {
        ctx.var("mother_marienu").set(1)?;
        ctx.lines_as(
            "Housewife Marianne",
            args![
                "Oh!!.... This...",
                "what is this...??",
                "How could you do something like this to my boy?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Housewife Marianne", args!["Argh! Get out of my house right this instant!"])?;
        return ctx.close();
    }
    ctx.lines_as("Housewife Marianne", args!["Morrison!! Please eat some fruits!! Please~!"])?;
    ctx.next()?;
    ctx.lines_as("Housewife Marinaa", args!["Sigh~!! Like father, like son..."])?;
    ctx.next()?;
    // The menu is still shown, but no branch depends on the answer.
    runtime::select_values(ctx, &[Val::from("Talk:Cancel")])?;
    ctx.lines_as(
        "Housewife Marianne",
        args!["Morrison!! You're going to be in big trouble if you keep this up!"],
    )?;
    ctx.close()
}

pub fn morrison_juice(ctx: &Ctx) -> Script {
    if (ctx.var("misc_quest").get()?.number()? & 1) != 0 || ctx.var("morison_meat").get()? == 15 {
        ctx.lines_as(
            "Little Morrison",
            args![
                "Bleh... Forget it.",
                "I'm just going to shrivel to death just eating fruits. Don't bother worrying about me ."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("morison_meat").get()?.number()? > 9 {
        ctx.lines_as(
            "Little Morrison",
            args![
                "Ah... so full~",
                "I think I can live now.",
                "You don't have to give me any more Meat. I feel like I'm going to explode if I eat any more."
            ],
        )?;
        if ctx.var("morison_meat").get()? == 10 {
            ctx.next()?;
            ctx.var("morison_meat").set(ctx.var("morison_meat").get()?.number()? + 1)?;
            ctx.lines_as("Little Morrison", args!["Oh... and... um.", "Take this."])?;
            ctx.next()?;
            ctx.lines_as("Little Morrison", args!["It's a little something I've been saving to eat for later, but since you gave me Meat, I think I can pass on the sweets."])?;
            return ctx.close();
        }
        return ctx.close();
    }
    ctx.lines_as(
        "Little Morrison",
        args!["Agh....Noooo!!!", "No, don't make me eat it! I can't bear to taste fruits!"],
    )?;
    ctx.next()?;
    if ctx.items().count(517)? > 0 {
        match ctx.menu(&["Talk", "Show the Meat", "Cancel"])? {
            0 => {
                ctx.lines_as("Little Morrison", args!["Aaaagh!! Once or twice is enough!! I refuse to eat any more fruits! You have to peel them and there's so much juice that it makes you feel icky..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Little Morrison",
                    args!["And they're all slippery and sour... Even if I eat it, I still feel hungry. Aaaah~! Give me Meat!"],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Little Morrison",
                    args![
                        "Ooh! M-Meaaat~",
                        "Ah... hu...hungry...",
                        "The scent of Meat...",
                        "Excuse me...",
                        "C-can I please have one?"
                    ],
                )?;
                ctx.next()?;
                if ctx.menu(&["Give Meat", "Don't Give Meat"])? == 0 {
                    ctx.items().take(517, 1)?;
                    ctx.var("morison_meat").set(ctx.var("morison_meat").get()?.number()? + 1)?;
                    ctx.lines_as(
                        "Little Morrison",
                        args![
                            "Wow~!!! Meat!!",
                            "So yummy!",
                            "Thank you,",
                            "I think I can",
                            "live now.",
                            "*Chew chew*"
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.var("morison_meat").set(15)?;
                ctx.lines_as(
                    "Little Morrison",
                    args!["Waah...!", "Fine, I get it now.", "Adults are all the same!"],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Little Morrison",
                    args!["Aaaaah!!! No matter what, I'm not going to eat fruits and vegetables!"],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    if ctx.menu(&["Talk", "Cancel"])? == 0 {
        ctx.lines_as("Little Morrison", args!["Aaah!! I can't eat any more fruits!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Little Morrison",
            args!["I refuse to eat any more fruits! You have to peel them and there's so much juice that it makes you feel icky..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Morrison",
            args![
                "And they're all slippery and sour... Even if I eat it, I still feel hungry.",
                "Aaaah~!",
                "Give me Meat!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Little Morrison",
        args!["Aaaaah!!! No matter what, I'm not going to eat fruits and vegetables!"],
    )?;
    ctx.close()
}

fn explain_fruit_juice(ctx: &Ctx) -> Script {
    ctx.lines_as("Merchant Marx Hansen", args!["Before humans were able to develop such vast societies, they gathered fruit from trees to survive. Fruits may have been nature's blessing that allowed us to exist in the world."])?;
    ctx.next()?;
    ctx.lines_as("Merchant Marx Hansen", args!["Since life became so prosperous, the younger generation seems not to eat fruit any more. So, I started thinking of ways to make fruit easier to eat."])?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant Marx Hansen",
        args!["I realized that when you make fruit juice, it's more convenient to eat and has a much better taste."],
    )
}

pub fn marx_hansen_juice(ctx: &Ctx) -> Script {
    let mut l_fruit = 0;
    let mut l_juice = 0;
    let mut l_make = 0;
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please come back later -",
            "- after you put some items into kafra storage. -"
        ])?;
        return ctx.close();
    }
    if (ctx.var("misc_quest").get()?.number()? & 1) != 0 {
        ctx.lines_as(
            "Merchant Marx Hansen",
            args!["Welcome.", "Did you come to", "process fruits as well?"],
        )?;
        ctx.next()?;
        match ctx.menu(&["Make Juice.", "Talk and get information about fruit processing.", "Cancel."])? {
            0 => {
                ctx.lines_as(
                    "Merchant Marx Hansen",
                    args!["What kind of fruit juice would you like to make?"],
                )?;
                ctx.next()?;
                match ctx.menu(&["Apple Juice", "Banana Juice", "Carrot Juice", "Grape Juice", "Cancel"])? {
                    0 => {
                        l_fruit = 512;
                        l_juice = 531;
                    }
                    1 => {
                        l_fruit = 513;
                        l_juice = 532;
                    }
                    2 => {
                        l_fruit = 515;
                        l_juice = 534;
                    }
                    3 => {
                        l_fruit = 514;
                        l_juice = 533;
                    }
                    4 => {
                        ctx.lines_as("Merchant Marx Hansen", args!["Well then...", "See you next time."])?;
                        return ctx.close();
                    }
                    _ => {}
                }
                if ctx.call(Function::CountItem, args![l_fruit])? == 0
                    || ctx.call(Function::CountItem, args![713])? == 0
                    || ctx.player().zeny()? < 3
                {
                    ctx.lines_as(
                        "Merchant Marx Hansen",
                        args![
                            "Oh no...",
                            Val::from("You don't have all the necessary materials. To make ")
                                + ctx.call(Function::GetItemName, args![l_juice])?
                                + Val::from(", I need 1 ")
                                + ctx.call(Function::GetItemName, args![l_fruit])?
                                + Val::from(" and 1 Empty Bottle. I will also need a 3 zeny fee.")
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Merchant Marx Hansen",
                        args!["When you have prepared everything, I will blend the fruit to give you delicious juice."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as("Merchant Marx Hansen", args!["How many would you like?"])?;
                ctx.next()?;
                match ctx.menu(&["As many as I can.", "I want a certain amount.", "Cancel."])? {
                    0 => {
                        l_make = ctx.call(Function::CountItem, args![l_fruit])?.number()?;
                        if ctx.call(Function::CountItem, args![713])?.number()? < l_make {
                            l_make = ctx.call(Function::CountItem, args![713])?.number()?;
                        }
                        if ctx.player().zeny()? / 3 < l_make {
                            l_make = ctx.player().zeny()? / 3;
                        }
                    }
                    1 => {
                        ctx.lines_as(
                            "Merchant Marx Hansen",
                            args![
                                Val::from("Choose a number less than 100. If you don't want to, put '0'. You can make up to ")
                                    + ctx.call(Function::CountItem, args![l_fruit])?
                                    + Val::from(" bottles of juice.")
                            ],
                        )?;
                        ctx.next()?;
                        l_make = loop {
                            let (input, _) = runtime::input_number(ctx, None, None)?;
                            if input == 0 {
                                ctx.lines_as("Merchant Marx Hansen", args!["Well then...", "Come again."])?;
                                return ctx.close();
                            }
                            if input.number()? <= 100 {
                                break input.number()?;
                            }
                            ctx.lines_as(
                                "Merchant Marx Hansen",
                                args!["More than 100 bottles is impossible. Choose a different amount."],
                            )?;
                            ctx.next()?;
                        };
                    }
                    2 => {
                        ctx.lines_as("Merchant Marx Hansen", args!["Well then...", "Come again."])?;
                        return ctx.close();
                    }
                    _ => {}
                }
                let total_zeny = 3 * l_make;
                if ctx.call(Function::CountItem, args![l_fruit])?.number()? < l_make
                    || ctx.call(Function::CountItem, args![713])?.number()? < l_make
                    || ctx.player().zeny()? < total_zeny
                {
                    ctx.lines_as("Merchant Marx Hansen", args!["Oh no...", "You don't have all the necessary materials. I can't help a situation like this. I guess you collect what you need."])?;
                    return ctx.close();
                }
                ctx.call(Function::DelItem, args![l_fruit, l_make])?;
                ctx.call(Function::DelItem, args![713, l_make])?;
                ctx.player().set_zeny(ctx.player().zeny()? - total_zeny)?;
                ctx.call(Function::GetItem, args![l_juice, l_make])?;
                ctx.lines_as(
                    "Merchant Marx Hansen",
                    args!["Here you go! Fresh and delicious juice as promised. It should be very refreshing and palatable."],
                )?;
                ctx.next()?;
                ctx.lines_as("Merchant Marx Hansen", args!["Well then...", "Come again."])?;
                return ctx.close();
            }
            1 => {
                explain_fruit_juice(ctx)?;
                ctx.next()?;
                ctx.lines(args![
                    "# Fruit Juice Information #",
                    "^CC4E5C- Apple Juice -^000000",
                    "Apple x 1 ea, Empty Bottle x 1 ea, 3 zeny.",
                    "^E3CF57- Banana Juice -^000000",
                    "Banana x 1 ea, Empty Bottle x 1 ea, 3 zeny.",
                    "^ED9121- Carrot Juice -^000000",
                    "Carrot x 1 ea, Empty Bottle x 1 ea, 3 zeny.",
                    "^CC00FF- Grape Juice -^000000",
                    "Grape x 1 ea, Empty Bottle x 1 ea, 3 zeny."
                ])?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as("Merchant Marx Hansen", args!["Hey!", "If you visit", "somebody, talk to them!"])?;
                return ctx.close();
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Merchant Marx Hansen", args!["Welcome.", "How may I help you?"])?;
        ctx.next()?;
        if ctx.menu(&["Talk", "Cancel"])? == 0 {
            explain_fruit_juice(ctx)?;
            return ctx.close();
        }
        ctx.lines_as("Merchant Marx Hansen", args!["Hey!", "If you vist", "somebody, talk to them!"])?;
        return ctx.close();
    }
    Ok(())
}
