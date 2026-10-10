use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn ibrahim_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Ibrahim",
            args![
                "You're carrying too much",
                "stuff with you. Dump it",
                "in Kafra Storge, sell it,",
                "drop it, whatever, before",
                "you come talk to me, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()?.number()? < 14 {
        ctx.lines_as(
            "Ibrahim",
            args![
                "Have you heard of the",
                "Four Cursed Jewels?",
                "I hear that one of them,",
                "the Diamond of Destruction,",
                "just appeared recently.",
                "What I'd do to find it..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()? == 14 && ctx.var("BaseLevel").get()?.number()? > 65) {
        ctx.lines_as(
            "Ibrahim",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?")),
                "Don't be too suprised.",
                "I've heard of you here",
                "and there. Heh heh~!"
            ],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("How can I help you?:What'd you hear?:Bye!")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                        && !subject2.loosely_equals(&Val::from(2))
                        && !subject2.loosely_equals(&Val::from(3));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "I wanted to ask you about",
                                "the Four Cursed Jewels.",
                                "All the merchants in the",
                                "area are talking about them.",
                                "Anyway, I was able to obtain",
                                "the Diamond of Destruction!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "(^666666Hmm... He might",
                                "be talking about that",
                                "jewel Muff gave to Belder",
                                "as collateral for his debt.^000000)"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "I paid a hefty sum for it,",
                                "but then I lost it after",
                                "just a few days. I kept it",
                                "under high security, but",
                                "that didn't stop the thief..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "It's a risky job, but",
                                "I want to employ you to",
                                "find where my diamond went.",
                                "It'd be best if you could get",
                                "it back, but I can understand",
                                "if you can't. You up for it?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure, I'll help you.:No, sorry.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "Great! I knew it was",
                                        "a good idea to find you.",
                                        "The thief didn't leave any",
                                        "clues, but he should have",
                                        "some problems selling the gem.",
                                        "It's one of a kind, you know."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "Visiting the large",
                                        "jewelry shops in the",
                                        "Rune-Midgarts Kingdom",
                                        "would be a good start.",
                                        "Their merchants know the",
                                        "most about those jewels."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "The thief will also have",
                                        "to sell the jewel to a jeweler",
                                        "that can afford astronomical",
                                        "prices. There are three big",
                                        "locations you should check."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "Find ^0000FFJhonnita^000000 in Alberta,",
                                        "^0000FFKimeunbang^000000 in Payon, and",
                                        "^0000FFLeblo^000000 in Geffen. They wanted ",
                                        "that diamond badly too, so",
                                        "they'll remember if they",
                                        "heard any clues about it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "Please don't let them",
                                        "know I've hired you to",
                                        "investigate that diamond.",
                                        "They won't speak to you",
                                        "if they figure out that",
                                        "I've sent you to them."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ibrahim",
                                    args!["Well, I'll be right", "here. I hope you come", "back with some good news."],
                                )?;
                                ctx.call(Function::SetQuest, vec![Val::from(3110)])?;
                                ctx.var("diamond_edq").set(Val::from(15))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "Really? I went through",
                                        "a lot of trouble to find",
                                        "you, but... I understand.",
                                        "If you have a change of",
                                        "heart, I'll be willing to",
                                        "hire you for your expertise."
                                    ],
                                )?;
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
                        if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                        {
                            ctx.lines_as(
                                "Ibrahim",
                                args![
                                    "So it's true...",
                                    "People say you're",
                                    "just a Novice, but",
                                    "you still help them",
                                    "out with their problems."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
                                ctx.lines_as(
                                    "Ibrahim",
                                    args![
                                        "I've got a few buddies",
                                        "in the Swordman Association.",
                                        "They dropped your name when",
                                        "I mentioned I was looking",
                                        "for someone smart and strong."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
                                    ctx.lines_as(
                                        "Ibrahim",
                                        args![
                                            "Heh, so you really",
                                            "are one of those holy",
                                            "knights. I'm glad to",
                                            "have met you for myself."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                                        ctx.lines_as(
                                            "Ibrahim",
                                            args![
                                                "I didn't think a member",
                                                "of the clergy would be so",
                                                "strong, but you know, your",
                                                "reputation precedes you."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?) {
                                            ctx.lines_as(
                                                "Ibrahim",
                                                args![
                                                    "Hey, if I didn't know",
                                                    "your name, how could I call",
                                                    "myself a Merchant? You set",
                                                    "an example for us all~"
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?) {
                                                ctx.lines_as(
                                                    "Ibrahim",
                                                    args![
                                                        "We have a mutual friend,",
                                                        "you know that? Aragham",
                                                        "really spoke highly of you."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) {
                                                    ctx.lines_as(
                                                        "Ibrahim",
                                                        args![
                                                            "I know a guy who knows",
                                                            "a guy in the Alchemist Guild.",
                                                            "Around their parts, it sounds",
                                                            "like you're pretty hot stuff."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.var("job_magician").get()?) {
                                                        ctx.lines_as(
                                                            "Ibrahim",
                                                            args![
                                                                "Magic is totally beyond",
                                                                "my understanding, but",
                                                                "if it gets the job done,",
                                                                "it can't be all bad.",
                                                                "I hear you're really",
                                                                "good at that stuff."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                                                            ctx.lines_as(
                                                                "Ibrahim",
                                                                args![
                                                                    "Heh, it sounds like almost",
                                                                    "anyone who's anyone in",
                                                                    "Morocc knows who you are.",
                                                                    "Pleasure to make your",
                                                                    "acquaintance~"
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?) {
                                                                ctx.lines_as(
                                                                    "Ibrahim",
                                                                    args![
                                                                        "Anyone who's used",
                                                                        "a bow seems to drop",
                                                                        "your name when it comes",
                                                                        "to accuracy and heroism.",
                                                                        "Hard to believe that I'm",
                                                                        "actually talking to you~"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                                            {
                                                                ctx.lines_as(
                                                                    "Ibrahim",
                                                                    args!["Is it really true?", "I hear you sing", "like an angel..."],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx
                                                                .var("BaseClass")
                                                                .get()?
                                                                .loosely_equals(&ctx.constant("JOB_DANCER")?)
                                                            {
                                                                ctx.lines_as(
                                                                    "Ibrahim",
                                                                    args!["Is it really true?", "I hear you dance", "like the devil..."],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if (ctx
                                                                .var("Class")
                                                                .get()?
                                                                .loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?)
                                                                || ctx
                                                                    .var("Class")
                                                                    .get()?
                                                                    .loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR2")?))
                                                            {
                                                                ctx.lines_as(
                                                                    "Ibrahim",
                                                                    args![
                                                                        "Me and your old master,",
                                                                        "Phoenix, we go way back.",
                                                                        "He had nothing but pride",
                                                                        "when he talked about you~"
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx
                                                                .var("Class")
                                                                .get()?
                                                                .loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?)
                                                            {
                                                                ctx.lines_as(
                                                                    "Ibrahim",
                                                                    args![
                                                                        "I heard your name from",
                                                                        "this spiritual message",
                                                                        "I got from a dream. That",
                                                                        "doesn't sound crazy to",
                                                                        "you, does it?"
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
                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "Aren't you the least",
                                "bit curious how I know",
                                "about you? Wait, come back!",
                                "I'm not really that suspicious!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    if ctx.var("diamond_edq").get()? == 15 {
        ctx.lines_as(
            "Ibrahim",
            args![
                "Find ^0000FFJhonnita^000000 in Alberta,",
                "^0000FFKimeunbang^000000 in Payon, and",
                "^0000FFLeblo^000000 in Geffen. They wanted ",
                "that diamond badly too, so",
                "they'll remember if they",
                "heard any clues about it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ibrahim",
            args![
                "Please don't let them",
                "know I've hired you to",
                "investigate that diamond.",
                "They won't speak to you",
                "if they figure out that",
                "I've sent you to them."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()?.number()? > 15 && ctx.var("diamond_edq").get()?.number()? < 25) {
        ctx.lines_as(
            "Ibrahim",
            args![
                "Are you still investigating",
                "the diamond's whereabouts?",
                "Well, I hope you can bring",
                "me back good news soon."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 25 {
        ctx.lines_as(
            "Ibrahim",
            args![
                "Oh, you're back.",
                "Made any headway in",
                "your investigation of the",
                "Diamond of Destruction?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Explain What You Learned")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Ibrahim",
            args![
                "What?! The Z Gang stole",
                "my diamond? No wonder!",
                "I bet they wanted to use its",
                "curse for something evil.",
                "How about the jewel?",
                "Did you retrieve it?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Give Diamond of Destruction")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Ibrahim",
            args![
                "Oh! Great! So it was",
                "in the Z Gang's hideout?",
                "Wow, even after hearing so",
                "much about you, you still",
                "exceeded my expectations!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7723), Val::from(1)])?;
        ctx.lines_as(
            "Ibrahim",
            args![
                "Thank God, I finally",
                "have this back! Ah, and",
                "as for your payment...",
                "Well, I don't know if",
                "it's enough for what you",
                "did, but I hope you like it."
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(3118)])?;
        ctx.var("diamond_edq").set(Val::from(26))?;
        ctx.call(Function::GetItem, vec![Val::from(732), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Ibrahim",
            args![
                "Now that I have the",
                "Diamond of Destruction",
                "again, I can research",
                "more about its strange",
                "properties. I'll contact you",
                "again if I need your help."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 26 {
        if ctx.var("jewel_nd").get()? == 0 {
            if (ctx.var("treasure_nd").get()?.number()? > 10 && ctx.var("BaseLevel").get()?.number()? > 65) {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Ah, I hear from a friend",
                        "in Morocc that you happened",
                        "to obtain the Unlucky Emerald.",
                        "I'm sure it's one of the Four",
                        "Cursed Jewels. You wouldn't",
                        "mind if I look at it, do you?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No!:Sure, why not?")])? {
                    1 => {
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "No? Well, I'm sure you",
                                "have your reasons. Still,",
                                "maybe I can learn something",
                                "that would benefit both of",
                                "us if I examined it."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Ibrahim", args!["Ah, I appreciate it!", "Now, let's see here..."])?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                            ctx.lines_as(
                                "Ibrahim",
                                args![
                                    "Yes, judging from its",
                                    "characteristic shape, this",
                                    "is the cursed emerald.",
                                    "Like me, you now possess",
                                    "one of the Four Cursed Jewels."
                                ],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as(
                                "Ibrahim",
                                args!["Hm? Did you happen to", "misplace your emerald?", "I mean, it's not with you..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Ibrahim",
                    args!["Tell me, what were", "you planning to do with", "the Unlucky Emerald?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I'm going to sell it.:I'm going to study it.")])? {
                    1 => {
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "Oh, you weren't going",
                                "to keep it? I wish I could",
                                "afford to buy it from you...",
                                "But I've had enough trouble",
                                "buying the Diamond of",
                                "Destruction."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "You already met several",
                                "renown jewelers while",
                                "you were looking for the",
                                "dimaond. Why don't you ask",
                                "them if they'll buy that",
                                "Unlucky Emerald from you?"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(1))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "Ah, I applaud your",
                                "thirst for knowledge,",
                                "your quest to seek truth.",
                                "If you really want an expert",
                                "to look at the emerald, talk",
                                "to this scholar in Comodo."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ibrahim",
                            args![
                                "This man is an antique",
                                "appraiser, an specialist in",
                                "ancient artifacts and treasure.",
                                "I highly recommend consulting",
                                "him since he's been invaluable",
                                "in my own jewel research."
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(10))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else if (ctx.var("treasure_nd").get()?.number()? > 10 && ctx.var("BaseLevel").get()?.number()? < 66) {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "I heard that you obtained",
                        "the Unlucky Emerald from",
                        "a friend in Morocc. Let me",
                        "warn you that you might not",
                        "be strong enough to handle",
                        "its power. Be careful."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ibrahim",
                    args!["If you're not wary,", "its curse might destroy", "you. Understand?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Thank you for all",
                        "you've done for me. I'll",
                        "continue my research in the",
                        "cursed jewel's powers, and",
                        "I'll contact you if I need your",
                        "help again. Take care~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("jewel_nd").get()? == 1 {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "I don't have the funds",
                        "to buy your emerald at",
                        "a fair price, but why don't",
                        "you ask the other jewelers?",
                        "They might be interested",
                        "in buying your jewel."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("jewel_nd").get()? == 2 {
                ctx.lines_as(
                    "Ibrahim",
                    args!["Ah, you're back~", "So were you able to", "sell the Unlucky Emerald?"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Everyone seems scared!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Ah, I see now. They",
                        "must be too afraid of",
                        "that particular jewel's",
                        "curse. Huh. Weird."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "I guess the Diamond of",
                        "Destruction is considered",
                        "pretty safe since it's been",
                        "around for a while without",
                        "any strange incidents."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "You just dug up the",
                        "emerald? I suppose it",
                        "might be too soon to sell it.",
                        "Not too many people will",
                        "test their luck with that gem,",
                        "now that I think about it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Ah, here's an idea.",
                        "Why don't you ask my",
                        "scholar friend in Comodo",
                        "to examine it? Maybe you",
                        "can learn more about the",
                        "emerald's so-called curse."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "I'll go ahead and send",
                        "him a message ahead of",
                        "time so that he'll know to",
                        "expect your arrival, okay?"
                    ],
                )?;
                ctx.var("jewel_nd").set(Val::from(9))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("jewel_nd").get()? == 9 || ctx.var("jewel_nd").get()? == 10) {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Why don't you visit my",
                        "scholar friend in Comodo,",
                        "and ask him to examine your",
                        "Unlucky Emerald? The more",
                        "you learn about that so-called",
                        "curse, the better. Good luck!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Ibrahim",
                    args![
                        "Oh, so what have you",
                        "learned about the Unlucky",
                        "Emerald so far? Hmmm. Yes,",
                        "very intriguing. Good luck",
                        "with your research efforts~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as(
        "Ibrahim",
        args![
            "Have you heard of",
            "the Four Cursed Jewels?",
            "I wish to retrieve my",
            "Diamond of Destruction,",
            "but whoever has it has",
            "kept it well hidden..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ibrahim(ctx: &Ctx) -> Script {
    ibrahim_body(ctx, Vec::new()).map(|_| ())
}

fn ibrahim_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 14 && ctx.var("BaseLevel").get()?.number()? > 65) {
        ctx.lines_as(
            "Ibrahim",
            args![
                "Psst, adventurer!",
                ((Val::from("You're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", right?")),
                "There's something",
                "I'd like to discuss",
                "with you in private!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn ibrahim_ontouch(ctx: &Ctx) -> Script {
    ibrahim_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn jhonnita_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 15 || ctx.var("diamond_edq").get()? == 16) {
        ctx.lines_as(
            "Jhonnita",
            args![
                "Oh, it's always nice",
                "to see a new face~",
                "So were you interested",
                "in selling or buying",
                "a jewel? We only carry",
                "the best. Hahahaha!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("What's the most popular jewel?:Do you know Diamond of Destruction?")],
        )? {
            1 => {
                ctx.lines_as(
                    "Jhonnita",
                    args![
                        "Ah, take a look at this",
                        "lustrous ruby. Isn't it",
                        "just breathtaking? It can",
                        "be yours for the specially",
                        "discounted price of just",
                        "1,000,000 zeny!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jhonnita",
                    args![
                        "Wait, where are you--?",
                        "Damn, I shouldn't have",
                        "marked it up so much.",
                        "Rookie mistake, rookie",
                        "mistake! I'm a professional!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Jhonnita",
                    args![
                        "Diamond of Destruction?",
                        "Heh heh! A rookie like you",
                        "wouldn't appreciate its true",
                        "value. I wanted it so badly,",
                        "but that Ibrahim was much",
                        "luckier that I was..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jhonnita",
                    args![
                        "It makes me so upset to",
                        "think about how I lost to",
                        "him. All the big jewelers",
                        "are trying to snatch up",
                        "the Four Cursed Jewels...",
                        "But only one showed up so far."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("diamond_edq").get()? == 26 {
        if ctx.var("jewel_nd").get()? == 0 {
            ctx.lines_as(
                "Jhonnita",
                args![
                    "Hey. You look familiar.",
                    "You're not one of my",
                    "regulars, are you?",
                    "No, couldn't be.",
                    "You're not dressed",
                    "richly enough. No offense."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 1 {
            ctx.lines_as(
                "Jhonnita",
                args![
                    "So you here to sell",
                    "any Garlets, or did you",
                    "come to spend your life",
                    "savings on my jewels?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Show me some jewels...:I have a rare emerald to sell...")])? {
                1 => {
                    ctx.lines_as(
                        "Jhonnita",
                        args![
                            "It couldn't hurt to",
                            "show you my wares, but...",
                            "I doubt you can afford them.",
                            "Jewels aren't cheap, you know?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines_as(
                            "Jhonnita",
                            args![
                                "Oh, I get it now. You want",
                                "to give one as a present to",
                                "some woman. Isn't that typical?",
                                "Can't you come up with a more",
                                "creative way to buy some",
                                "woman's love? Huh?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Jhonnita",
                            args!["Sorry, sweetheart, but", "I have a strict ''no window", "shopping'' policy. "],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                        ctx.lines_as(
                            "Jhonnita",
                            args![
                                "Rare emerald, eh?",
                                "It better not be something",
                                "like a Garlet or a Zargon!",
                                "Sometimes you guys make",
                                "that kind of mistake.",
                                "Okay, let's see..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Jhonnita", args!["......", "Umm..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jhonnita",
                            args![
                                "...........................Um.",
                                "It's an incredibly nice",
                                "emerald, but I can't take",
                                "this off your hands. Er,",
                                "would you be on your way?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh? But why?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jhonnita",
                            args![
                                "J-just get out of my",
                                "shop! Go... Go to Payon",
                                "and talk to Kimeunbang!",
                                "Y-yeah, and don't come back!"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Jhonnita",
                            args!["What'd I tell you, huh?", "I don't want you here", "in my shop anymore!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                _ => {}
            }
        } else if ctx.var("jewel_nd").get()? == 2 {
            ctx.lines_as(
                "Jhonnita",
                args![
                    "Look, I'll level with you.",
                    "Rumors spread fast amongst",
                    "us jewelers. Nobody will get",
                    "near you if they know you",
                    "have that jewel. You get it",
                    "now? Please... Stay back..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Jhonnita",
                args![
                    "I don't want anything",
                    "to do with your weird",
                    "emerald or with you.",
                    "Aren't you leaving",
                    "already? Go away!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Jhonnita",
            args![
                "Are you here to buy or",
                "sell any jewels? I always",
                "buy high and sell low.",
                "Come to me for all of",
                "your gem related needs~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn jhonnita(ctx: &Ctx) -> Script {
    jhonnita_body(ctx, Vec::new()).map(|_| ())
}

fn kimeunbang_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("diamond_edq").get()? == 15 {
        ctx.lines_as(
            "Kimeunbang",
            args![
                "Zzzz... Huh?",
                "Why'd you wake me?",
                "Such a good dream too.",
                "Go to another jewelry shop,",
                "I don't feel like opening up."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("What's wrong?:Diamond of Destruction?")])? {
            1 => {
                ctx.lines_as(
                    "Kimeunbang",
                    args![
                        "I'm just disappointed.",
                        "I was so close to getting",
                        "that Diamond of Destruction.",
                        "So close! Belder, my business",
                        "partner, actually snatched",
                        "it up from some poor sap."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kimeunbang",
                    args![
                        "Somehow, Belder ended up",
                        "returning the jewel. I was",
                        "going to buy the jewel from",
                        "the original owner, but then",
                        "this upstart jeweler got to",
                        "it before I could. Damn it!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kimeunbang",
                    args![
                        "This was a monumental",
                        "failure. I haven't been",
                        "able to sleep, I don't want",
                        "to work. Ugh. Well, thanks",
                        "for letting me get that off",
                        "my chest. I feel a bit better."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kimeunbang",
                    args![
                        "Here, take this as",
                        "a little gift. Not all of us",
                        "jewelers are as greedy",
                        "as you think we are. Heh."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(722), Val::from(1)])?;
                ctx.var("diamond_edq").set(Val::from(16))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kimeunbang",
                    args![
                        "Oh! That diamond is the",
                        "very reason for my agony!",
                        "I was so close to getting it...",
                        "But then someone got to the",
                        "diamond before I did. Argh!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("diamond_edq").get()? == 26 {
        if ctx.var("jewel_nd").get()? == 0 {
            ctx.lines_as(
                "Kimeunbang",
                args![
                    "I don't feel like opening",
                    "my shop any time soon, but",
                    "you can always come back later."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 1 {
            ctx.lines_as("Kimeunbang", args!["Oh, hello. Looks like", "I'm back in business~"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I have a jewel to sell.:How are you?")])? {
                1 => {
                    ctx.lines_as(
                        "Kimeunbang",
                        args!["Really? Alright, just", "give me a minute to", "appraise your gem."],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                        ctx.lines_as("Kimeunbang", args!["...........", "..........."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kimeunbang",
                            args![
                                "I'm sorry, but I just",
                                "can't buy this emerald.",
                                "It's... It's too big of",
                                "a risk. The rumors I keep",
                                "hearing, they're just horrible."
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Kimeunbang",
                            args![
                                "Hm? Where's this",
                                "jewel you were talking",
                                "about? Did you forget",
                                "to bring it with you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Kimeunbang",
                        args![
                            "I don't even feel like",
                            "talking about my business.",
                            "I've been considering closing",
                            "up shop for good, actually."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("jewel_nd").get()? == 2 {
            ctx.lines_as(
                "Kimeunbang",
                args![
                    "No jeweler around will",
                    "buy that cursed emerald.",
                    "I'm really sorry. You can",
                    "try talking to Ibrahim, though.",
                    "He's the one that got that",
                    "Diamond of Destruction."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Kimeunbang",
                args![
                    "Ugh... The stress",
                    "of the jewel business",
                    "is finally getting to me.",
                    "Oh well, I've had a good run..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Kimeunbang",
            args![
                "I don't feel like opening",
                "my shop any time soon, but",
                "you can always come back later."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn kimeunbang(ctx: &Ctx) -> Script {
    kimeunbang_body(ctx, Vec::new()).map(|_| ())
}

fn leblo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 15 || ctx.var("diamond_edq").get()? == 16) {
        ctx.lines_as(
            "Leblo",
            args![
                "Oh, I've heard about",
                "you. You're the one that's",
                "been asking questions about",
                "the Diamond of Destruction."
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            "Leblo",
            args![
                "Don't be so suprised:",
                "word travels fast in the",
                "jewel industry. Well, if you",
                "want to learn more, you'll",
                "have to do something for",
                "me first. We have a deal?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Deal.:No, thanks.")])? {
            1 => {
                ctx.lines_as(
                    "Leblo",
                    args![
                        "I've been suffering back",
                        "pain lately, and none of",
                        "the medicines do any good.",
                        "I resorted to seeing this",
                        "famous doctor in Payon, but",
                        "then I totally screwed it up."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Leblo",
                    args![
                        "She was such a nag, and",
                        "I said something... unpleasant",
                        "to her. Now she refuses to",
                        "examine me! But if I sent",
                        "you, then you could get",
                        "some medicine for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Leblo",
                    args![
                        "She knows my symptoms",
                        "so if you make her happy,",
                        "she'll give you my medicine...",
                        "I hope. Anyway, her name is",
                        "^0000FFWola^000000, and she's in Payon."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3110), Val::from(3111)])?;
                ctx.var("diamond_edq").set(Val::from(17))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Leblo",
                    args!["Oh, yeah? Well, that's", "fine with me. I don't", "have anything to lose."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("diamond_edq").get()? == 17 {
        ctx.lines_as(
            "Leblo",
            args![
                "Please look for a female",
                "doctor named ^0000FFWola^000000 in Payon.",
                "Do what you can to get my",
                "medicine from her... Oh,",
                "and try to be nice~ "
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 21 {
        ctx.lines_as(
            "Leblo",
            args![
                "Oh? Is that... Is that my",
                "medicine? Thank goodness!",
                " Hm? Oh, so I misunderstood",
                " her. I thought she was awfully",
                " arrogant, but she was just",
                " concerned for my health."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leblo",
            args![
                "I better make it a point",
                "to properly apologize to",
                "her the next time I see her."
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
            "Leblo",
            args![
                "Right, I need to tell",
                "you about the Diamond of",
                "Destruction. Listen carefully.",
                "A while ago, a young couple",
                "tried to sell me that very",
                "jewel. Suspicious, eh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leblo",
            args![
                "I had no interest in",
                "buying it, so I actually",
                "offered them a much lower",
                "price than they hoped.",
                "Now that I think about it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leblo",
            args![
                "That man and woman looked",
                "like the members of that",
                "infamous Z Gang. Heh!",
                "I should have reported it,",
                "but... You know. Back",
                "pain and all that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leblo",
            args![
                "I hear the Rogue Guild",
                "is secretly investigating",
                "the Z Gang, so that couple",
                "must have plenty of enemies.",
                "So... Yeah. The Z Gang has",
                "the Diamond of Destruction."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Leblo",
            args![
                "You should talk with one",
                "of the investigators of the",
                "Rogue Guild if you want to",
                "learn more about the Z Gang.",
                "Pretty useful info, huh?",
                "Take care, my friend."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3114), Val::from(3115)])?;
        ctx.var("diamond_edq").set(Val::from(22))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 22 {
        ctx.lines_as(
            "Leblo",
            args![
                "You should talk with one",
                "of the investigators of the",
                "Rogue Guild if you want to",
                "learn more about the Z Gang.",
                "Pretty useful info, huh?",
                "Take care, my friend."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 26 {
        if ctx.var("jewel_nd").get()? == 0 {
            ctx.lines_as(
                "Leblo",
                args![
                    "Hm? I'm sorry, but",
                    "I'm pretty busy with",
                    "all my other customers.",
                    "Besides, you don't strike",
                    "me as a jewel seller or",
                    "buyer. Please excuse me..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("jewel_nd").get()? == 1 {
            ctx.lines_as("Leblo", args!["Oh, hey. Did you", "have any business", "with me today?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I have a jewel to sell.:No, not really.")])? {
                1 => {
                    ctx.lines_as(
                        "Leblo",
                        args!["Oh, yeah? Well then,", "I'm curious now. You", "mind if I take a look?"],
                    )?;
                    ctx.next()?;
                    if ctx.call(Function::CountItem, vec![Val::from(7725)])?.number()? > 0 {
                        ctx.lines_as(
                            "Leblo",
                            args![
                                "Now, what do we ha--",
                                "Oh. I'm sorry, but I just",
                                "can't buy this. I hate to",
                                "break it to you, but I hear",
                                "that thing has an awful curse.",
                                "See what you can do about that!"
                            ],
                        )?;
                        ctx.var("jewel_nd").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Leblo",
                            args![
                                "Hmm. The jewel",
                                "you were just",
                                "talking about...",
                                "Did you remember",
                                "to bring it with you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Leblo",
                        args![
                            "Oh, yeah? Well, that's",
                            "fine too. No skin off",
                            "my nose, that's what",
                            "I always say."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("jewel_nd").get()? == 2 {
            ctx.lines_as(
                "Leblo",
                args![
                    "Oh. I'm sorry, but I just",
                    "can't buy this. I hate to",
                    "break it to you, but I hear",
                    "that thing has an awful curse.",
                    "See what you can do about that!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Leblo",
                args![
                    "I'm sorry that you're",
                    "the one that currently",
                    "has that jewel. Hopefully,",
                    "the curse is just a rumor...",
                    "But, well, I didn't get this",
                    "old by taking chances..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Leblo",
        args!["Hello, are you interested", "in buying or selling any", "precious jewels?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn leblo(ctx: &Ctx) -> Script {
    leblo_body(ctx, Vec::new()).map(|_| ())
}
