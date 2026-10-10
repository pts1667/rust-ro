use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn book_touching_man_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 59 {
        if ctx.call(Function::CheckWeight, vec![Val::from(703), Val::from(3)])? == 0 {
            ctx.mes("- You have too many items in your inventory to proceed with this quest. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("barmunt_crow").get()? == 0 {
            ctx.mes("^660000Morocc, the City of the Desert, has been completely devastated by Satan Morocc.^000000")?;
            ctx.next()?;
            ctx.mes("^660000The people of Morocc were resilient enough to endure the region's harsh weather, but their toughness to the weather could not prepare them enough as they witnessed the destruction brought down upon the city by Satan Morocc.^000000")?;
            ctx.next()?;
            ctx.mes("^660000This man, who looks as dry and thin as a fish dried up under the sun, still has a smile that reminds you of a benevolent god, and is touching a book in his hand.^000000")?;
            ctx.next()?;
            ctx.mes("^660000Watching him makes you giggle, he looks no better than the dried corpses in the pyramid, but he is obviously alive and doesn't appear to be a monster. Somehow, you find yourself compelled to approach this interesting looking man.^000000")?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
            ctx.lines_as("Book-Touching Man", args!["Ah... Mammi..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Book-Touching Man",
                args!["Your healthy golden skin is glowing under the blessed sunlight of Morocc."],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines_as("Book-Touching Man", args!["Your smile outshines the aura of the gods of Valhalla. You are a living gospel. Your smile even makes Goddess Freya hide in the shadows in shame! Ah.... Mammi, my Mammi!"])?;
            ctx.next()?;
            ctx.mes("^660000You wave your hand close to his face to get his attention. He seems, however, to be in his own dreams, completely oblivious of your presense.^000000")?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Speak:Pass Him By")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Are you talking to yourself?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.lines_as(
                        "Book-Touching Man",
                        args![
                            "Wah! Oh, my god, you must be a messenger of the devil trying to interrupt me from feeling Mammi!",
                            "I'm not afraid of you, so bring it on!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^660000Surprised by your voice, he yelled at you, which seemed bizarre.",
                        "You wonder what he meant by feeling her when he was just watching and touching a book.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Who's Mammi?"])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.lines_as(
                        "Book-Touching Man",
                        args!["What? Don't you know Mammi, one of the three greatest Rune-Midgarts' idols?"],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_OHNO")?])?;
                    ctx.lines_as("Book-Touching Man", args!["Man, you don't know anything, do you?! Well, judging by the stupid look on your face, I guess you're an adventurer running around on meaningless errands for others. Heh heh!"])?;
                    ctx.next()?;
                    ctx.mes("^660000Umm... You now regret speaking to him in the first place.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Book-Touching Man", args!["My Mammi is different from those other celebrities that only rely on their beauty and lack any real substance.", "She always carries books with her despite her hectic schedule. That alone should tell you that she's more intelligent than all the others! Ah... Mammi..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Book-Touching Man",
                        args!["She even mentioned in an interview in 'Morocc Times' that she wants to meet a man that loves books."],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_COOL")?])?;
                    ctx.lines_as(
                        "Book-Touching Man",
                        args![
                            "That means... She's waiting for someone like me. Hehehe!",
                            "You see, every book in Rune-Midgarts' Library has the name Benjamin written on them... That's me!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000This Benjamin seems to be a huge fan of a female celebrity called Mammi. He didn't shut his mouth and worshipped her beauty for a while, but then he suddenly stopped and looked depressed.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Benjamin", args!["I have a problem...."])?;
                    ctx.next()?;
                    ctx.lines_as("Benjamin", args!["My Mammi said that she's fallen in love with a book written by ^FF0000'Oliver Hilpert'^000000 of Schwarzwald: ^FF0000'The Crow of the Fate,'^000000 but I didn't have a chance of even looking at it!"])?;
                    ctx.next()?;
                    ctx.mes("^660000The Crow of the Fate^000000?")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Benjamin",
                        args!["Unless you're blind, you know how terrible Morocc is nowadays."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Benjamin", args!["I really need to read that book and tell her that I'm here for her, but I don't even have the time to regain my breath because of this situation in our hands!"])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as(
                        "Benjamin",
                        args![
                            "My Mammi pillow cover is buried under debris... Gasp!",
                            "Mammi, I'm coming! Don't you die yet!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000He seems to see an image of Mammi, and he shouted her name, gasping with anxiety. You feel so sorry for his agony.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000He inhaled deeply, and then suddenly turned his head toward you.^000000")?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                    ctx.lines_as("Benjamin", args!["....Ye... Yes, you!"])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh?"])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_PROFUSELY_SWEAT")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Benjamin", args!["You look bored. You must be waiting for someone to give you something to do!", "Help me connect with Mammi. Who knows? I may let you have a conversation with Mammi, whom everyone in Rune-Midgarts adores."])?;
                    ctx.next()?;
                    ctx.mes("^660000Didn't you just call me an adventurer that runs around on meaningless errands for others...?^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Benjamin", args!["All I'm asking is to check out a book from Prontera Library for me. It sounds easy, doesn't it?", "Well, you don't look like someone who loves reading, but I hope you at least know how to check out a book from a library."])?;
                    ctx.next()?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines_as(
                            "Benjamin",
                            args![
                                "If you help me, I'll show you a limited picture book edition of Mammi... Once. Hehehehe.",
                                "What do you say? You do want to help me, right?"
                            ],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as("Benjamin", args!["If you help me, I'll show you a limited picture book edition of Mammi... Once. Hehehehe.", "So, aren't you interested in looking at another girl's pictures? Umm, if you want, I can introduce you to a handsome guard in Morocc...", "What do you say? You do want to help me, right?"])?;
                        ctx.next()?;
                    }
                    ctx.mes("^660000What do you want to do?^000000")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Help him:Don't help him")])? {
                        1 => {
                            ctx.lines_as("Benjamin", args!["Wow, are you really going to help me?"])?;
                            ctx.next()?;
                            ctx.lines_as("Benjamin", args!["Thank you so much! Then remember this title: 'The Crow of the Fate' written by 'Oliver Hilpert.' I'm sure you can find it in Prontera Library.", "Hehehe, Mammi! I'm almost there!"])?;
                            ctx.var("barmunt_crow").set(Val::from(1))?;
                            ctx.call(Function::SetQuest, vec![Val::from(2063)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Benjamin",
                                args!["What, don't you know how to check out a book from a library? Oh, my..."],
                            )?;
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
                    ctx.lines_as("Benjamin", args!["Oh... Mammi... My Mammi..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else if ctx.var("barmunt_crow").get()? == 1 {
            ctx.lines_as("Benjamin", args!["Don't you forget this title: 'The Crow of the Fate' written by 'Oliver Hilpert.' I'm sure you can find it in Prontera Library."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("barmunt_crow").get()? == 10 {
            ctx.lines_as("Benjamin", args!["Oh... Mammi... My Mammi..."])?;
            ctx.next()?;
            ctx.mes(
                "^660000Just like the last time that you saw him, Bejamin is living in his dreams, rubbing a book on his face.^000000",
            )?;
            ctx.next()?;
            ctx.mes("^660000You carefully approached him, and then gave him a pat on the shoulder.^000000")?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.lines_as("Benjamin", args!["Grrrr! Who dares touch me?!"])?;
            ctx.next()?;
            ctx.mes("^660000...Umm... Obviously you weren't careful enough... ^000000")?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args![
                    "Who dares interrupt me from feeling Mammi? Do you want a piece of me... Err?",
                    "Gosh, it's you. What took you so long? Wasn't Prontera Library, like, five minutes away?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Ahchoo! Thanks to you, I couldn't find it from the library. So I travelled all the way up to Schwarzwald... Ahchoo!"
                ],
            )?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
            ctx.next()?;
            ctx.mes("^660000Feeling angry at him, you exaggerated a fit of sneezing which you have had for quite a while.^000000")?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["Oh... you did?", "Hehehe, you're more reliable than I thought. Thanks for the book... Wait, does that mean I have to go up to Schwarzwald to return this? Man!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args![
                    "...No, I can do anything as long as it's for Mammi. Heh heh, you're a kind adventurer. Thanks.",
                    "Was it cold up there? I guess it is since Schwarzwald is located up north from here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![".................."],
            )?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["Let me think. If this book is not yet available in Prontera Library, it must mean that not many people of Rune-Midgarts have read this book."])?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args!["Grrrr... Mammi! Here I am, following your noble tastes to read such a rare book! Mammi...!"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args![
                    "I feel so excited to think about having a deep conversation with Mammi about literature! Yay!",
                    "I should go read this book right away. Hehehe!"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args!["Oops, I must not forget to repay you. You know, I'm a polite man."],
            )?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["Look, it's the picture book of Mammi I told you about."])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mami01"), Val::from(4)])?;
            ctx.lines_as("Benjamin", args!["Ah... Look at this picture of her wearing the rolled-up puppy ears! Doesn't she look like a sad puppy wandering Comodo, the City of Lovers, all by herself?!", "How could no one shed a tear after watching this sad picture of her?! Argh, Mammi! I'm coming to make you the happiest woman in the world!"])?;
            ctx.next()?;
            ctx.call(Function::Cutin, vec![Val::from("mami02"), Val::from(4)])?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["'She looks different than I thought with these glasses and pony tails... She really looks like someone who loves literature.", "Wait, when did people start calling Comodo the City of Lovers? Hmm... This guy really thinks weird things!'"])?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["I've decided to give you this <Angel Mammi's Special picture book: In Comodo> which is full of her beautiful pictures in return of your favor. Ahem... What do you say?"])?;
            ctx.next()?;
            ctx.mes("^660000He is staring at you arrogantly. Obviously, he doesn't care if you like the book or not.^000000")?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I'll take it.:No, thanks.")])? {
                1 => {
                    ctx.mes("'^660000Well, if it's for free, and there's no harm keeping it.'^000000")?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["No, thanks. I'm not interested in her as much as you."],
                    )?;
                    ctx.next()?;
                    ctx.mes("...^660000Was what you were about to say, but you could not refuse his offer. Looking at his fanatical eyes, burning with love for her, even Odin could not refuse...^000000")?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines(args![
                "^660000Quickly blinking his eyes, he made you take Mammi's picture book in your arms",
                "demanding that you must encase the book so it would not be damaged.^000000"
            ])?;
            ctx.var("barmunt_crow").set(Val::from(11))?;
            ctx.call(Function::GetItem, vec![Val::from(7795), Val::from(1)])?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["Oh, right. If you're interested in this author, Oliver Hilpert, I suggest you read the prequel of <The Crow of the Fate>, ^3131FF<The Trace of the Fate>^000000 as well as the sequel."])?;
            ctx.next()?;
            ctx.lines_as("Benjamin", args!["That was also a bestseller."])?;
            ctx.next()?;
            ctx.lines_as(
                "Benjamin",
                args!["Now I need to start reading this book for the moment that I'll meet my Mammi, so please leave me alone."],
            )?;
            ctx.next()?;
            ctx.mes("^660000As soon as he finished, he returned to escaping reality by indulging himself in his dreams of Mammi.^000000")?;
            ctx.next()?;
            ctx.lines(args![
                "^660000The Trace of the Fate, huh?",
                "Well, I don't have anything else to do. I might want to read the book...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Wait, does it mean that I have to go back to Juno Library?"],
            )?;
            ctx.next()?;
            ctx.mes("^660000Thinking of travelling back to Juno made you sigh in frustration.^000000")?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2067), Val::from(2068)])?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("barmunt_crow").get()? == 11 {
            ctx.mes("^660000You have embarked on a journey to Juno to read the prequel of <The Crow of the Fate>: <The Trace of the Fate>.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("barmunt_crow").get()? == 15 {
            if (ctx.call(Function::CountItem, vec![Val::from(7797)])? == 1 && ctx.call(Function::CountItem, vec![Val::from(7796)])? == 1) {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Hey, Benjamin!",
                        "I've got the perfect thing to help you appeal to Mammi. Do you want to see?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Benjamin", args!["WH... WHAT?!", "What is they? Show me!?"])?;
                ctx.next()?;
                ctx.mes("^660000You have shown him an autograph and a written note by Oliver Hilpert.^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Benjamin",
                    args!["Oh, my god!", "These will surely help me get closer to Mammi!", "Give them to me!"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give:Don't Give")])? {
                    1 => {
                        ctx.lines_as(
                            "Benjamin",
                            args![
                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "You are the best! Thanks!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Benjamin",
                            args![
                                "Maaaammmiii!",
                                "I'm going to see her tomorrow as soon as the morning comes! Mammi, I'm coming!"
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(7797), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7796), Val::from(1)])?;
                        ctx.var("barmunt_crow").set(Val::from(16))?;
                        ctx.call(Function::GetExperience, vec![Val::from(900000), Val::from(900000)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Benjamin", args!["Huh? What, are you kidding me?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as("Benjamin", args!["Oh... Mammi... Oh..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as("Benjamin", args!["Oh... Mammi... Oh..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.mes("With dreamy eyes, the man is looking down at a book in his hand.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn book_touching_man_garas(ctx: &Ctx) -> Script {
    book_touching_man_garas_body(ctx, Vec::new()).map(|_| ())
}

fn library_curator_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("barmunt_crow").get()? == 0 {
        ctx.lines_as("Curator Guys", args!["Our library's Monster Encyclopedia has every monster in the Rune-Midgarts Kingdom categorized by dungeon, to help our readers find them easily.", "We also have many essential books for adventurers. Why don't you take a look?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Curator Guys",
            args![
                "The other library across the street also has Monster Encyclopedias.",
                "If you're interested, feel free to drop by that library as well."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 1 {
        ctx.lines_as("Curator Guys", args!["Our library's Monster Encyclopedia has every monster in the Rune-Midgarts Kingdom categorized by dungeon, to help our readers find them easily.", "We also have many essential books for adventurers. Why don't you take a look?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Search books.:Look around the library.")])? {
            1 => {
                ctx.lines_as(
                    "Curator Guys",
                    args![
                        "Do you have a specific book in mind?",
                        "No problem, I'm here to assist you.",
                        "What kind of book are you looking for?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Say the title.:Say the author.:Search by keyword.")])? {
                    1 => {
                        ctx.lines_as(
                            "Curator Guys",
                            args!["Oh, do you know the title?", "Sure, now what's the name?"],
                        )?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        ctx.next()?;
                        if runtime::compare(&l_input_s.clone(), &Val::from("The Crow of the Fate")) == 1 {
                            ctx.lines_as(
                                "Curator Guys",
                                args![((Val::from("") + l_input_s.clone()) + Val::from("...?")), "Alright, let me see..."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as("Curator Guys", args!["Hmm... I'm sorry. I've worked at this place for a few decades, but I don't think I've ever seen such a book."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            "Curator Guys",
                            args!["Oh, do you know the author?", "Sure, now what's the name?"],
                        )?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        ctx.next()?;
                        if runtime::compare(&l_input_s.clone(), &Val::from("Oliver Hilpert")) == 1 {
                            ctx.lines_as(
                                "Curator Guys",
                                args![((Val::from("") + l_input_s.clone()) + Val::from("...?")), "Alright, let me see..."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as("Curator Guys", args!["Hmm... I'm sorry. I've worked at this place for a few decades, but I don't think I've ever heard of such an author."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    3 => {
                        ctx.lines_as(
                            "Curator Guys",
                            args![
                                "You must be having a hard time remembering the book's title or author.",
                                "No problem; why don't you tell me at least a little bit of what you remember?"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        if (((runtime::compare(&l_input_s.clone(), &Val::from("Fate")) == 1
                            || runtime::compare(&l_input_s.clone(), &Val::from("Crow")) == 1)
                            || runtime::compare(&l_input_s.clone(), &Val::from("Oliver")) == 1)
                            || runtime::compare(&l_input_s.clone(), &Val::from("Hilpert")) == 1)
                        {
                            ctx.lines_as(
                                "Curator Guys",
                                args![((Val::from("") + l_input_s.clone()) + Val::from("...?")), "Alright, let me see..."],
                            )?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as("Curator Guys", args!["Hmm... I'm sorry. I've worked at this place for a few decades, but I don't think I've ever seen a book whose name is even close to the words you said."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    _ => {}
                }
                ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                ctx.lines_as("Curator Guys", args!["Oh, right! I think I know the book:"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Curator Guys",
                    args!["^3131FFOliver Hilpert's", "<The Crow of the Fate>^000000!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["That's right! That's what I'm looking for!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Curator Guys", args!["Haha... But I have bad news for you.", "The book has become so popular that you can't even find it from Schwarzwald, the author's country.", "We tried to get a hold of the book, but it was too late. It'll take a while for the next editon to be published, so why don't you read a different book for now?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Curator Guys",
                    args![
                        "If you are not busy and really want to read the book, I have a suggestion: try Juno Library.",
                        "You can find every book published in Schwarzwald from the library. Chances are, you'll find it there."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Curator Guys", args!["Well, Juno is quite far from here, but if you're really enthusiastic about reading such a popular book, it should be worthwile."])?;
                ctx.next()?;
                ctx.mes(
                    "^660000The book isn't for you, but you believe in 100% customer satisfaction. Let's go to Juno Library now.^000000",
                )?;
                ctx.var("barmunt_crow").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2063), Val::from(2064)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Curator Guys",
                    args![
                        "Each book tells you a story of life, wisdom, happiness, and sensation.",
                        "Look around! They are the witnesses of mankind. And everyday, we have new ones joining them."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("barmunt_crow").get()? == 2 {
        ctx.lines_as("Curator Guys", args!["^3131FFThe Crow of the Fate^000000 was sold out even before we got a hold of one copy. You won't be able to find the book anywhere in the Rune-Midgarts Kingdom.", "If you really want to read the book, you should try Juno Library."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Curator Guys", args!["Our library's Monster Encyclopedia has every monster in the Rune-Midgarts Kingdom, categorized by dungeon to help the readers find them easily.", "We also have many essential books for adventurers. Why don't you take a look?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Curator Guys",
            args![
                "The other library across the street also has Monster Encyclopedias.",
                "If you're interested, feel free to drop by that library as well."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn library_curator_garas(ctx: &Ctx) -> Script {
    library_curator_garas_body(ctx, Vec::new()).map(|_| ())
}

fn library_master_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("barmunt_crow").get()? == 2 {
        ctx.lines_as("Dog", args!["Bowwow!", "Grrr... Bowwow!"])?;
        ctx.next()?;
        ctx.mes("^660000A dog is barking loudly at the front of the library.^000000")?;
        ctx.next()?;
        ctx.lines_as("Library Master", args!["You damn bird! Fly away, leave!"])?;
        ctx.next()?;
        ctx.mes("^660000A man is also yelling at the front of the library.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What's happening here?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args![
                "It's nothing, but there's a gigantic crow sitting on the roof of this library, crowing for at least a few hours now.",
                "I was trying to drive it away because... It feels ominous somehow."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Library Master", args!["Go! Fly away, you damn bird! Leave!!"])?;
        ctx.next()?;
        ctx.mes("^660000You looked up, and found a huge crow sitting on the roof of the library as if it owned the place.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args![
                "Leave! Gosh, it won't go away!",
                "I'm afraid that creepy bird will scare away my patrons!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args!["Umm... Oh, well.", "(Grabbing a pebble)", "I said go away, you stupid bird!"],
        )?;
        ctx.next()?;
        ctx.mes("(WHIZZ-)")?;
        ctx.next()?;
        ctx.mes("(THUD!)")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PIERCE")?])?;
        ctx.next()?;
        ctx.mes("(Gush)")?;
        ctx.next()?;
        ctx.lines(args![
            "^660000Hit by the pebble which the man threw, the crow quickly flew up into the air.^000000",
            "^660000But then it started flying above the roof in a circle, and looked even spookier.^000000"
        ])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BAT")?])?;
        ctx.next()?;
        ctx.lines_as("Library Master", args!["Grrr... Can't you just go away?!"])?;
        ctx.next()?;
        ctx.lines(args![
            "^660000He started yelling at the crow, but it didn't seem to care. Instead, it flew up a little higher.",
            "Eventually he turned his head away from the crow, and gave up on driving it away. Then the dog stopped barking as well.^000000"
        ])?;
        ctx.next()?;
        ctx.mes("^660000You looked down on the ground, and then found a couple of feathers dropped from the crow when it was hit by the pebble.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Wow, look at the size of these feathers. I guess such a huge crow needs big feathers to fly."],
        )?;
        ctx.next()?;
        ctx.lines_as("Library Master", args!["When I was young, I heard that bird feathers bring Odin's servants to bless their owner, and so many poets used them as bookmarks."])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args!["I used to have a small feather. I guess it's only natural that I've become a library master. Hahaha!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args![
                "You know the old saying, 'you can memorize Edda after 3 years of working at a library.'",
                "Why don't you keep one of the feathers? Who knows? It may bring you good luck."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I guess so."])?;
        ctx.next()?;
        ctx.lines(args![
            "^660000You picked up one of the big dark feathers from the ground.",
            "Looking at the lustrous feather somehow made you feel so happy, as if you had obtained treasure.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Ahchoo!"])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args![
                "God bless you. Oh, did you catch cold?",
                "You should take good care of yourself, especially if you're an adventurer."
            ],
        )?;
        ctx.next()?;
        ctx.mes("^660000You started feeling cold and began to shiver.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            "Library Master",
            args![
                "Try to drink hot Jellopy broth mixed with honey. My mother used to make that for me whenever I caught cold.",
                "I gurantee you that you'll get better quickly."
            ],
        )?;
        ctx.next()?;
        ctx.mes("^660000He sounded sincere, but it doesn't sound like that kind of broth would even be effective...^000000")?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.var("barmunt_crow").set(Val::from(3))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Library Master", args!["Be careful to not catch cold!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn library_master_garas(ctx: &Ctx) -> Script {
    library_master_garas_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GarasCatchStep {
    Start,
    OnTouch,
}

fn garas_catch_run(ctx: &Ctx, mut step: GarasCatchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GarasCatchStep::Start => {
                step = GarasCatchStep::OnTouch;
                continue 'machine;
            }
            GarasCatchStep::OnTouch => {
                if ctx.var("barmunt_crow").get()? == 2 {
                    ctx.lines_as("Dog", args!["Bowwow!", "Grrr... Bowwow!"])?;
                    ctx.next()?;
                    ctx.mes("^660000A dog is barking loudly at the front of the library.^000000")?;
                    ctx.next()?;
                    ctx.lines_as("Library Master", args!["You damn bird! Fly away, leave!"])?;
                    ctx.next()?;
                    ctx.mes("^660000A man is also yelling at the front of the library.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What's happening here?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Library Master", args!["It's nothing, but there's a gigantic crow sitting on the roof of this library, crowing for at least a few hours now.", "I was trying to drive it away because... It feels ominous somehow."])?;
                    ctx.next()?;
                    ctx.lines_as("Library Master", args!["Go! Fly away, you damn bird! Leave!!"])?;
                    ctx.next()?;
                    ctx.mes(
                        "^660000You looked up, and found a huge crow sitting on the roof of the library as if it owned the place.^000000",
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args![
                            "Leave! Gosh, it won't go away!",
                            "I'm afraid that creepy bird will scare away my patrons!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args!["Umm... Oh, well.", "(Grabbing a pebble)", "I said go away, you stupid bird!"],
                    )?;
                    ctx.next()?;
                    ctx.mes("(WHIZZ-)")?;
                    ctx.next()?;
                    ctx.mes("(THUD!)")?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PIERCE")?])?;
                    ctx.next()?;
                    ctx.mes("(Gush)")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^660000Hit by the pebble which the man threw, the crow quickly flew up into the air.^000000",
                        "^660000But then it started flying above the roof in a circle, and looked even spookier.^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BAT")?])?;
                    ctx.next()?;
                    ctx.lines_as("Library Master", args!["Grrr... Can't you just go away?!"])?;
                    ctx.next()?;
                    ctx.lines(args!["^660000He started yelling at the crow, but it didn't seem to care. Instead, it flew up a little higher.", "Eventually he turned his head away from the crow, and gave up on driving it away. Then the dog stopped barking as well.^000000"])?;
                    ctx.next()?;
                    ctx.mes("^660000You looked down on the ground, and then found a couple of feathers dropped from the crow when it was hit by the pebble.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Wow, look at the size of these feathers. I guess such a huge crow needs big feathers to fly."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Library Master", args!["When I was young, I heard that bird feathers bring Odin's servants to bless their owner, and so many poets used them as bookmarks."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args!["I used to have a small feather. I guess it's only natural that I've become a library master. Hahaha!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args![
                            "You know the old saying, 'you can memorize Edda after 3 years of working at a library.'",
                            "Why don't you keep one of the feathers? Who knows? It may bring you good luck."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I guess so."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^660000You picked up one of the big dark feathers from the ground.",
                        "Looking at the lustrous feather somehow made you feel so happy, as if you had obtained treasure.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Ahchoo!"])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args![
                            "God bless you. Oh, did you catch cold?",
                            "You should take good care of yourself, especially if you're an adventurer."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000You started feeling cold and began to shiver.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Library Master",
                        args![
                            "Try to drink hot Jellopy broth mixed with honey. My mother used to make that for me whenever I caught cold.",
                            "I gurantee you that you'll get better quickly."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000He sounded sincere, but it doesn't sound like that kind of broth would even be effective...^000000")?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_THINK")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.var("barmunt_crow").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn garas_catch(ctx: &Ctx) -> Script {
    garas_catch_run(ctx, GarasCatchStep::Start, Vec::new()).map(|_| ())
}

pub fn garas_catch_ontouch(ctx: &Ctx) -> Script {
    garas_catch_run(ctx, GarasCatchStep::OnTouch, Vec::new()).map(|_| ())
}

fn garas_eff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn garas_eff(ctx: &Ctx) -> Script {
    garas_eff_body(ctx, Vec::new()).map(|_| ())
}

fn dog_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Dog", args!["Bowwow!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dog_garas(ctx: &Ctx) -> Script {
    dog_garas_body(ctx, Vec::new()).map(|_| ())
}

fn library_part_timer_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("barmunt_crow").get()? == 3 {
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "People should learn to put away books after pulling them out.",
                "All these books piled up in the cart make me feel so frustrated, you know? Sigh..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Library Part-Timer", args!["...Now how may I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I'm looking for a book called <The Crow of the Fate> written by Oliver Hilpert... Ahchoo!"],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_KEK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "Ah, I remember that one... It's the one most frequently left in the cart.",
                "Please enter the room on the ^3131FFright side^000000. You can find it in the Bestseller corner."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Thanks."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 7 {
        ctx.lines_as("Library Part-Timer", args!["How may I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Yes, umm... ahchoo! Where can I find old news articles...? Ahchoo!"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "Oh, you can find them in a corner of the right room.",
                "Please be careful when you handle them since most of them are pretty ancient."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Thanks... Ahchoo!"])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 11 {
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "People should learn to put away books after pulling them out.",
                "All these books piled up in the cart make me feel so frustrated, you know? Sigh..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Library Part-Timer", args!["...How may I help you?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Cough... I'm looking for a book called <The Crow of the Fate> written by Oliver Hilpert."],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "Ah, I remember that one... It's the one most frequently left in the cart.",
                "Go ^3131FFupstairs^000000, and check the left shelf."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Thanks."])?;
        ctx.next()?;
        ctx.lines_as("Library Part-Timer", args!["Oliver Hilpert has attained huge fame despite his young age.", "I've read his <The Trace of the Fate> at least several times. I mean, that book is so captivating that I can never get tired of reading it."])?;
        ctx.next()?;
        ctx.lines_as("Library Part-Timer", args!["Umm... The most impressive scene was... Err..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Library Part-Timer",
            args!["Right! When the hero was meeting his end in the burning mansion... ^FF0000-- BEEP --^000000"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BASH3D2")?])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(264), Val::from(186)])?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 12 {
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "Excuse me, are you okay? I mean, you just kind of drifted off all of a sudden.",
                "You look a little pale."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Library Part-Timer",
            args![
                "Why don't you go inside and take a rest? Read <The Trace of the Fate> or something.",
                "You can find the book at the left bookshelf upstairs."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn library_part_timer_garas(ctx: &Ctx) -> Script {
    library_part_timer_garas_body(ctx, Vec::new()).map(|_| ())
}

fn hot_bestseller_corner_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("barmunt_crow").get()? == 3 {
        ctx.mes("^660000The countless number of books filling this room tell you why this place is called the Greatest Library of Juno, the City of Scholars.^000000")?;
        ctx.next()?;
        ctx.mes("^660000Books tagged as 'Bestseller of The Month' are stored in the middle of the shelf that's at your eye level.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^660000It was not that difficult to find 'The Crow of the Fate'",
            "because it was the only book leaning against the wall of an empty shelf.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ah, luckily there's one... Ahchoo!"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Sniff... So now all I have to do is just check this out and deliver it to that Mammi fanatic."],
        )?;
        ctx.next()?;
        ctx.mes("^FF0000-- BEEP --^000000")?;
        ctx.next()?;
        ctx.mes("^660000You were about to pull out the book, complaining and grumbling, when suddenly it felt as if your brain exploded. You black out...^000000")?;
        ctx.var("barmunt_crow").set(Val::from(4))?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BASH3D2")?])?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(247), Val::from(33)])?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 4 {
        ctx.mes("^660000The countless number of books filling this room tell you why this place is called the Greatest Library of Juno, the City of Scholars.^000000")?;
        ctx.next()?;
        ctx.mes("^660000Books tagged as 'Bestseller of The Month' are stored in the middle of the shelf that's at your eye level.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^660000It was not that difficult to find 'The Crow of the Fate'",
            "because it was the only book leaning against the wall of an empty shelf.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ah, luckily there's one... Ahchoo!"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Sniff... So now all I have to do is just check this out and deliver it to that Mammi fanatic."],
        )?;
        ctx.next()?;
        ctx.mes("^FF0000-- BEEP --^000000")?;
        ctx.next()?;
        ctx.mes("^660000You were about to pull out the book, complaining and grumbling, when suddenly it felt as if your brain exploded. You black out...^000000")?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BASH3D2")?])?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(247), Val::from(33)])?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 5 {
        ctx.mes("^660000The countless number of books filling this room tell you why this place is called the Greatest Library of Juno, the City of Scholars.^000000")?;
        ctx.next()?;
        ctx.mes("^660000Books tagged as 'Bestseller of The Month' are stored in the middle of the shelf that's at your eye level.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^660000It was not that difficult to find 'The Crow of the Fate'",
            "because it was the only book leaning against the wall of an empty shelf.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ah, luckily there's one... Ahchoo!"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Sniff... So now all I have to do is just check this out and deliver it to that Mammi fanatic."],
        )?;
        ctx.next()?;
        ctx.mes("^FF0000-- BEEP --^000000")?;
        ctx.next()?;
        ctx.mes("^660000You were about to pull out the book, complaining and grumbling, when suddenly it felt as if your brain exploded. You black out...^000000")?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BASH3D2")?])?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(247), Val::from(33)])?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hot_bestseller_corner(ctx: &Ctx) -> Script {
    hot_bestseller_corner_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GarasPathStep {
    Start,
    OnTouch,
}

fn garas_path_run(ctx: &Ctx, mut step: GarasPathStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GarasPathStep::Start => {
                step = GarasPathStep::OnTouch;
                continue 'machine;
            }
            GarasPathStep::OnTouch => {
                if ctx.var("barmunt_crow").get()? == 6 {
                    ctx.mes("...................................")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "...................................",
                        "..................................."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "...................................",
                        "...................................",
                        "..................................."
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^660000You are still inside the library,",
                        "but you do not know if this is a dream or reality.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.mes("^660000You turned your head toward the bookshelf, and found the bookshelf with the empty shelf.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Phew... I guess I had a dream or something... Ahchoo!"],
                    )?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^660000It was an indescribable dream: a burning laboratory in an unknown place in Schwarzwald.",
                        "You tried to think hard, but could not figure out what the dream meant.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.mes("^660000You carefully opened the book again, but nothing happened.^000000")?;
                    ctx.next()?;
                    ctx.mes("<....at the end of the sooty fog and chaotic screams, there was a creature that looked too outrageous to be a human...>")?;
                    ctx.next()?;
                    ctx.mes("...................................")?;
                    ctx.next()?;
                    ctx.lines(args!["<- Eva, what are you doing? Come over here! Hurry!-", "-I can't leave him here!-", "Ignoring the collapsing laboratory ceiling, Eva approached a test tube, carefully took out something, and then put it under her jacket. >"])?;
                    ctx.next()?;
                    ctx.mes("^660000...................................^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000The books was telling the exact scene that you have seen in your dream.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Deja Vu? Am I having Deja Vu?"],
                    )?;
                    ctx.next()?;
                    ctx.mes("^660000Of course, it was just a dream, but still was a very mysterious experience.^000000")?;
                    ctx.next()?;
                    ctx.mes("...................................")?;
                    ctx.next()?;
                    ctx.mes("...................................")?;
                    ctx.next()?;
                    ctx.lines(args!["'^660000Now I wonder if the scene is describing an accident that really happened in the past. If there was such big fire, I'm sure I can verify it in a newspaper.", "...I just want to know it out of curiosity...^000000'"])?;
                    ctx.var("barmunt_crow").set(Val::from(7))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2064), Val::from(2065)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("barmunt_crow").get()? == 7 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["If I want to read news articles about fires, which section should I go to?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn garas_path(ctx: &Ctx) -> Script {
    garas_path_run(ctx, GarasPathStep::Start, Vec::new()).map(|_| ())
}

pub fn garas_path_ontouch(ctx: &Ctx) -> Script {
    garas_path_run(ctx, GarasPathStep::OnTouch, Vec::new()).map(|_| ())
}

fn old_news_scrapbook_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_newspp = Val::from(0);
    if ctx.var("barmunt_crow").get()? == 7 {
        ctx.mes("^660000It is a folder with an wooden frame holding a thick pile of old newspapers.^000000")?;
        ctx.next()?;
        ctx.mes("^660000You carefully looked through the newspapers to see if you could find any articles about fire accidents.^000000")?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                ctx.mes("...................................")?;
                ctx.next()?;
                ctx.mes("...................................")?;
                ctx.next()?;
                l_newspp = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                if (l_newspp.clone() == 1 || l_newspp.clone() == 2) {
                    ctx.lines(args![
                        "- Page 1 -",
                        "Renowned Assassin Sieglinde: 'I serve you only.'",
                        " ",
                        " Sieglinde has attained the title of Greatest Assassin",
                        "for his professionalism",
                        "and fame among ladies,",
                        "for his bad-boy image characterized by his sharpened Katar:",
                        "a handsome man that lives a life of romantic loneliness.",
                        "Recently this Assassin-Turned-Into-Celebrity has made",
                        "a true confession with a smile on his face:",
                        "he has been secretly going out with a lady,",
                        "and they are planning to wed next year.",
                        " The lady in question is known as Priestess M,",
                        "and she works in Prontera Church, but he refused to give us more information",
                        "about his fiancee for the sake of her safety.",
                        "As soon as the news spread,",
                        "his female fans showed a negative reaction",
                        "saying, 'This is ridiculous.",
                        "Assassins must live alone under the shadows.'",
                        "Hopefully the fans' reaction won't cause too much worry",
                        "to Siglinde and his soon-to-be wife.",
                        " "
                    ])?;
                    ctx.next()?;
                } else if (l_newspp.clone() == 3 || l_newspp.clone() == 4) {
                    ctx.lines(args![
                        "- Page 4 -",
                        "Fight Scene in Juno",
                        " ",
                        "Last night, two young men were witnessed",
                        "violently fighting each other in Juno Plaza.",
                        "According to the Juno soldier that arrested them,",
                        "Mr. B had not been so happy with Mr. A's",
                        "careless behavior, and it seems",
                        "that his patience finally ran out last night.",
                        "It seems the fight started when Mr. A put his bare sweaty feet on Mr. B's thigh,",
                        "saying that his feet are too sweaty.'",
                        "Blame the hot weather, people!",
                        " "
                    ])?;
                    ctx.next()?;
                } else if l_newspp.clone() == 5 {
                    ctx.lines(args![
                        "God's Warning: A Secret Lab Reduced to Ashes",
                        " ",
                        " The smoke clouding the sky of Juno last night",
                        "turned out to be from",
                        "a secret laboratory set on fire.",
                        " More information about the laboratory",
                        "has not yet been released, but according to the witnesses,",
                        "the laboratory was found empty",
                        "without anybody or anything inside",
                        "except broken pieces of test tubes",
                        "which proved that the place was used as a laboratory.",
                        " Scientist Sion, the accident scene investigator,",
                        "suspects from the remnants of machines inside",
                        "that the laboratory must have been used",
                        "to test biotechnology projects,",
                        "and expressed his belief that the fire was the wrath of God,",
                        "righteous punishment for trifling with his creations.",
                        " "
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "...........................",
                            "...........................",
                            "This is the place I've seen in my dream!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("Wait, then what about the place where they escaped...?")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "You quickly checked the date of the news. It was too long ago",
                        "for you to find any more information from searching news articles."
                    ])?;
                    ctx.next()?;
                    ctx.mes("What if they have any descendents or successors?")?;
                    ctx.next()?;
                    ctx.lines(args!["You feel your heart beating faster with excitement.", "You are now convinced this was not just your dream or illusion. You had a vision of what really happened in the past."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I can't believe this is happening!",
                            "Think hard... It was ^FF0000an area connected to a northern cave^000000."
                        ],
                    )?;
                    ctx.var("barmunt_crow").set(Val::from(8))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2065), Val::from(2066)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (l_newspp.clone() == 6 || l_newspp.clone() == 7) {
                    ctx.lines(args![
                        "- Culture Page -",
                        "- Please Save My Water -",
                        " ",
                        "A recently acclaimed young author,",
                        "Jean Cadoc's new series",
                        "'Please Save My Water' has been ranked at the top",
                        "for the most checked-out book in libraries.",
                        "'Please Save My Water' is about a girl called Ujer",
                        "who was blessed by Mother Nature.",
                        "Her father left a will on his deathbed",
                        " to 'protect the water' against the evil,",
                        "and she is fighting against a group of evil villains",
                        "that are contaminating nature so that she can purify the water.",
                        "This book will be soon selected as an essential academic book of",
                        "the Schwarzwald Republic for",
                        "promoting morality and",
                        "promoting environmental protection throughout the nation.",
                        " "
                    ])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        "Einbech Mine Collapsed (2)",
                        " ",
                        "According to an employee of Rekenber Corporation,",
                        "which has been leading Einbech's mining business,",
                        "three dead bodies have been discovered,",
                        "but one survivor was admitted into a hospital.",
                        "He is in a critical condition,",
                        "and also informed us that they are",
                        "still searching for survivors,",
                        "and have hired specialists to investigate",
                        "the cause of the accident.",
                        " "
                    ])?;
                    ctx.next()?;
                }
            }
        }
    } else {
        ctx.mes("^660000It is a folder with a wooden frame holding a thick pile of old newspapers.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn old_news_scrapbook(ctx: &Ctx) -> Script {
    old_news_scrapbook_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_man_oliver_h_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(617), Val::from(3)])? == 0 {
        ctx.mes("- You have too many items in your inventory to proceed with this quest. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("barmunt_crow").get()?.number()? > 14 {
        ctx.mes("^660000The writer was writing in the notebook at an extremely fast pace.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Let's not disturb him, especially when he is full of creative ideas. That doesn't happen very often."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 13 {
        ctx.call(Function::Cutin, vec![Val::from("oliver_pre"), Val::from(2)])?;
        if ctx.call(Function::CountItem, vec![Val::from(7795)])?.number()? < 1 {
            ctx.lines_as("Oliver Hilpert", args!["Lady Mammi..."])?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Oliver Hilpert",
            args!["Did you change your mind about selling Lady Mammi's picture book to me?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
            1 => {
                ctx.call(Function::Cutin, vec![Val::from("oliver_smile"), Val::from(2)])?;
                ctx.lines_as("Oliver Hilpert", args!["Wow, thanks!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Oliver Hilpert",
                    args!["Umm... I haven't received my publishing advance yet, but do you mind taking this instead?"],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as("Oliver Hilpert", args!["Okay... I see..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as("Oliver Hilpert", args!["I haven't had a chance to open it, but my publisher said it has something very rare inside. I hope that it's enough to pay you for your book."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Haha, thanks.", "You know, in Morocc... Ahchoo! ...There's someone who's as big a fan of Mammi's as you... Ahchoo! So why don't you go meet him?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I got that book from him as a gift. He may have more rare books of her... Ahchoo!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["Oh, thank you so much for such valuable information!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["Wah! I must write down the story that I saw in my dream last night before I forget!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["I hope you'll also like my next book. Then I must go... Thank you for the picture book!"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_smile"), Val::from(255)])?;
        ctx.lines(args!["^660000He started running around like a chicken with its head cut off. He quickly opened his bag, took out his notebook, ran to a table, and then started writing down something at a fast speed.", "At first, he looked pretty silly, but now he strikes you as a man that's very serious about his writing.^000000"])?;
        ctx.next()?;
        ctx.mes("^660000You were about to leave when you found a piece of paper on the ground.^000000")?;
        ctx.next()?;
        ctx.mes("^660000The piece of paper is covered with scribbles, and is labeled <The Crow of the Fate> at the top. Oliver must have dropped this note containing information about his novel, <The Crow of the Fate>.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3131FFAncient weapon = Some kind of power source^000000",
            "^3131FFSeclusion - A female disciple's letter^000000",
            "^3131FFThe stepmother = Lover from a past life?!^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ENHANCE")?])?;
        ctx.call(Function::DelItem, vec![Val::from(7795), Val::from(1)])?;
        ctx.var("barmunt_crow").set(Val::from(14))?;
        ctx.call(Function::GetItem, vec![Val::from(7796), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(7797), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(270), Val::from(270)])?;
        return Err(Stop::End);
    } else if ctx.var("barmunt_crow").get()? == 14 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This dream seems pretty meaningful somehow.", "I'd better give him this note back."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, Mr. Hilpert."],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_think"), Val::from(2)])?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![".................", ".................", "...........Huh?"],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_think"), Val::from(255)])?;
        ctx.mes("You felt sorry for interrupting him from writing.")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I think you've dropped this."],
        )?;
        ctx.next()?;
        ctx.mes("^660000You rummaged your pocket to find the memo, and then happened to drop the crow feather which you picked up outside the library. Then suddenly...^000000")?;
        ctx.next()?;
        ctx.mes("(WHACK)")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_DETOXICATION")?])?;
        ctx.next()?;
        ctx.mes("^660000Oliver picked up the feather more quickly than you could, and then tore it into pieces before you could even say anything.^000000")?;
        ctx.next()?;
        ctx.mes("^660000You looked angrily at him, and found out that he turned into a completely different person; he no longer looked stupid or serious but extremely coldhearted.^000000")?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_hum"), Val::from(2)])?;
        ctx.lines_as("Oliver Hilpert", args!["Errr?!", "What's happened to this feather?"])?;
        ctx.next()?;
        ctx.mes("[Oliver Hilpert]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_KEK")?])?;
        ctx.lines(args!["Wah! Did I do this?", "Oh my god, I'm sorry! I'm so sorry!"])?;
        ctx.next()?;
        ctx.mes("^660000He changed back to himself, and started apologizing to you for his sudden rage.^000000")?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![
                "I... I don't know how to apologize...",
                "It's just that... I became so angry that I--! Argh, I'm sorry!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Well, it's okay. I happened to pick it up from the street. You don't have to apologize."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["If you say so, thank you for your understanding.", "Phew, I felt so guilty..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Here, take your memo back."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["Oh, you can just throw it away. It's no longer useful.", "Phew..."],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_pre"), Val::from(2)])?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["I'm having great ideas right now. I'm sorry, but I should go."],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_pre"), Val::from(255)])?;
        ctx.mes("^660000As soon as he talked to you, he went back to his writing and became completely absorbed in his work.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "He has such amazing concentration.",
                "I guess not everyone can make bestselling books, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "...Wait, I'm not sneezing anymore...",
                "The headache and the heavy feeling in my chest is gone too!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What if it was all caused by that crow feather? What if...?"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("It was cursed by the pebble.:The crow has something to do with Oliver.")],
        )? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Ah, I see.",
                        "I think when the crow was hit by the pebble, its wisdom must have turned into disaster."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Argh, that was all because of the library master!",
                        "Luckily the feather is gone, so hopefuly I won't suffer any more disaster."
                    ],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["I remember Mr. Zid saying Eva had left a black feather behind her."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Also, the female assistant turned into a crow flying away, and then the crow feather made me sneeze and cough."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![".................."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Yes, Mr. Hilpert's soul is cursed by the Crow!",
                        "I wonder if he threw rocks at crows like the library master..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["..................", "I guess it's a stupid idea."],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh well, whatever.",
                "What matters is that I'm cured!",
                "If I have another dream, I'll deal with it then~"
            ],
        )?;
        ctx.next()?;
        ctx.mes("^660000As you left the library, you thought that thinking Mr. Oliver is your half would be a better idea.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That was pretty fun, but also pretty tiring. Where should I head for my next adventure?"],
        )?;
        ctx.var("barmunt_crow").set(Val::from(15))?;
        ctx.call(Function::CompleteQuest, vec![Val::from(2068)])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        ctx.mes("^660000A suspicious-looking young man is searching passing faces as if he is looking for someone.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn suspicious_man_oliver_h(ctx: &Ctx) -> Script {
    suspicious_man_oliver_h_body(ctx, Vec::new()).map(|_| ())
}

fn worn_out_book_garas_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(617), Val::from(3)])? == 0 {
        ctx.mes("- You have too many items in your inventory to proceed with this quest. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("barmunt_crow").get()? == 12 {
        if ctx.call(Function::CountItem, vec![Val::from(7795)])?.number()? < 1 {
            ctx.mes("- You felt as if you have left something important behind. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("^660000The book's cover was seriously damaged, considering its recent publishing date. Many people must have turned its pages.^000000")?;
        ctx.next()?;
        ctx.mes("^660000You carefully turned the first page, worrying that you might have another crazy dream.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            ".........................",
            ".........................",
            "........................."
        ])?;
        ctx.next()?;
        ctx.lines(args!["< Since he was young - it was before he could even remember exactly how young he was - his dreams have filled his brain with immense knowledge that he had never seen or learned from anywhere else.", "He was a little boy outside, but wiser than any grown sage. He spoke many languages so fluently that they sounded like Edda chanted by Odin.", "To him, his small and dark underground village was a prison blocking him from satisfying his curiosity and desire for freedom.", "He expressed his desire to learn more knowledge of the outside world to stepmother Eva, but she always ignored his wish with a stern look.", "Eventually, her unreasonable dissuasion could no longer stop him from escaping his reality.", " ", "- Gasp! Gasp! -", "He ran through the darkness which seemed to have no end. He was running away from everything that tried to confine him.>"])?;
        ctx.next()?;
        ctx.lines(args![
            ".........................",
            ".........................",
            "........................."
        ])?;
        ctx.next()?;
        ctx.lines(args!["< - Umm...-", "-...He's awake.-", "- Hello, can you see me? How do you feel? -", "An old man slowly walked into the room. With a smile on the face, he sat down on a chair. The little boy felt afraid for a little while after seeing people outside his village for the first time,", "but then he forced himself up from the bed, and looked around.", " ", "- Where am I? Why am I here? - ", "- I was riding a carriage last night, and found you lying in the street. My name is Haeji. You may call me Mr. Haeji, son.-", "-.......-", "- What's your name? -", "-......Al.-", "- Do you know why you're on the street? Where do you live? Your parents must be dying to see you.-", "Haeji's comment instantly brought the monsterous image of Eva to Al's mind. Inwardly, he screamed in fear.", "- I live nowhere, and I don't have parents. -", "Al had to lie no matter how long, he thought. The old man seemed to understand him: his eyes showed sympathy for the little boy. Haeji insisted that Al could stay in his mansion as long as he wanted.", "Later, after learning that Haeji was a professor of Juno City University, Al was convinced that the fates were on his side.>"])?;
        ctx.next()?;
        ctx.lines(args![
            ".........................",
            ".........................",
            "........................."
        ])?;
        ctx.next()?;
        ctx.mes("^660000You felt somebody staring at you reading the book. You turned your head, and found a dandy and yet suspicious-looking young man.^000000")?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_pre"), Val::from(2)])?;
        ctx.mes("^660000The young man approached and shook your hand with a grin on his face.^000000")?;
        ctx.next()?;
        ctx.lines_as("Suspicious-Looking Man", args!["Hey, you're reading <The Trace of the Fate>!", "Hahaha, nice to meet you. I'm the author of that book. I've come by to check how well my new book, <The Crow of the Fate>, is doing, and it's a great honor to meet a fan of mine! I feel embarrassed, but thanks! Hehe."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Cough, no, I'm not a fa..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["Oh, right! Do you want my autograph?"])?;
        ctx.next()?;
        ctx.mes("(Scribble)")?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["^660000He took out a big piece of paper from his bag, drew something really fast, and then handed it to you.^000000"],
        )?;
        ctx.next()?;
        ctx.mes("Haha, look. I've signed it as big as I could so that you can brag about this to your friends!")?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["You know, I just wrote what I saw in my dream; I didn't know people would love my stories so much! It's very surprising to me."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Dream?"])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_think"), Val::from(2)])?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![
                "Yes, I've been having dreams that are so vivid, they feel like reality.",
                "I was told great writers usually find their inspiration from their dreams. I guess I was born to be one of them. Hahaha!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["You felt something uncomfortable about this so-called dream. You wanted to understand the meaning of what you've seen while reading the book.", "If Oliver was dreaming exactly the same things as you, that might mean he and you are sharing dreams. Then again, you've discovered the cave village in the book actually exists."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["The hero Al was having dreams..."],
        )?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_pre"), Val::from(2)])?;
        ctx.lines_as("Oliver Hilpert", args!["Wah!", "Isn't that over there?!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Huh?"])?;
        ctx.next()?;
        ctx.mes("^660000You quickly turned your head following his eyes. There was nothing but your bag.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Ah... Ahchoo! Mr. Hilpert, are you interested in my bag? It's just a simple bag I got from the Novice Training Grounds..."
            ],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["No, no!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["I was looking at the thing shining under the cover of the bag!"],
        )?;
        ctx.next()?;
        ctx.mes("^660000You checked what the thing in your bag was, and it was...^000000")?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_smile"), Val::from(2)])?;
        ctx.lines_as("Oliver Hilpert", args!["Lady Mammi!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BIGTHROB")?])?;
        ctx.next()?;
        ctx.mes("^660000Yes, it was the picture book of Mammi, an idol whom Benjamin of Morocc insists to be one of the three greatest idols in the Rune-Midgarts Kingdom. That must be why Oliver is squealing in delight.^000000")?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_THINK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ahchoo! Are you also a Mammi fan?"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![
                "Are you kidding?! Of course!",
                "Tell me, where you can find another intelligent and beautiful woman like her?",
                "I believe she's the reincarnation of the Wisdom Goddess who was expelled to the Midgard Continent by jealous goddesses."
            ],
        )?;
        ctx.next()?;
        ctx.mes("^660000Mammi seems to have a strange level of popularity that you can never understand.^000000")?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["May I see?"])?;
        ctx.next()?;
        ctx.mes("^660000You have given him Mammi's picture book.^000000")?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["................."])?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["...Oh!"])?;
        ctx.call(Function::Cutin, vec![Val::from("mami01"), Val::from(4)])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["...Ooooh!"])?;
        ctx.call(Function::Cutin, vec![Val::from("mami02"), Val::from(4)])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_smile"), Val::from(2)])?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![
                "Ah... This is why I'm in love with Lady Mammi.",
                "Adventurer, you must be an enthusiastic fan of her as well. Where did you find this rare picture book?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["No, I got it as a gift from someone... Ahchoo!"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["What? Now I'm shocked!", "It's shocking to see you unimpressed by the beauty of Lady Mammi,", "but it is more shocking that God has given such a rare picture book of her", "to someone like you who does not appreciate its value!", "I flew over to the Rune-Midgarts Kingdom on the earliest airship", "to get this book on the day it was released, but the books were sold out. The person right in front of me got the last one!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...............", "I... I see..."],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SWEAT")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["Umm...", "If you don't really like that book, why don't you sell it to me?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args![
                "As I told you earlier, it's something I'm willing to cross continents to get. Please?",
                "I really want to buy your book!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sell:Don't Sell")])? {
            1 => {
                ctx.lines_as("Oliver Hilpert", args!["Wow, thanks!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Oliver Hilpert",
                    args!["Umm... I haven't received my publishing advance yet, but do you mind taking this instead?"],
                )?;
                ctx.next()?;
            }
            2 => {
                ctx.lines_as("Oliver Hilpert", args!["Okay... I see..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.var("barmunt_crow").set(Val::from(13))?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as("Oliver Hilpert", args!["I haven't had a chance to open it, but my publisher said it has something very rare inside. I hope that it's enough to pay you for your book."])?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Haha, thanks.", "You know, in Morocc... Ahchoo! ...There's someone who's as big a fan of Mammi's as you... Ahchoo! So why don't you go meet him?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I got that book from him as a gift. He may have more rare books of her... Ahchoo!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Oliver Hilpert", args!["Oh, thank you so much for such valuable information!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["Wah! I must write down the story that I saw in my dream last night before I forget!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Oliver Hilpert",
            args!["I hope you'll also like my next book. Then I must go... Thank you for the picture book!"],
        )?;
        ctx.call(Function::Cutin, vec![Val::from("oliver_smile"), Val::from(255)])?;
        ctx.next()?;
        ctx.lines(args!["^660000He started running around like a chicken with its head cut off. He quickly opened his bag, took out his notebook, ran to a table, and then started writing down something at a fast speed.", "At first, he looked pretty silly, but now he strikes you as a man that's very serious about his writing.^000000"])?;
        ctx.next()?;
        ctx.mes("^660000You were about to leave when you found a piece of paper on the ground.^000000")?;
        ctx.next()?;
        ctx.mes("^660000The piece of paper is covered with scribbles, and is labeled <The Crow of the Fate> at the top. Oliver must have dropped this note containing information about his novel, <The Crow of the Fate>.^000000")?;
        ctx.next()?;
        ctx.lines(args![
            "^3131FFAncient weapon = Some kind of power source^000000",
            "^3131FFSeclusion - A female disciple's letter^000000",
            "^3131FFThe stepmother = Lover from a past life?!^000000"
        ])?;
        ctx.close_window()?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ENHANCE")?])?;
        ctx.call(Function::DelItem, vec![Val::from(7795), Val::from(1)])?;
        ctx.var("barmunt_crow").set(Val::from(14))?;
        ctx.call(Function::GetItem, vec![Val::from(7796), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(7797), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
        ctx.call(Function::Warp, vec![Val::from("que_ba"), Val::from(270), Val::from(270)])?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        return Err(Stop::End);
    } else {
        ctx.mes("^660000The book's cover was seriously damaged, considering its recent publishing date. Many people must have turned its pages.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn worn_out_book_garas(ctx: &Ctx) -> Script {
    worn_out_book_garas_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Bpast11Step {
    Start,
    OnTouch,
}

fn bpast_1_1_run(ctx: &Ctx, mut step: Bpast11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Bpast11Step::Start => {
                step = Bpast11Step::OnTouch;
                continue 'machine;
            }
            Bpast11Step::OnTouch => {
                if ctx.var("barmunt_crow").get()?.number()? < 4 {
                    ctx.call(Function::Warp, vec![Val::from("ama_dun03"), Val::from(119), Val::from(110)])?;
                } else {
                    ctx.call(Function::EndStatus, vec![ctx.constant("SC_BLIND")?])?;
                    ctx.lines_as("???", args!["Waaaaah!"])?;
                    ctx.next()?;
                    ctx.mes("^660000Startled by a terrifying scream, you regained consciousness. It seems that you were lost in time for quite a while.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000You scrambled to stand on you feet, and then followed the sound of the scream to an old and abandoned building.^000000")?;
                    ctx.next()?;
                    ctx.mes("^660000The old, huge and somehow intimidating building was expelling dark red smoke out of broken and open windows.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn bpast_1_1(ctx: &Ctx) -> Script {
    bpast_1_1_run(ctx, Bpast11Step::Start, Vec::new()).map(|_| ())
}

pub fn bpast_1_1_ontouch(ctx: &Ctx) -> Script {
    bpast_1_1_run(ctx, Bpast11Step::OnTouch, Vec::new()).map(|_| ())
}

fn female_researcher_bpast_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("barmunt_crow").get()?.number()? < 4 {
        ctx.call(Function::Warp, vec![Val::from("ama_dun03"), Val::from(119), Val::from(110)])?;
    } else if ctx.var("barmunt_crow").get()? == 4 {
        ctx.mes("^660000While trying to remember what happened, you encounter a woman who passes by you. You instinctively reach for her shoulder to get her attention.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Ahchoo! Ahchoo! Gosh...", "Hey, where am I...?"],
        )?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VENOMDUST")?])?;
        ctx.next()?;
        ctx.mes("- Pzzzz -")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
        ctx.next()?;
        ctx.mes("^660000Surprisingly, your arm passed through her body.^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...Where the hell am I?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Warp, vec![Val::from("yuno_in04"), Val::from(100), Val::from(3)])?;
    }
    return Err(Stop::End);
}

pub fn female_researcher_bpast(ctx: &Ctx) -> Script {
    female_researcher_bpast_body(ctx, Vec::new()).map(|_| ())
}

fn researcher_bpast_2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    ctx.lines_as("Researcher", args!["Fire!", "Everybody, move, move!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn researcher_bpast_2_1(ctx: &Ctx) -> Script {
    researcher_bpast_2_1_body(ctx, Vec::new()).map(|_| ())
}

fn researcher_bpast_2_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CLOAKING")?])?;
    ctx.lines_as(
        "Researcher",
        args!["Cough, cough!", "Argh... I can't breathe... I have to get out..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn researcher_bpast_2_2(ctx: &Ctx) -> Script {
    researcher_bpast_2_2_body(ctx, Vec::new()).map(|_| ())
}
