use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn defaria_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(714), Val::from(1)])? == 0 {
        ctx.mes("- You cannot proceed with the quest when you're carrying too many items with you. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_morocc2").get()?.number()? < 14 {
        ctx.lines_as(
            "Defaria",
            args![
                "Please step aside.",
                "I'm experiencing a turning point in my life right now.",
                "....."
            ],
        )?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
        ctx.next()?;
        ctx.mes("- The fire sparked, but then it immediatly blew out. -")?;
        ctx.call(
            Function::NpcSpecialEffect,
            vec![ctx.constant("EF_FIREARROW")?, ctx.constant("AREA")?, Val::from("Wet Firewood#moc2")],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Defaria",
            args![
                "...............",
                "You know, that was my ninety-third attempt to make a fire with wet firewood."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Defaria",
            args!["Wait, was that the ninty fourth ot ninety fifth?", "..I've.. I've lost count."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("mao_morocc2").get()? == 14 {
            ctx.lines_as(
                "Defaria",
                args![
                    "Please step aside.",
                    "I'm experiencing a turning point in my life right now.",
                    "......"
                ],
            )?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
            ctx.next()?;
            ctx.mes("- The fire sparked, but then it immediately blew out.-")?;
            ctx.call(
                Function::NpcSpecialEffect,
                vec![ctx.constant("EF_FIREARROW")?, ctx.constant("AREA")?, Val::from("Wet Firewood#moc2")],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Defaria",
                args![
                    "...............",
                    "Are you also here...............",
                    "to make fun of my stupid attempt to roast these sweet potatoes with wet firewood?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("...No.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Defaria",
                args!["...No? Then what brings you here?", "...Huh?", "A spell scroll? May I see?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Defaria",
                args![
                    "...Isn't... Isn't this?!",
                    "...Hmm...",
                    "Eek... So Echinacea referred you to me, huh?",
                    "...Does that means she knows everything...?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Defaria",
                args![
                    "...Alright then, I'll tell you.",
                    "As you can see, this scroll is sealed.",
                    "It can only be unsealed by a secret method.",
                    "...And..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Defaria",
                args![
                    "...It is from....",
                    "...Arunafeltz?",
                    "The way it's sealed is the same method used by the temple..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Defaria",
                args![
                    "I don't know how you've gotten this scroll,",
                    "...but I'll try to help you on behalf of the respectable scholars of Arunafeltz."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Defaria", args!["But remember; I'm not doing this officially. Please don't make me disgrace my country by helping you. Do you understand what I'm saying?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Got it.:No.")])? {
                1 => {
                    ctx.lines_as(
                        "Defaria",
                        args![
                            "Good. Then I expect you to help me by gathering the items I need to unseal the scroll:",
                            "^4d4dff30 Holy Waters, Runes of Darkness, and Bloody Runes^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Defaria", args!["I'll be waiting for your return."])?;
                    ctx.var("mao_morocc2").set(Val::from(15))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7023), Val::from(7024)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Defaria", args!["At first, you looked like someone who is reliable and understands the difference between official and private matters.", "Your answer just proved me wrong.", "Echinacea must be getting desperate if she's asking people like you for help."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("mao_morocc2").get()? == 15 {
                if (((ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7511)])?.number()? > 29)
                    && ctx.call(Function::CountItem, vec![Val::from(7563)])?.number()? > 29)
                    && ctx.call(Function::CountItem, vec![Val::from(6028)])?.number()? > 0)
                {
                    ctx.lines_as(
                        "Defaria",
                        args![
                            "Oh, you've brought everything!",
                            "Good, it wasn't hard, was it?",
                            "Okay, can you come back in a little bit?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(523), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7511), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7563), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(6028), Val::from(1)])?;
                    ctx.var("mao_morocc2").set(Val::from(16))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(7024), Val::from(7025)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Defaria",
                        args![
                            "To unseal the scroll,",
                            "I need ^4d4dff30 Holy Waters, Runes of the Darkness, and Bloody Runes^000000.",
                            "Don't forget to bring the spell scroll along with them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Defaria",
                        args![
                            "While you're gathering the items, I'll be roasting my sweet potatoes.",
                            "If I could only make this fire work..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("mao_morocc2").get()? == 16 {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                        ctx.lines_as(
                            "Defaria",
                            args![
                                "Welcome back; you came at the perfect time.",
                                "First, this spell scroll contains a teleportation spell.",
                                "You know what teleportation is, don't you?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Defaria",
                            args![
                                "You know,",
                                "it's a spell that instantly moves you to a different location.",
                                "That's the spell contained in this scroll."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Defaria",
                            args![
                                "The only difference is...",
                                "This scroll will teleport you to a specific location.",
                                "Such scrolls are generally used by ^4d4dfforganizations to gather their members^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Defaria",
                            args![
                                "I don't know where this scroll will lead you..",
                                "You'll be the one using this scroll. Am I right?",
                                "Here, please take this."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "'To gather members...?'",
                                "...Right! I remember",
                                "Rayan mentioned a gathering somewhere!",
                                "So this scroll will take me to that meeting?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Defaria",
                            args![
                                "Um, do you mind? Would you stop talking to yourself and just leave already?",
                                "I'm sorry, but this old man needs some rest, alright?"
                            ],
                        )?;
                        ctx.var("mao_morocc2").set(Val::from(17))?;
                        ctx.call(Function::GetItem, vec![Val::from(14595), Val::from(1)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(7025), Val::from(7026)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Defaria",
                            args!["Don't be so impatient.", "You can come back later.", "It'll take some time."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_morocc2").get()? == 17 {
                        ctx.lines_as(
                            "Defaria",
                            args!["What are you so worried about?", "You'll see how it works as soon as you use it."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("mao_morocc2").get()? == 101 {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...."])?;
                            ctx.next()?;
                            ctx.lines_as("Defaria", args!["..Huh? What do you want?"])?;
                            ctx.next()?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            if l_input_s.clone() == "Dandelion" {
                                ctx.lines_as(
                                    "Defaria",
                                    args!["..I know they're involved in this case, but", "we shouldn't talk about them."],
                                )?;
                                ctx.next()?;
                                'b2: {
                                    let subject2 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Just tell me what you know.:Alright..:")],
                                    )?);
                                    let mut matched2 = false;
                                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Why should I go through all that trouble?",
                                                "You can go and ask Echinaea.",
                                                "This old man needs some rest, alright?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("I'll do anything for you.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Defaria", args!["Anything?", "Are you serious?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Haha, got you!:Yes!")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Defaria",
                                                    args!["What th--! How dare you fool an old man!", "You little punk!"],
                                                )?;
                                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Defaria",
                                                    args![
                                                        "I may be old,",
                                                        "but I'm a representative of great Arunafeltz's scholars!",
                                                        "It's a mistake to trifle with me like that!"
                                                    ],
                                                )?;
                                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                                                ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                                                ctx.next()?;
                                                ctx.lines_as("Defaria", args!["Did you think I'm stupid just because I was trying to make a fire with wet firewood?!", "..*Pant Pant*.. Phew, even yelling tires me out.."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Defaria",
                                                    args![
                                                        "Then...",
                                                        "...Umm... Can you bring me some Logs?",
                                                        "...And some sweet potatoes too, if you don't mind."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                let choice = runtime::select_values(ctx, &[Val::from("Sure thing. How many?")])?;
                                                ctx.var("@menu").set(choice)?;
                                                ctx.lines_as(
                                                    "Defaria",
                                                    args![
                                                        "...Let's see...",
                                                        "I need sweet potatoes for me, Mr. Kidd, and Echi... Umm...",
                                                        "...And then I need... Yes!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Defaria",
                                                    args![
                                                        "Please bring me ^4d4dff20 Sweet Potatoes and 30 Logs^000000.",
                                                        "I'll be waiting for your return."
                                                    ],
                                                )?;
                                                ctx.var("mao_morocc2").set(Val::from(102))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(7037), Val::from(7038)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "You seem to give up really easily.",
                                                "I don't blame you. Sometimes giving up is the best decision,",
                                                "not to mention that it takes a certain kind of courage."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Defaria", args!["For example, Tarous that know when to abandon their hopes of getting cheese, you know, the bait in traps, are the hardest ones to catch. You know?", "..Hahaha!"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            } else if l_input_s.clone() == "Arunafeltz" {
                                ctx.lines_as("Defaria", args!["...Arunafeltz...?", "Yes, I'm from Arunafeltz...", "So what?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Defaria",
                                    args!["Be more specific about your question,", "and stop being so ambiguous."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("mao_morocc2").get()? == 102 {
                                if (ctx.call(Function::CountItem, vec![Val::from(516)])?.number()? > 19
                                    && ctx.call(Function::CountItem, vec![Val::from(7201)])?.number()? > 29)
                                {
                                    ctx.lines_as(
                                        "Defaria",
                                        args![
                                            "Oh, you've really brought what I wanted!",
                                            "Please give them to me..",
                                            "No, wait, can you pile them on the firewood over there?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Defaria",
                                        args![
                                            "Make it a nice, neat pile, please.",
                                            "Oh, you forgot to put the sweet potatoes on top.",
                                            "Yes, good. That's perfect."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Defaria", args!["Hopefully I can make a fire with your nice logs...."])?;
                                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                                    ctx.next()?;
                                    ctx.call(
                                        Function::NpcSpecialEffect,
                                        vec![ctx.constant("EF_FIREWALL")?, ctx.constant("AREA")?, Val::from("Wet Firewood#moc2")],
                                    )?;
                                    ctx.call(
                                        Function::NpcSpecialEffect,
                                        vec![ctx.constant("EF_TORCH")?, ctx.constant("AREA")?, Val::from("Wet Firewood#moc2")],
                                    )?;
                                    ctx.lines_as(
                                        "Defaria",
                                        args![
                                            "Oh, this is as great as I'd hoped!",
                                            "I guess my magic hasn't gotten that rusty after all.."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(516), Val::from(20)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7201), Val::from(30)])?;
                                    ctx.var("mao_morocc2").set(Val::from(103))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Defaria",
                                        args![
                                            "If you want me to do you a favor, then",
                                            "you should bring me ^4d4dff20 Sweet Potatoes and 30 Logs^000000 in return."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("mao_morocc2").get()? == 103 {
                                    ctx.mes("- Defaria is blowing on a roast sweet potato, peeling off it's skin. -")?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Excuse me, sir...")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines_as("Defaria", args!["What..?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Didn't you forget something?",
                                            "... Dandelion? I wanted you to tell me what you knew.",
                                            "I was hoping you'd remember something about Dandelion."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Defaria",
                                        args![
                                            "So,",
                                            "do you want me to confirm",
                                            "if Dandelion is really supported by Arunafeltz?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Defaria",
                                        args!["Umm... I don't know what to say... I don't feel comfortable talking about politics."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Defaria", args!["...Do you mind delivering these to Mr. Kidd?"])?;
                                    ctx.next()?;
                                    ctx.mes("- Defaria has given you 2 Nice Sweet Potatoes. -")?;
                                    ctx.var("mao_morocc2").set(Val::from(104))?;
                                    ctx.call(Function::GetItem, vec![Val::from(549), Val::from(2)])?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(7038), Val::from(7039)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("mao_morocc2").get()? == 104 {
                                        ctx.lines_as(
                                            "Defaria",
                                            args!["Um, I just told you to deliver those sweet potatoes to Mr. Kidd."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 105 {
                                        ctx.lines_as("Defaria", args!["Say, did he like them?", "Hahaha...", "... ... "])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Dandelion was like a ghost.",
                                                "There are many different types of people in this world.",
                                                "Some people want war while others are against it."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Defaria", args!["The Dandelions were the former case.", "Those right-ists believed in a distorted patriotism. They thought shedding blood was the only way to make peace."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "They organized Dandelion,",
                                                "and they happened to ally themselves with the warmongers in the government."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "That's how they grew so strong.",
                                                "At first, their purpose was limited to gathering intel and spying on other countries.",
                                                "This is just what I think, but the right information can cause chaos in a country."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("Then what..?")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Can't you see it by now? Isn't it obvious?",
                                                "Well, if you can't take care of your own problems,",
                                                "then you won't be able to handle your enemies."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("So Arunafeltz planned all this!")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "No, it isn't that simple. The world isn't just black and white, you know?",
                                                "Just bring these to Echinacea, will you?",
                                                "They're cooked very well."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("- You have received 2 steaming hot Nice Sweet Potatoes. -")?;
                                        ctx.var("mao_morocc2").set(Val::from(106))?;
                                        ctx.call(Function::GetItem, vec![Val::from(549), Val::from(2)])?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(7039), Val::from(7040)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 106 {
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "I told you to deliver those cooked sweet potatoes to Echinacea.",
                                                "Don't eat them, alright?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()? == 107 {
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Muhahahaha!",
                                                "Did Echinacea really say that?",
                                                "She's an executive of the Rune-Midgarts army.",
                                                "I can tell that what she said came from the bottom of her heart."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "I'm relieved that things turned out this way.",
                                                "After all, they're the reason why the three countries decided to cooperate.",
                                                "Of course, it's too early to say if the cooperation will last long."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Defaria", args!["I just hope that the three countries will use this chance to form lasting, friendly relationships. Wouldn't that be great for the sake of everyone in the world?", "Hrrm, but I'm not holding my breath for world peace."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args!["I'm just a mere human who can't foresee the future, you know?", "..Hahaha.."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Ah, your sweet potatoes are now roasted very well.",
                                                "Here, enjoy them.",
                                                "Thank you for trying so hard to please this whimsical old man."
                                            ],
                                        )?;
                                        ctx.var("mao_morocc2").set(Val::from(108))?;
                                        ctx.call(Function::GetItem, vec![Val::from(633), Val::from(5)])?;
                                        ctx.call(Function::CompleteQuest, vec![Val::from(7041)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("mao_morocc2").get()?.number()? > 108 {
                                        ctx.lines(args![
                                            "- Defaria is lost in thought, standing with a poker in one hand",
                                            "and a sweet potato in the other. -"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines(args!["- Sometimes, he looks to the sky, sighs, and then says, 'I'm envious of young people these days ..'", "or something like 'I don't care anymore, no matter what happens.' -"])?;
                                        ctx.next()?;
                                        ctx.mes("- It sounds like he's having some pretty deep moments, so let's not disturb him. -")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Defaria",
                                            args![
                                                "Please leave me alone unless you have extremely important business with me.",
                                                "I have so many things that I want to do..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn defaria_moc2(ctx: &Ctx) -> Script {
    defaria_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn wet_firewood_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("mao_morocc2").get()?.number()? > 102 {
        ctx.lines(args![
            "You've found some half-burnt logs of high quality.",
            "It seems somebody already roasted sweet potatoes with these logs."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "You've found camp firewood that is too wet to make a fire.",
            "It seems somebody has been futilely attempting to make a fire with them."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn wet_firewood_moc2(ctx: &Ctx) -> Script {
    wet_firewood_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn sharp_looking_boy_dan_07_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Jack]")?;
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
        ctx.lines(args![
            "Almost half of this town has been destroyed,",
            "and yet some people still come to visit.",
            "I must say that it's amazing that our stronghold is still intact, and withstood all the devastation."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "As you see, half of this town is gone. I guess there's no real reason to control access to this tavern.",
            "Just feel free to go in."
        ])?;
        ctx.next()?;
        ctx.lines_as("Jack", args!["I'm impressed by your courage to step into this hell."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn sharp_looking_boy_dan_07(ctx: &Ctx) -> Script {
    sharp_looking_boy_dan_07_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob0101Step {
    Start,
    OnTouch,
}

fn que_job01_01_run(ctx: &Ctx, mut step: QueJob0101Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob0101Step::Start => {
                step = QueJob0101Step::OnTouch;
                continue 'machine;
            }
            QueJob0101Step::OnTouch => {
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(9), Val::from(94)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Jack", args!["Oh, not everyone can enter that place...!", "...", "Nevermind."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jack",
                        args!["Since everything has changed,", "just feel free to enter wherever you want!"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(9), Val::from(94)])?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn que_job01_01(ctx: &Ctx) -> Script {
    que_job01_01_run(ctx, QueJob0101Step::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_01_ontouch(ctx: &Ctx) -> Script {
    que_job01_01_run(ctx, QueJob0101Step::OnTouch, Vec::new()).map(|_| ())
}

fn bar_master_moc2_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
        ctx.lines_as("Karred", args!["Welcome.", "Care for a drink?"])?;
    } else {
        ctx.lines_as("Karred", args!["What would you like to drink?"])?;
    }
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Order a Drink:About the mission:Quit")],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Karred",
                args![
                    "We have Tropical Sograt",
                    "and Vermillion on the Beach.",
                    "Which one would you like to drink?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Tropical Sograt:Vermilion on the Beach:How about a free drink?")],
            )? {
                1 => {
                    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                        ctx.lines_as(
                            "Karred",
                            args![
                                "Are you sure you can even hold a glass of alcohol?",
                                "You're carrying too many things on you already."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                        if ctx.var("Zeny").get()?.number()? < 800 {
                            ctx.lines_as(
                                "Karred",
                                args![
                                    "I can give you a special discount, but",
                                    "you don't have enough money to even pay the special price.",
                                    "Why don't you check your money first?",
                                    "This drink is only 800 zeny for Assassins!"
                                ],
                            )?;
                        } else {
                            ctx.lines_as(
                                "Karred",
                                args![
                                    "I'll give you a special discount.",
                                    "Here, drink up.",
                                    "This drink is mainly made of fruit juice,",
                                    "but you still shouldn't drink too much."
                                ],
                            )?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(800))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(12112), Val::from(1)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("Zeny").get()?.number()? < 1000 {
                        ctx.lines_as(
                            "Karred",
                            args![
                                "Do you even have any money?",
                                "Why don't you check your money first?",
                                "It's 1000 zeny for one glass."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Karred",
                            args![
                                "There you go.",
                                "This drink is mainly made of fruit juice,",
                                "but you still shouldn't drink too much."
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                        ctx.call(Function::GetItem, vec![Val::from(12112), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                        ctx.lines_as(
                            "Karred",
                            args![
                                "Are you sure you can even hold a glass of alcohol?",
                                "You're carrying too many things on you already."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                        if ctx.var("Zeny").get()?.number()? < 800 {
                            ctx.lines_as(
                                "Karred",
                                args![
                                    "I can give you a special discount, but",
                                    "you don't have enough money to even pay the special price.",
                                    "Why don't you check your money first?",
                                    "This drink is only 800 zeny for Assassins!"
                                ],
                            )?;
                        } else {
                            ctx.lines_as("Karred", args!["I'll give you a special discount.", "Here, enjoy."])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(800))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(12113), Val::from(1)])?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("Zeny").get()?.number()? < 1000 {
                        ctx.lines_as(
                            "Karred",
                            args![
                                "Do you even have any money?",
                                "Why don't you check your money first?",
                                "It's 1000 zeny for one glass."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Karred", args!["There you go.", "Don't drink too much, alright?"])?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                        ctx.call(Function::GetItem, vec![Val::from(12113), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                3 => {
                    ctx.lines_as("Karred", args![".........", "Get out."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                ctx.lines_as("Karred", args!["How many I help you?"])?;
                ctx.next()?;
                if ctx.var("mao_morocc2").get()? == 4 {
                    ctx.lines_as(
                        "Karred",
                        args![
                            "...Rin? I see. So you're assisting Mr. Kidd.",
                            "Rin...she's in the room inside.",
                            "She was gravely injured not that long ago..."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Karred",
                        args![
                            "You can't expect your job to always go smoothly.",
                            "Of course there will be risks.",
                            "Why don't you go pay her a visit?"
                        ],
                    )?;
                    ctx.var("mao_morocc2").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("mao_morocc2").get()?.number()? > 4 {
                    ctx.lines_as(
                        "Karred",
                        args![
                            "Rin needs to rest for now. I hope you'll do what you can to help her out.",
                            "But please do take care of yourself as well."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karred",
                        args![
                            "You and Rin are like my children to me, and such injuries...",
                            "Sigh...it's torture for me to see."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Karred",
                        args!["I hope you'll come by more often. This place is open to you always, even if you don't have missions."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else if ctx.var("mao_morocc2").get()?.number()? > 4 {
                ctx.lines_as(
                    "Karred",
                    args![
                        "Rin needs to rest in bed for a while. I hope you'll help her complete her duties.",
                        "...This is quite embarrassing to admit, but I see her like my own daughter."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Litheron",
                    args![
                        "Ah, just forget what I said. That was out of line.",
                        "I guess I'm drunk after dealing with alcohol all day."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Karred",
                    args![
                        "How may I help you?",
                        "For your information, the 'recommendation' has lost its effect. You'd better go back."
                    ],
                )?;
                if ctx.var("mao_morocc2").get()? == 4 {
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("I'm here for Rin!")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Karred", args!["...? Why are you looking for Rin?"])?;
                    ctx.next()?;
                    ctx.lines(args!["......", "........."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karred",
                        args![
                            "I see. So you're a member of the Ash Vacuum expedition.",
                            "Rin is in the inner room.",
                            "...She's been badly injured and is resting in bed. Don't do anything to shock her, alright?"
                        ],
                    )?;
                    ctx.var("mao_morocc2").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Karred", args!["Hmmm."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn bar_master_moc2_01(ctx: &Ctx) -> Script {
    bar_master_moc2_01_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob0104Step {
    Start,
    OnTouch,
}

fn que_job01_04_run(ctx: &Ctx, mut step: QueJob0104Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob0104Step::Start => {
                step = QueJob0104Step::OnTouch;
                continue 'machine;
            }
            QueJob0104Step::OnTouch => {
                if ((ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("mao_request").get()?.number()? > 0)
                    || ctx.var("mao_morocc2").get()?.number()? > 4)
                {
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(61), Val::from(50)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Litheron",
                        args![
                            "Hey, wait.",
                            "Only authorized personnel can enter the inner room.",
                            "Hey Master, is it okay to let this guy in?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("prt_curse").get()? == 24 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I, um, I'm looking",
                                "for somebody named",
                                "Marjana? I learned",
                                "that she's around",
                                "here somewhere?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Litheron",
                            args!["Marjana? How did you", "know that? Hey master,", "what do I do with this guy?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Master",
                            args![
                                "Hmm. I sense no ill intent",
                                "from this adventurer. I've",
                                "also heard a rumor that the",
                                "Prontera Church needs to",
                                "investigate poison for",
                                "some reason."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Master",
                            args![
                                "However, there is no",
                                "way to tell if this person",
                                "has been sent by Prontera",
                                "Church. I suppose whether",
                                "this person can enter is",
                                "really up to you, Litheron."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Litheron",
                            args![
                                "Hah! Did you hear that?",
                                "Alright, how about this?",
                                "I'll let you in if you buy me",
                                "a drink. Besides, you can't risk making trouble here: this place",
                                "is full of deadly Assassins."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure:Why should I?!")])? {
                            1 => {
                                ctx.mes("[Litheron]")?;
                                if ctx.var("Zeny").get()?.number()? > 999 {
                                    ctx.lines(args![
                                        "Heh, that's what",
                                        "I'm talking about!",
                                        "Hey, bartender! Gimme",
                                        "the usual! I like your",
                                        "style, adventurer..."
                                    ])?;
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Litheron",
                                        args![
                                            "Alright, you can come",
                                            "on in. But don't you dare",
                                            "breathe a word about this",
                                            "bar to another living soul."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(61), Val::from(50)])?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines(args![
                                        "Huh...",
                                        "Oh, you don't even",
                                        "have enough zeny to",
                                        "buy water here. Oh boy...",
                                        "If you really want to enter,",
                                        "make sure you have the cash!"
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                                    return Err(Stop::End);
                                }
                            }
                            2 => {
                                ctx.lines_as(
                                    "Litheron",
                                    args![
                                        "Not the saavy type,",
                                        "are you...? Fine, fine.",
                                        "If you're not gonna do",
                                        "me any favors, then why",
                                        "should I help you? Go away!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Bar Master",
                            args![
                                "If you keep outsiders away from the entrance, every drink you drink today will be on the house.",
                                "So keep up the good work."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Bar Master",
                            args!["Yay! Did you hear that?", "That's my cue to get you outta here!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(85), Val::from(77)])?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn que_job01_04(ctx: &Ctx) -> Script {
    que_job01_04_run(ctx, QueJob0104Step::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_04_ontouch(ctx: &Ctx) -> Script {
    que_job01_04_run(ctx, QueJob0104Step::OnTouch, Vec::new()).map(|_| ())
}

fn idle_knight_dan_08_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Litheron",
        args!["What, haven't you seen a Knight before?", "You think Knights don't belong here?"],
    )?;
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
        ctx.mes("Looks like this is a case of the pot calling the kettle black.")?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Litheron",
        args![
            "You know, I'm just here to take a break from the town restoration work.",
            "There aren't many places that you can have a drink without being caught. You know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn idle_knight_dan_08(ctx: &Ctx) -> Script {
    idle_knight_dan_08_body(ctx, Vec::new()).map(|_| ())
}

fn tao_dan_09_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Tao]")?;
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
        ctx.lines(args!["Welcome, meow.", "Say, how's it like outside? Meow?"])?;
        ctx.next()?;
        if ctx.var("prt_curse").get()? == 24 {
            match runtime::select_values(ctx, &[Val::from("Is Marjana in?:I'm here to see you, Tao.")])? {
                1 => {
                    ctx.lines_as(
                        "Tao",
                        args![
                            "She's in the room on your left, meow~",
                            "But then, how d'you know she's here, meow?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Tao",
                        args![
                            "This is no place for playing, meow..",
                            "... Hey, it's awkward you looking at me like that, meow.."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("mao_morocc2").get()?.number()? > 4 {
            ctx.lines_as(
                "Tao",
                args![
                    "Tao knows what you're thinking, meow~ You're here to see big sister Rin, am I right, meow? ",
                    "Hehe, where is she, I wonder, meow~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Tao",
                args![
                    "If you don't have any reason to be here, you should leave, meow!",
                    "Not everyone can enter this place, meow.",
                    "Somehow you're here, but you can't go in the room."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tao",
                args![
                    "No, wait... I think you may be able to go in... I don't know, meow!",
                    "Tao is tired of being a door keeper, meow! Meow!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("Why are you here, meow?")?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Hey kid, are you alone?:What are you, meow?!:Why do you keep meowing?!")],
        )? {
            1 => {
                ctx.lines_as(
                    "Tao",
                    args!["Who are you calling a kid?", "Tao is a good door keeper!", "Don't call me a kid!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tao",
                    args![
                        "I can tell you've got some reason for being here, but you've put yourself in trouble!",
                        "You're mistaken if you thought I would tell you",
                        "Master and Marjana are in the left room. Grrr.. Meow!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Tao", args!["Are you making fun of me being a door keeper, meow?!", "Grrr..."])?;
                ctx.next()?;
                ctx.lines_as("Tao", args!["You make me mad, meow. Get lost!!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Tao",
                    args![
                        "It's a long story, meow.",
                        "It's been a long time since I started meowing one day..",
                        "That day, Tao wanted to have a Big Ribbon so badly."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tao",
                    args!["So Tao went to find Wild Roses.", "Meow?! Why am I telling you this, meow?!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Tao", args!["Grrr!", "What are you, meow?!"])?;
                ctx.next()?;
                ctx.lines_as("Tao", args!["Get out!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn tao_dan_09(ctx: &Ctx) -> Script {
    tao_dan_09_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob01Room1Step {
    Start,
    OnInit,
    OnReset,
    OnTouch,
}

fn que_job01_room_1_run(ctx: &Ctx, mut step: QueJob01Room1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob01Room1Step::Start => {
                step = QueJob01Room1Step::OnInit;
                continue 'machine;
            }
            QueJob01Room1Step::OnInit => {
                step = QueJob01Room1Step::OnReset;
                continue 'machine;
            }
            QueJob01Room1Step::OnReset => {
                ctx.var("$@moc_mao_room1").set(Val::from(0))?;
                return Err(Stop::End);
            }
            QueJob01Room1Step::OnTouch => {
                ctx.mes("[Tao]")?;
                if ctx.var("prt_curse").get()? == 24 {
                    if ctx.var("$@moc_mao_room1").get()? == 0 {
                        ctx.var("$@moc_mao_room1").set(Val::from(1))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from(" #room1timer::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnEnable")])?;
                        ctx.lines(args![
                            "Ah, that place is protected",
                            "by security magic, so you'll",
                            "only have ^4D4DFF4 minutes^000000 to remain",
                            "there. Don't waste time, meow!"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(11), Val::from(7)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "Sooo sorry, meow~",
                            "Someone else is already",
                            "inside. Just come back",
                            "again later, meow?"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_request").get()?.number()? > 0 {
                        if ctx.var("$@moc_mao_room1").get()? == 0 {
                            ctx.var("$@moc_mao_room1").set(Val::from(1))?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from(" #room1timer::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#moc_master_1::OnEnable")])?;
                            ctx.lines(args![
                                "Ah, that place is protected",
                                "by security magic, so you'll",
                                "only have ^4D4DFF4 minutes^000000 to remain",
                                "there. Don't waste time, meow!"
                            ])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(11), Val::from(7)])?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "Sooo sorry, meow~",
                                "Someone else is already",
                                "inside. Just come back",
                                "again later, meow?"
                            ])?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.lines(args![
                            "Who are you, meow?!",
                            "Only authorized personnel can enter the inner room, meow!",
                            "You are not authorized!",
                            "Get out!"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
}

pub fn que_job01_room_1(ctx: &Ctx) -> Script {
    que_job01_room_1_run(ctx, QueJob01Room1Step::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_room_1_oninit(ctx: &Ctx) -> Script {
    que_job01_room_1_run(ctx, QueJob01Room1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn que_job01_room_1_onreset(ctx: &Ctx) -> Script {
    que_job01_room_1_run(ctx, QueJob01Room1Step::OnReset, Vec::new()).map(|_| ())
}

pub fn que_job01_room_1_ontouch(ctx: &Ctx) -> Script {
    que_job01_room_1_run(ctx, QueJob01Room1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Room1timerStep {
    Start,
    OnEnable,
    OnStop,
    OnTimer240000,
    OnTimer245000,
    OnTimer250000,
}

fn room1timer_run(ctx: &Ctx, mut step: Room1timerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Room1timerStep::Start => {
                step = Room1timerStep::OnEnable;
                continue 'machine;
            }
            Room1timerStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("You will now enter the Master Zone, Area 1."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Room1timerStep::OnStop => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("The security magic in the Master Zone, Area 1 is deactivated."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room1_warp13::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#moc_master_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("que_job01#room_1::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Room1timerStep::OnTimer240000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room1_warp13::OnEnable")])?;
                return Err(Stop::End);
            }
            Room1timerStep::OnTimer245000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room1_warp13::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#moc_master_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnInit")])?;
                return Err(Stop::End);
            }
            Room1timerStep::OnTimer250000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("The security magic Master Zone, Area 1 is now activated."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("que_job01#room_1::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn room1timer(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::Start, Vec::new()).map(|_| ())
}

pub fn room1timer_onenable(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room1timer_onstop(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::OnStop, Vec::new()).map(|_| ())
}

pub fn room1timer_ontimer240000(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn room1timer_ontimer245000(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::OnTimer245000, Vec::new()).map(|_| ())
}

pub fn room1timer_ontimer250000(ctx: &Ctx) -> Script {
    room1timer_run(ctx, Room1timerStep::OnTimer250000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Room1Warp13Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn room1_warp13_run(ctx: &Ctx, mut step: Room1Warp13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Room1Warp13Step::Start => {
                step = Room1Warp13Step::OnInit;
                continue 'machine;
            }
            Room1Warp13Step::OnInit => {
                step = Room1Warp13Step::OnDisable;
                continue 'machine;
            }
            Room1Warp13Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("#room1_warp13")])?;
                return Err(Stop::End);
            }
            Room1Warp13Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#room1_warp13")])?;
                return Err(Stop::End);
            }
            Room1Warp13Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room1_warp13(ctx: &Ctx) -> Script {
    room1_warp13_run(ctx, Room1Warp13Step::Start, Vec::new()).map(|_| ())
}

pub fn room1_warp13_oninit(ctx: &Ctx) -> Script {
    room1_warp13_run(ctx, Room1Warp13Step::OnInit, Vec::new()).map(|_| ())
}

pub fn room1_warp13_ondisable(ctx: &Ctx) -> Script {
    room1_warp13_run(ctx, Room1Warp13Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn room1_warp13_onenable(ctx: &Ctx) -> Script {
    room1_warp13_run(ctx, Room1Warp13Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn room1_warp13_ontouch(ctx: &Ctx) -> Script {
    room1_warp13_run(ctx, Room1Warp13Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob01Room1OutStep {
    Start,
    OnTouch,
}

fn que_job01_room1_out_run(ctx: &Ctx, mut step: QueJob01Room1OutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob01Room1OutStep::Start => {
                step = QueJob01Room1OutStep::OnTouch;
                continue 'machine;
            }
            QueJob01Room1OutStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from(" #room1timer::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn que_job01_room1_out(ctx: &Ctx) -> Script {
    que_job01_room1_out_run(ctx, QueJob01Room1OutStep::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_room1_out_ontouch(ctx: &Ctx) -> Script {
    que_job01_room1_out_run(ctx, QueJob01Room1OutStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ValdesMocMaster1Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn valdes_moc_master_1_run(ctx: &Ctx, mut step: ValdesMocMaster1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ValdesMocMaster1Step::Start => {
                ctx.mes("[Valdes]")?;
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                    ctx.lines(args![
                        "What is it? I have nothing to ask you to do.",
                        "Could you please leave me alone? I have a bad headache."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args!["I'll ask you for a help some other time.", "You look pretty passionate after all."],
                    )?;
                    if ctx.call(Function::CountItem, vec![Val::from(6029)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(6029), ctx.call(Function::CountItem, vec![Val::from(6029)])?],
                        )?;
                    }
                    if ctx.call(Function::CountItem, vec![Val::from(7418)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(7418), ctx.call(Function::CountItem, vec![Val::from(7418)])?],
                        )?;
                    }
                    if ctx.call(Function::CountItem, vec![Val::from(7416)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(7416), ctx.call(Function::CountItem, vec![Val::from(7416)])?],
                        )?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "...Sorry, but could you please leave?",
                        "I'm really stressed out right now.",
                        "If you're here for the request that we made earlier, you can just forget about it."
                    ])?;
                    if ctx.call(Function::CountItem, vec![Val::from(6029)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(6029), ctx.call(Function::CountItem, vec![Val::from(6029)])?],
                        )?;
                    }
                    if ctx.call(Function::CountItem, vec![Val::from(7418)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(7418), ctx.call(Function::CountItem, vec![Val::from(7418)])?],
                        )?;
                    }
                    if ctx.call(Function::CountItem, vec![Val::from(7416)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(7416), ctx.call(Function::CountItem, vec![Val::from(7416)])?],
                        )?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ValdesMocMaster1Step::OnInit => {
                step = ValdesMocMaster1Step::OnDisable;
                continue 'machine;
            }
            ValdesMocMaster1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Valdes#moc_master_1")])?;
                return Err(Stop::End);
            }
            ValdesMocMaster1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Valdes#moc_master_1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn valdes_moc_master_1(ctx: &Ctx) -> Script {
    valdes_moc_master_1_run(ctx, ValdesMocMaster1Step::Start, Vec::new()).map(|_| ())
}

pub fn valdes_moc_master_1_oninit(ctx: &Ctx) -> Script {
    valdes_moc_master_1_run(ctx, ValdesMocMaster1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn valdes_moc_master_1_ondisable(ctx: &Ctx) -> Script {
    valdes_moc_master_1_run(ctx, ValdesMocMaster1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn valdes_moc_master_1_onenable(ctx: &Ctx) -> Script {
    valdes_moc_master_1_run(ctx, ValdesMocMaster1Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum QueJob01Room2Step {
    Start,
    OnInit,
    OnReset,
    OnTouch,
}

fn que_job01_room_2_run(ctx: &Ctx, mut step: QueJob01Room2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            QueJob01Room2Step::Start => {
                return Err(Stop::End);
            }
            QueJob01Room2Step::OnInit => {
                step = QueJob01Room2Step::OnReset;
                continue 'machine;
            }
            QueJob01Room2Step::OnReset => {
                ctx.var("$@moc_mao_room2").set(Val::from(0))?;
                return Err(Stop::End);
            }
            QueJob01Room2Step::OnTouch => {
                if (ctx.var("mao_morocc2").get()?.number()? > 4 && ctx.var("mao_morocc2").get()?.number()? < 10) {
                    if ctx.var("$@moc_mao_room2").get()? == 0 {
                        ctx.var("$@moc_mao_room2").set(Val::from(1))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from(" #room2timer::OnEnable")])?;
                        ctx.lines_as(
                            "Tao",
                            args![
                                "The room is also sealed with a security magic spell, meow.",
                                "You have ^4d4dff4 minutes^000000",
                                "to finish your quest, meow."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(80), Val::from(21)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Tao",
                            args!["We already have a guest inside.", "You can wait or come back later, meow."],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_morocc2").get()? == 21 {
                        if ctx.var("$@moc_mao_room2").get()? == 0 {
                            ctx.var("$@moc_mao_room2").set(Val::from(1))?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from(" #room2timer::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc_room2_2::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc_room2_2::OnEnable")])?;
                            ctx.lines_as(
                                "Tao",
                                args![
                                    "The room is also sealed with a security magic spell, meow.",
                                    "You have ^4d4dff4 minutes^000000",
                                    "to finish your quest, meow."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(144), Val::from(57)])?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Tao",
                                args!["We already have a guest inside.", "You can wait or come back later, meow."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                            return Err(Stop::End);
                        }
                    } else {
                        if (ctx.var("mao_morocc2").get()?.number()? > 21 && ctx.var("mao_morocc2").get()?.number()? < 29) {
                            if ctx.var("$@moc_mao_room2").get()? == 0 {
                                ctx.var("$@moc_mao_room2").set(Val::from(1))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from(" #room2timer::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc_room2_2::OnEnable")])?;
                                ctx.lines_as(
                                    "Tao",
                                    args![
                                        "The room is also sealed with a security magic spell, meow.",
                                        "You have ^4d4dff4 minutes^000000",
                                        "to finish your quest, meow."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(144), Val::from(57)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Tao",
                                    args!["We already have a guest inside.", "You can wait or come back later, meow."],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                                return Err(Stop::End);
                            }
                        } else {
                            ctx.lines_as(
                                "Tao",
                                args![
                                    "This is the patient's room, meow.",
                                    "Why don't you leave the patient alone to rest?"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
}

pub fn que_job01_room_2(ctx: &Ctx) -> Script {
    que_job01_room_2_run(ctx, QueJob01Room2Step::Start, Vec::new()).map(|_| ())
}

pub fn que_job01_room_2_oninit(ctx: &Ctx) -> Script {
    que_job01_room_2_run(ctx, QueJob01Room2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn que_job01_room_2_onreset(ctx: &Ctx) -> Script {
    que_job01_room_2_run(ctx, QueJob01Room2Step::OnReset, Vec::new()).map(|_| ())
}

pub fn que_job01_room_2_ontouch(ctx: &Ctx) -> Script {
    que_job01_room_2_run(ctx, QueJob01Room2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Room2timerStep {
    Start,
    OnEnable,
    OnStop,
    OnTimer240000,
    OnTimer245000,
    OnTimer250000,
}

fn room2timer_run(ctx: &Ctx, mut step: Room2timerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Room2timerStep::Start => {
                step = Room2timerStep::OnEnable;
                continue 'machine;
            }
            Room2timerStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("Master Zone 2 is now under surveillance."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Room2timerStep::OnStop => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("The magic shield of Master Zone 2 has been reset."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_1_warp::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_2_warp::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc_room2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc_room2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("que_job01#room_2::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Room2timerStep::OnTimer240000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_1_warp::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_2_warp::OnEnable")])?;
                return Err(Stop::End);
            }
            Room2timerStep::OnTimer245000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_1_warp::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#room2_2_warp::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rin#moc_room2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Rayan#moc_room2_2::OnDisable")])?;
                return Err(Stop::End);
            }
            Room2timerStep::OnTimer250000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("Master Zone 2 has been released from surveillance."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("que_job01#room_2::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn room2timer(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::Start, Vec::new()).map(|_| ())
}

pub fn room2timer_onenable(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room2timer_onstop(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::OnStop, Vec::new()).map(|_| ())
}

pub fn room2timer_ontimer240000(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn room2timer_ontimer245000(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::OnTimer245000, Vec::new()).map(|_| ())
}

pub fn room2timer_ontimer250000(ctx: &Ctx) -> Script {
    room2timer_run(ctx, Room2timerStep::OnTimer250000, Vec::new()).map(|_| ())
}
