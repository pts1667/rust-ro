use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn old_scholar_tyus_hellion_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("hellionq").get()?.number()? < 36 {
        ctx.lines_as(
            "Sir Chilias'Tyus",
            args![
                "Greetings...",
                "My name is Sir Chilias'Tyus.",
                "I've lived a long time here in",
                "Rune-Midgarts and I've come",
                "to see and know a lot of things. Power, jealously, hardship..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So what exactly", "brings you here to", "the middle of Morocc?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Chilias'Tyus",
            args![
                "This land has grown corrupt",
                "with the diseases of greed",
                "and selfishness. Everywhere",
                "you go, people are heartless,",
                "but I still believe that I'll find the one worthy enough to help me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sir Chilias'Tyus",
            args![
                "There is no shortage of",
                "brave and strong warriors",
                "in these dangerous times.",
                "But for the thing I seek,",
                "I must have the help of one",
                "who is truly pure of heart..."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("What are you looking for?:Pure of heart? Hah! Good luck!")],
        )?) == 1
        {
            if ctx.var("BaseLevel").get()?.number()? < 60 {
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "Heh heh~ I see your eyes",
                        "glimmer with enthusiasm,",
                        "but it seems this task might",
                        "be a little too much for you.",
                        "I'm sorry, but that's the",
                        "reality I can't ignore..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "Go out in to the world,",
                        "and grow in strength.",
                        "Perhaps after you become",
                        "more capable, I'll find that",
                        "I can rely on your help."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "I suppose I should tell you",
                    "first about my grandfather,",
                    "who was once a young and",
                    "feareless adventurer, much",
                    "like yourself. Yes, he traveled the world with his three friends..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "The four of them went on",
                    "many expeditions together",
                    "and earned themselves a",
                    "reputation as a courageous",
                    "team that worked especially",
                    "well together after many years."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "Then, on what was supposed",
                    "to be a typical journey, they found an uncharted cave. Curious, and",
                    "in need of rest, the four ventured inside. There, they encountered",
                    "that most horrible creature..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "It was a ferocious beast",
                    "with horns as large and as",
                    "sharp as their weapons, with",
                    "eyes that blazed with hellfire.",
                    "My grandfather could tell that",
                    "it was thirsty for blood."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "They ran from the beast",
                    "as quickly as they could,",
                    "but the four were separated",
                    "in the cave's darkness. It was",
                    "all they could do to hide in",
                    "the darkness of the cave..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "From his hiding place, my",
                    "grandfather heard the screams",
                    "of his friends as the monster",
                    "slaughtered them, one by one.",
                    "The screams and the monster's",
                    "growls were getting closer..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "Unable to endure the",
                    "suffering of his trusted",
                    "comrades, my grandfather",
                    "jumped out hiding and tried",
                    "to save his friends. The beast",
                    "found him and they battled."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "It was no use. The beast",
                    "was too powerful and my",
                    "grandfather was knocked out.",
                    "When he came to, his friends",
                    "were all dead and a shining",
                    "gem lay in a pool of blood."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "My grandfather buried",
                    "his friends that day. He",
                    "took that gem as a memento",
                    "of their final expedition, but",
                    "wondered why the beast",
                    "had decided to spare him."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "It wasn't long before he",
                    "experienced the gem's curse",
                    "firsthand. The gem inexplicably",
                    "drew people to it, taking the",
                    "greed in their hearts and",
                    "twisting their minds..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "Grandfather believed the",
                    "beast let him live so that",
                    "the gem would be set loose",
                    "upon the outside world, free",
                    "to pollute it with its evil. He had to do something..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "However, the gem proved",
                    "to be formed out of pure evil",
                    "and could not be destroyed",
                    "by human means. In the end,",
                    "my grandfather hid the gem",
                    "someplace safe..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "However, the threat of the",
                    "gem still exists. Grandfather",
                    "and my father worked for years to find a way to seal its darkness.",
                    "Only now have I been able to create a bracelet to seal its evil power."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "For my grandfather and his",
                    "friends to truly rest in peace,",
                    "I must find an adventurer of",
                    "pure heart who can find the",
                    "gem and bring it to me so that",
                    "I can finally seal its power."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "I think... I think you",
                    "might be the hero that",
                    "I've been waiting for. Will",
                    "you help an old man for the",
                    "sake of all that is good?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Of course, I will.:I'm sorry, but I can't.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "Good, good, but first",
                        "I must test to see if you",
                        "are worthy of holding the",
                        "Hellion's gem. I can't tell",
                        "you exactly where it is, but",
                        "I will give you some clues."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "First, speak to Clanux",
                        "Heffron in Prontera. He",
                        "has offered to help me find",
                        "the gem, but I do not know if",
                        "I can trust him. Please help me protect this world from the gem..."
                    ],
                )?;
                ctx.var("hellionq").set(Val::from(36))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "I understand...",
                    "I know I am asking much",
                    "of you, and this task is not",
                    "one to take lightly. However,",
                    "I believe that you are wholly",
                    "capable of accomplishing this."
                ],
            )?;
            ctx.var("hellionq").set(Val::from(37))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Sir Chilias'Tyus",
            args![
                "Times are bad when",
                "even the youth have",
                "been jaded. But I refuse",
                "to believe that hope is",
                "truly lost in this age..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hellionq").get()? == 36 {
            ctx.lines_as(
                "Sir Chilias'Tyus",
                args![
                    "Please seek out",
                    "Clanux Heffron in",
                    "Prontera. He may have",
                    "information regarding a",
                    "clue leading to the Hellion",
                    "gem. Good luck to you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hellionq").get()? == 37 {
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "So have you considered",
                        "my proposal? I believe that",
                        "you may be the one who will",
                        "help me in sealing away the",
                        "Hellion gem's evil forever."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I will help you.:...Sorry about that.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Sir Chilias'Tyus",
                        args![
                            "Good, good, but first",
                            "I must test to see if you",
                            "are worthy of holding the",
                            "Hellion's gem. I can't tell",
                            "you exactly where it is, but",
                            "I will give you some clues."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Chilias'Tyus",
                        args![
                            "First, speak to Clanux",
                            "Heffron in Prontera. He",
                            "has offered to help me find",
                            "the gem, but I do not know if",
                            "I can trust him. Please help me protect this world from the gem..."
                        ],
                    )?;
                    ctx.var("hellionq").set(Val::from(36))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Sir Chilias'Tyus",
                    args![
                        "I understand...",
                        "I know I am asking much",
                        "of you, and this task is not",
                        "one to take lightly. However,",
                        "I believe that you are wholly",
                        "capable of accomplishing this."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (ctx.var("hellionq").get()?.number()? > 37 && ctx.var("hellionq").get()?.number()? < 45) {
                    ctx.lines_as(
                        "Sir Chilias'Tyus",
                        args![
                            "Try to see what you can",
                            "learn from Clanux Heffron.",
                            "I don't know how cooperative",
                            "he will be, but I'm sure that",
                            "he'll tell you something one",
                            "way or another."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("hellionq").get()? == 45 || ctx.var("hellionq").get()? == 46) {
                        ctx.lines_as(
                            "Sir Chilias'Tyus",
                            args![
                                "Ah, yes! This is one piece",
                                "of the tablet that will lead",
                                "you to the Hellion's gem!",
                                "This engraving was definitely",
                                "made by my grandfather, and Christopher was one of his friends."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Chilias'Tyus",
                            args![
                                "I suppose he hid these pieces",
                                "in the places that reminded him",
                                "of his friends. Now according",
                                "to the tablet's message, you",
                                "should find the next piece in ''the city of thickest forest.''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Sir Chilias'Tyus",
                            args![
                                "I'm sure that the",
                                "next tablet piece is",
                                "in Payon, so go there",
                                "and see if you can find",
                                "it. Good luck to you,",
                                "kind adventurer."
                            ],
                        )?;
                        ctx.var("hellionq").set(Val::from(47))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hellionq").get()? == 47 {
                            ctx.lines_as(
                                "Sir Chilias'Tyus",
                                args![
                                    "Now that I think about",
                                    "it, there's a guy in Payon",
                                    "that I know named Grout'the",
                                    "Tuccok who knew my grandfather.",
                                    "He might know something, but",
                                    "he's a little, well... Hmm..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if (ctx.var("hellionq").get()?.number()? > 47 && ctx.var("hellionq").get()?.number()? < 57) {
                                ctx.lines_as(
                                    "Sir Chilias'Tyus",
                                    args![
                                        "I think you're doing",
                                        "a good job in finding",
                                        "those clues in Payon.",
                                        "Please do your best in",
                                        "finding the rest of the",
                                        "pieces of that tablet."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("hellionq").get()? == 57 {
                                    if (ctx.call(Function::CountItem, vec![Val::from(7333)])?.number()? > 0
                                        && ctx.call(Function::CountItem, vec![Val::from(7334)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "While you were gone,",
                                                "I felt a little bad that you",
                                                "were searching for the",
                                                "tablets pieces on your own,",
                                                "so I managed to find this",
                                                "piece here in Morocc. But..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Why are they glowing like",
                                                "this? In any case, we have",
                                                "this piece I found here in",
                                                "Morocc, and the pieces you",
                                                "found in Prontera and Payon. There's one more left in Geffen."
                                            ],
                                        )?;
                                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Anyway, on the tablet",
                                                "piece that I found, it",
                                                "says ''Hakim loved magic,",
                                                "and always enjoyed its",
                                                "wondrous city,'' so I'm sure",
                                                "the last piece is in Geffen."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Please, take this third",
                                                "tablet piece and find the",
                                                "final piece so that we can",
                                                "fulfill my dear grandfather's",
                                                "greatest wish and forever",
                                                "seal away the Hellion's gem."
                                            ],
                                        )?;
                                        ctx.var("hellionq").set(Val::from(58))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7335), Val::from(1)])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "When you get to Geffen,",
                                                "please speak to Welshyun,",
                                                "who's been helping me in",
                                                "creating a device to seal the",
                                                "gem's darkness. He is worthy of trust and will surely help us."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Sir Chilias'Tyus",
                                        args![
                                            "Where are the tablet pieces?",
                                            "Have you hoarded them away",
                                            "to steal the Hellion's gem",
                                            "for yourself?! I must smite",
                                            "you now before you are",
                                            "consumed by its darkness!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("hellionq").get()?.number()? > 57 && ctx.var("hellionq").get()?.number()? < 66) {
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Please visit the",
                                                "sage named Welshyun",
                                                "in Geffen. We are good",
                                                "friends, so I am sure",
                                                "that you can trust him."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("hellionq").get()? == 66 {
                                        if (((ctx.call(Function::CountItem, vec![Val::from(7333)])?.number()? > 0
                                            && ctx.call(Function::CountItem, vec![Val::from(7334)])?.number()? > 0)
                                            && ctx.call(Function::CountItem, vec![Val::from(7335)])?.number()? > 0)
                                            && ctx.call(Function::CountItem, vec![Val::from(7336)])?.number()? > 0)
                                        {
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "You have all four",
                                                    "pieces of the tablet?",
                                                    "That's great news! Ah,",
                                                    "and the gem is embedded",
                                                    "in each of the tablet pieces.",
                                                    "We're so close to finishing!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Ah, would you go back",
                                                    "to Welshyun? I know it's",
                                                    "a hassle, but he is probably",
                                                    "the only one who can combine",
                                                    "the tablet pieces into the its",
                                                    "complete form. Thank you..."
                                                ],
                                            )?;
                                            ctx.var("hellionq").set(Val::from(67))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Where are the tablet pieces?",
                                                "Have you hoarded them away",
                                                "to steal the Hellion's gem",
                                                "for yourself?! I must smite",
                                                "you now before you are",
                                                "consumed by its darkness!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                                        return Err(Stop::End);
                                    } else if ctx.var("hellionq").get()? == 67 {
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Ah, would you go back",
                                                "to Welshyun? I know it's",
                                                "a hassle, but he is probably",
                                                "the only one who can combine",
                                                "the tablet pieces into the its",
                                                "complete form. Thank you..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("hellionq").get()? == 68 {
                                        if (ctx.call(Function::CountItem, vec![Val::from(7332)])?.number()? > 0
                                            && ctx.call(Function::CountItem, vec![Val::from(7337)])?.number()? > 0)
                                        {
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "At long last. We have",
                                                    "everything. The Tablet",
                                                    "and the Hellion's gem.",
                                                    "Now I can finally use",
                                                    "this bracelet to seal",
                                                    "its power forever..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "But your eyes...",
                                                    "They seem so tired",
                                                    "and I can sense some",
                                                    "sort of pain from them.",
                                                    "Did something happen?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "I learned the secret of",
                                                    "the Hellion's gem. It...",
                                                    "It turns people into",
                                                    "Hellion Revenants.",
                                                    "It's what happened",
                                                    "to your grandfather..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Sir Chilias'Tyus", args!["What...", "What did you just say?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "The tablet had a map that",
                                                    "let me to the chamber where",
                                                    "your grandfather locked himself",
                                                    "up before he completed turned",
                                                    "into the Hellion Revenant.",
                                                    "It was horrible..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "At the entrance, he",
                                                    "left a message that said",
                                                    "that he wanted to be killed.",
                                                    "So that he could finally join",
                                                    "his friends instead of living",
                                                    "as a monster. So I... So I..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "It's okay. I know you",
                                                    "did the right thing. It's",
                                                    "what my grandfather",
                                                    "would have wanted most.",
                                                    "Thank you for finally freeing",
                                                    "his soul and giving him peace."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Here, I think you",
                                                    "should have this Eye",
                                                    "of Hellion, in case you",
                                                    "wanted a memento of",
                                                    "your dear grandfather."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "I'd appreciate that.",
                                                    "Grandfather's eye...",
                                                    "It's full of the painful",
                                                    "experiences of the people",
                                                    "who were turned into Hellion",
                                                    "Revenants against their will."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "But... Now is not the",
                                                    "time for sentiment. For",
                                                    "the sake of my grandfather,",
                                                    "I must seal the power of",
                                                    "the Hellion's gem now!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Darkness that writhes,",
                                                    "souls lost in the inferno,",
                                                    "I offer you comfort, I offer",
                                                    "you peace. To the despairing",
                                                    "ones, to the shameless ones,",
                                                    "I give guidance to heaven..."
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Instead of sadness,",
                                                    "let there be joy. Instead",
                                                    "of anger, let there be",
                                                    "love. Souls that are",
                                                    "lost will now find",
                                                    "their way..."
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args!["Light and hope...", "Heaven and earth...", "Cast away the darkness."],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "I did it.",
                                                    "After all these",
                                                    "long years, I finally did",
                                                    "it. Thanks to your help."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Please take this",
                                                    "Nile Rose, which will",
                                                    "keep the power of the gem",
                                                    "in check. I trust that you will",
                                                    "keep its secret and protect it",
                                                    "from those motivated by greed."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7332), Val::from(1)])?;
                                            ctx.call(
                                                Function::DelItem,
                                                vec![Val::from(7337), ctx.call(Function::CountItem, vec![Val::from(7337)])?],
                                            )?;
                                            ctx.var("hellionq").set(Val::from(69))?;
                                            ctx.call(Function::GetItem, vec![Val::from(2658), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                                            ctx.call(Function::GetExperience, vec![Val::from(1200000), Val::from(0)])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Thank you, kind",
                                                    "adventurer, for bringing",
                                                    "peace to my grandfather's",
                                                    "soul and for working to",
                                                    "protect peace in our world.",
                                                    "You are the truest of heroes."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^3355FFYou have received the",
                                                "Nile Rose in which the",
                                                "Hellion's gem is sealed.",
                                                "The Eye of the Hellion has",
                                                "granted you some experience",
                                                "through its strange powers.^000000"
                                            ])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.call(Function::CountItem, vec![Val::from(7332)])?.number()? > 0 {
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "At long last. We have",
                                                    "everything. The Tablet",
                                                    "and the Hellion's gem.",
                                                    "Now I can finally use",
                                                    "this bracelet to seal",
                                                    "its power forever..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Darkness that writhes,",
                                                    "souls lost in the inferno,",
                                                    "I offer you comfort, I offer",
                                                    "you peace. To the despairing",
                                                    "ones, to the shameless ones,",
                                                    "I give guidance to heaven..."
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Instead of sadness,",
                                                    "let there be joy. Instead",
                                                    "of anger, let there be",
                                                    "love. Souls that are",
                                                    "lost will now find",
                                                    "their way..."
                                                ],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL6")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args!["Light and hope...", "Heaven and earth...", "Cast away the darkness."],
                                            )?;
                                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SANCTUARY")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "I did it.",
                                                    "After all these",
                                                    "long years, I finally did",
                                                    "it. Thanks to your help."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Please take this",
                                                    "Nile Rose, which will",
                                                    "keep the power of the gem",
                                                    "in check. I trust that you will",
                                                    "keep its secret and protect it",
                                                    "from those motivated by greed."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7332), Val::from(1)])?;
                                            ctx.var("hellionq").set(Val::from(70))?;
                                            ctx.call(Function::GetItem, vec![Val::from(2658), Val::from(1)])?;
                                            ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Thank you, kind",
                                                    "adventurer, for bringing",
                                                    "peace to my grandfather's",
                                                    "soul and for working to",
                                                    "protect peace in our world.",
                                                    "You are the truest of heroes."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Where is the complete",
                                                "tablet? We need that in",
                                                "order to extract the Hellion's",
                                                "gem in order to seal its power!"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if (ctx.var("hellionq").get()?.number()? > 68 && ctx.var("hellionq").get()?.number()? < 71) {
                                        if ctx.call(Function::CountItem, vec![Val::from(7337)])?.number()? > 0 {
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "Long time no see. So, are you doing well to keep the promise with me?",
                                                    "Please handle it with care",
                                                    "to prevent the evil power within from being released."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Sir Chilias'Tyus",
                                                args![
                                                    "You still have the Hellion's Eye.",
                                                    "I can see the pain in your eyes.",
                                                    "Let me lighten your burden. It will be better this way."
                                                ],
                                            )?;
                                            ctx.call(
                                                Function::DelItem,
                                                vec![Val::from(7337), ctx.call(Function::CountItem, vec![Val::from(7337)])?],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Sir Chilias'Tyus", args!["May God bless you."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Sir Chilias'Tyus",
                                            args![
                                                "Long time no see. So, are you doing well to keep the promise with me?",
                                                "Please handle it with care",
                                                "to prevent the evil power within from being released."
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
    ctx.lines_as("Sir Chilias'Tyus", args!["... ... ..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn old_scholar_tyus_hellion(ctx: &Ctx) -> Script {
    old_scholar_tyus_hellion_body(ctx, Vec::new()).map(|_| ())
}

fn old_scholar_tyus_hellion_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("hellionq").get()? == 57 && ctx.call(Function::CountItem, vec![Val::from(7334)])?.number()? > 0) {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
        ctx.lines(args![
            "^3355FFOne of the Tablet Pieces",
            "that you have is beginning to",
            "shine with light, as if it were",
            "responding to something...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn old_scholar_tyus_hellion_ontouch(ctx: &Ctx) -> Script {
    old_scholar_tyus_hellion_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn clanux_heffron_hellion_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("hellionq").get()?.number()? < 36 {
        ctx.lines_as(
            "Clanux Heffron",
            args![
                "What's wrong with",
                "this map?! I can't",
                "find a blasted thing",
                "on it! How can finding",
                "something be so hard?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hellionq").get()? == 36 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Excuse me, but are", "you Clanux Heffron?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Clanux Heffron",
                args![
                    "Why, who are you",
                    "and what the heck",
                    "do you want? Oh...",
                    "Sent by Chilias, eh?",
                    "What does he want?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("He wants me to help you.:He wants you to help me.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "So he sent me an",
                        "assistant, huh? Well...",
                        "Seeing as he gave me this",
                        "clue and I've taken so long",
                        "and still don't have anything to show for it, I don't blame him."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "Well, if he sent you to",
                        "help me out, I guess I can",
                        "tell you everything. For the",
                        "clues he had me look for,",
                        "I've only found two so far.",
                        "But I can't figure them out..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "Let's see, there's this",
                        "broken sword and this",
                        "strange lookin' cogwheel.",
                        "What do these clues mean?!",
                        "Did the old man give you",
                        "any hints to help me out?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Where did you find these?:No, I'm sorry.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Clanux Heffron",
                        args![
                            "Oh, these I found in Prontera.",
                            "That old coot hid these real",
                            "good. But yeah, I doubt you'd",
                            "find anything, but would you try",
                            "checking out the old Swordman",
                            "Training ground for more clues?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Clanux Heffron",
                        args![
                            "Man, that Hellion's gem",
                            "must really be worth a pretty",
                            "penny if the old man went",
                            "through so much trouble to",
                            "even hide these clues. Oh",
                            "man, I want it so bad..."
                        ],
                    )?;
                    ctx.var("hellionq").set(Val::from(39))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "Wha--? Then why are",
                        "you even here to help",
                        "me then? Huh. That old",
                        "coot must not trust me",
                        "enough, but I can't exactly",
                        "blame him. Alright then..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "If you're gonna help me,",
                        "go search the old Swordman",
                        "Training Grounds over here in",
                        "Prontera. I doubt it, but there's",
                        "a chance I mighta missed",
                        "something there. Get to it!"
                    ],
                )?;
                ctx.var("hellionq").set(Val::from(39))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Clanux Heffron",
                args![
                    "Help you...? What, with",
                    "clues to the location of",
                    "the Hellion's gem? Screw",
                    "that, pal! Why should I help",
                    "you if we're gonna compete",
                    "for the same prize?!"
                ],
            )?;
            ctx.var("hellionq").set(Val::from(38))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hellionq").get()? == 38 {
                ctx.lines_as(
                    "Clanux Heffron",
                    args![
                        "Hey, why do you keep",
                        "botherin' me?! Go and",
                        "find your own clues for the",
                        "Hellion's gem on your own,",
                        "jerkface! Now getouttahere!"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("W-wait! Let's work together!:Fine. I don't need your help!")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Clanux Heffron",
                        args![
                            "Work together, eh? You",
                            "know, normally I'd tell you",
                            "to make like a moonwalker",
                            "and just beat it, but I gotta",
                            "admit that I ain't doin' so",
                            "good on my own on this..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Clanux Heffron",
                        args![
                            "Alright, fine.",
                            "First, look through",
                            "the old Swordman Training",
                            "Grounds here in Prontera.",
                            "There might be some clues left over there for you to find..."
                        ],
                    )?;
                    ctx.var("hellionq").set(Val::from(39))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Fine.", "I don't need", "your help!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Clannux Heffron",
                    args!["Yeah right!", "You'll be back!", "...They always", "come back."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hellionq").get()? == 39 {
                    ctx.lines_as(
                        "Clanux Heffron",
                        args![
                            "Look pal, I found this broken",
                            "sword, a bonafide clue if I ever saw one, in the old Swordman",
                            "Training Grounds in Prontera.",
                            "You should be able to find",
                            "something else there... maybe."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hellionq").get()? == 40 {
                        ctx.lines_as(
                            "Clanux Heffron",
                            args!["Hey, you're done", "searching? Were you", "able to find anything?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Nothing...:In fact, I found this.")])?) == 1 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "^333333(This guy's a real punk!",
                                    "I better not share any new",
                                    "information I've found, just",
                                    "in case he doesn't already",
                                    "know about it.)^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "I... I wasn't able to",
                                    "find anything that looked",
                                    "like a clue. I'm sorry, but",
                                    "I let you down big time..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Clanux Heffron",
                                args![
                                    "Aw nuts! Alright, if you",
                                    "go back to the old man, let",
                                    "him know that I found a broken",
                                    "sword from the old Swordman",
                                    "Training Grounds and this",
                                    "cogwheel from the Tool Shop."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Clanux Heffron",
                                args![
                                    "The clues I've found so",
                                    "far don't make any sense!",
                                    "So if you can, convince him to",
                                    "drop me a couple more hints.",
                                    "And give this to the old guy",
                                    "to prove I found it, will you?"
                                ],
                            )?;
                            ctx.var("hellionq").set(Val::from(41))?;
                            ctx.call(Function::GetItem, vec![Val::from(7093), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Clanux Heffron",
                                args![
                                    "That's all I know.",
                                    "At this point you're",
                                    "probably better off",
                                    "searching for these",
                                    "clues on your own..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Clanux Heffron",
                            args![
                                "So that's what you",
                                "found? Huh. I'm even",
                                "more confused. Alright.",
                                "Well, I found a broken",
                                "sword in the Swordman",
                                "Training Grounds."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Clanux Heffron",
                            args![
                                "Oh, and I also found",
                                "this old cogwheel in the",
                                "Tool Shop around here.",
                                "Here, just take it. Maybe",
                                "you'll have better luck",
                                "figuring out what it means."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Clanux Heffron",
                            args![
                                "But once you do",
                                "learn anything new,",
                                "to me and let me know!",
                                "That's an order, got it?"
                            ],
                        )?;
                        ctx.var("hellionq").set(Val::from(41))?;
                        ctx.call(Function::GetItem, vec![Val::from(7093), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hellionq").get()? == 41 {
                            ctx.lines_as(
                                "Clanux Heffron",
                                args![
                                    "Hey... Didn't I tell",
                                    "you to check out the",
                                    "Tool Shop in case there",
                                    "was anything I missed?",
                                    "Now go! That's an order!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("hellionq").get()? == 42 {
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Wha--? I had no idea",
                                        "that sort of thing could",
                                        "be hidden in that training",
                                        "I missed it. Oh, so the Veggie Lady gave you a password and all?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Huh, I can't make heads",
                                        "or tails of that password.",
                                        "I guess I have no choice but",
                                        "to give you this, the cogwheel clue I found from the Tool Shop."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "I mean, you've been lucky",
                                        "enough till now, so you'll",
                                        "probably be able to stumble",
                                        "upon the answer behind this",
                                        "weird, weird riddle. But you",
                                        "gotta tell me the answer too!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Alright, now take your",
                                        "cogwheel and get outta",
                                        "here. The sooner you get",
                                        "that clue, the sooner I'll",
                                        "get that Hellion's gem."
                                    ],
                                )?;
                                ctx.var("hellionq").set(Val::from(43))?;
                                ctx.call(Function::GetItem, vec![Val::from(7093), Val::from(1)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("hellionq").get()? == 43 {
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Hey, hurry up and check",
                                        "the Tool Shop and figure",
                                        "out the meaning of that",
                                        "password! Maybe that weird",
                                        "machine there is a part of",
                                        "this whole puzzle? Nah..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("hellionq").get()? == 44 {
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args!["Hey...", "Didja find anything", "new in the Tool Shop?"],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Pretend that nothing happened.:Share what you learned.")],
                                )?) == 1
                                {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Nope...",
                                            "I couldn't find any",
                                            "new leads. I guess",
                                            "I have no choice but to",
                                            "go back to Chilias'Tyus,",
                                            "unless you know anything..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Clanux Heffron",
                                        args!["Bah!", "Freakin' useless!", "What kind of assistant", "are you anyway, huh?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Well, I put that cogwheel",
                                        "into that weird machine in",
                                        "the Tool Shop and some kind",
                                        "of number pad came out. Then...",
                                        "Uh, I couldn't figure out what",
                                        "the password was. I'm stuck!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Oh yeah? Hah! Well, now",
                                        "that the easy part is done, I'll just figure out that secret",
                                        "password myself! Hahaha! That Hellion's gem is as good as mine!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("hellionq").get()? == 45 {
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Hey, so you have",
                                        "anything new to report?",
                                        "Oh, and did you learn",
                                        "anything from that weird",
                                        "machine in the Tool Shop?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Tell him just a little bit.:Don't tell him.")],
                                )?) == 1
                                {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Well, I put that cogwheel",
                                            "into that weird machine in",
                                            "the Tool Shop and some kind",
                                            "of number pad came out. Then..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "^333333(Wait, I can't trust this",
                                            "guy!)^000000 I put in every single",
                                            "password that I could think",
                                            "of, but nothing happened!",
                                            "I think I'm stuck..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Clanux Heffron",
                                        args![
                                            "Oh yeah? Hah! Well, now",
                                            "that the easy part is done, I'll just figure out that secret",
                                            "password myself! Hahaha! That Hellion's gem is as good as mine!"
                                        ],
                                    )?;
                                    ctx.var("hellionq").set(Val::from(46))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Huh. You should be able",
                                        "to get some number pad to",
                                        "come out from that machine.",
                                        "Yeah, I was able to get that",
                                        "far, but I haven't been able",
                                        "to figure out the password..."
                                    ],
                                )?;
                                ctx.var("hellionq").set(Val::from(46))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("hellionq").get()?.number()? > 45 && ctx.var("hellionq").get()?.number()? < 71) {
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "Oh hey, it's you.",
                                        "Listen I got this number",
                                        "pad to pop out of that old",
                                        "machine in the Tool Shop,",
                                        "I still don't know what the",
                                        "password for it might be."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Clanux Heffron",
                                    args![
                                        "If I couldn't figure",
                                        "it out, and I understand",
                                        "if you couldn't figure this",
                                        "out, then this puzzle must",
                                        "be freakin' impossible!"
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
    ctx.lines_as(
        "Clanux Heffron",
        args!["Oh man, I am totally", "lost. What the heck", "should I be looking for?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn clanux_heffron_hellion(ctx: &Ctx) -> Script {
    clanux_heffron_hellion_body(ctx, Vec::new()).map(|_| ())
}

fn prt_key_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hellionq").get()? == 39 {
        ctx.lines(args![
            "^3355FFIt's a training dummy",
            "with a gash in its body",
            "that looks like it was made",
            "by the thrust of a sharp sword.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Didn't Clanux mention that he",
                "found a broken sword around",
                "here? Maybe this was exactly",
                "where he found it. Let's see..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Inspect the dummy's gash")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines(args![
            "^3355FFInside of the dummy's body is",
            "a steel bearing with a roughly",
            "etched message that reads:^000000",
            "^4d4dff''Veggie Lady N9 W3 BINGO.''^000000"
        ])?;
        ctx.var("hellionq").set(Val::from(40))?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "What the heck does this",
                "even mean? A Veggie Lady?",
                "Does Clanux know anything",
                "at all about this clue...?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt's a training dummy",
        "that was used to hone the",
        "skills of new Swordmen...",
        "But the Swordman Training",
        "Grounds have moved to Izlude."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prt_key_1(ctx: &Ctx) -> Script {
    prt_key_1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PrtKey11Step {
    Start,
    OnTouch,
}

fn prt_key_1_1_run(ctx: &Ctx, mut step: PrtKey11Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    'machine: loop {
        match step {
            PrtKey11Step::Start => {
                step = PrtKey11Step::OnTouch;
                continue 'machine;
            }
            PrtKey11Step::OnTouch => {
                if (ctx.var("hellionq").get()? == 40 || ctx.var("hellionq").get()? == 41) {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, according to the",
                            "message in that old training",
                            "dummy, this is where I'm",
                            "supposed to go. Nine steps",
                            "north, 3 steps west. And now...",
                            "Veggie Lady. Okay, okay..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Huh. That's creepy.",
                            "There's one right there",
                            "staring at me. But after",
                            "the words ''Veggie Lady,''",
                            "all the message says is, um,",
                            "what was that last word again?"
                        ],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    if l_input_s.clone() == "BINGO" {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Of course!",
                                "''BINGO!'' But",
                                "I don't see a hall",
                                "full of old people",
                                "around here, I--"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Veggie Lady",
                            args![
                                "Finally, you say it.",
                                "Yeesh, I thought I was",
                                "going to wait forever for",
                                "somebody to say that",
                                "password. Okay, here's",
                                "the code number you want..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["C-code number?", "Wait, wh-what...?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Veggie Lady",
                            args![
                                "Awww...I don't know!",
                                "I'm only gonna say it",
                                "now:^4D4DFF 3847147298^000000. One",
                                "more time, in case you",
                                "didn't get it:^4D4DFF 3847147298^000000.",
                                "Don't forget it, adventurer."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Wait... What does", "this all mean? Won't", "you tell me more?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Veggie Lady",
                            args![
                                "No. I've been given",
                                "explicit instructions",
                                "to not tell you more.",
                                "In fact, I hate talking.",
                                "That's why I sell vegetables."
                            ],
                        )?;
                        if ctx.var("hellionq").get()? == 40 {
                            ctx.var("hellionq").set(Val::from(42))?;
                        } else if ctx.var("hellionq").get()? == 41 {
                            ctx.var("hellionq").set(Val::from(43))?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Holy cow...", "This is all", "really cloak and", "dagger type stuff!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            ((Val::from("") + l_input_s.clone()) + Val::from("!")),
                            "Hmmm. No, no that couldn't",
                            "be it. What in the world was that word and why can't I remember it",
                            "when I seemingly need it most?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn prt_key_1_1(ctx: &Ctx) -> Script {
    prt_key_1_1_run(ctx, PrtKey11Step::Start, Vec::new()).map(|_| ())
}

pub fn prt_key_1_1_ontouch(ctx: &Ctx) -> Script {
    prt_key_1_1_run(ctx, PrtKey11Step::OnTouch, Vec::new()).map(|_| ())
}

fn unknown_machine_prt_key_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("hellionq").get()? == 43 {
        ctx.lines(args![
            "^3355FFIt's a weird looking",
            "machine that looks like",
            "it hasn't been used in a",
            "while. It looks like the",
            "cogwheel that you have",
            "would fit perfectly in it...^000000"
        ])?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(7093)])?.number()? > 0 {
            if Val::from(runtime::select_values(ctx, &[Val::from("Insert Cogwheel.:Ignore it.")])?) == 1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I just know this",
                        "cogwheel will fit",
                        "into this machine!",
                        "What I don't know is",
                        "what will happen once",
                        "this machine works..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FF*Click...!*",
                    "Once the cogwheel is",
                    "fit into the machine and",
                    "turned, the device hums",
                    "to life and a panel opens,",
                    "revealing a numeric keypad.^000000"
                ])?;
                ctx.call(Function::DelItem, vec![Val::from(7093), Val::from(1)])?;
                ctx.var("hellionq").set(Val::from(44))?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "I guess that I've got",
                        "to input some kind of",
                        "numeric password...",
                        "Ah, right, the numbers",
                        "that Veggie Lady gave me!"
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s.clone() == "3847147298" {
                    ctx.lines(args![
                        "^3355FFThe machine responds to",
                        "the password with a pleasant",
                        "chime, confirming that you've",
                        "input the correct numbers.",
                        "The keypad slides open to",
                        "reveal a piece of a tablet.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe message engraved on",
                        "this tablet reads: ''This is for",
                        "Christopher, my dear friend",
                        "who I met in Prontera. To the",
                        "one who finds this, please seek",
                        "out the next piece of this tablet",
                        "in the city of thickest forest.''^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe message is signed",
                        "by someone named Tyus.",
                        "It would be best to bring",
                        "this back to Sir Chilia'Tyus",
                        "and confirm that this was",
                        "made by his grandfather...^000000"
                    ])?;
                    ctx.var("hellionq").set(Val::from(45))?;
                    ctx.call(Function::GetItem, vec![Val::from(7333), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![
                    "The machine responds to",
                    "the password with an abrupt,",
                    "screeching beep and the entire",
                    "machine shuts down. You'll have to try entering the password again."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Eh...", "Forget it."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Uh oh...", "Where did I put", "the cogwheel that", "Clanux Heffron gave me?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hellionq").get()? == 44 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Okay, let me see", "if I can enter the", "right number this time..."],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "3847147298" {
            ctx.lines(args![
                "^3355FFThe machine responds to",
                "the password with a pleasant",
                "chime, confirming that you've",
                "input the correct numbers.",
                "The keypad slides open to",
                "reveal a piece of a tablet.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe message engraved on",
                "this tablet reads: ''This is for",
                "Christopher, my dear friend",
                "who I met in Prontera. To the",
                "one who finds this, please seek",
                "out the next piece of this tablet",
                "in the city of thickest forest.''^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe message is signed",
                "by someone named Tyus.",
                "It would be best to bring",
                "this back to Sir Chilia'Tyus",
                "and confirm that this was",
                "made by his grandfather...^000000"
            ])?;
            ctx.var("hellionq").set(Val::from(45))?;
            ctx.call(Function::GetItem, vec![Val::from(7333), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "The machine responds to",
                "the password with an abrupt,",
                "screeching beep and the entire",
                "machine shuts down. You'll have to try entering the password again."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFIt's some sort of",
        "strange looking machine",
        "with a mysterious purpose.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn unknown_machine_prt_key(ctx: &Ctx) -> Script {
    unknown_machine_prt_key_body(ctx, Vec::new()).map(|_| ())
}
