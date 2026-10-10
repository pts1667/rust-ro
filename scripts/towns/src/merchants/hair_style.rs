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

pub fn roving_hair_dresser(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Rui Vishop",
        args![
            "That Veronica...",
            "Hah! Best hair dresser my ass.",
            "She's not the best hair dresser...",
            "...",
            "I am!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rui Vishop",
        args![
            "I, Rui Vishop, the man to whom",
            "all scalps are canvases",
            "waiting to be transformed into",
            "works of magnificent art~!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["What are you?", "Do my hair, please!", "....."])? {
        0 => {
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "Do you not know that I, Rui",
                    "Vishop, maestro of the shears",
                    "and sculptor of hair, am an",
                    "artist far ahead of his time?!",
                    "Well, I suppose an adventurer",
                    "like yourself wouldn't know..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "As a hair sculptor, I find joy",
                    "in bestowing upon others the",
                    "supreme favour of doing their",
                    "hairstyle at a reasonable price."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "Recently, however, I happened to",
                    "overhear that some tyro has had",
                    "the audacity to call herself a",
                    "hair dresser."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "So one day I went there,",
                    "pretending to be a customer.",
                    "I was apalled to see the boring,",
                    "lifeless hairstyles that she was",
                    "giving all of her clients..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "It wasn't hard to notice that her",
                    "skills, or lack thereof, are a",
                    "joke. She brings shame to the",
                    "great and honorable",
                    "profession of hair dressing.",
                    "A complete and utter disgrace!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "But the worst part was...",
                    "she forced her customers to",
                    "choose a hairstyle before she",
                    "styled their hair!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "That's not how talented hair",
                    "dressers do their job! She",
                    "should know what hair style will",
                    "fit a customer without ever",
                    "asking them!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "If by any chance you decide to",
                    "do your hair, don't even think",
                    "about giving her patronage.",
                    "Instead, you may ask for my",
                    "services. I assure you, I am",
                    "faaaar better than her."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "Do you understand? I mean, don't",
                    "let her ruin your hair needlessly!",
                    "You could get a Swordman to hack",
                    "away at your hair if you want a",
                    "hairstyle that horrible~!"
                ],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.player().base_level()? < 60 {
                ctx.lines_as(
                    "Rui Vishop",
                    args![
                        "Hmm, I must say, your current",
                        "style fits you best. Trust me, I know what I am saying."
                    ],
                )?;
                return ctx.close();
            }
            if ctx.player().zeny()? < 199800 {
                ctx.lines_as(
                    "Rui Vishop",
                    args![
                        "Ah, I see that that you can",
                        "recognize genius when it is",
                        "right before you. In light",
                        "of your good taste, I will",
                        "only require money for my",
                        "services."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rui Vishop",
                    args![
                        "Simply pay me the small",
                        "fee of 199,800 zeny. You must",
                        "know that I am doing you a",
                        "huge favor by charging you",
                        "such a small amount. My",
                        "art is priceless, after all."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Rui Vishop",
                args!["Alright, I will be taking my", "199,800 zeny service charge now."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "If you don't wish to do",
                    "this right now, though I can't",
                    "imagine why, you may ask that",
                    "stupid hair dresser to",
                    "do her clumsy work on you..."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["No, please do my hair.", "Umm, I changed my mind."])? == 0 {
                ctx.lines_as("Rui Vishop", args!["O~k~a~y!", "Now, let us begin~!"])?;
                ctx.next()?;
                ctx.lines_as("Rui Vishop", args!["Wooooo~oooohhhh!! Toohhhhh~oooohhhh!!"])?;
                ctx.next()?;
                ctx.lines_as("Rui Vishop", args!["Woooooo~aaaaaaahhhhh!!"])?;
                ctx.next()?;
                ctx.lines_as("Rui Vishop", args!["Voila!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Rui Vishop",
                    args![
                        "Oh, great~ it's awesome!",
                        "Another Vishop masterpiece~",
                        "Once more I've outdone myself.",
                        "It's such a unique and talented",
                        "style! Yes, I am the best! Wooohahahahahaha!"
                    ],
                )?;
                ctx.player().set_zeny(ctx.player().zeny()? - 199800)?;
                ctx.call(Function::SetLook, args![1, ctx.call(Function::Rand, args![1, 19])?])?;
                ctx.call(Function::SetLook, args![6, ctx.call(Function::Rand, args![1, 8])?])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "Bah! Alright! It's your decision.",
                    "But don't blame me later!",
                    "One day you'll wake up, realize",
                    "you're ugly and regret not",
                    "having my genius shape every lock",
                    "of hair on your head."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Rui Vishop",
                args![
                    "What? What a shame!",
                    "Will you let that...that",
                    "charlatan of a hair dresser ruin",
                    "your hairstyle!? I'm sure the",
                    "heavens are crying tears of",
                    "pity at mankind's ignorance..."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn assistant_beautician_li(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Assistant Beautician",
        args!["Wah?! Sweet Jiminy,", "you freaked me out!", "What are you doing?!"],
    )?;
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.next()?;
    ctx.lines_as(
        "Assistant Beautician",
        args!["Oh! Um, a customer!", "H-h-h-h-h-hello! Can", "I help you with anything?"],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("What do you do?:Please change my hairstyle.:Who is Prince Shammi?")],
        )?);
        let mut matched1 = false;
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Assistant Beautician",
                args![
                    "Oh! Me...? I'm",
                    "just an assistant",
                    "beautician, but I'm",
                    "training hard everyday",
                    "so that I can become",
                    "a real professional!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Assistant Beautician",
                args![
                    "Yeah, I do all sorts of",
                    "grunt work for the boss while",
                    "I'm in training. Sometimes, he",
                    "makes me work pretty hard.",
                    "In fact, I better get back to work before he gets angry at me!"
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.player().base_level()? < 60 {
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Me...? Oh no,",
                        "no I can't! I mean,",
                        "I'd love to but, I'm",
                        "still in training and",
                        "I can't take responsibility",
                        "if I mess up on a little kid!"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.player().zeny()? < 250000 {
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Well... I'm just an",
                        "assistant, but I have been",
                        "studying hairstyling after",
                        "work. If you want, just bring",
                        "me 250,000 zeny and I'll try",
                        "my best to change your hair~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "I'd appreciate it if you'd",
                        "give me this chance! The",
                        "boss doesn't think I'm ready",
                        "for styling real people yet, so",
                        "I haven't had much practice!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "I just know I could",
                        "do a good job on your",
                        "hair! Just... Just please",
                        "understand if I mess up.",
                        "It won't be too bad, I promise~"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Assistant Beautician",
                args![
                    "You're really going to",
                    "give me a chance to practice?",
                    "Oh, I love you so much! Okay,",
                    "I'll need 250,000 zeny to make",
                    "up for the material expenses.",
                    "Is that okay with you?"
                ],
            )?;
            ctx.next()?;
            let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Of course~:On second thought...")])?);
            if subject2.loosely_equals(&Val::from(1)) {
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Great! Now, please",
                        "choose a hairstyle",
                        "from ''1'' to ''23.''",
                        "Um, if you need to",
                        "cancel, just enter ''0.''"
                    ],
                )?;
                ctx.next()?;
                let (l_input, _) = runtime::input_number(ctx, None, None)?;
                if l_input == 0 {
                    ctx.lines_as(
                        "Assistant Beautician",
                        args!["Awwww...", "I guess you don't", "trust me after all..."],
                    )?;
                    return ctx.close();
                }
                if l_input.number()? < 1 || l_input.number()? > 23 {
                    ctx.lines_as(
                        "Assistant Beautician",
                        args![
                            "Huh? I thought I asked",
                            "you to enter a number from",
                            "''1'' to ''23?'' What did I do",
                            "wrong this time? Hmmm..."
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Assistant Beautician",
                    args!["So this is the", "style you want me", "to try to do for you?"],
                )?;
                let sex = if ctx.var("Sex").get()? == constants::SEX_MALE { "m" } else { "f" };
                let zero_pad = if l_input.number()? < 10 { "0" } else { "" };
                ctx.call(
                    Function::Cutin,
                    args![Val::from(format!("hair_{sex}_{zero_pad}")) + l_input.clone() + Val::from(".BMP"), 4],
                )?;
                ctx.next()?;
                let subject3 = Val::from(runtime::select_values(ctx, &[Val::from("Yes, let's try it~:Cancel.")])?);
                if subject3.loosely_equals(&Val::from(2)) {
                    ctx.lines_as(
                        "Assistant Beautician",
                        args!["Oooh, there must", "be some style that", "you like, right? Hmmm..."],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return ctx.end();
                }
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Great, you finally",
                        "picked one! What, which",
                        "one did you pick again?",
                        "Ah, I found it, I found it!",
                        "Haha! No problem here!",
                        "Now it's time to style!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Nude, args![])?;
                ctx.lines_as("Assistant Beautician", args!["Bwwwwaaaahhhh!"])?;
                ctx.next()?;
                ctx.lines_as("Assistant Beautician", args!["Yap! Pwwwaaattt!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Beautician",
                    args!["Waaaah!", "Oh crap!", "Wait, I can...", "I can fix this!"],
                )?;
                ctx.next()?;
                let l_style_r = ctx.call(Function::Rand, args![1, 23])?;
                let l_color_r = ctx.call(Function::Rand, args![1, 8])?;
                ctx.lines_as("Assistant Beautician", args!["^333333*Pant Pant Pant*^000000"])?;
                ctx.next()?;
                ctx.player().set_zeny(ctx.player().zeny()? - 250000)?;
                ctx.call(Function::SetLook, args![constants::VAR_HEAD, l_style_r.clone()])?;
                ctx.call(Function::SetLook, args![constants::VAR_HEADPALETTE, l_color_r])?;
                ctx.lines_as("Assistant Beautician", args!["Bwahahaha! Success!"])?;
                if l_input.loosely_equals(&l_style_r) {
                    ctx.lines(args!["So... How do you like", "your new style? I love it!"])?;
                } else {
                    ctx.lines(args![
                        "Wha...? This isn't what",
                        "you wanted? Uh oh... Um...",
                        "Well, next time I know I can",
                        "do a much better job! Right!"
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Oh, you're such a",
                        "sweetheart for helping",
                        "me! Thank you for using",
                        "my service and come again~"
                    ],
                )?;
                ctx.npc().emotion(constants::ET_CHUP)?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return ctx.end();
            }
            if subject2.loosely_equals(&Val::from(2)) {
                ctx.lines_as(
                    "Assistant Beautician",
                    args![
                        "Huh? Oh no, you're",
                        "quitting? Well, I guess",
                        "I couldn't trust me to",
                        "style my hair either...",
                        "You're... You're right."
                    ],
                )?;
                return ctx.close();
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Assistant Beautician",
                args![
                    "Prince Shammi?",
                    "He's only a genius when",
                    "it comes to hairstyling!",
                    "I'm just his apprentice, but",
                    "maybe someday, I can be a",
                    "force in the fashion world too!"
                ],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}
