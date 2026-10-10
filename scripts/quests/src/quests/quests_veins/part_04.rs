use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lockenlock_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rachel_camel").get()?.is_true() && ctx.var("rachel_camel").get()?.number()? < 4) {
        ctx.lines(args![
            "^3355FFIt's a drunkard...",
            "The scent of pure",
            "alcohol wafts around him.",
            "There's a certain beauty",
            "to his disheveled misery.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rachel_camel").get()? == 4 {
            if ctx.call(Function::CountItem, vec![Val::from(503)])?.number()? > 0 {
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me...?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args!["Huh? Arrrgh...", "My head... What", "do you want?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Oh, I'd like", "to make a key."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Keys? Yeah, yeah...",
                        "That's what I do.",
                        "If you've got the lock,",
                        "it'll be a piece of cake."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Ugh, but I'm so thirsty",
                        "and this headache is",
                        "killing me. You mind",
                        "bringing me a Yellow",
                        "Potion first?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Sure, I guess.", "I can part with just", "1 Yellow Potion.", "Here you go."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Ah, that hit the spot!",
                        "Wait, wait... Now I feel",
                        "dizzy... What's going...",
                        "What's going on...?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Okay, okay...",
                        "I'm alright now.",
                        "So what'd you say you",
                        "needed? A key? Did you",
                        "bring the lock with you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "I'm an expert in crafting",
                        "keys and locks. Hell, my locks",
                        "are strong enough to hold down",
                        "a dragon, you know that? I'll",
                        "have you know that the Rachel",
                        "Army's a regular customer~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "(^333333This guy made locks for",
                        "the Rachel army?! It might",
                        "not be a good idea to let him",
                        "know that I'm trying to free",
                        "one of their prisoners. Who",
                        "knows if he's loyal to them?^000000)"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "So what happened was...",
                        "I lost my key, but I can't",
                        "bring the lock here with me.",
                        "I think I'd end up breaking",
                        "it if I brought it with me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Oh, yeah? No problem.",
                        "Just take me to the lock.",
                        "I'll charge you extra, though,",
                        "especially since my knees",
                        "are going bad. So where",
                        "are we going exactly?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Wait!", "We can't do that!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "What do you mean?",
                        "You're not trying to",
                        "open up a bank safe",
                        "or something, are you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "No, it's nothing like",
                        "that! It's just that the lock",
                        "is in a dangerous place.",
                        "This is really important...",
                        "Please, you have to help me!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Huh. Well, bottom line,",
                        "I can't make a key without",
                        "looking at the lock. Let me",
                        "think of a way I can help you.",
                        "Give me a second, will you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "...............................",
                        "Well, I guess you can try",
                        "to make a mold of the lock.",
                        "It'll have to be perfect, so",
                        "this'll get pretty expensive."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Go to the market and",
                        "find a lady selling organic",
                        "soap. You need to get a bottle",
                        "of Chamelepu Soap. You will",
                        "need that exact type of soap:",
                        "nothing else will do."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Chamelpu Soap?", "What is th--"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "No time to explain.",
                        "You'd better hurry and",
                        "find her before she closes",
                        "shop for the day. The shop",
                        "owner's a beauty, so it'll",
                        "be tough for you to miss her."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["..........."])?;
                ctx.call(Function::DelItem, vec![Val::from(503), Val::from(1)])?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3063), Val::from(3064)])?;
                ctx.var("rachel_camel").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "^3355FFIt's a drunkard...",
                    "This man must be the",
                    "Mr. Lockenlock that you seek.",
                    "You'd better follow Toby's",
                    "advice and bring this man",
                    "a Yellow Potion first.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("rachel_camel").get()? == 5 {
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "Go to the market and",
                        "find a lady selling organic",
                        "soap, and get a bottle of",
                        "Chamelepu Soap. You will",
                        "need that exact type of soap:",
                        "nothing else will do."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Chamelpu Soap?", "What is th--"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Locksmith Lockenlock",
                    args![
                        "No time to explain.",
                        "You'd better hurry and",
                        "find her before she closes",
                        "shop for the day. The shop",
                        "owner's a beauty, so it'll",
                        "be tough for you to miss her."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rachel_camel").get()? == 6 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Wait... I should be",
                            "bringing Ms. Ivory all",
                            "of the soap ingredients.",
                            "What were they again...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "^4D4DFF10 Milk^000000,",
                            "^4D4DFF100 Green Herbs^000000,",
                            "^4D4DFF50 Jellopies^000000, and",
                            "^4D4DFF5 Empty Bottles^000000.",
                            "I better get those..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rachel_camel").get()? == 7 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I need to talk to", "someone named Saraman", "to get the soap ingredients..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rachel_camel").get()? == 8 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Let's see...",
                                    "I need to bring",
                                    "Mr. Saruman all the",
                                    "things he needs to",
                                    "stimulate a camel's ",
                                    "appetite. I need to get..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "^4D4DFF1 Unripe Apple^000000,",
                                    "^4D4DFF5 Monster's Feed^000000,",
                                    "^4D4DFF1 Empty Bottle^000000, and",
                                    "^4D4DFF1 Yellow Potion^000000."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rachel_camel").get()? == 9 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "I have everything I need",
                                        "to stimulate a camel's",
                                        "appetite. Now I need to",
                                        "feed the camel so that I can",
                                        "get the soap ingredients and",
                                        "5 lumps of camel dung."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("rachel_camel").get()? == 10 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Right now, my time",
                                            "would be better spent",
                                            "looking for the Silk Sand",
                                            "Camel for the ingredients."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("rachel_camel").get()? == 11 {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "I'd better find Mr. Saraman's",
                                                "lost camel, feed it camel",
                                                "appetite stimulants, and",
                                                "then get the soap ingredient",
                                                "and 5 lumps of camel dung",
                                                "if I want to free Curdie."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("rachel_camel").get()? == 12 {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Well... I found the",
                                                    "camel. Now I need to get",
                                                    "all the soap ingredients.",
                                                    "The sooner I do that, the",
                                                    "sooner I can help Curdie."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if (ctx.var("rachel_camel").get()?.number()? > 12
                                                && ctx.var("rachel_camel").get()?.number()? < 17)
                                            {
                                                ctx.lines(args![
                                                    "^3355FFYou already found the",
                                                    "camel, so you need to collect",
                                                    "the soap ingredients if you",
                                                    "want to free Curdie.^000000"
                                                ])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("rachel_camel").get()? == 17 {
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "I managed to get the soap",
                                                            "ingredients: 5 of those",
                                                            "camel dung lumps. I should",
                                                            "head back to Mr. Saraman to",
                                                            "tell him where his camel is,",
                                                            "and then go to Ms. Ivory."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("rachel_camel").get()? == 18 {
                                                        ctx.lines_as(
                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                            args!["Let's see...", "Shouldn't I be going", "to see Ms. Ivory now?"],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("rachel_camel").get()? == 19 {
                                                            ctx.lines_as(
                                                                "Locksmith Lockenlock",
                                                                args![
                                                                    "Oh, so you're finally",
                                                                    "back with the Chamelpu",
                                                                    "Soap. What took so long?",
                                                                    "All you had to do was go",
                                                                    "to the market and buy it."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args![
                                                                    "...............................",
                                                                    "...............................",
                                                                    "...............................",
                                                                    "...............................",
                                                                    "...............................",
                                                                    "..............................."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Locksmith Lockenlock",
                                                                args![
                                                                    "Uh, anyway, did Ms. Ivory",
                                                                    "tell you how to use the soap?",
                                                                    "You just pour it into the",
                                                                    "keyhole, and then pour the",
                                                                    "soap back into the bottle.",
                                                                    "Do it carefully and quickly."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Locksmith Lockenlock",
                                                                args![
                                                                    "Make sure you close the",
                                                                    "bottle tightly when you're",
                                                                    "done so no air gets into it.",
                                                                    "If you take more than thirty",
                                                                    "seconds, the soap won't retain",
                                                                    "the lock's shape very well."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Locksmith Lockenlock", args!["Pour the liquid soap to the key hole in the lock,", "pour the soap back to the bottle.", "and close the lid so tightly that the air wouldn't go inside the bottle.", "Remember, you can't take longer than 30 seconds to finish the procedures."])?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                "Locksmith Lockenlock",
                                                                args![
                                                                    "Finally, bring the bottle",
                                                                    "back here for me so that",
                                                                    "I can make the key. But when",
                                                                    "you come back, I need to make",
                                                                    "sure that the key you're making",
                                                                    "isn't for anything illegal..."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                args!["Alright."],
                                                            )?;
                                                            ctx.call(Function::ChangeQuest, vec![Val::from(3078), Val::from(3079)])?;
                                                            ctx.var("rachel_camel").set(Val::from(20))?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("rachel_camel").get()? == 20 {
                                                                ctx.lines_as(
                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                    args![
                                                                        "I'd better use the soap to",
                                                                        "make a key mold that I can",
                                                                        "bring over to Mr. Lockenlock."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("rachel_camel").get()? == 21 {
                                                                ctx.lines_as(
                                                                    "Locksmith Lockenlock",
                                                                    args![
                                                                        "Oh, you're back...",
                                                                        "So did you manage",
                                                                        "to make the key mold?"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                    args![
                                                                        "Yes, I did. Would you",
                                                                        "please hurry? This is an",
                                                                        "emergency, and it could",
                                                                        "get really bad if I don't",
                                                                        "get this key made soon..."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Locksmith Lockenlock",
                                                                    args![
                                                                        "Let me see... Well,",
                                                                        "it's not really perfect, but",
                                                                        "I should be able to fashion",
                                                                        "a key for this lock. Ooh...",
                                                                        "But you know what? I don't",
                                                                        "have any materials on me."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Locksmith Lockenlock",
                                                                    args![
                                                                        "I've already made all",
                                                                        "the keys and locks for this",
                                                                        "town, so no one's really had",
                                                                        "to send in any orders lately.",
                                                                        "That's why I didn't have any",
                                                                        "materials onhand. Sorry."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Locksmith Lockenlock",
                                                                    args![
                                                                        "All I need is ^4D4DFF1 Steel^000000.",
                                                                        "It won't take me more",
                                                                        "than five minutes to make",
                                                                        "the key, so I can get it done",
                                                                        "as soon as you can bring",
                                                                        "me the Steel. I'll be waiting."
                                                                    ],
                                                                )?;
                                                                ctx.call(Function::ChangeQuest, vec![Val::from(3080), Val::from(3081)])?;
                                                                ctx.var("rachel_camel").set(Val::from(22))?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("rachel_camel").get()? == 22 {
                                                                if ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 0 {
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "Here's the Steel that",
                                                                            "you need. Would you",
                                                                            "please make the key now?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "...............................",
                                                                            "I've been studying",
                                                                            "this key mold, and",
                                                                            "I just realized something...",
                                                                            "Tell me right now: what",
                                                                            "are you using this key for?"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args!["Huh...?", "What are you...?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "I asked you first.",
                                                                            "Tell me what you intend",
                                                                            "to do with this key! If you",
                                                                            "don't, I can't help you."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Don't lie to me.",
                                                                            "This mold... This is",
                                                                            "the lock for the shackles",
                                                                            "that I've made under the",
                                                                            "orders of the Rachel Army.",
                                                                            "I have the master key for that."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "These shackles are only",
                                                                            "supposed to be used for",
                                                                            "prisoners! If I release one",
                                                                            "of them, they will hunt you",
                                                                            "down and hold me accountable."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                        args![
                                                                            "The truth is... I really",
                                                                            "am trying to free somebody",
                                                                            "the Rachel Army imprisoned...",
                                                                            "She's Curdie, a young girl",
                                                                            "that they locked up in",
                                                                            "Thor Volcano..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "What...?!",
                                                                            "Are you serious?",
                                                                            "Y-you're... No way.",
                                                                            "You're not lying. This...",
                                                                            "I'm sorry. You shouldn't",
                                                                            "have gotten involved, but..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Oh God!",
                                                                            "They... They",
                                                                            "really imprisoned",
                                                                            "an innocent child?!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Here. Take it.",
                                                                            "Take the master key.",
                                                                            "Bring it to Thor Volcano",
                                                                            "and rescue that poor kid.",
                                                                            "We could get in a lot of",
                                                                            "trouble for doing this, but..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "My conscience won't allow",
                                                                            "them to do something like",
                                                                            "this. Rescue that kid, and",
                                                                            "then throw the key away",
                                                                            "somewhere when you're done."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "That way, if the army",
                                                                            "comes to interrogate me,",
                                                                            "I'll just say that it was",
                                                                            "stolen from my shop by",
                                                                            "some thief. I should get",
                                                                            "rid of all my keys too..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Hurry up and go!",
                                                                            "Make sure that you",
                                                                            "bring that child back",
                                                                            "safe to her family!"
                                                                        ],
                                                                    )?;
                                                                    ctx.call(
                                                                        Function::ChangeQuest,
                                                                        vec![Val::from(3081), Val::from(3082)],
                                                                    )?;
                                                                    ctx.var("rachel_camel").set(Val::from(23))?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "All I need is ^4D4DFF1 Steel^000000.",
                                                                            "It won't take me more",
                                                                            "than five minutes to make",
                                                                            "the key, so I can get it done",
                                                                            "as soon as you can bring",
                                                                            "me the Steel. I'll be waiting."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                            } else {
                                                                if ctx.var("rachel_camel").get()? == 23 {
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Here. Take it.",
                                                                            "Take the master key.",
                                                                            "Bring it to Thor Volcano",
                                                                            "and rescue that poor kid.",
                                                                            "We could get in a lot of",
                                                                            "trouble for doing this, but..."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Hurry up and go!",
                                                                            "Make sure that you",
                                                                            "bring that child back",
                                                                            "safe to her family!"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx.var("rachel_camel").get()? == 24 {
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Hey, that kid you saved...",
                                                                            "Curdie. She's safely back",
                                                                            "in town. I feel so responsible",
                                                                            "about what happened. I mean,",
                                                                            "she was locked up in shackles",
                                                                            "that I designed for the army."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Anyway, I'm packing my",
                                                                            "things. I'm thinking of",
                                                                            "leaving this town for good.",
                                                                            "The army's rotten to the core",
                                                                            "for doing something like this."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "You're a real good person,",
                                                                            "and I'm glad I got to know",
                                                                            "you. I don't know if we'll",
                                                                            "ever meet again, but it's",
                                                                            "good that there are people",
                                                                            "like you in this world."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx.var("rachel_camel").get()? == 25 {
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Hey, that kid you saved...",
                                                                            "Curdie. She's safely back",
                                                                            "in town. I feel so responsible",
                                                                            "about what happened. I mean,",
                                                                            "she was locked up in shackles",
                                                                            "that I designed for the army."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "Anyway, I'm packing my",
                                                                            "things. I'm thinking of",
                                                                            "leaving this town for good.",
                                                                            "The army's rotten to the core",
                                                                            "for doing something like this."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Locksmith Lockenlock",
                                                                        args![
                                                                            "You're a real good person,",
                                                                            "and I'm glad I got to know",
                                                                            "you. I don't know if we'll",
                                                                            "ever meet again, but it's",
                                                                            "good that there are people",
                                                                            "like you in this world."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    ctx.lines_as(
                                                                        "Lockenlock",
                                                                        args!["Zzzz...", "Zzzz... Argh!", "...Zzz..."],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^3355FFHe's drunk and", "fast asleep.^000000"])?;
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
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn lockenlock(ctx: &Ctx) -> Script {
    lockenlock_body(ctx, Vec::new()).map(|_| ())
}

fn ivory_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rachel_camel").get()?.is_true() && ctx.var("rachel_camel").get()?.number()? < 5) {
        ctx.lines_as(
            "Organic Soap Maker Ivory",
            args![
                "I need to make more of",
                "my soap, but I've run out",
                "of ingredients. Well, there's",
                "not much I can do without them,",
                "so maybe it'd be better if",
                "I just close up shop today..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rachel_camel").get()? == 5 {
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me...?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "Oh, I'm sorry, but I'm",
                    "closing shop right now",
                    "because I just ran out of",
                    "soap ingredients. If you",
                    "ordered something, then you",
                    "could just come back tomorrow."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Um, this is an emergency!",
                    "Mr. Lockenlock told me to",
                    "come here to get some kind",
                    "of special soap. I need it",
                    "to make a mold for him",
                    "to make a key for me..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "Oh? You locked yourself",
                    "out? Ah, Mr. Lockenlock",
                    "must have been talking",
                    "about my organic Chamelepu",
                    "soap. It's an artistic soap",
                    "that you can shape easily~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Yes, that's right!", "Chamelepu Soap!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "My Chamelepu Soap",
                    "is pretty popular. It's",
                    "a liquid soap that you can",
                    "pour into anything, and it'll",
                    "harden into any shape that",
                    "you want. Neat, huh?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "Well, as I told you earlier,",
                    "I ran out of every soap ingredient,",
                    "so I cannot make any more soap today.",
                    "You should come back tomorrow evening",
                    "if you want to buy the soap."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "If you really need it",
                    "right away, I can still",
                    "make it for you if can",
                    "bring all the ingredients.",
                    "They might be a bit hard",
                    "to obtain, though..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "That's fine. The",
                    "important thing for me",
                    "is to get this soap as",
                    "soon as I possibly can!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "Alright, first I want",
                    "you to bring me the basic",
                    "stuff. Bring these items",
                    "in the exact amounts I ask",
                    "for, alright? Ratios are pretty",
                    "important in making soap."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Organic Soap Maker Ivory",
                args![
                    "^4D4DFF10 Milk^000000,",
                    "^4D4DFF100 Green Herbs^000000,",
                    "^4D4DFF50 Jellopies^000000, and",
                    "^4D4DFF5 Empty Bottles^000000.",
                    "Then we can move",
                    "on to the hard part."
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3064), Val::from(3065)])?;
            ctx.var("rachel_camel").set(Val::from(6))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rachel_camel").get()? == 6 {
                if (((ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 99
                    && ctx.call(Function::CountItem, vec![Val::from(909)])?.number()? > 49)
                    && ctx.call(Function::CountItem, vec![Val::from(519)])?.number()? > 9)
                    && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 4)
                {
                    ctx.lines_as(
                        "Organic Soap Maker Ivory",
                        args![
                            "Oh, great! You brought",
                            "everything! Now... It's",
                            "time for you to do the,",
                            "um, hard part."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "...............................",
                            "...............................",
                            "..............................."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Organic Soap Maker Ivory",
                        args![
                            "Please tell the",
                            "Silk Sand Camel farm",
                            "owner in town that I sent",
                            "you, and show him the",
                            "ingredients that you've",
                            "gathered for me so far."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Wait...",
                            "Why would I want",
                            "to see the guy that",
                            "takes care of Silk Sand",
                            "Camels? What does he",
                            "have to do with soap?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Organic Soap Maker Ivory",
                        args![
                            "Don't sweat it for now...",
                            "Just go visit Mr. Saraman,",
                            "the Camel Farm owner.",
                            "I can't wait for you that long,",
                            "so please come back here",
                            "as soon as possible."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Mr. Saraman...?",
                            "Okay, so I need to visit",
                            "him if I really need you",
                            "to make the soap..."
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3065), Val::from(3066)])?;
                    ctx.var("rachel_camel").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Organic Soap Maker Ivory",
                        args![
                            "Alright, first I want",
                            "you to bring me the basic",
                            "stuff. Bring these items",
                            "in the exact amounts I ask",
                            "for, alright? Ratios are pretty",
                            "important in making soap."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Organic Soap Maker Ivory",
                        args![
                            "^4D4DFF10 Milk^000000,",
                            "^4D4DFF100 Green Herbs^000000,",
                            "^4D4DFF50 Jellopies^000000, and",
                            "^4D4DFF5 Empty Bottles^000000.",
                            "Then we can move",
                            "on to the hard part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("rachel_camel").get()? == 7 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I need to talk to", "someone named Saraman", "to get the soap ingredients..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rachel_camel").get()? == 8 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Let's see...",
                                "I need to bring",
                                "Mr. Saruman all the",
                                "things he needs to",
                                "stimulate a camel's ",
                                "appetite. I need to get..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "^4D4DFF1 Unripe Apple^000000,",
                                "^4D4DFF5 Monster's Feed^000000,",
                                "^4D4DFF1 Empty Bottle^000000, and",
                                "^4D4DFF1 Yellow Potion^000000."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rachel_camel").get()? == 9 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I have everything I need",
                                    "to stimulate a camel's",
                                    "appetite. Now I need to",
                                    "feed the camel so that I can",
                                    "get the soap ingredients and",
                                    "5 lumps of camel dung."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rachel_camel").get()? == 10 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Right now, my time",
                                        "would be better spent",
                                        "looking for the Silk Sand",
                                        "Camel for the ingredients."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("rachel_camel").get()? == 11 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "I'd better find Mr. Saraman's",
                                            "lost camel, feed it camel",
                                            "appetite stimulants, and",
                                            "then get the soap ingredient",
                                            "and 5 lumps of camel dung",
                                            "if I want to free Curdie."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("rachel_camel").get()? == 12 {
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args![
                                                "Well... I found the",
                                                "camel. Now I need to get",
                                                "all the soap ingredients.",
                                                "The sooner I do that, the",
                                                "sooner I can help Curdie."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if (ctx.var("rachel_camel").get()?.number()? > 12 && ctx.var("rachel_camel").get()?.number()? < 17)
                                        {
                                            ctx.lines(args![
                                                "^3355FFYou already found the",
                                                "camel, so you need to collect",
                                                "the soap ingredients if you",
                                                "want to free Curdie.^000000"
                                            ])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("rachel_camel").get()? == 17 {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "I managed to get the soap",
                                                    "ingredients: 5 of those",
                                                    "camel dung lumps. I should",
                                                    "head back to Mr. Saraman to",
                                                    "tell him where his camel is,",
                                                    "and then go to Ms. Ivory."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("rachel_camel").get()? == 18 {
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args![
                                                    "Hm, did Soony give you",
                                                    "any trouble? I'm guessing",
                                                    "that's why it's been taking",
                                                    "you so long to get all that",
                                                    "camel dung over here."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "It was a pretty big",
                                                    "hassle... But hopefully,",
                                                    "this will all be worth it.",
                                                    "Anyway, take this camel",
                                                    "dung. I don't want to",
                                                    "handle it for much longer."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args![
                                                    "Yes, I can understand that.",
                                                    "Even though it's in bottles,",
                                                    "it's pretty gross that these",
                                                    "bottles are still warm...",
                                                    "Anyway, we're good to go.",
                                                    "Let me make you some soap~"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args![
                                                    "You spent a lot of",
                                                    "time and energy to get",
                                                    "these, so I won't charge",
                                                    "you for my service. Besides,",
                                                    "you brought enough materials",
                                                    "that I'll have some left over."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^3355FFMs. Ivory put on a pair",
                                                "of long gloves, and mixed",
                                                "the ingredients. She then",
                                                "placed them in a clean bottle.^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args![
                                                    "Here's your soap! After",
                                                    "you pour the soap into",
                                                    "something, don't forget",
                                                    "to put it into a larger bottle",
                                                    "after about twenty seconds."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args![
                                                    "And um... Don't let",
                                                    "anyone else know what",
                                                    "I use to make this soap.",
                                                    "People won't buy it if they",
                                                    "knew they were washing their",
                                                    "faces with... You know..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Don't worry, I won't", "tell anyone. Thanks", "for making the soap! "],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^3355FFNow that you have the",
                                                "Silk Sand Camel Soap.",
                                                "you should bring it",
                                                "to Mr. Lockenlock.^000000"
                                            ])?;
                                            ctx.call(Function::ChangeQuest, vec![Val::from(3077), Val::from(3078)])?;
                                            ctx.var("rachel_camel").set(Val::from(19))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("rachel_camel").get()? == 19 {
                                            ctx.lines(args![
                                                "^3355FFNow that you have the",
                                                "Silk Sand Camel Soap.",
                                                "you should bring it",
                                                "to Mr. Lockenlock.^000000"
                                            ])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("rachel_camel").get()?.number()? > 19
                                            && ctx.var("rachel_camel").get()?.number()? < 26)
                                        {
                                            ctx.lines_as(
                                                "Organic Soap Maker Ivory",
                                                args!["^333333*Phew!*^000000 It's been", "a long day. I think", "I'll close up shop now~"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                "Beautiful Lady",
                                                args![
                                                    "Hm? Did you need",
                                                    "something? I'm still",
                                                    "setting up shop now",
                                                    "so I'm not really ready",
                                                    "to sell anything yet."
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
    }
}

pub fn ivory(ctx: &Ctx) -> Script {
    ivory_body(ctx, Vec::new()).map(|_| ())
}

fn saraman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rachel_camel").get()?.is_true() && ctx.var("rachel_camel").get()?.number()? < 7) {
        ctx.lines_as("Saraman", args!["Zzzzz...", "Zzz... Zzzzzz..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rachel_camel").get()? == 7 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Excuse me. Hello~",
                    "Ms. Ivory sent me",
                    "here with these soap",
                    "ingredients? She said",
                    "I had to come to you if",
                    "I wanted her to make it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args!["Soap, eh? Oh, I see.", "You must be here to get", "some fresh camel dung."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Wh-what?", "My God!", "A-are you sure?", "Tell me you're joking!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Camel Farm Owner Saraman", args!["Sure as sin, and", "honest to God."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "...............................",
                    "...............................",
                    "..............................."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Yeah, those ingredients",
                    "you brought me? They're for",
                    "making soap alright. ^FF0000That's",
                    "what we feed the camels^000000 so",
                    "that they make a whole lotta",
                    "dung. Dung to make soap."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Oh, don't worry. Camel",
                    "dung is sterile, completely",
                    "safe. In fact, it smells nice,",
                    "has medicinal properties, and",
                    "it's considered a delicacy",
                    "in certain countries."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Sadly, Silk Sand Camels",
                    "are almost extinct, so I only",
                    "have one on this farm. That's",
                    "why we have a special contract with Ms. Ivory to make her soap."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Say... I wasn't expecting",
                    "her to send a deliveryman",
                    "until tomorrow. Why are",
                    "you here so early anyway?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Actually, this is kind",
                    "of an emergency. You see,",
                    "I need the soap to make a",
                    "key mold because I lost--"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Never mind, never mind.",
                    "I'm sorry I asked! So this",
                    "is a personal favor for you,",
                    "huh? Well, there's a bit of",
                    "a problem that we need to",
                    "solve first. Listen up..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "We can only get camel",
                    "dung after a camel eats,",
                    "right? Well, my camel isn't",
                    "used to eating so late in the",
                    "day. Even if we put food in",
                    "front of her, she won't eat it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "What? Isn't there",
                    "something we can do?",
                    "I mean, I'm talking about",
                    "a life or death matter!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "*Sigh* ...This isn't good... Okay then, let's do this.",
                    "Sometimes I make an appetite stimulant for the camel",
                    "when she's sick and wouldn't eat anything.",
                    "We can try feeding her the stimulant, and make it poop."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Well, we can't forcefeed",
                    "her, but we can get her to",
                    "munch a bit on some appetite",
                    "stimulant. I usually use that",
                    "if she's sick, but if you say",
                    "that this is an emergency..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "Alright, I guess we",
                    "can try it. But I want you",
                    "to bring me the ingredients.",
                    "Get me... Let's see now..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Camel Farm Owner Saraman",
                args![
                    "^4D4DFF1 Unripe Apple^000000,",
                    "^4D4DFF5 Monster's Feed^000000,",
                    "^4D4DFF1 Empty Bottle^000000, and",
                    "^4D4DFF1 Yellow Potion^000000."
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3066), Val::from(3067)])?;
            ctx.var("rachel_camel").set(Val::from(8))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("rachel_camel").get()? == 8 {
                if (((ctx.call(Function::CountItem, vec![Val::from(528)])?.number()? > 4
                    && ctx.call(Function::CountItem, vec![Val::from(503)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(619)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
                {
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Oh good, you're back.",
                            "Did you bring everything?",
                            "Here, I need to mix it all",
                            "together first before you",
                            "can feed it to my camel.",
                            "Just a minute now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFSaraman mixed all",
                        "of the ingredients,",
                        "and gingerly poured",
                        "them into a bottle.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Alright, now bring",
                            "this to the camels over",
                            "there. Only my Silk Sand",
                            "Camel will know to eat it,",
                            "so she'll come after you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Once she nibbles this",
                            "appetite stimulant, she'll",
                            "eat her feed like crazy.",
                            "Then the dung will flow",
                            "like a faucet. Come on,",
                            "why don't you give it a try?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Oh, right. You should",
                            "be able to get 5 lumps",
                            "of camel dung with those",
                            "ingredients. That's a good",
                            "amount to collect since that's",
                            "what Ms. Ivory usually orders."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(528), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(503), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(619), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3067), Val::from(3068)])?;
                    ctx.var("rachel_camel").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Well, we can't forcefeed",
                            "her, but we can get her to",
                            "munch a bit on some appetite",
                            "stimulant. I usually use that",
                            "if she's sick, but if you say",
                            "that this is an emergency..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Alright, I guess we",
                            "can try it. But I want you",
                            "to bring me the ingredients.",
                            "Get me... Let's see now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "^4D4DFF1 Unripe Apple^000000,",
                            "^4D4DFF5 Monster's Feed^000000,",
                            "^4D4DFF1 Empty Bottle^000000, and",
                            "^4D4DFF1 Yellow Potion^000000."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("rachel_camel").get()? == 9 {
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Once she nibbles this",
                            "appetite stimulant, she'll",
                            "eat her feed like crazy.",
                            "Then the dung will flow",
                            "like a faucet. Come on,",
                            "why don't you give it a try?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Camel Farm Owner Saraman",
                        args![
                            "Oh, right. You should",
                            "be able to get 5 lumps",
                            "of camel dung with those",
                            "ingredients. That's a good",
                            "amount to collect since that's",
                            "what Ms. Ivory usually orders."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rachel_camel").get()? == 10 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Mr. Saraman, none",
                                "of the camels will eat",
                                "this appetite stimulant...",
                                "Am I doing something wrong?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Camel Farm Owner Saraman",
                            args![
                                "Oh, yes, well...",
                                "One of my workers",
                                "just came by, and told me",
                                "that my Silk Sand Camel",
                                "disappeared somewhere...",
                                "This is terrible news!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Camel Farm Owner Saraman",
                            args![
                                "My precious Silk Sand",
                                "Camel... It's my biggest",
                                "business investment! I'm",
                                "ruined without it! Please...",
                                "I'll reward you if you can",
                                "find my camel for me!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Camel Farm Owner Saraman",
                            args![
                                "Damn it! My stupid worker",
                                "forgot to tie her up, so",
                                "she ended up running away!",
                                "Ugh! Please help me find her!",
                                "Without her, you won't be able",
                                "to get your special camel dung."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Camel Farm Owner Saraman",
                            args![
                                "Wait, don't panic...",
                                "It'll all be alright.",
                                "This camel moves very slowly",
                                "so she shouldn't be far from",
                                "here. Please find my camel",
                                "Soony as soon as you can!"
                            ],
                        )?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3069), Val::from(3070)])?;
                        ctx.var("rachel_camel").set(Val::from(11))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rachel_camel").get()? == 11 {
                            ctx.lines_as("Camel Farm Owner Saraman", args!["Soony! Soony...!", "Wh-where are you?!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Camel Farm Owner Saraman",
                                args![
                                    "Please, help me find",
                                    "my Soony, my Silk Sand",
                                    "Camel. You won't be able to",
                                    "make your soap without her!",
                                    "And my business is really",
                                    "dependent on my Soony!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rachel_camel").get()? == 12 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Well... I found the",
                                        "camel. Now I need to get",
                                        "all the soap ingredients.",
                                        "The sooner I do that, the",
                                        "sooner I can help Curdie."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("rachel_camel").get()?.number()? > 12 && ctx.var("rachel_camel").get()?.number()? < 17) {
                                ctx.lines(args![
                                    "^3355FFYou already found the",
                                    "camel, so you need to collect",
                                    "the soap ingredients if you",
                                    "want to free Curdie.^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("rachel_camel").get()? == 17 {
                                ctx.lines_as(
                                    "Camel Farm Owner Saraman",
                                    args![
                                        "Oh, it's you!",
                                        "Did you find my",
                                        "Soony? Where is she?",
                                        "What happened to her?",
                                        "Oh God, I don't know what",
                                        "I'll do without that camel!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Don't worry, Mr. Saraman,",
                                        "I found Soony at the outskirts",
                                        "of town. She hurt her leg so",
                                        "I think it'd be a good idea if",
                                        "you sent some people to",
                                        "help bring her back."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Camel Farm Owner Saraman",
                                    args![
                                        "Thank god! Thank you, thank you so much!",
                                        "I'll send my workers over there immediately."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Camel Farm Owner Saraman",
                                    args![
                                        "Thank goodness, you have",
                                        "no idea how valuable that",
                                        "camel is! I'll send some of",
                                        "my men to bring her home",
                                        "immediately! Thank you,",
                                        "you just saved my business!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Camel Farm Owner Saraman",
                                    args![
                                        "Here, I want you to",
                                        "have this. Consider it",
                                        "a little thank you gift for",
                                        "what you've done for me.",
                                        "Good luck with getting",
                                        "that soap you want made."
                                    ],
                                )?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(3076), Val::from(3077)])?;
                                ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                ctx.var("rachel_camel").set(Val::from(18))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("rachel_camel").get()? == 18 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Let's see...", "Shouldn't I be going", "to see Ms. Ivory now?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("rachel_camel").get()?.number()? > 18 && ctx.var("rachel_camel").get()?.number()? < 24) {
                                ctx.lines_as(
                                    "Camel Farm Owner Saraman",
                                    args![
                                        "Thank you for finding my",
                                        "precious Silk Sand Camel",
                                        "Soony. Come again sometime,",
                                        "and maybe we can special my",
                                        "special camel yogurt together."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Saraman", args!["Zzz... Zzz~", "Zzz..."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFWatching this man",
                                    "snore also makes you",
                                    "want to take a snooze.^000000"
                                ])?;
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

pub fn saraman(ctx: &Ctx) -> Script {
    saraman_body(ctx, Vec::new()).map(|_| ())
}

fn camel_camelcc1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rachel_camel").get()? == 9 {
        ctx.lines(args![
            "^3355FFThe camel sniffed the",
            "appetite stimulant, but",
            "brusquely turned its",
            "nose away from it.^000000"
        ])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3068), Val::from(3069)])?;
        ctx.var("rachel_camel").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rachel_camel").get()? == 10 {
        ctx.lines(args![
            "^3355FFThe camel sniffed the",
            "appetite stimulant, but",
            "brusquely turned its",
            "nose away from it.",
            "This probably isn't the",
            "camel you're looking for.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Camel", args!["*Neigh* ~", "*Chew Chew*"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn camel_camelcc1(ctx: &Ctx) -> Script {
    camel_camelcc1_body(ctx, Vec::new()).map(|_| ())
}
