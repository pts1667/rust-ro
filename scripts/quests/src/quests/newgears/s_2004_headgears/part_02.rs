use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hat_merchant_zero_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Mad Hatter",
        args![
            "Hey there. Um...",
            "I'm the 'Mad Hatter.' Well, at least that's what I call myself."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Mad Hatter", args!["If you give me some stuff, I can renovate some of your old hats so that they really show your personality. Whether you're laid back or a party animal, I'll be able to make a hat just for you!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Mad Hatter",
        args!["You wanna take a look at the trendy, stylish hats I can make for you?"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Party Hat:Straw Hat:Cowboy Hat:Sombrero:Beanie")])? {
        1 => {
            if ((ctx.call(Function::CountItem, vec![Val::from(2236)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7151)])?.number()? > 99)
                && ctx.call(Function::CountItem, vec![Val::from(7111)])?.number()? > 99)
            {
                ctx.lines_as("Mad Hatter", args!["Party Hat! It's a Santa Hat I've remade by adding colorful paper for a festive look. Celebrate good times with a Party Hat! Come on!"])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Oh wait! Please check one thing before we start."])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["If the Santa Hat you use to create this hat was upgraded or compounded with a card, all those upgrades will be lost. So, to be safe, I suggest you put everything in your inventory, other than the things to make the Party Hat, into Kafra Storage."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["It seems to me that you have everything. Alright! Shall I make you a Party Hat?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No thanks.:Yes.")])?) == 1 {
                    ctx.lines_as("Mad Hatter", args!["Umm, okay...", "I guess being the life of the party is a bit too much for you, eh? Hmpf. Well, I know a good library in Juno where you can hang out..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FF*Rustle Rustle Splat Splat*^000000",
                    "^3355FF*Squick Squick Grind Grind*^000000",
                    "^3355FF*Swish Swish Rustle Rustle*^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2236), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7151), Val::from(100)])?;
                ctx.call(Function::DelItem, vec![Val::from(7111), Val::from(100)])?;
                ctx.lines_as("Mad Hatter", args!["It's done! Your new life, as the life of the party, has officially begun. All you need to do is wear this Party Hat. Though, knowing a couple jokes would probably help too."])?;
                ctx.call(Function::GetItem, vec![Val::from(5060), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "Oh by the way, there's some stuff left over. I guess you should hold onto it. You know, in case you need it later."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(914), Val::from(10)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Oh... man. Everyone needs a Party Hat! Even boring types get jazzed with pizazz wearing one of these!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "You can wear them for birthday parties, Christmas parties. Um, slumber parties. P-political parties.",
                        "......",
                        "And hunting parties?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "Anyway, this is what I need to make a Party Hat...",
                        "1 ^3355FFSanta Hat^000000,",
                        "100 ^3355FFOil Paper^000000 and",
                        "100 ^3355FFSlick Paper^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["I'll be right here when you decide to crank your life up a notch with a Party Hat."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            if ((ctx.call(Function::CountItem, vec![Val::from(2280)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7197)])?.number()? > 299)
                && ctx.call(Function::CountItem, vec![Val::from(7150)])?.number()? > 299)
            {
                ctx.lines_as("Mad Hatter", args!["A Straw Hat! Made out of a Sakkat with bamboo and tough vines, it gives you a relaxed, rustic feeling when you wear it."])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Oh wait! Please check one thing before we start.", "If the Sakkat you use to create this hat was upgraded, any cards, slots or upgrades will disappear once I make a new hat out of it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["To be safe, I suggest you put everything else, other than the required hat items, into Kafra Storage."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Ah! I see you brought everything. Alright! Do you want me to make you a Straw Hat now?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No thanks.:Yes.")])?) == 1 {
                    ctx.lines_as(
                        "Mad Hatter",
                        args![
                            "Man...",
                            "I think you need this hat more than I thought. You have got to learn to just chill and relax~!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FF*Crunch Chew Chew Chew*^000000",
                    "^3355FF*Squish Squish Fwsht Fwsht*^000000",
                    "^3355FF*Crunch Crunch Tap Tap*^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2280), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7197), Val::from(300)])?;
                ctx.call(Function::DelItem, vec![Val::from(7150), Val::from(300)])?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "It's done!",
                        "Now you can wear this hat, feel cool, and relax with style.",
                        "Please come again~"
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(5062), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Mad Hatter", args!["A Straw Hat! It shades you from the heat of the sun and keeps farmers cool when they're working on the fields. It's a hat for cool, relaxed types and the hat of choice for some famous heroes!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "Anyways, I need all these to make you a Straw Hat...",
                        "1 ^3355FFSakkat^000000,",
                        "300 ^3355FFTough Vines^000000 and",
                        "300 ^3355FFPiece of Bamboo^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["I'll be right here when you decide that you want the perfect hat for laid back, kick back moods. Be cool~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        3 => {
            if (((ctx.call(Function::CountItem, vec![Val::from(2248)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7030)])?.number()? > 107)
                && ctx.call(Function::CountItem, vec![Val::from(7194)])?.number()? > 107)
                && ctx.call(Function::CountItem, vec![Val::from(7120)])?.number()? > 3)
            {
                ctx.lines_as(
                    "Mad Hatter",
                    args!["I take a Western Grace, add some manliness, um, do some finangling, and Voila~! A Cowboy Hat!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["It's a hat for adventure and exploration. Although it's a manly hat like no other, with a manly factor of 9, it also looks great on girls."])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Oh wait! Please check one thing before we start.", "If the Western Grace you use to create this hat was upgraded, any cards or upgrades will be lost once I make a new hat out of it."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["To be safe, I suggest you put everything, other than the required materials, into Kafra Storage."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Ah! You brought everything already. Excellent. Do you want me to make a Cowboy Hat now?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No thanks.:Yes.")])?) == 1 {
                    ctx.lines_as(
                        "Mad Hatter",
                        args![
                            "Well, I see.",
                            "I guess you just don't appreciate the value of cowboy heroes, do you?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FF*Clik-clak Clik-clak Pam Pam Pam Pam*^000000",
                    "^3355FF*Bang Bang Ping Ping*^000000",
                    "^3355FF*Uk Uk Uk Uk Uhoh Uhoh*^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2248), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7030), Val::from(108)])?;
                ctx.call(Function::DelItem, vec![Val::from(7194), Val::from(108)])?;
                ctx.call(Function::DelItem, vec![Val::from(7120), Val::from(4)])?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Yeeeeehaw! With this hat on, you can tame broncos, rob stagecoaches, and look really stylish in duels!"],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(5075), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Mad Hatter",
                    args!["A Cowboy Hat! This is a very popular that brings up images of frontier life and gunplay."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Wearing a Cowboy Hat will make guys look tougher and more heroic, and will add untamed cuteness to the ladies."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "I need these items to make a Cowboy Hat...",
                        "1 ^3355FFWestern Grace^000000,",
                        "108 ^3355FFClaw of Desert Wolf^000000,",
                        "108 ^3355FFSoft Blade of Grass^000000 and",
                        "4 ^3355FFBurning Horseshoe^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["So when you decide to give your life a boost with rustic style, come back to me with those items so I can make you a Cowboy Hat."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        4 => {
            if ((ctx.call(Function::CountItem, vec![Val::from(5062)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(952)])?.number()? > 49)
                && ctx.call(Function::CountItem, vec![Val::from(1907)])?.number()? > 0)
            {
                ctx.lines_as("Mad Hatter", args!["Ah yes. Now here's a classy hat for mariachi heroes and Latin lovers. You can't deny the sex appeal that just pours from someone wearing a Sombrero."])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Oh wait! Please check one thing before we start.", "If the Straw Hat you use to creating this hat was upgraded, any cards or upgrades in this hat will be lost once I make the Sombrero."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["To be safe, I suggest you put everything you don't need to make this hat into Kafra Storage."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Ah! You brought everything already. Alright! Do you want me to make you a Sombrero right away?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No thanks.:Yes.")])?) == 1 {
                    ctx.lines_as(
                        "Mad Hatter",
                        args![
                            "Fine, fine.",
                            "But think of all the sex appeal you're missing out on. Come on, you know you need a Sombrero."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FFZinga Zinga Zang Zang^000000",
                    "^3355FFDing Ding Ding Dan Dan Dan^000000",
                    "^3355FFAdios Amigo^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(5062), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(952), Val::from(50)])?;
                ctx.call(Function::DelItem, vec![Val::from(1907), Val::from(1)])?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["With this Sombrero, you'll look like you have all kinds of style. Adios, cool guy~"],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(5067), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Mad Hatter", args!["Now, everyone needs a Sombrero. They're one of the most versatile hats in existence. You can wear them to formal events, you can wear them while napping, you can wear them just to hang out..."])?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Look, you don't need me to explain how cool a Sombrero is. It's just one of those things that everyone naturally knows. So I know you want one."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "So this is what I need to make a Sombrero...",
                        "1 ^3355FFStraw Hat^000000,",
                        "50 ^3355FFCactus Needle^000000 and",
                        "1 ^3355FFGuitar^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["That's all you need to have the coolest hat ever.", "(Woohoo~!)"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        5 => {
            if (ctx.call(Function::CountItem, vec![Val::from(2226)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7038)])?.number()? > 499)
            {
                ctx.lines_as("Mad Hatter", args!["A Beanie...!", "Whaddup, broham. This hat keeps your head toasty, but let's you play things cool with your homeys. And, um, fly girls."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "Oh wait! Please check one thing before we start.",
                        "If the Cap you use to make this Beanie was upgraded, any cards or upgrades will be lost once I make this hat!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["To be safe, I suggest you put everything not needed to make this hat into Kafra Storage."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["Ah! You brought everything already. Alright! How about I make you that Beanie right now?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No thanks.:Yes.")])?) == 1 {
                    ctx.lines_as(
                        "Mad Hatter",
                        args!["Okay...", "Dork. Come back When you decide to join the cool guys, word?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "^3355FF*Crunch Chew Chew Chew*^000000",
                    "^3355FF*Squish Squish Fwsht Fwsht*^000000",
                    "^3355FF*Crunch Crunch Tap Tap*^000000"
                ])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2226), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7038), Val::from(500)])?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["There you go! Your very own Beanie! Cute, but still 'dope!'"],
                )?;
                ctx.next()?;
                ctx.lines_as("Mad Hatter", args!["Uh! Uh!", "I'm in da house!"])?;
                ctx.call(Function::GetItem, vec![Val::from(5076), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Mad Hatter", args!["A Beanie!", "Whether you plan to go to a house party, get your groove back, or be as fresh as a prince, you'll be chillin' with this fly style."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "But well...",
                        "Although it can be a street fashion, Beanies also look really cute on some people."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args![
                        "Anyway, this is what I need to make a Beanie...",
                        "1 ^3355FFCap^000000 and",
                        "500 ^3355FFYarn^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mad Hatter",
                    args!["When you decide to represent yo' hood, come to me with the stuff. I'll hook you up."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn hat_merchant_zero(ctx: &Ctx) -> Script {
    hat_merchant_zero_body(ctx, Vec::new()).map(|_| ())
}

fn nehris_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Nehris",
        args!["The Monster Museum has prepared a special event for your support~!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Nehris",
        args!["Accessories typically worn by monsters are now available for adventurers to wear!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Nehris", args!["Of course, since we're on a trial period, we can only provide 3 monster accessories to our fans. However, according to your response, we may add more monster accessories in the future."])?;
    ctx.next()?;
    ctx.lines_as(
        "Nehris",
        args![
            "Simply bring the required materials, and pay a small fee, and we will provide you with the monsters accessory of your choice."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Nehris", args!["Please choose the", "accessory you are", "interested in."])?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("Decorative Golden Bell:Crown of Ancient Queen:Crown of Mistress")],
    )? {
        1 => {
            ctx.lines_as("Nehris", args!["This Golden Bell is fashioned after the one worn by the 'Sohee' monster. It's a cute accessory I personally recommend for us ladies. A little oversized, but I believe you'll like it a lot."])?;
            ctx.next()?;
            if (((ctx.call(Function::CountItem, vec![Val::from(10016)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 2)
                && ctx.var("Zeny").get()?.number()? > 19999)
            {
                ctx.lines_as(
                    "Nehris",
                    args!["Thank you for", "bringing everything.", "Please wait one moment."],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF* Clang Clang! Scrape Scrape! Jingle Jingle! *^000000")?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(10016), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(714), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(969), Val::from(3)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5091), Val::from(1)])?;
                ctx.lines_as(
                    "Nehris",
                    args![
                        "Yoohoo~",
                        "There you go. Here's your beautiful Golden Bell. I doubt it will jingle if you shake your head, though."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nehris",
                    args!["Oh, one more thing, don't confuse yourself with Sohee and commit suicide!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Nehris",
                    args!["If you'd like me to make another one, please come again. Thank you for visiting the Monster Museum."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Nehris",
                args![
                    "The required materials for the Decorative Golden Bell are...",
                    "^3355FF1 Golden Bell, 1 Emperium, 3 Gold^000000 and",
                    "20,000 zeny."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Nehris", args!["Just come ask me", "when you're ready.", "Thank you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Nehris",
                args!["This crown is inspired by the one worn by the 'Isis' monster. It has an elegant look fitting for ancient royalty."],
            )?;
            ctx.next()?;
            if (((ctx.call(Function::CountItem, vec![Val::from(10006)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 2)
                && ctx.var("Zeny").get()?.number()? > 19999)
            {
                ctx.lines_as(
                    "Nehris",
                    args!["Thank you for", "bringing everything.", "Please wait one moment."],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF* Clang Clang! Scrape Scrape! Jingle Jingle! *^000000")?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(10006), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(714), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(969), Val::from(3)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5080), Val::from(1)])?;
                ctx.lines_as("Nehris", args!["Tah dah!", "Here's your Crown of Ancient Queen! I don't know if you can get anyone to bow before you while wearing this, but at least you'll look elegant."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nehris",
                    args!["If you'd like me to make another one, please come again. Thank you for visiting the Monster Museum."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Nehris",
                args![
                    "The required materials for the Crown of Ancient Queen are...",
                    "^3355FF1 Queen's Hair Ornament, 1 Emperium, 3 Gold^000000 and",
                    "20,000 zeny."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Nehris", args!["Just come ask me", "when you're ready.", "Thank you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as("Nehris", args!["This crown is an imitation of the one worn by the 'Mistress' monster. Wearing this will give you a look of brusque authority."])?;
            ctx.next()?;
            if (((ctx.call(Function::CountItem, vec![Val::from(2249)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(714)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 2)
                && ctx.var("Zeny").get()?.number()? > 39999)
            {
                ctx.lines_as("Nehris", args!["Thank you for", "bringing everything."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nehris",
                    args![
                        "^FF0000By the way, if you use an upgraded",
                        "Coronet for making this item,",
                        "the item you create out of the Coronet",
                        "will not have its upgraded status.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Nehris", args!["^FF0000Also, please only have one Coronet in your inventory to make the Crown of Mistress. We are not responsible for any loss."])?;
                ctx.next()?;
                ctx.lines_as("Nehris", args!["Are you sure you want to create the Crown of Mistress?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Yes:Wait a minute...!")])?) == 1 {
                    ctx.lines_as("Nehris", args!["Please wait one moment."])?;
                    ctx.next()?;
                    ctx.mes("^3355FF* Clang Clang! Scrape Scrape! Jingle Jingle! *^000000")?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(2249), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(714), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(969), Val::from(3)])?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(40000))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(5081), Val::from(1)])?;
                    ctx.lines_as("Nehris", args!["Yay! You've got a great looking crown!"])?;
                    ctx.next()?;
                    ctx.lines_as("Nehris", args!["I don't know if everyone will obey your commands while wearing this, but at least you can feel beautifully regal."])?;
                    ctx.next()?;
                    ctx.lines_as("Nehris", args!["Why don't you go take a picture with Mistress? That would be funny to see if she wasn't too busy trying to maul you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nehris",
                        args!["If you'd like me to make you another one, please come again. Thank you for visiting the Monster Museum."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Nehris",
                    args!["No problem, just be sure to understand our precautions well, okay?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Nehris",
                args![
                    "The required materials for the Crown of Mistress are...",
                    "^3355FF1 Coronet, 1 Emperium, 3 Gold^000000 and",
                    "40,000 zeny."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Nehris", args!["Just come ask me", "when you're ready.", "Thank you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn nehris_1(ctx: &Ctx) -> Script {
    nehris_1_body(ctx, Vec::new()).map(|_| ())
}

fn muscle_man_alarm_mask_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines_as(
            "Muscle Man",
            args![
                "Hmmm...",
                "It's seems you're carrying too much stuff for me to give anything to you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muscle Man",
            args![
                "Talk to me again after you've freed up some of your inventory space by putting some of your things into the Kafra Storage."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Muscle Man",
        args![
            "Oooh yeah!",
            "Sometimes, even I can't believe how much this body of mine ripples with sexy love muscles~! I can barely hold them all in..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["It's...", "It's sexy time!"])?;
    ctx.next()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
    ctx.call(
        Function::SetNpcDisplay,
        vec![Val::from("Muscle Man#Alarm Mask"), Val::from(1193)],
    )?;
    ctx.call(Function::EnableNpc, vec![Val::from(" #Alarm Mask Man1")])?;
    ctx.call(Function::EnableNpc, vec![Val::from(" #Alarm Mask Man2")])?;
    ctx.call(Function::EnableNpc, vec![Val::from(" #Alarm Mask Man3")])?;
    ctx.call(Function::EnableNpc, vec![Val::from(" #Alarm Mask Man4")])?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man1")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man2")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man3")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man4")],
    )?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    if (ctx.call(Function::CountItem, vec![Val::from(1095)])?.number()? > 2999
        && ctx.call(Function::CountItem, vec![Val::from(2288)])?.number()? > 0)
    {
        if Val::from(runtime::select_values(ctx, &[Val::from("Give him items:Cancel")])?) == 1 {
            ctx.lines_as(
                "Muscle Man",
                args!["Ooh... Finally!", "You brought them!", "Excellent, excellent!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Muscle Man",
                args![
                    "Alright...",
                    "Now I can use this Mr.Scream and all these clock hands to continue my testing."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(1095), Val::from(3000)])?;
            ctx.call(Function::DelItem, vec![Val::from(2288), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(5086), Val::from(1)])?;
            ctx.lines_as("Muscle Man", args!["And, this is yours. All yours."])?;
            ctx.next()?;
            ctx.lines_as(
                "Muscle Man",
                args![
                    "Well then... Farewell!",
                    "Don't forget to bring me more Mr.Screams and clock hands if you find more of them!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Muscle Man",
            args![
                "Hmmm...?",
                "That's funny, I thought you would have that Mr. Scream mask and all those clock hands by now..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Muscle Man", args!["But...", "Not everyone is fortunate to have a magnificent, flawless physique. There are people in this world that have tragically lost appendages in accidents."])?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["When I realized this, I decided to help these people and began to research technology for artificial limbs. My studies have lead me to Alarms and their own mechanical limbs known as 'Ruimento.'"])?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args![
            "The Alarms, and their 'Ruimento,' are made from an ancient mysterious technology. You can only find these in the Clock Tower."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["So I am trying to research a method to reproduce Ruimetto, except in a smaller form for the human body. If I'm successful, it would be a medical miracle!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args!["However, I need help in completing my research. Hence, the reason I've told you my story, adventurer."],
    )?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["I desperately need materials to continuing testing so that I can develop a prototype. I'll need ^0000FFClock Hand^000000 from Alarms and ^0000FFMr. Scream^000000 in order to continue my research."])?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["I would collect these things myself but everytime I go up to the Clock Tower, I can't bear to break the Alarms, and just stare in awe at their mechanical, masculine physiques."])?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["So...", "If you help me out, I promise to give you, um..."])?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["...this Alarm Mask!", "....."])?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["I'd wear it myself, if it weren't for the fact that people never look at my face anyway. Everyone I meet seems to be fixated on my chiseled form. But trust me, this mask is pretty cool."])?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args!["Please adventurer...", "Help me give muscles to the world."],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Alright.:.........")])?) == 1 {
        ctx.lines_as("Muscle Man", args!["Oh~~! You know how I feel about Alarm's beautiful mechanic muscle! I am glad to have met someone who shares my good sense of taste."])?;
        ctx.next()?;
        ctx.lines_as(
            "Muscle Man",
            args![
                "Okay, here's the deal.",
                "Bring me...",
                "^0000FF3000 Clock Hand^000000 and",
                "^0000FF1 Mr. Scream^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Muscle Man", args!["With those, I will be able to continue my research, at least for a while. As I told you, I will give you this awesome ^0000FFAlarm Mask^000000 for those items!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Muscle Man",
            args!["Now! Go for it!", "Let's spread our love for muscle around the globe!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Muscle Man",
        args![
            "Oh~! I'm sorry!!",
            "I guess I must have pumped one of these glorious biceps on accident."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args!["I forget how breathtaking the sheer majesty of my body can be. It's alright, take your time, savor the visual pleasure..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args![
            "But listen, I'll need:",
            "^0000FF3000 Clock Hand^000000 and",
            "^0000FF1 Mr. Scream^000000",
            "in order to continue my research."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Muscle Man", args!["As I told you, I will give you this really nifty ^0000FFAlarm Mask^000000 for those items, as well as do any sort of pose for you."])?;
    ctx.next()?;
    ctx.lines_as(
        "Muscle Man",
        args!["Now! Go for it!", "Let's spread our love for muscle around the globe!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn muscle_man_alarm_mask(ctx: &Ctx) -> Script {
    muscle_man_alarm_mask_body(ctx, Vec::new()).map(|_| ())
}

fn muscle_man_alarm_mask_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
    ctx.call(
        Function::SetNpcDisplay,
        vec![Val::from("Muscle Man#Alarm Mask"), Val::from(748)],
    )?;
    ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man1")])?;
    ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man2")])?;
    ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man3")])?;
    ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man4")])?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man1")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man2")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man3")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_CLAYMORE")?, ctx.constant("AREA")?, Val::from(" #Alarm Mask Man4")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn muscle_man_alarm_mask_ontimer4000(ctx: &Ctx) -> Script {
    muscle_man_alarm_mask_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlarmMaskMan1Step {
    Start,
    OnInit,
}

fn alarm_mask_man1_run(ctx: &Ctx, mut step: AlarmMaskMan1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlarmMaskMan1Step::Start => {
                step = AlarmMaskMan1Step::OnInit;
                continue 'machine;
            }
            AlarmMaskMan1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alarm_mask_man1(ctx: &Ctx) -> Script {
    alarm_mask_man1_run(ctx, AlarmMaskMan1Step::Start, Vec::new()).map(|_| ())
}

pub fn alarm_mask_man1_oninit(ctx: &Ctx) -> Script {
    alarm_mask_man1_run(ctx, AlarmMaskMan1Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlarmMaskMan2Step {
    Start,
    OnInit,
}

fn alarm_mask_man2_run(ctx: &Ctx, mut step: AlarmMaskMan2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlarmMaskMan2Step::Start => {
                step = AlarmMaskMan2Step::OnInit;
                continue 'machine;
            }
            AlarmMaskMan2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alarm_mask_man2(ctx: &Ctx) -> Script {
    alarm_mask_man2_run(ctx, AlarmMaskMan2Step::Start, Vec::new()).map(|_| ())
}

pub fn alarm_mask_man2_oninit(ctx: &Ctx) -> Script {
    alarm_mask_man2_run(ctx, AlarmMaskMan2Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlarmMaskMan3Step {
    Start,
    OnInit,
}

fn alarm_mask_man3_run(ctx: &Ctx, mut step: AlarmMaskMan3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlarmMaskMan3Step::Start => {
                step = AlarmMaskMan3Step::OnInit;
                continue 'machine;
            }
            AlarmMaskMan3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alarm_mask_man3(ctx: &Ctx) -> Script {
    alarm_mask_man3_run(ctx, AlarmMaskMan3Step::Start, Vec::new()).map(|_| ())
}

pub fn alarm_mask_man3_oninit(ctx: &Ctx) -> Script {
    alarm_mask_man3_run(ctx, AlarmMaskMan3Step::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlarmMaskMan4Step {
    Start,
    OnInit,
}

fn alarm_mask_man4_run(ctx: &Ctx, mut step: AlarmMaskMan4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlarmMaskMan4Step::Start => {
                step = AlarmMaskMan4Step::OnInit;
                continue 'machine;
            }
            AlarmMaskMan4Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from(" #Alarm Mask Man4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alarm_mask_man4(ctx: &Ctx) -> Script {
    alarm_mask_man4_run(ctx, AlarmMaskMan4Step::Start, Vec::new()).map(|_| ())
}

pub fn alarm_mask_man4_oninit(ctx: &Ctx) -> Script {
    alarm_mask_man4_run(ctx, AlarmMaskMan4Step::OnInit, Vec::new()).map(|_| ())
}

fn educated_traveller_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 5000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're over weight -",
            "- to receive more items from this NPC. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(7030)])?.number()? > 0 {
        ctx.lines_as(
            "Lee Hester",
            args!["Hm...?", "What's that", "you have on you?", "Ah, a Claw of Desert Wolf~!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["You can only get one of those if you've been traveling for a long time in the desert. I guess someone like you also appreciates the beauty of my homeland."])?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["Ahahaha~", "If you've spent enough time in the Morocc desert, I'm sure you also appreciate the value of a high quality hat to beat the heat."])?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["Tell you what. I've learned a lot studying about animals, so if you give me the right materials, I'll make you a ^3355FFtruly great hat^000000."])?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["Just remember...", "If you give me any equipment compounded with cards or with upgrades, ^ff0000any upgraded abilities and cards in those items will be lost after I make something out of them.^000000 So, please keep that in mind."])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Drooping Cat:Smokie Leaf:Lazy Smokie:Blue Fish:That's... okay.")],
        )? {
            1 => {
                if ((ctx.call(Function::CountItem, vec![Val::from(2233)])?.number()? > 0
                    && ctx.call(Function::CountItem, vec![Val::from(983)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(7206)])?.number()? > 299)
                {
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "Ah...",
                            "You brought everything.",
                            "Excellent! Okay, give me some time to create the 'Drooping Cat.'"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("*Scratch scratch...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*", "*Scratch scratch...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFHmm...", "Those don't sound like hat-making noises...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["Oh, don't worry. It's almost done."])?;
                    ctx.next()?;
                    ctx.mes("*Swish swish...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*", "*Chikachikachoom*"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["There you go. *Phew~*", "Your own 'Drooping Cat' hat, powered by a scientific principle! Also, would you mind if I keep that Desert Wolf Claw? Heh heh, thanks~"])?;
                    ctx.call(Function::DelItem, vec![Val::from(2233), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(983), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7206), Val::from(300)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7030), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(5058), Val::from(1)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "Why don't I make you...",
                            "a 'Drooping Cat?'",
                            "Having a cat for a hat makes a certain sort of sense if you'll just hear me out..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["At the Juno Library, I read in this issue of 'Funtime Kid Science' that all cats, whether they're wild or domesticated, are hunters that have mastered the art of saving their energy."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["After investing many hours napping in the sun, they've developed the perfect position to nap in any place and in any condition: the lazy feline droop."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "It's impossible for humans to even imitate this special feline skill without the inspiration from this hat!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["And even if you don't manage to copy the napping cat droop, at least this'll keep the sun off the top of your head, if not out of your eyes."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "The items I need are...",
                            "1 ^ff0000slotted Circlet^000000,",
                            "1 ^ff0000Black Dyestuffs^000000",
                            "and 300 ^ff0000Black Cat Doll^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["I will make a Drooping Cat right away when you bring all of those items."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                if ctx.call(Function::CountItem, vec![Val::from(945)])?.number()? > 599 {
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "Ah...",
                            "You brought everything.",
                            "Excellent! Okay, give me some time to put this 'Smokie Leaf' together."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("*Scratch scratch...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*", "*Scratch scratch...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFHmm...", "Those don't sound like hat-making noises...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["Oh, don't worry. It's almost done."])?;
                    ctx.next()?;
                    ctx.mes(".........")?;
                    ctx.next()?;
                    ctx.mes("*Swish swish...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*", "*P-peeeeeeeeep*"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["There you go. *Phew~*", "Your own 'Smokie Leaf' hat, powered by a scientific principle! Also, would you mind if I keep that Desert Wolf Claw? Heh heh, thanks~"])?;
                    ctx.call(Function::DelItem, vec![Val::from(945), Val::from(600)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7030), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(5064), Val::from(1)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Lee Hester", args!["How about...", "a 'Smokie Leaf?'"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I know, I know, wearing a big ol' Leaf on top of your head sounds impractical. But, it makes a certain sort of sense if you'll just hear me out."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["Do you ever notice how on a hot day, you feel a lot cooler when you wear white, but you're a lot warmer when you wear black?"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["After some intense thinking, and reading some 'Funtime Kid Science' magazines in Juno, I realized that the color black absorbs sunlight and heat, and that the color white reflects it."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["So...", "What happens if you're wearing no colors? If my ^333333*ahem*^000000 hypothesis is correct, you wouldn't feel any heat if you were invisible!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["That's why I've created this Racoon Leaf, in an attempt to imitate Smokie's uncanny invisibility powers."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "It...",
                            "I haven't gotten it to work the way I want to yet. But it's a big leaf! It'll offer some shade."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "All I need to make this is...",
                            "600 ^ff0000Raccoon Leaf^000000.",
                            "It's weird, but I need a whole lot of little leaves to make a big one."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            3 => {
                if ((ctx.call(Function::CountItem, vec![Val::from(1026)])?.number()? > 999
                    && ctx.call(Function::CountItem, vec![Val::from(7065)])?.number()? > 99)
                    && ctx.call(Function::CountItem, vec![Val::from(945)])?.number()? > 9)
                {
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "Ah....",
                            "You brought everything.",
                            "Excellent! Okay, give me some time to create the 'Lazy Smokie' hat.",
                            "...."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("*Scratch scratch...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*", "*Scratch scratch...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFHmm...", "Those don't sound like hat-making noises...^000000"])?;
                    ctx.next()?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["Oh, don't worry. It's almost done."])?;
                    ctx.next()?;
                    ctx.mes(".........")?;
                    ctx.next()?;
                    ctx.mes("*Swish swish...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*", "*Glunk.*"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["There you go. *Phew~*", "Your own 'Lazy Smokie' hat, powered by a scientific principle! Also, would you mind if I keep that Desert Wolf Claw? Heh heh, thanks~"])?;
                    ctx.call(Function::DelItem, vec![Val::from(1026), Val::from(1000)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7065), Val::from(100)])?;
                    ctx.call(Function::DelItem, vec![Val::from(945), Val::from(10)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7030), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(5084), Val::from(1)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Lee Hester", args!["Why don't I make...", "a 'Lazy Smokie?'"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I know, I know, having a Racoon on your head sounds ridiculous, but it makes a certain sort of sense if you'll just hear me out."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["So I learned from an issue of 'Funtime Kid Science' at Juno Library that different colors absorb or reflect sunlight."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["Wearing one color makes you feel warmer, and another makes you feel cooler on a hot day."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["So I was thinking...", "What if I had no colors?"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["And, as we all know, you wear no colors if you're invisible, or when you're completely nude. I've already conducted my nudity experiment with sad and pitiable results..."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["So now I'm experimenting with invisibility. I've tried using a human sized Smokie Leaf to become invisible, but it didn't really work yet. So I got to thinking..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["If I somehow connected a Smokie to me, would I become invisible when he becomes invisible?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I haven't been able to try it yet, because Smokies are pretty heavy and I've got to build up my neck muscles before I can strap a Smokie to my head."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["So of course I need a practice Smokie to wear on my head before I'm ready for the real thing."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["And that was how I invented the 'Lazy Smokie' hat."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "The items I need are...",
                            "1000 ^ff0000Acorn^000000,",
                            "100 ^ff0000Sea-Otter Fur^000000 and",
                            "10 ^ff0000Raccoon Leaf^000000."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            4 => {
                if ((((ctx.call(Function::CountItem, vec![Val::from(624)])?.number()? > 0
                    && ctx.call(Function::CountItem, vec![Val::from(959)])?.number()? > 299)
                    && ctx.call(Function::CountItem, vec![Val::from(551)])?.number()? > 49)
                    && ctx.call(Function::CountItem, vec![Val::from(1023)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(938)])?.number()? > 99)
                {
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "Ah...",
                            "You brought everything.",
                            "Excellent! Okay, give me some time to create the 'Blue Fish' hat out of all this fish."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("*Scratch scratch...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Scratch scratch...*", "*Tap tap...*", "*Scratch scratch...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFHmm...", "Those don't sound like hat-making noises...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["Oh, don't worry. It's almost done."])?;
                    ctx.next()?;
                    ctx.mes("*Swish swish...*")?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*"])?;
                    ctx.next()?;
                    ctx.lines(args!["*Swish swish...*", "*Chomp chomp...*", "*Wataaaaa~!*"])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["There you go. *Phew~*", "Your own 'Blue Fish' hat, powered by a scientific principle! Also, would you mind if I keep that Desert Wolf Claw? Heh heh, thanks~"])?;
                    ctx.call(Function::DelItem, vec![Val::from(624), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(959), Val::from(300)])?;
                    ctx.call(Function::DelItem, vec![Val::from(551), Val::from(50)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1023), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(938), Val::from(100)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7030), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(5065), Val::from(1)])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Lee Hester",
                        args!["Ah! I got it!", "How about I make you...", "a 'Blue Fish' hat?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I know putting a Fish on top of your head doesn't sound like the best idea, but it makes a certain sort of sense if you'll just hear me out."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["So I was reading some issues of 'Funtime Kid Science' at the Juno Library, and read about animals that take naps under the sun when they feel really hot."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            " So then I thought...",
                            "Animals that don't take naps under the sun must never feel hot?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I decided to make a hat of that one perfect beast that was immune to the sun, never having to succumb its power."])?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["I set to work immediately, listing all the animals I know that don't take naps in the sun. Then, I removed any animals from the list that I didn't like, especially the ones with really long names."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["Finally, I had only three creatures on my list: the Emu, the Giraffe and the Fish."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Lee Hester", args!["First, I removed Emu, because everyone kept telling me I was spelling it wrong. And as everyone knows, a Giraffe hat would just look ridiculous. So, of course, I chose the Fish."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args!["And that, my friend, was how I invented the 'Blue Fish' hat."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lee Hester",
                        args![
                            "The items I need are...",
                            "^ff00001 Rotten Fish,",
                            "300 Stinky Scale,",
                            "50 Sushi,",
                            "1 Fish Tail^000000 and",
                            "^ff0000100 Sticky Mucus^000000."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            5 => {
                ctx.lines_as("Lee Hester", args!["Oh... I see.", "I'm a little disappointed, but you probably already have a nice hat to keep the sun out of your eyes. Well, if you change your mind, I'll be here."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Lee Hester", args!["Isn't it such", "a beautiful day", "today?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Lee Hester",
            args![
                "Beautiful...",
                "But insanely hot.",
                "I live here, I should know. Still, nothing can beat the Morocc sunset."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["It's amazing how so many different animals can live in the desert. I've been spending some time researching the ways of animals, reading books in the Juno Library..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Lee Hester",
            args!["Trying to figure out the Animal Kingdom's secrets of dealing with the heat."],
        )?;
        ctx.next()?;
        ctx.lines_as("Lee Hester", args!["I hope you're having a good time here in Morocc. If you ever question the special abilities of animals, just think of the majestic Desert Wolf that's able to thrive in the harsh Morocc deserts."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_WRAP")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn educated_traveller(ctx: &Ctx) -> Script {
    educated_traveller_body(ctx, Vec::new()).map(|_| ())
}

fn nine_tails_kitsune_mask_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn nine_tails_kitsune_mask(ctx: &Ctx) -> Script {
    nine_tails_kitsune_mask_body(ctx, Vec::new()).map(|_| ())
}

fn nine_tails_kitsune_mask_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Nine Tails#Kitsune Mask")])?;
    return Err(Stop::End);
}

pub fn nine_tails_kitsune_mask_oninit(ctx: &Ctx) -> Script {
    nine_tails_kitsune_mask_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn nine_tails_kitsune_mask_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    ctx.lines_as("Nine Tails", args!["Yelp! Yelp yelp!", "Yelp! Yelp yelp!"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Shake head:Nod head")])?) == 1 {
        ctx.lines_as("Nine Tails", args!["Yelp! Yelp yelp!"])?;
        ctx.close_window()?;
        ctx.call(
            Function::SetVariableOfNpc,
            vec![Val::from(".mymobs1"), Val::from("SpawnManager#Kitsune"), Val::from(0), Val::from(1)],
        )?;
        let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(25)])?;
        if subject1 == 1 {
            l_x = Val::from(118);
            l_y = Val::from(66);
        } else if subject1 == 2 {
            l_x = Val::from(103);
            l_y = Val::from(194);
        } else if subject1 == 3 {
            l_x = Val::from(116);
            l_y = Val::from(50);
        } else if subject1 == 4 {
            l_x = Val::from(49);
            l_y = Val::from(94);
        } else if subject1 == 5 {
            l_x = Val::from(232);
            l_y = Val::from(232);
        } else if subject1 == 6 {
            l_x = Val::from(245);
            l_y = Val::from(86);
        } else if subject1 == 7 {
            l_x = Val::from(217);
            l_y = Val::from(133);
        } else if subject1 == 8 {
            l_x = Val::from(50);
            l_y = Val::from(157);
        } else if subject1 == 9 {
            l_x = Val::from(245);
            l_y = Val::from(60);
        } else if subject1 == 10 {
            l_x = Val::from(220);
            l_y = Val::from(77);
        } else if subject1 == 11 {
            l_x = Val::from(198);
            l_y = Val::from(62);
        } else if subject1 == 12 {
            l_x = Val::from(158);
            l_y = Val::from(41);
        } else if subject1 == 13 {
            l_x = Val::from(57);
            l_y = Val::from(210);
        } else if subject1 == 14 {
            l_x = Val::from(251);
            l_y = Val::from(207);
        } else if subject1 == 15 {
            l_x = Val::from(86);
            l_y = Val::from(130);
        } else if subject1 == 16 {
            l_x = Val::from(216);
            l_y = Val::from(233);
        } else if subject1 == 17 {
            l_x = Val::from(192);
            l_y = Val::from(245);
        } else if subject1 == 18 {
            l_x = Val::from(117);
            l_y = Val::from(234);
        } else if subject1 == 19 {
            l_x = Val::from(144);
            l_y = Val::from(255);
        } else if subject1 == 20 {
            l_x = Val::from(190);
            l_y = Val::from(216);
        } else if subject1 == 21 {
            l_x = Val::from(63);
            l_y = Val::from(66);
        } else if subject1 == 22 {
            l_x = Val::from(247);
            l_y = Val::from(183);
        } else if subject1 == 23 {
            l_x = Val::from(37);
            l_y = Val::from(225);
        } else if subject1 == 24 {
            l_x = Val::from(247);
            l_y = Val::from(118);
        } else if subject1 == 25 {
            l_x = Val::from(154);
            l_y = Val::from(130);
        }
        ctx.call(
            Function::Monster,
            vec![
                Val::from("pay_dun03"),
                l_x.clone(),
                l_y.clone(),
                Val::from("Nine Tail"),
                Val::from(1180),
                Val::from(1),
                Val::from("SpawnManager#Kitsune::OnMyMobDead"),
            ],
        )?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Nine Tails#Kitsune Mask")])?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Nine Tails#Kitsune Mask")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Nine Tails#Kitsune Man")])?;
    return Err(Stop::End);
}

pub fn nine_tails_kitsune_mask_ontouch(ctx: &Ctx) -> Script {
    nine_tails_kitsune_mask_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn nine_tails_kitsune_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you are carrying -",
            "- too many items with you. -",
            "- Please come back after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(1022)])?.number()? > 998 {
        ctx.lines_as(
            "Nine Tails",
            args!["Oh! You finally brought what I've asked, human. Okay, now I can bring her back to life!"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Give items:Cancel")])?) == 1 {
            ctx.call(Function::DelItem, vec![Val::from(1022), Val::from(999)])?;
            ctx.call(Function::GetItem, vec![Val::from(5069), Val::from(1)])?;
            ctx.lines_as("Nine Tails", args!["Now...", "Take this", "and get out", "of my sight~!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.close_window()?;
    } else {
        ctx.lines_as("Nine Tails", args!["......Errr?", "Now why am I in human form again?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Nine Tails",
            args!["I guess... I need to practice transformation more often. *Sigh*"],
        )?;
        ctx.next()?;
        ctx.lines_as("Nine Tails", args!["Anyway, I've got a bone to pick with you humans. You're an aggressive and greedy race, always killing things and taking stuff. And despite all the hunting, you never seem to be satisfied."])?;
        ctx.next()?;
        ctx.lines_as("Nine Tails", args!["You always intrude our territory to slay us foxes for our nice, soft fur. But it's not only us you harass. The Sohees always complain that you barbaric humans keep stealing their clothing!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Nine Tails",
            args!["My wife was killed as a victim of human greed, probably skinned for her fabulous pelt..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Nine Tails", args!["All I want is to bring her back to life, no matter what the cost. That's why I'm swallowing my pride and asking you for help."])?;
        ctx.next()?;
        ctx.lines_as(
            "Nine Tails",
            args![
                "I need...",
                "^0000FF999^000000 ^FF0000Nine Tail^000000",
                "in order to cast the Fox Resurrection spell."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nine Tails",
            args![
                "I know how you humans think...",
                "You want some sort of reward for whatever mayhem you cause. Alright then..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nine Tails", args!["I will give you this ^0000FFKitsune Mask^000000 if you bring me what I asked. How does that sound? Remember, ^0000FF999 ^FF0000Nine Tail^000000."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(
        Function::SetVariableOfNpc,
        vec![Val::from(".mymobs2"), Val::from("SpawnManager#Kitsune"), Val::from(0), Val::from(1)],
    )?;
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(25)])?;
    if subject1 == 1 {
        l_x = Val::from(118);
        l_y = Val::from(66);
    } else if subject1 == 2 {
        l_x = Val::from(103);
        l_y = Val::from(194);
    } else if subject1 == 3 {
        l_x = Val::from(116);
        l_y = Val::from(50);
    } else if subject1 == 4 {
        l_x = Val::from(49);
        l_y = Val::from(94);
    } else if subject1 == 5 {
        l_x = Val::from(232);
        l_y = Val::from(232);
    } else if subject1 == 6 {
        l_x = Val::from(245);
        l_y = Val::from(86);
    } else if subject1 == 7 {
        l_x = Val::from(217);
        l_y = Val::from(133);
    } else if subject1 == 8 {
        l_x = Val::from(50);
        l_y = Val::from(157);
    } else if subject1 == 9 {
        l_x = Val::from(245);
        l_y = Val::from(60);
    } else if subject1 == 10 {
        l_x = Val::from(220);
        l_y = Val::from(77);
    } else if subject1 == 11 {
        l_x = Val::from(198);
        l_y = Val::from(62);
    } else if subject1 == 12 {
        l_x = Val::from(158);
        l_y = Val::from(41);
    } else if subject1 == 13 {
        l_x = Val::from(57);
        l_y = Val::from(210);
    } else if subject1 == 14 {
        l_x = Val::from(251);
        l_y = Val::from(207);
    } else if subject1 == 15 {
        l_x = Val::from(86);
        l_y = Val::from(130);
    } else if subject1 == 16 {
        l_x = Val::from(216);
        l_y = Val::from(233);
    } else if subject1 == 17 {
        l_x = Val::from(192);
        l_y = Val::from(245);
    } else if subject1 == 18 {
        l_x = Val::from(117);
        l_y = Val::from(234);
    } else if subject1 == 19 {
        l_x = Val::from(144);
        l_y = Val::from(255);
    } else if subject1 == 20 {
        l_x = Val::from(190);
        l_y = Val::from(216);
    } else if subject1 == 21 {
        l_x = Val::from(63);
        l_y = Val::from(66);
    } else if subject1 == 22 {
        l_x = Val::from(247);
        l_y = Val::from(183);
    } else if subject1 == 23 {
        l_x = Val::from(37);
        l_y = Val::from(225);
    } else if subject1 == 24 {
        l_x = Val::from(247);
        l_y = Val::from(118);
    } else if subject1 == 25 {
        l_x = Val::from(154);
        l_y = Val::from(130);
    }
    ctx.call(
        Function::Monster,
        vec![
            Val::from("pay_dun03"),
            l_x.clone(),
            l_y.clone(),
            Val::from("Nine Tail"),
            Val::from(1180),
            Val::from(1),
            Val::from("SpawnManager#Kitsune::OnMyMobDead2"),
        ],
    )?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BLASTMINEBOMB")?])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Nine Tails#Kitsune Man")])?;
    return Err(Stop::End);
}

pub fn nine_tails_kitsune_man(ctx: &Ctx) -> Script {
    nine_tails_kitsune_man_body(ctx, Vec::new()).map(|_| ())
}

fn nine_tails_kitsune_man_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::SetVariableOfNpc,
        vec![Val::from(".mymobs2"), Val::from("SpawnManager#Kitsune"), Val::from(0), Val::from(1)],
    )?;
    ctx.call(
        Function::Monster,
        vec![
            Val::from("pay_dun03"),
            Val::from(48),
            Val::from(83),
            Val::from("Nine Tail"),
            Val::from(1180),
            Val::from(1),
            Val::from("SpawnManager#Kitsune::OnMyMobDead2"),
        ],
    )?;
    ctx.call(Function::DisableNpc, vec![Val::from("Nine Tails#Kitsune Man")])?;
    return Err(Stop::End);
}

pub fn nine_tails_kitsune_man_oninit(ctx: &Ctx) -> Script {
    nine_tails_kitsune_man_oninit_body(ctx, Vec::new()).map(|_| ())
}

pub fn spawnmanager_kitsune(ctx: &Ctx) -> Script {
    spawnmanager_kitsune_run(ctx, SpawnmanagerKitsuneStep::Start, Vec::new()).map(|_| ())
}

pub fn spawnmanager_kitsune_oninit(ctx: &Ctx) -> Script {
    spawnmanager_kitsune_run(ctx, SpawnmanagerKitsuneStep::OnInit, Vec::new()).map(|_| ())
}

pub fn spawnmanager_kitsune_onmymobdead(ctx: &Ctx) -> Script {
    spawnmanager_kitsune_run(ctx, SpawnmanagerKitsuneStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn spawnmanager_kitsune_onmymobdead2(ctx: &Ctx) -> Script {
    spawnmanager_kitsune_run(ctx, SpawnmanagerKitsuneStep::OnMyMobDead2, Vec::new()).map(|_| ())
}
