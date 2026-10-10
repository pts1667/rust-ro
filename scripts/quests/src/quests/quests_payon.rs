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

pub fn granny(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![908, 1])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        return ctx.close();
    }
    if ctx.items().count(1049)? > 3 {
        ctx.lines_as(
            "Granny",
            args!["I wish I could make some clothing for my grandchildren for the festival season..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Granny",
            args!["Oh! Would you give me your ^3355FFSkirt of Virgin^000000? I'd need four of them. Please, I'm begging you."],
        )?;
        ctx.next()?;
        match ctx.menu(&["Alright.", "No way!"])? {
            0 => {
                ctx.items().take(1049, 4)?;
                ctx.lines_as(
                    "Granny",
                    args!["Oh! Thank you so much~ Now I can make some clothes for Kitty Cutty~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Granny", args!["Ah, Wait! I almost forgot. I was quite a popular actress back in my youth. My actor friends gave me a wedding present when I left the Troupe. I've kept it to remind me of those precious times."])?;
                ctx.next()?;
                ctx.lines_as("Granny", args!["If it's okay...", "I'd like to give you this."])?;
                ctx.next()?;
                ctx.items().give(2293, 1)?;
                ctx.lines_as(
                    "Granny",
                    args!["Goodbye, adventurer~!", "Thank you for making an old woman so happy..."],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as("Granny", args!["*Sigh* How can I get ^3355FFSkirt of Virgin^000000 by myself? I'm just so weak and feeble, even rolling in my rocking chair exhausts me. *Sniff* All I want is to make my grandchildren happy..."])?;
                return ctx.close();
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Granny", args!["I gather Mushrooms on the Mountain of Payon everyday. Time passes and before I know it, it's already fesitval season. I wish I could make clothing for my family this time of the year."])?;
        ctx.next()?;
        ctx.lines_as(
            "Granny",
            args!["...", "But for that, I need ^3355FF4 Skirt of Virgin^000000..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Granny", args!["Young traveler, please help me. When you find some ^3355FFSkirt of Virgin^000000, please bring them to me. I'd really appreciate it..."])?;
        return ctx.close();
    }
    Ok(())
}

pub fn mystic_lady(ctx: &Ctx) -> Script {
    ctx.lines_as("Mystic Lady", args!["My family has produced and sold the special Winter product ^3355FFEar Muffs^000000 for many years. We just moved here, but the weather is always warm so we can hardly make a living."])?;
    ctx.next()?;
    ctx.lines_as(
        "Mystic Lady",
        args!["If you plan to travel to colder regions, I suggest that you bring some ^3355FFEar Muffs^000000..."],
    )?;
    ctx.npc().emotion(constants::ET_THINK)?;
    ctx.next()?;
    ctx.lines_as("Mystic Lady", args!["Ear Muffs are my family's specialty, and we provide it to customers who have ^FF33551 Cursed Ruby^000000, ^3311AA1 Headset,^000000 ^3355FF 200 Feathers^000000 and ^DDDD005000 Zeny^000000."])?;
    ctx.npc().emotion(constants::ET_THINK)?;
    ctx.next()?;
    match ctx.menu(&["Oh Yeah? That sounds good.", "No thank you, Ma'am."])? {
        0 => {
            if ctx.items().count(724)? > 0 && ctx.items().count(949)? > 199 && ctx.items().count(5001)? > 0 && ctx.player().zeny()? > 4999 {
                ctx.mes("[Mystic Lady]")?;
                ctx.items().take(724, 1)?;
                ctx.items().take(949, 200)?;
                ctx.items().take(5001, 1)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                ctx.mes("Here, I will give you this pre-made one.")?;
                ctx.items().give(2283, 1)?;
                ctx.next()?;
                ctx.lines_as(
                    "Mystic Lady",
                    args!["Thank you for buying our product. You won't regret purchasing our Ear Muffs: Satisfaction guaranteed~"],
                )?;
                ctx.npc().emotion(constants::ET_THINK)?;
                return ctx.close();
            }
            ctx.lines_as("Mystic Lady", args!["Oh dear...", "You don't have enough money or items. Unfortunately, we can't give any discounts. Please understand that we have to make a living..."])?;
            ctx.npc().emotion(constants::ET_THINK)?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Mystic Lady",
                args!["That's fine.", "I believe we", "will meet again.", "...One of these days."],
            )?;
            ctx.npc().emotion(constants::ET_THINK)?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn boy(ctx: &Ctx) -> Script {
    if ctx.items().count(701)? > 4 {
        ctx.lines_as("Young Man", args!["ArrrGggghh!", "Ah, No I can't..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Young Man",
            args![
                "*Huk*",
                "Now even my eyes have gone mad!! Why do these bugs suddenly seem so cute?! Nooooooo~!!!"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Show Ora Ora", "Give Ora Ora", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Young Man",
                    args!["Oooooohhhh!!!", "Is, is this", "the one?!", "This is...", "Ora Ora!!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Young Man",
                    args!["I'll make it short! Give it to me, and I will give you my treasure! So how's my idea? Wanna deal?"],
                )?;
                return ctx.close();
            }
            1 => {
                if ctx.items().count(701)? > 4 {
                    ctx.items().take(701, 5)?;
                }
                ctx.lines_as(
                    "Young Man",
                    args![
                        "Muhahahahah!!!",
                        "I finally have this!",
                        "Ora Ora!",
                        "Stupid and Disgusting",
                        "Thief Bugs!!",
                        "You will pay!!!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Young Man",
                    args![
                        "Huk...Huk...",
                        "Sorry, I was out of control with pleasure for a while. Now, I will give my treasure as promised."
                    ],
                )?;
                ctx.next()?;
                ctx.items().give(5004, 1)?;
                ctx.lines_as(
                    "Young Man",
                    args!["When you wear this over your mouth and start to breathe, it filters junk out before getting to your lungs."],
                )?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["Kakakakakaka!", "I can't wait", "to use this", "Ora Ora!"])?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Young Man",
                    args![
                        "Huhuhuhuhu...",
                        "What if I can't get rid of all these Thief Bugs? I might even grow to love them... *Huk*"
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    } else {
        ctx.lines_as(
            "Young Man",
            args![
                "Wahhhhhh! I, I...",
                "I can't take it any more!",
                "You little stinky filthy bastards!"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Continue", "Cancel"])? {
            0 => {
                ctx.lines_as("Young Man", args!["Oh man...", "It all started when my parents passed away when I was a little kid. Early on I had to work for a living, and had a really hard time."])?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["I've worked for 10 years in hopes of buying my dream house where I could find some sense of peace and comfort again. Eventually I thought I had enough zeny to afford my very own sweet home."])?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["Unfortunately I didn't have enough money to buy the nice house that I had my eye on. But then, I found this house, which was bigger and cheaper than the first. I bought it without thinking..."])?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["Sweet Jesus! Turns out, it's a Heaven for Thief Bugs!!!"])?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["I tried to do everything I could do. I coaxed them, killed them, but it only brought peace for a moment! I even tried to burn this house down...!"])?;
                ctx.next()?;
                ctx.lines_as("Young Man", args!["*Gasp* I mean, really, do I need ^3355FF Ora Ora^000000 to get rid of them? People say it is very rarely seen in distant countries."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Young Man",
                    args!["If I could afford 5 of them, I won't have to be frustrated with these disgusting bugs any more..."],
                )?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Young Man",
                    args!["This never ending fight with the bugs has really exhausted me. I give up. Do as you wish you scumbags!"],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn young_man_12(ctx: &Ctx) -> Script {
    ctx.lines_as("Young man", args!["...What is it?"])?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Can you make me a special item?", "Hey."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Young man", args!["Huh? So you already know what I specialize in, eh? I suppose there's no need for the usual secrecy. Tell me what you want."])?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Helm of Angel.", "Deviruchi Cap.", "I will come next time."])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Young man",
                        args!["1 Helm (with slot).", "1 Angel Wing.", "5 Fang of Garm.", "That's all I require."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["Wait! Just so you know, I don't care if the items in your inventory have been upgraded, or have cards attached."])?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["I mean, any cards or upgrades in the items I will use to make something for you will be lost once I make the item. So be careful when you hand stuff over to me."])?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["Did you gather the items? Then hand them over."])?;
                    ctx.next()?;
                    match ctx.menu(&["Give him the items.", "Don't give him the items."])? {
                        0 => {
                            if ctx.items().count(2229)? > 0 && ctx.items().count(2254)? > 0 && ctx.items().count(7036)? > 4 {
                                ctx.lines_as("Young man", args![".....Hm.", "Fine, here's your Helm of Angel. Take it."])?;
                                ctx.items().take(2229, 1)?;
                                ctx.items().take(2254, 1)?;
                                ctx.items().take(7036, 5)?;
                                ctx.close_window()?;
                                ctx.items().give(5025, 1)?;
                                return ctx.end();
                            }
                            ctx.lines_as(
                                "Young man",
                                args!["...The number of items is not right. You better double check."],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Young man", args!["Do as you wish..."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Young man",
                        args!["600 Little Evil Horn.", "40 Talon of Griffon.", "That's all I require."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["Wait! Just so you know, I don't care if the items in your inventory have been upgraded, or have cards attached."])?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["I mean, any cards or upgrades in the items I will use to make something for you will be lost once I make the item. So be careful when you hand stuff over to me."])?;
                    ctx.next()?;
                    ctx.lines_as("Young man", args!["Did you gather the items? Then hand them over."])?;
                    ctx.next()?;
                    match ctx.menu(&["Give him the items.", "Don't give him the items."])? {
                        0 => {
                            if ctx.items().count(1038)? > 599 && ctx.items().count(7048)? > 39 {
                                ctx.lines_as("Young man", args![".....Hm. Here's your Deviruchi Hat. Please take it."])?;
                                ctx.items().take(1038, 600)?;
                                ctx.items().take(7048, 40)?;
                                ctx.close_window()?;
                                ctx.items().give(5038, 1)?;
                                return ctx.end();
                            }
                            ctx.lines_as(
                                "Young man",
                                args!["...The number of items isn't right. You better double check."],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Young man", args!["Do as you wish..."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Young man", args!["Hm. Alright.", "Then we shall meet again."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Young man", args!["Hm.", "Take care."])?;
            return ctx.close();
        }
    }
    Ok(())
}
