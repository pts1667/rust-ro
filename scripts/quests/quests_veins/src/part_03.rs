use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kid_camelcamel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true() {
        if ctx.var("rachel_camel").get()? == 0 {
            ctx.lines_as("Kid Karyn", args!["*Sob*"])?;
            ctx.next()?;
            ctx.lines(args!["^3355FFThis sobbing child", "looks really upset...^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hey, why are you", "crying? Are you lost?", "Where's your mommy?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Kid Karyn", args!["My... Mom's", "at home..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["......", "........."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["I see. So...", "Are you having", "trouble finding", "your way back home?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "No! I'm ten years old!",
                    "I can find my way home,",
                    "even with my eyes closed!",
                    "^333333*Sniff sniff*^000000 Uuuuuuweeeh~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Well..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Can you tell me", "why you're crying?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "I... ^333333*Sniff*^000000",
                    "I-I went to... Th-...",
                    "Volcan.... w-w-with",
                    "my sist-- Waaaaaaah!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Poor kid... Come on,",
                    "take a deep breath so",
                    "you can tell me about what",
                    "happened a little more slowly."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "*^333333Sob*^000000 ...It's just...",
                    "I went to Thor Volcano",
                    "with my little sister to see",
                    "which one of us was braver...",
                    "But then, we met some...",
                    "Scary people there... and..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Kid Karyn", args!["I got scared, so... So...", "My sister... Wah~! *Sob*"])?;
            ctx.next()?;
            ctx.lines_as("Kid Karyn", args!["*Sob*"])?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "I-I ran away from them...",
                    "But I left my sister over",
                    "there with those weird men..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "Mom's sick, and dad's",
                    "always at work... He's",
                    "the captain of a ship, so...",
                    "I don't think they can help."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "Can you help me please,",
                    "and bring my sister back?",
                    "*Sniff* Please? I promise",
                    "that I can pay you as soon",
                    "as my dad comes back!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Wait... I have a question.",
                    "You said that someone took",
                    "away your sister at Thor",
                    "Volcano? What did they look",
                    "like? Are you sure that there",
                    "were people there?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kid Karyn",
                args![
                    "I... I don't know!",
                    "I got so scared, I just",
                    "ran away! I... I didn't",
                    "mean to leave my sister!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hmmm...", "Maybe your sister was", "kidnapped by bandits..."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Decline Request:Accept Request")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Sorry kid, but I've got",
                            "things to do. I'm sure",
                            "someone else will come",
                            "along to save your sister."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kid Karyn", args!["Wah~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Okay, I'll see what", "I can do. I'll try my best", "to find your sister."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kid Karyn",
                        args![
                            "Thank you so much!",
                            "Please find my sister",
                            "Curdie soon! Oh, I hope",
                            "she's okay! If she's not...",
                            "^333333*Sob*^000000 I don't know what",
                            "I'll do! Waaaaaaaah~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Alright...",
                            "Wish me luck.",
                            "I'll go search Thor",
                            "Volcano to find your",
                            "little sister Curdie."
                        ],
                    )?;
                    ctx.call(Function::SetQuest, vec![Val::from(3060)])?;
                    ctx.var("rachel_camel").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("rachel_camel").get()? == 1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I'd better search", "Thor Volcano for Curdie,", "Karyn's little sister."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rachel_camel").get()? == 2 {
                    ctx.lines_as(
                        "Kid Karyn",
                        args!["W-were you able", "to find my sister?", "Is she alright?", "What happened?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I found her,", "she's alright but..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kid Karyn", args!["What? Why isn't", "she with you?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Curdie is... Well, she's",
                            "been shackled down. We",
                            "need to find a way to free her.",
                            "Do you know where there's",
                            "a forge or a locksmith that",
                            "might be able to help her?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kid Karyn",
                        args![
                            "Oh! Oh, there's a",
                            "locksmith in the market",
                            "street! You can ask him",
                            "to help free Curdie!"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3061), Val::from(3062)])?;
                    ctx.var("rachel_camel").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rachel_camel").get()? == 3 {
                        ctx.lines_as(
                            "Kid Karyn",
                            args![
                                "Let's see...",
                                "I'd better find the",
                                "locksmith in the market",
                                "street, and ask him to help",
                                "me unlock Curdie's shackles."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rachel_camel").get()? == 4 {
                            ctx.lines_as(
                                "Kid Karyn",
                                args![
                                    "Huh... I'd better talk",
                                    "to Mr. Lockenlock first, the",
                                    "locksmith in the market street."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rachel_camel").get()? == 5 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Hmm... I'd better talk",
                                        "to Ms. Ivory, the organic",
                                        "soap maker if I want to help",
                                        "Karyn free his sister Curdie."
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
                                                        if (ctx.var("rachel_camel").get()?.number()? > 11
                                                            && ctx.var("rachel_camel").get()?.number()? < 17)
                                                        {
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
                                                            if ctx.var("rachel_camel").get()? == 17 {
                                                                ctx.lines_as(
                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                    args![
                                                                        "I managed to get the soap",
                                                                        "ingredient and 5 of those",
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
                                                                        args![
                                                                            "Let's see...",
                                                                            "Shouldn't I be going",
                                                                            "to see Ms. Ivory now?"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if ctx.var("rachel_camel").get()? == 19 {
                                                                        ctx.lines_as(
                                                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                            args![
                                                                                "I have the Silk Sand Camel",
                                                                                "Soap now, so I should go",
                                                                                "bring it to Mr. Lockenlock."
                                                                            ],
                                                                        )?;
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
                                                                        } else {
                                                                            if ctx.var("rachel_camel").get()? == 21 {
                                                                                ctx.lines_as(
                                                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                                                    args![
                                                                                        "Making the key is more",
                                                                                        "important that telling Karyn",
                                                                                        "about what's happened."
                                                                                    ],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                if ctx.var("rachel_camel").get()? == 22 {
                                                                                    ctx.lines_as(
                                                                                        ctx.call(
                                                                                            Function::StrCharInfo,
                                                                                            vec![Val::from(0)],
                                                                                        )?,
                                                                                        args![
                                                                                            "I need to bring",
                                                                                            "1 Steel to Mr. Lockenlock",
                                                                                            "so that he can make a key",
                                                                                            "that will finally free Curdie."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    if ctx.var("rachel_camel").get()? == 23 {
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "I finally got the",
                                                                                                "key that I can use",
                                                                                                "to free Curdie. I'm",
                                                                                                "gonna go save her now."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Thank you so much!",
                                                                                                "Please bring back Curdie",
                                                                                                "as soon as you can! ^333333*Sob*^000000"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("rachel_camel").get()? == 24 {
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Hey, Karyn! I sent",
                                                                                                "your sister back to town",
                                                                                                "with a Butterfly Wing.",
                                                                                                "Did she come back safe?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Yes, Curdie's back",
                                                                                                "and she's resting in",
                                                                                                "the hospital right now.",
                                                                                                "Thank you so much for",
                                                                                                "all of your help!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "I'm not sure what's wrong",
                                                                                                "with Curdie, though. Ever",
                                                                                                "since she got back, she gets",
                                                                                                "frightened whenever she",
                                                                                                "sees the soldiers in town."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "I promise to tell my dad",
                                                                                                "about what you did after he",
                                                                                                "comes back from overseas.",
                                                                                                "Thank you for everything",
                                                                                                "that you did for us!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Umm... I see...",
                                                                                                "I'll come by later",
                                                                                                "when Curdie's released",
                                                                                                "from the hospital."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Yes, please do.",
                                                                                                "I promise to help you",
                                                                                                "whenever you need me!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Hahahaha!",
                                                                                                "Well... I guess that's",
                                                                                                "pretty reassuring. Until then,",
                                                                                                "take good care of your mother",
                                                                                                "and sister. You got that?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as("Kid Karyn", args!["Got it!"])?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Good, good...",
                                                                                                "It's a promise, then.",
                                                                                                "I'll see you later~"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.call(
                                                                                            Function::CompleteQuest,
                                                                                            vec![Val::from(3083)],
                                                                                        )?;
                                                                                        ctx.var("rachel_camel").set(Val::from(25))?;
                                                                                        ctx.call(
                                                                                            Function::SpecialEffect,
                                                                                            vec![ctx.constant("EF_ABSORBSPIRITS")?],
                                                                                        )?;
                                                                                        ctx.call(
                                                                                            Function::GetExperience,
                                                                                            vec![Val::from(1000000), Val::from(700000)],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("aru_monas").get()?.number()? < 15 {
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Oh, hello! Thank you",
                                                                                                "so much for helping me",
                                                                                                "last time! When I get",
                                                                                                "older, I'm gonna do my",
                                                                                                "best to help you the same",
                                                                                                "way that you helped me!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("aru_monas").get()? == 15 {
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Hi! It's good to see",
                                                                                                "you again! Kurdi just came",
                                                                                                "back home from the hospital,",
                                                                                                "and is getting better everyday!",
                                                                                                "Ah, and I've been taking care",
                                                                                                "of her, just like I promised~"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Really? That's good",
                                                                                                "You should be proud~"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args!["^666666*Blush*^000000 Heh heh!"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Anyway, I was wondering",
                                                                                                "if you could help me.",
                                                                                                "Your dad's a fisherman,",
                                                                                                "right? Would you mind",
                                                                                                "asking him if I could",
                                                                                                "borrow his boat?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Well... Actually...",
                                                                                                "The pope said that my",
                                                                                                "dad isn't allowed to use",
                                                                                                "his boat for a while...",
                                                                                                "I'm not sure why..."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "So yeah. Dad's not allowed",
                                                                                                "to go out to sea right now.",
                                                                                                "But it should be okay if it's",
                                                                                                "you, right? Hey, you helped",
                                                                                                "me, so I hafta help you, right?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "My dad is really mad that",
                                                                                                "he can't use his boat so",
                                                                                                "maybe we better not tell",
                                                                                                "him you want to use it. Let's",
                                                                                                "keep it our secret, okay?"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args!["S-sure thing!"],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Anyway, his boat is the",
                                                                                                "only one on the south beach",
                                                                                                "since all the other fishermen",
                                                                                                "dock theirs boats in other",
                                                                                                "places. Also, my dad's boat",
                                                                                                "will rust if no one uses it."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            ctx.call(
                                                                                                Function::StrCharInfo,
                                                                                                vec![Val::from(0)],
                                                                                            )?,
                                                                                            args![
                                                                                                "Great! Thanks so much",
                                                                                                "for your help, Karyn~",
                                                                                                "I'll be sure to take",
                                                                                                "good care of your dad's",
                                                                                                "boat. I only need it for",
                                                                                                "a little while, anyway."
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.next()?;
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Heh heh! Thanks!",
                                                                                                "I'm happy that",
                                                                                                "I can help you too!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.var("aru_monas").set(Val::from(16))?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if ctx.var("aru_monas").get()?.number()? < 26 {
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Hi! How do you like",
                                                                                                "fishing on my dad's boat?",
                                                                                                "Oh, and Kurdi says hi!"
                                                                                            ],
                                                                                        )?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else {
                                                                                        ctx.lines_as(
                                                                                            "Kid Karyn",
                                                                                            args![
                                                                                                "Dad says that he'll",
                                                                                                "be able to go fishing",
                                                                                                "again soon! I hope that",
                                                                                                "he'll catch a lot of fish!"
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        ctx.lines_as("Kid Karyn", args!["^333333*Sob*^000000...", "^333333*Sob*^000000..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn kid_camelcamel(ctx: &Ctx) -> Script {
    kid_camelcamel_body(ctx, Vec::new()).map(|_| ())
}

fn little_curdie_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rachel_camel").get()? == 1 {
        ctx.lines(args![
            "^3355FFYou come across",
            "a little girl lying on the",
            "ground unconscious.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hey, kid! Wake up!", "Can you hear me?"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFShe has a pulse, but",
            "despite your verbal",
            "entreaties, she won't",
            "open her eyes. You",
            "lightly slap her cheek",
            "to wake her up.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as("Little Curdie", args!["Huh?!", "...Ah, owwww~", "W-waaaaaaaah!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Sorry! I didn't mean", "to make you cry! Are...", "Are you alright?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Curdie",
            args![
                "Huh? Wh-who are you?",
                "Oh no, you have to get",
                "out of here! You'll be in",
                "trouble if they catch you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Are you Curdie?",
                "Your brother Karyn",
                "asked me to rescue you.",
                "Come on, we've got to",
                "get you out of here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Curdie",
            args![
                "Karyn...?",
                "Oh, oh no! I... They",
                "locked me in these",
                "shackles and I can't move!",
                "You have to leave before",
                "those scary men come back!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What...?!", "Those bastards!", "Tying up a little", "girl like this..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Argh! And I can't just",
                "use brute force to shatter",
                "these shackles! I might",
                "end up hurting you...!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Little Curdie",
            args![
                "Don't worry about me...",
                "Just hurry and leave!",
                "I... I'll be alright! Now",
                "hurry! Someone's coming!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Okay, I'll go...",
                "But sit tight, and",
                "wait for me to come",
                "back. I'll figure out",
                "some way to free you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Little Curdie", args!["^333333*Sob*^000000 R-really...?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I promise.",
                "I'm sure that someone",
                "in town will know of a way",
                "to unlock your shackles.",
                "I'll be back as soon as I can!"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3060), Val::from(3061)])?;
        ctx.var("rachel_camel").set(Val::from(2))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rachel_camel").get()? == 2 {
            ctx.lines(args![
                "^3355FFSomeone in town",
                "must have the",
                "expertise to unlock",
                "these shackles. It's your",
                "only hope to free Curdie",
                "from these chains.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (((ctx.var("rachel_camel").get()? == 3 || ctx.var("rachel_camel").get()? == 6) || ctx.var("rachel_camel").get()? == 8)
                || ctx.var("rachel_camel").get()? == 22)
            {
                ctx.lines(args!["^3355FFCurdie is lying", "feebly on the ground.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rachel_camel").get()? == 4 {
                    ctx.lines_as(
                        "Little Curdie",
                        args!["I hate the metal", "clanging sounds...", "Th-the sparks,", "they're... They're..."],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFCurdie is curled up on",
                        "the ground, eyes tightly",
                        "shut, her entire body",
                        "trembling with fear.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rachel_camel").get()? == 5 {
                        ctx.lines(args![
                            "^3355FFOn the ground, you see",
                            "some equipment that looks",
                            "similarly to that used by",
                            "the Rachel soldiers.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rachel_camel").get()? == 7 {
                            ctx.lines(args![
                                "^3355FFIt seems that someone",
                                "has come by to give",
                                "Curdie food and water.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rachel_camel").get()? == 9 {
                                ctx.lines(args![
                                    "^3355FFCurdie squints at you",
                                    "as you walk by. It seems",
                                    "that her vision gets worse",
                                    "the longer she's locked",
                                    "up in this cave."
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("rachel_camel").get()? == 10 {
                                    ctx.lines(args![
                                        "^3355FFYou'd better find the",
                                        "Silk Sand Camel and get",
                                        "the soap ingredients if",
                                        "you really want to free",
                                        "Curdie from her shackles.^000000"
                                    ])?;
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
                                        if (ctx.var("rachel_camel").get()?.number()? > 11 && ctx.var("rachel_camel").get()?.number()? < 17)
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
                                                    ctx.lines(args![
                                                        "^3355FFYou should be leaving",
                                                        "to see Ms. Ivory now if",
                                                        "you really want to free",
                                                        "Curdie from her shackles.^000000"
                                                    ])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rachel_camel").get()? == 19 {
                                                    ctx.lines(args![
                                                        "^3355FFNow that you have the",
                                                        "Silk Sand Camel Soap,",
                                                        "you should bring it",
                                                        "over to Mr. Lockenlock.^000000"
                                                    ])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rachel_camel").get()? == 20 {
                                                    ctx.lines(args![
                                                        "^3355FFCurdie is exactly",
                                                        "where you left her.",
                                                        "There's a bowl of cold",
                                                        "soup next to her, so it's",
                                                        "clear that someone has",
                                                        "been feeding her.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou pour the soap into",
                                                        "the shackle's lock to create",
                                                        "a mold that Mr. Lockenlock",
                                                        "can use to make a key.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args!["Will...", "Will I always", "be stuck here?", "I... I want my mommy..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Oh... You're awake?",
                                                            "Don't worry, Curdie,",
                                                            "I'm sure that I'll be",
                                                            "able to get you free soon.",
                                                            "Try to hold on a bit longer."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args![
                                                            "When the door is open,",
                                                            "I see blazing flames...",
                                                            "And I hear... the sound",
                                                            "of machines? These men",
                                                            "wearing the same clothes",
                                                            "keep marching past me..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args![
                                                            "I... They scare me so much!",
                                                            "Th-the man that brings me",
                                                            "food says that they won't",
                                                            "let me go because of what",
                                                            "I saw inside there. They...",
                                                            "They won't let be go home..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFCurdie's eyes are",
                                                        "disfocused and are",
                                                        "pointed above your head.",
                                                        "She might not survive if",
                                                        "she's forced to remain",
                                                        "here for much longer.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Don't worry.",
                                                            "I'll come rescue",
                                                            "you as soon as I make",
                                                            "the key to unlock these",
                                                            "awful shackles. Don't worry..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args![
                                                            "If I'm not here the",
                                                            "next time you come,",
                                                            "then just run away.",
                                                            "Don't even tell my brother.",
                                                            "Get far away before they",
                                                            "can catch you. I-I'm serious..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args![
                                                            "Even if I don't see you",
                                                            "again... I just... I just",
                                                            "want to thank you for doing",
                                                            "your best to help me."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Everything will be",
                                                            "alright. I just have",
                                                            "to hurry a little bit.",
                                                            "Alright, it's time to go."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou extract the soap",
                                                        "from the lock, and",
                                                        "carefully wrap it.",
                                                        "Now you need to take",
                                                        "the mold back to town",
                                                        "to Mr. Lockenlock.^000000"
                                                    ])?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3079), Val::from(3080)])?;
                                                    ctx.var("rachel_camel").set(Val::from(21))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rachel_camel").get()? == 21 {
                                                    ctx.lines(args![
                                                        "^3355FFYou have to hurry back",
                                                        "to town and bring the",
                                                        "key mold to Mr. Lockenlock",
                                                        "so that he can make a key",
                                                        "to unlock Curdie's shackles.^000000"
                                                    ])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rachel_camel").get()? == 23 {
                                                    ctx.lines_as("Little Curdie", args!["Y-you...", "Is it really you?"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args!["Hang on, Curdie!", "I hope this key works..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou unlock the",
                                                        "shackles with",
                                                        "Mr. Lockenlock's key.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.mes("^333333*Crack*^000000")?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Little Curdie", args!["Aaaah!", "M-my... My...!"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFCurdie's legs are",
                                                        "swollen from the",
                                                        "weight and pressure",
                                                        "of wearing the shackles",
                                                        "for such a long time.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Little Curdie", args!["I can't move my legs!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "Well, there's no other",
                                                            "choice. Curdie, I'm",
                                                            "going to send you back to",
                                                            "town with a Butterfly Wing.",
                                                            "Try not to move, alright?"
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Little Curdie",
                                                        args!["Oh! Thank you...", "I... I can go home...", "Thank you s-so much..."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFYou use the power",
                                                        "of a Butterfly Wing to",
                                                        "send Curdie back to",
                                                        "town. Hopefully, she'll",
                                                        "arrive safely and see",
                                                        "her brother Karyn again.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                        args![
                                                            "What did she see behind",
                                                            "the steel door in this old",
                                                            "volcano? It must have been",
                                                            "dangerous... Something",
                                                            "related to the Rachel Army..."
                                                        ],
                                                    )?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3082), Val::from(3083)])?;
                                                    ctx.var("rachel_camel").set(Val::from(24))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as("Little Curdie", args![" ...Wah...."])?;
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

pub fn little_curdie(ctx: &Ctx) -> Script {
    little_curdie_body(ctx, Vec::new()).map(|_| ())
}
