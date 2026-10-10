use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum DorianIzludeStep {
    Start,
    OnTouch,
}

fn dorian_izlude_run(ctx: &Ctx, mut step: DorianIzludeStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    'machine: loop {
        match step {
            DorianIzludeStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ugh, just like a member",
                            "of the working class:",
                            "hoarding all your items",
                            "like a packrat? Have the",
                            "decency to relocate your",
                            "goods to Kafra Storage, please."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()?.number()? < 6 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "You must be in awe",
                            "of my elegant, artistic",
                            "touch. In my hands, almost",
                            "anything can become a",
                            "work of art... Even you~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()? == 6 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "I am Inventor Dorian,",
                            "and I welcome you to my",
                            "workship. All I create is",
                            "infused with art's essence~",
                            "So... How may I assist you?"
                        ],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("I want to use your Magic Dryer.:I want to be your student.:No, thanks.")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Magic Dryer? No, no, no.",
                                    "That, my friend, is my",
                                    "Mystic Heater de Elegance.",
                                    "Don't disgrace my creation",
                                    "like that. What, pray tell,",
                                    "do you need to dry?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("This Bond of Debt:My Hair:I'm just curious.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "Mm? My masterpiece is far",
                                            "too sophisticated for such",
                                            "petty purposes, but... You",
                                            "intrigue me. Show me this",
                                            "''bond of debt'' you wish",
                                            "to restore. "
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "Ah, now I understand.",
                                            "This print is beyond",
                                            "recognition. Only my creation",
                                            "can restore the life that",
                                            "nature has taken away! Yes...",
                                            "You were right to come here."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "Allow me to explain how",
                                            "the Mystic Dryer de Elegance",
                                            "works. A glamourous ruby",
                                            "engine whose design was born",
                                            "of my genius, generates a",
                                            "powerful magnetic field."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "The ruby engine's",
                                            "magnetic field operates",
                                            "according to the ^FF00003 Centrifuge",
                                            "Wavelength Theory^000000. Now,",
                                            "pearl inset sensors detect",
                                            "the amount of calibration..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
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
                                        "Dorian",
                                        args![
                                            "...............................",
                                            "...............................",
                                            "...............................",
                                            "...............................",
                                            "Blah-blah--that's just",
                                            "about it in a nutshell."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Zzz...:Oh, please!")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Dorian",
                                                args![
                                                    "You... You fell asleep?",
                                                    "But I was dutifully explaining",
                                                    "the machine's operation!",
                                                    "No, no, I won't hear your",
                                                    "excuses. What's that?",
                                                    "You were listening?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Dorian",
                                                args![
                                                    "Hm. Well then, let me",
                                                    "ask you this: what is the",
                                                    "main theory by which the",
                                                    "Mystic Heater de Elegance",
                                                    "operates? Hmm? Well?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            let (input, status) = runtime::input_text(ctx, None, None)?;
                                            l_input_s = input;
                                            if l_input_s.clone() == "3 Centrifuge Wavelength Theory" {
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "That's right. My apologies.",
                                                        "I suppose I misjudged you.",
                                                        "Yes, the magnetic field can",
                                                        "restore the shape and integrity",
                                                        "of damaged, inanimate objects.",
                                                        "However, there is a problem."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "The ruby engine is broken,",
                                                        "and I've been too busy with",
                                                        "other projects as ordered by",
                                                        "the Rune-Midgarts Kingdom.",
                                                        "I'll need your assistance",
                                                        "to fix the engine. Alright?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "If you help me fix the",
                                                        "Mystic Dryer de Elegance,",
                                                        "I'll allow you to use it",
                                                        "as much as you like. First,",
                                                        "I'd like you to gather all",
                                                        "of the repair materials."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "Please procure",
                                                        "^FF000020 Rusty Screws^000000,",
                                                        "^FF000010 Iron Ores^000000,",
                                                        "^FF00005 Steel^000000,",
                                                        "^FF00002 Rubies^000000, and",
                                                        "^FF000010 Red Gemstones^000000."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "Once you bring all the",
                                                        "repair materials, I'll",
                                                        "explain how you can",
                                                        "repair the Mystic",
                                                        "Dryer de Elegance."
                                                    ],
                                                )?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(3103), Val::from(3104)])?;
                                                ctx.var("diamond_edq").set(Val::from(9))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Dorian",
                                                    args![
                                                        "Oh? You sound dissatisfied",
                                                        "with my explanation. Well...",
                                                        "I surely can't let you use the",
                                                        "machine unless you fathom",
                                                        "how it works. Otherwise,",
                                                        "you just might break it."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Dorian",
                                                args![
                                                    "Were you too bored while",
                                                    "listening to me speak?",
                                                    "Ah! This wound! It's so...",
                                                    "Alas! It's too emotional!",
                                                    "I am seriously hurt!"
                                                ],
                                            )?;
                                            ctx.var("diamond_edq").set(Val::from(7))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "How can you compare my",
                                            "Mystic Dryer de Elegance",
                                            "to a simple hair dryer?",
                                            "Your comment offends me!",
                                            "I'm no mere engineer...",
                                            "I'm a true virtuoso!"
                                        ],
                                    )?;
                                    ctx.var("diamond_edq").set(Val::from(7))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "Allow me to explain how",
                                            "the Mystic Dryer de Elegance",
                                            "works. A glamourous ruby",
                                            "engine whose design was born",
                                            "of my genius, generates a",
                                            "powerful magnetic field."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
                                        args![
                                            "The ruby engine's",
                                            "magnetic field operates",
                                            "according to the ^FF00003 Centrifuge",
                                            "Wavelength Theory^000000. Now,",
                                            "pearl inset sensors detect",
                                            "the amount of calibration..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Dorian",
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
                                        "Dorian",
                                        args![
                                            "...............................",
                                            "...............................",
                                            "...............................",
                                            "...............................",
                                            "Blah-blah--that's just",
                                            "about it in a nutshell."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Dorian", args!["I know, I know.", "Astounding, isn't it?"])?;
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
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                ctx.lines_as(
                                    "Dorian",
                                    args![
                                        "Ah, my apologies.",
                                        "Too many have asked, so",
                                        "I now only accept beautiful",
                                        "female students. Don't be",
                                        "too disappointed: I know",
                                        "you can find someone else."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Dorian",
                                    args![
                                        "Mademoiselle, it is my",
                                        "honor to meet you: even",
                                        "the moon covers its face,",
                                        "and roses raise their thorns,",
                                        "shameful of themselves",
                                        "and jealous of your beauty."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Dorian",
                                    args![
                                        "Ah! But why go under the",
                                        "pretense of being my student",
                                        "when we can leisurely spend",
                                        "time together as lovers?",
                                        "Won't you--Mademoiselle?",
                                        "Wait, where are you going?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "I don't have enough",
                                    "time for trivel, so",
                                    "please leave me to my",
                                    "artistic endeavors and",
                                    "bother me no longer."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                if ctx.var("diamond_edq").get()? == 7 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ugh, your behavior",
                            "from before disgusts me...",
                            "But those pitiful eyes...",
                            "Seeking sympathy. Ah,",
                            "you're only human.",
                            "I shall forgive you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Alright then. Please",
                            "listen carefully this time,",
                            "and don't insult my pride",
                            "again. When you are ready,",
                            "please come talk to me."
                        ],
                    )?;
                    ctx.var("diamond_edq").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()? == 9 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ah, did you bring all",
                            "the materials I need to",
                            "repair the Mystic Dryer",
                            "de Elegance? Let me take",
                            "a glance at what you brought..."
                        ],
                    )?;
                    ctx.next()?;
                    if ((((ctx.call(Function::CountItem, vec![Val::from(7317)])?.number()? > 19
                        && ctx.call(Function::CountItem, vec![Val::from(1002)])?.number()? > 9)
                        && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 4)
                        && ctx.call(Function::CountItem, vec![Val::from(723)])?.number()? > 1)
                        && ctx.call(Function::CountItem, vec![Val::from(716)])?.number()? > 4)
                    {
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Ah, well done. Now",
                                "that the materials are",
                                "ready, I can instruct you on",
                                "how to repair the machine."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7317), Val::from(20)])?;
                        ctx.call(Function::DelItem, vec![Val::from(1002), Val::from(10)])?;
                        ctx.call(Function::DelItem, vec![Val::from(999), Val::from(5)])?;
                        ctx.call(Function::DelItem, vec![Val::from(723), Val::from(2)])?;
                        ctx.call(Function::DelItem, vec![Val::from(716), Val::from(5)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3104), Val::from(3105)])?;
                        ctx.var("diamond_edq").set(Val::from(10))?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "I expect you to fail",
                                "a few times, but the",
                                "Mystic Dryer de Elegance",
                                "is designed for durability:",
                                "even if you tried, you'd",
                                "have trouble scratching it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "First, insert a new rough",
                                "ruby into the ruby engine,",
                                "and tighten the screw on",
                                "the joint until you hear",
                                "a ^0000FFclick^000000. You'll break it",
                                "if you tighten it further."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Next you'll need to",
                                "operate the four switches",
                                "coded by the colors red,",
                                "blue, yellow, and green."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Press the ^0000FFRed^000000 switch",
                                "if the engine is in 1st",
                                "Gear. Press the ^0000FFBlue^000000",
                                "switch in 2nd Gear,",
                                "^0000FFYellow^000000 for 3rd, and",
                                "^0000FFGreen^000000 for 4th."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "The engine's sound is the",
                                "only indicator of which",
                                "gear is running. You'll",
                                "have to figure that out",
                                "on your own. I'm sorry",
                                "I designed it that way..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Ah, one last tip!",
                                "If you enter the correct",
                                "engine gear, the machine",
                                "will start without a problem,",
                                "even if you press the same",
                                "switch more than once."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Please let me know",
                                "if you get it started so",
                                "I can verify if the Mystic",
                                "Dryer de Elegance is fully",
                                "operational. Good luck now~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Oh? You're still missing",
                                "some materials. Well, we're",
                                "in no rush. If you already",
                                "forgot, then let me remind",
                                "you what you need to bring..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Dorian",
                            args![
                                "Please procure",
                                "^FF000020 Rusty Screws^000000,",
                                "^FF000010 Iron Ores^000000,",
                                "^FF00005 Steel^000000,",
                                "^FF00002 Rubies^000000, and",
                                "^FF000010 Red Gemstones^000000."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if ctx.var("diamond_edq").get()? == 10 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Did you already forget",
                            "my instructions for fixing my",
                            "Mystic Dryer de Elegance?",
                            "I can't blame you since",
                            "they're a bit complicated."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Instructions:Cancel")])? {
                        1 => {
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "First, insert a new rough",
                                    "ruby into the ruby engine,",
                                    "and tighten the screw on",
                                    "the joint until you hear",
                                    "a ^0000FFclick^000000. You'll break it",
                                    "if you tighten it further."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Next you'll need to",
                                    "operate the four switches",
                                    "coded by the colors red,",
                                    "blue, yellow, and green."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Press the ^0000FFRed^000000 switch",
                                    "if the engine is in 1st",
                                    "Gear. Press the ^0000FFBlue^000000",
                                    "switch in 2nd Gear,",
                                    "^0000FFYellow^000000 for 3rd, and",
                                    "^0000FFGreen^000000 for 4th."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "The engine's sound is the",
                                    "only indicator of which",
                                    "gear is running. You'll",
                                    "have to figure that out",
                                    "on your own. I'm sorry",
                                    "I designed it that way..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Ah, one last tip!",
                                    "If you enter the correct",
                                    "engine gear, the machine",
                                    "will start without a problem,",
                                    "even if you press the same",
                                    "switch more than once."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Please let me know",
                                    "if you get it started so",
                                    "I can verify if the Mystic",
                                    "Dryer de Elegance is fully",
                                    "operational. Good luck now~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Dorian",
                                args![
                                    "Really? It's no problem",
                                    "for me to explain it again",
                                    "to you. Don't be shy now:",
                                    "not all of us can be geniuses~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if ctx.var("diamond_edq").get()? == 11 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ah, so you failed to",
                            "fix the Mystic Dryer de",
                            "Elegance? That's expected:",
                            "I didn't really design it for the common user in mind. Well,",
                            "why don't you try it again?"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3106), Val::from(3105)])?;
                    ctx.var("diamond_edq").set(Val::from(10))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()? == 12 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ah, I underestimated you.",
                            "You really repaired the",
                            "Mystic Dryer de Elegance~",
                            "This is a testament to my",
                            "incredible teaching prowess.",
                            "Congratulations are in order!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Now all you have to do",
                            "is insert the damaged",
                            "document and press the",
                            "switch. Simple, yes?"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3107), Val::from(3108)])?;
                    ctx.var("diamond_edq").set(Val::from(13))?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Well, I'm afraid my",
                            "usefulness to you has",
                            "been all used up. Ah,",
                            "but I'll make sure to",
                            "contact you if I need",
                            "your kind of help. Farewell~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()?.number()? > 12 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Ah, hello! How have",
                            "you been? Are you in need",
                            "of my artistic inventions?",
                            "Or did you just miss me?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = DorianIzludeStep::OnTouch;
                continue 'machine;
            }
            DorianIzludeStep::OnTouch => {
                if ctx.var("diamond_edq").get()?.number()? < 9 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Wh-what are you doing?",
                            "D-don't sully my wonderful",
                            "masterpieces with your",
                            "uncultured hands!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("diamond_edq").get()? == 9 {
                    ctx.lines_as(
                        "Dorian",
                        args![
                            "Can't you see that the",
                            "machine won't work without",
                            "the materials? Even a simple",
                            "child should understand that!"
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

pub fn dorian_izlude(ctx: &Ctx) -> Script {
    dorian_izlude_run(ctx, DorianIzludeStep::Start, Vec::new()).map(|_| ())
}

pub fn dorian_izlude_ontouch(ctx: &Ctx) -> Script {
    dorian_izlude_run(ctx, DorianIzludeStep::OnTouch, Vec::new()).map(|_| ())
}

fn strangemachine_izlude_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_blue = Val::from(0);
    let mut l_bolt = Val::from(0);
    let mut l_bolt_rand = Val::from(0);
    let mut l_bolt_suc = Val::from(0);
    let mut l_engine = Val::from(0);
    let mut l_green = Val::from(0);
    let mut l_hit_status = Val::from(0);
    let mut l_red = Val::from(0);
    let mut l_switch_sound = Val::from(0);
    let mut l_yellow = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Dorian",
            args![
                "Ugh, just like a member",
                "of the working class:",
                "hoarding all your items",
                "like a packrat? Have the",
                "decency to relocate your",
                "goods to Kafra Storage, please."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 10 {
        l_bolt_rand = ctx.call(Function::Rand, vec![Val::from(2), Val::from(5)])?;
        ctx.lines(args![
            "^3355FFThis must be Dorian's",
            "Mystic Dryer de Elegance.",
            "What do you want to do?^000000"
        ])?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                l_switch_sound = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Check the Machine:Replace the Engine:Tighten the Screw:Press a Switch")],
                )? {
                    1 => {
                        if l_hit_status.clone() == 0 {
                            ctx.lines(args!["^3355FFThere's no power", "in this behemoth", "of a machine.^000000"])?;
                            ctx.next()?;
                        }
                    }
                    2 => {
                        if l_engine.clone() == 0 {
                            ctx.lines(args![
                                "^3355FFYou replace the rough",
                                "ruby in the ruby engine",
                                "with a new rough ruby.",
                                "It clicks into place",
                                "inside the engine core.^000000"
                            ])?;
                            ctx.next()?;
                            l_engine = (l_engine.clone() + Val::from(1));
                        } else if l_engine.clone().number()? > 0 {
                            ctx.lines(args!["^3355FFThe ruby in the", "engine has already", "been replaced.^000000"])?;
                            ctx.next()?;
                        }
                    }
                    3 => {
                        if l_engine.clone().number()? < 1 {
                            ctx.lines(args![
                                "^3355FFYou probably need",
                                "to replace the ruby",
                                "in the engine first.^000000"
                            ])?;
                            ctx.next()?;
                        } else if (l_engine.clone() == 1 && runtime::op(&l_bolt.clone(), "<", &l_bolt_rand.clone())?.is_true()) {
                            ctx.lines(args!["^3355FF*Krrrr-Krrrrr*^000000", "^3355FF*Krrrr-Krrrrr*^000000"])?;
                            ctx.next()?;
                            l_bolt = (l_bolt.clone() + Val::from(1));
                        } else if (l_engine.clone() == 1 && l_bolt.clone().loosely_equals(&l_bolt_rand.clone())) {
                            ctx.lines(args!["^3355FF*Krrrr-Krrrrr*^000000", "^0000FF*Click*^000000"])?;
                            ctx.next()?;
                            l_bolt = (l_bolt.clone() + Val::from(1));
                            l_bolt_suc = (l_bolt_suc.clone() + Val::from(1));
                        } else if (l_engine.clone() == 1 && runtime::op(&l_bolt.clone(), ">", &l_bolt_rand.clone())?.is_true()) {
                            ctx.lines(args!["^3355FF*Krrrr-Krrrrr*^000000", "^3355FF*KrrICK-ICK-ICK-KOOM*^000000"])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFYou tightened the",
                                "screw too tightly!",
                                "The joint is broken...",
                                "You should go back",
                                "to Dorian for help.^000000"
                            ])?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                            ctx.var("diamond_edq").set(Val::from(11))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args![
                                "^3355FFSomething must have",
                                "gone wrong. You'd be",
                                "better off starting",
                                "from the beginning...^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    4 => {
                        if (l_engine.clone().number()? < 1 && l_bolt_suc.clone().number()? < 1) {
                            ctx.lines(args![
                                "^3355FFYou need to replace",
                                "the engine's ruby and",
                                "tighten the main screw",
                                "before you mess with",
                                "these switches.^000000"
                            ])?;
                            ctx.next()?;
                        } else {
                            if (l_engine.clone() == 1 && l_bolt_suc.clone().number()? < 1) {
                                ctx.lines(args![
                                    "^3355FFYou need to tighten",
                                    "the scren before you can",
                                    "start the engine safely."
                                ])?;
                                ctx.next()?;
                            } else {
                                if (((l_red.clone().number()? >= 1 && l_blue.clone().number()? >= 1) && l_yellow.clone().number()? >= 1)
                                    && l_green.clone().number()? >= 1)
                                {
                                    ctx.lines(args![
                                        "^3355FFThe Mystic Dryer de",
                                        "Elegance started with",
                                        "a mighty buzz, and the",
                                        "sound lowers to a calm",
                                        "hum as it stabilizes.",
                                        "It looks like you fixed it!^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFYou should tell Dorian",
                                        "first so that you can",
                                        "use this machine.^000000"
                                    ])?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3107)])?;
                                    ctx.var("diamond_edq").set(Val::from(12))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if (l_engine.clone() == 1 && l_bolt_suc.clone() == 1) {
                                    if l_switch_sound.clone() == 1 {
                                        ctx.lines(args![
                                            "^3355FF*Buzz Buzz*^000000",
                                            "^3355FF*Buzz Buzz*^000000",
                                            "^3355FFThe machine is",
                                            "vibrating weakly.",
                                            "Which switch do",
                                            "you want to press?^000000"
                                        ])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("Red Switch:Blue Switch:Yellow Switch:Green Switch")],
                                        )? {
                                            1 => {
                                                if l_red.clone().number()? >= 0 {
                                                    ctx.lines(args!["^3355FF*Buzzz Buzzz*^000000", "^3355FF*Clang...!*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine's",
                                                        "vibrations have",
                                                        "grown stronger.",
                                                        "It looks like you",
                                                        "chose the right switch.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    l_red = (l_red.clone() + Val::from(1));
                                                } else {
                                                    ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine made some",
                                                        "violent, erratic sounds,",
                                                        "and vibrated violently",
                                                        "before coming to a sudden",
                                                        "stop. That was the wrong",
                                                        "switch. You'd better ask Dorian..."
                                                    ])?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                    ctx.var("diamond_edq").set(Val::from(11))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            _ => {
                                                ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFThe machine made some",
                                                    "violent, erratic sounds,",
                                                    "and vibrated violently",
                                                    "before coming to a sudden",
                                                    "stop. That was the wrong",
                                                    "switch. You'd better ask Dorian..."
                                                ])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                ctx.var("diamond_edq").set(Val::from(11))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else if l_switch_sound.clone() == 2 {
                                        ctx.lines(args![
                                            "^3355FF*Purr Purr*^000000",
                                            "^3355FF*Purr Purr*^000000",
                                            "^3355FFThe machine is vibrating",
                                            "a little bit more strongly.",
                                            "Which switch will",
                                            "you try now?^000000"
                                        ])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("Red Switch:Blue Switch:Yellow Switch:Green Switch")],
                                        )? {
                                            2 => {
                                                if l_blue.clone().number()? >= 0 {
                                                    ctx.lines(args!["^3355FF*Purrr Purr*^000000", "^3355FF*Clang...!*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine's",
                                                        "vibrations have",
                                                        "grown stronger.",
                                                        "It looks like you",
                                                        "chose the right switch.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    l_blue = (l_blue.clone() + Val::from(1));
                                                } else {
                                                    ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine made some",
                                                        "violent, erratic sounds,",
                                                        "and vibrated violently",
                                                        "before coming to a sudden",
                                                        "stop. That was the wrong",
                                                        "switch. You'd better ask Dorian..."
                                                    ])?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                    ctx.var("diamond_edq").set(Val::from(11))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            _ => {
                                                ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFThe machine made some",
                                                    "violent, erratic sounds,",
                                                    "and vibrated violently",
                                                    "before coming to a sudden",
                                                    "stop. That was the wrong",
                                                    "switch. You'd better ask Dorian..."
                                                ])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                ctx.var("diamond_edq").set(Val::from(11))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else if l_switch_sound.clone() == 3 {
                                        ctx.lines(args![
                                            "^3355FF*Bzzz Bzzz*",
                                            "*Bzzz Bzzz*",
                                            "The machine's vibrations",
                                            "are a bit more stable now.",
                                            "Which switch will you try?^000000"
                                        ])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("Red Switch:Blue Switch:Yellow Switch:Green Switch")],
                                        )? {
                                            3 => {
                                                if l_yellow.clone().number()? >= 0 {
                                                    ctx.lines(args!["^3355FF*Bzzzz Bzzzz*^000000", "^3355FF*Clang...!*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine's",
                                                        "vibrations have",
                                                        "grown stronger.",
                                                        "It looks like you",
                                                        "chose the right switch.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    l_yellow = (l_yellow.clone() + Val::from(1));
                                                } else {
                                                    ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine made some",
                                                        "violent, erratic sounds,",
                                                        "and vibrated violently",
                                                        "before coming to a sudden",
                                                        "stop. That was the wrong",
                                                        "switch. You'd better ask Dorian..."
                                                    ])?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                    ctx.var("diamond_edq").set(Val::from(11))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            _ => {
                                                ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFThe machine made some",
                                                    "violent, erratic sounds,",
                                                    "and vibrated violently",
                                                    "before coming to a sudden",
                                                    "stop. That was the wrong",
                                                    "switch. You'd better ask Dorian..."
                                                ])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                ctx.var("diamond_edq").set(Val::from(11))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else if l_switch_sound.clone() == 4 {
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^3355FF*Pzzzz Pzzz*",
                                            "*Pzzzz Pzzz*",
                                            "The machine is vibrating",
                                            "fairly strongly now. Which",
                                            "switch will you try?^000000"
                                        ])?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("Red Switch:Blue Switch:Yellow Switch:Green Switch")],
                                        )? {
                                            4 => {
                                                if l_green.clone().number()? >= 0 {
                                                    ctx.lines(args!["^3355FFPzzzzz Pzzzz*^000000", "^3355FF*Clang...!*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine's",
                                                        "vibrations have",
                                                        "grown stronger.",
                                                        "It looks like you",
                                                        "chose the right switch.^000000"
                                                    ])?;
                                                    ctx.next()?;
                                                    l_green = (l_green.clone() + Val::from(1));
                                                } else {
                                                    ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                    ctx.next()?;
                                                    ctx.lines(args![
                                                        "^3355FFThe machine made some",
                                                        "violent, erratic sounds,",
                                                        "and vibrated violently",
                                                        "before coming to a sudden",
                                                        "stop. That was the wrong",
                                                        "switch. You'd better ask Dorian..."
                                                    ])?;
                                                    ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                    ctx.var("diamond_edq").set(Val::from(11))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            _ => {
                                                ctx.lines(args!["^3355FF*Whiz Whiz*^000000", "^3355FF*Whiz Whiz*^000000"])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFThe machine made some",
                                                    "violent, erratic sounds,",
                                                    "and vibrated violently",
                                                    "before coming to a sudden",
                                                    "stop. That was the wrong",
                                                    "switch. You'd better ask Dorian..."
                                                ])?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(3105), Val::from(3106)])?;
                                                ctx.var("diamond_edq").set(Val::from(11))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    if ctx.var("diamond_edq").get()? == 12 {
        ctx.lines_as(
            "Dorian",
            args![
                "It sounds like you're",
                "done fixing my Mystic",
                "Dryer de Elegance.",
                "Why don't you come",
                "back so I can explain",
                "how you can use it?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()? == 13 && ctx.call(Function::CountItem, vec![Val::from(7722)])?.number()? < 1) {
        ctx.lines(args![
            "^3355FFYou lift the main",
            "operational switch,",
            "turning on a light and",
            "opening a convenient",
            "slot. You insert the",
            "wet bond of debt.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe engine's pistons",
            "and cylinders churn",
            "with a lively din, and",
            "when the heater stops,",
            "the slot reopens.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["So this is the original", "bond of debt that Muff", "lost? It looks... Perfect!"],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3108), Val::from(3109)])?;
        ctx.call(Function::GetItem, vec![Val::from(7722), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("diamond_edq").get()? == 13 && ctx.call(Function::CountItem, vec![Val::from(7722)])?.number()? > 0) {
        ctx.lines(args![
            "You already used this",
            "machine to restore the",
            "bond of debt. There's no",
            "need to mess around",
            "with it any longer."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dorian",
        args![
            "Who are you?",
            "Don't you dare lay",
            "a hand on my precious",
            "masterpiece inventions! "
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn strangemachine_izlude(ctx: &Ctx) -> Script {
    strangemachine_izlude_body(ctx, Vec::new()).map(|_| ())
}
