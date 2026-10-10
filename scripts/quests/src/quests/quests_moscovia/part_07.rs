use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn ryubaba_rus08_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3500 {
        ctx.lines_as(
            "Ryubaba",
            args!["What on earth do you have in your bag?", "Are you training for something?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as("Ryubaba", args!["Ah, how beautiful I am."])?;
        ctx.next()?;
        ctx.lines_as("Ryubaba", args!["Mmm, you must be an adventurer? What would you say? Have you seen somebody more beautiful than me? I doubt it. How can you find anyone more beautiful than me?"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("rhea_rus_main").get()? == 8 {
            if !(ctx.var("rhea_rus_ring").get()?.is_true()) {
                ctx.lines_as("Ryubaba", args!["Ah, how beautiful I am."])?;
                ctx.next()?;
                ctx.lines_as("Ryubaba", args!["Mmm, you must be an adventurer? What would you say? Have you seen somebody more beautiful than me? I doubt it. How can you find anyone more beautiful than me?"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("rhea_rus_ring").get()? == 1 {
                    if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                        && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                    {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...Hmm, did I forget to wear something...?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Ryubaba", args!["Ah, how beautiful I am."])?;
                    ctx.next()?;
                    ctx.lines_as("Ryubaba", args!["Mmm, you must be an adventurer? What would you say? Have you seen somebody more beautiful than me? I doubt it. How can you find anyone more beautiful than me?"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Are you crazy!:Yes, you are right.")])?) == 1 {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Are you crazy!? Don't you know how wide the world is and how many beautiful people there are? You're pretty full of it, country girl!"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]"))
                        ])?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
                            ctx.mes("Besides, if there was a most beautiful girl in the world it would be me, me, me!")?;
                        } else {
                            ctx.lines(args![
                                "Have you seen dancers dancing? Have you seen beautiful and pure priests?!",
                                "And nothing is like the kind Kafra Employees!!!"
                            ])?;
                        }
                        ctx.next()?;
                        ctx.lines_as("Ryubaba", args!["What!? You bastard!!", "I hate you! Get out of my face!"])?;
                        ctx.var("rhea_rus_ring").set(Val::from(2))?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(220), Val::from(210)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Yes, you are really beautiful."],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SMILE")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Ryubaba", args!["Huhu, oh you jest. You know what you're talking about. Ah, this is a Choco drink that I have made. Give it a taste, isn't it good?"])?;
                    ctx.var("rhea_rus_ring").set(Val::from(4))?;
                    ctx.call(Function::GetItem, vec![Val::from(573), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("rhea_rus_ring").get()? == 2 {
                        ctx.lines_as(
                            "Ryubaba",
                            args!["I have never met a person ruder than you!", "I hate you! Get out of my face!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(220), Val::from(210)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rhea_rus_ring").get()? == 3 {
                            if (ctx.call(Function::CountItem, vec![Val::from(748)])?.number()? > 0
                                || ctx.call(Function::CountItem, vec![Val::from(749)])?.number()? > 0)
                            {
                                ctx.lines_as("Ryubaba", args!["What? Why are you here? You get of my..."])?;
                                ctx.next()?;
                                ctx.lines_as("Ryubaba", args!["...Is that for me?"])?;
                                ctx.next()?;
                                ctx.mes("[Ryubaba]")?;
                                if ctx.call(Function::CountItem, vec![Val::from(748)])?.is_true() {
                                    ctx.call(Function::DelItem, vec![Val::from(748), Val::from(1)])?;
                                } else if ctx.call(Function::CountItem, vec![Val::from(749)])?.is_true() {
                                    ctx.call(Function::DelItem, vec![Val::from(749), Val::from(1)])?;
                                } else {
                                    ctx.mes("Well, I forgive.. eh?")?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ryubaba",
                                        args![
                                            "What? Are you trying to trick me!?",
                                            "I have never met a person ruder than you!",
                                            "I hate you! Get out of my face!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(220), Val::from(210)])?;
                                    return Err(Stop::End);
                                }
                                ctx.mes("Ok. Well, I forgive you.")?;
                                ctx.var("rhea_rus_ring").set(Val::from(4))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Ryubaba",
                                args!["I have never met a person ruder than you!", "I hate you! Get out of my face!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(220), Val::from(210)])?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("rhea_rus_ring").get()? == 4 {
                                ctx.lines_as("Ryubaba", args!["Huhu, have you ever met anyone as beautiful as I am?"])?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("You are...")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["...You are the most beautiful..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ryubaba",
                                    args!["Really? Everybody thinks so.", "Ah, I forgot asking you.", "Why are you here?"],
                                )?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                ctx.mes("[Ryubaba]")?;
                                if l_input_s.clone() == "Red Ring" {
                                    ctx.mes("A Red Ring?!")?;
                                    ctx.next()?;
                                } else {
                                    ctx.mes("Heh, what are you talking about? What is that?")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Ryubaba",
                                    args![
                                        "Ah, that ring... Yes, my father gave it to my baby sister, Mashenka.",
                                        "But, she went out to take a stroll and disappeared around the marsh."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Wait, your father told me that she died!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Ryubaba", args!["Right... Let me finish talking. We tried searching for her but we couldn't find her. She must have been wearing the ring when she disappeared around the marsh."])?;
                                ctx.var("rhea_rus_ring").set(Val::from(5))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("rhea_rus_ring").get()?.number()? > 4 && ctx.var("rhea_rus_ring").get()?.number()? < 7) {
                                    ctx.lines_as(
                                        "Ryubaba",
                                        args![
                                            "Ah, that ring... Yes, my father gave it to my baby sister, Mashenka.",
                                            "But, she went out to take a stroll and disappeared around the marsh."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Wait, your father told me that she died!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Ryubaba", args!["Right... Let me finish talking. We tried searching for her but we couldn't find her. She must have been wearing the ring when she disappeared around the marsh."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("rhea_rus_ring").get()? == 7 {
                                        if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                                            && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                                        {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["...Hmm, did I forget to wear something...?"],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as("Ryubaba", args!["Ah, you come here again. What can I do for you?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["... Listen to this song."],
                                        )?;
                                        ctx.next()?;
                                        if ctx.call(Function::CountItem, vec![Val::from(7883)])?.number()? > 0 {
                                            ctx.mes("- You play the flute -")?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^ff0000Red ring that my father gave me^000000",
                                                "^ff0000Red ring that my father gave me^000000",
                                                "^ff0000The red ring enchanted^000000",
                                                "^ff0000For the loveliest daughter^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^ff0000Red ring bringing up jealousy^000000",
                                                "^ff0000Red ring bringing up jealousy^000000",
                                                "^ff0000Red ring enchanted^000000",
                                                "^ff0000My sister envies^000000"
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines(args![
                                                "^ff0000Cold hand tightening my neck^000000",
                                                "^ff0000Cold eyes tightening my heart^000000",
                                                "^ff0000Cold marsh swallowing my body^000000",
                                                "^ff0000Red ring taken away^000000",
                                                "^ff0000Red ring enchanted^000000"
                                            ])?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_THINK")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["S, stop! You'd better stop!?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["Isn't this voice Mashenka's!? What did you do to her!?"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["... I hated her. My father... father loved only her! He didn't talk to me, only to her!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["I asked her to show me the ring. But, she wouldn't let me see it. She always got on my nerves! Stupid girl!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "She was your sister! How can you speak like that!?",
                                                    "Your father needs to know about all of this!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["No!!!", "Give me that flute!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["No! Your father has to know!"],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["Please, I am begging you... Don't tell my father..."])?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                            ctx.next()?;
                                            ctx.lines(args!["- Suddenly, she sheds -", "- tears and kneels down -"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["I have been always worried that someone would find out... He has been feeling bad ever since what happened. How much worse would he feel if he knew the truth...?"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["Could you please ignore this? Give me that flute... Ah... The red ring? How about exchanging it for the ring? Don't you need this?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["But..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Ryubaba", args!["Oh, please.. This is not only for my sake, but also for my old father.... please I beg you."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args!["...Mmmm, what should I do..."],
                                            )?;
                                            ctx.next()?;
                                            if Val::from(runtime::select_values(ctx, &[Val::from("Exchange:Not exchange")])?) == 1 {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["...Mmmm, what should I do...", "Ok, I will do it."],
                                                )?;
                                                ctx.next()?;
                                            } else {
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["...Mmmm, what should I do...", "No, I can't do it!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Ryubaba", args!["What!!!"])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                    args!["I have to let your father know the truth! He has to know what you have done!"],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Ryubaba", args!["W, wait!!!"])?;
                                                ctx.var("rhea_rus_ring").set(Val::from(8))?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(203), Val::from(80)])?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as("Ryubaba", args!["Are you sure!? Thank you!"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Ryubaba",
                                                args!["I have to repent for my sins and atone for them for the rest of my life."],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7883), Val::from(1)])?;
                                            ctx.call(Function::GetItem, vec![Val::from(7877), Val::from(1)])?;
                                            ctx.var("rhea_rus_ring").set(Val::from(9))?;
                                            {
                                                if ctx.var("BaseLevel").get()?.number()? < 56 {
                                                    ctx.call(Function::GetExperience, vec![Val::from(4700), Val::from(0)])?;
                                                } else {
                                                    if ctx.var("BaseLevel").get()?.number()? < 61 {
                                                        ctx.call(Function::GetExperience, vec![Val::from(6150), Val::from(0)])?;
                                                    } else {
                                                        if ctx.var("BaseLevel").get()?.number()? < 66 {
                                                            ctx.call(Function::GetExperience, vec![Val::from(10605), Val::from(0)])?;
                                                        } else {
                                                            if ctx.var("BaseLevel").get()?.number()? < 71 {
                                                                ctx.call(Function::GetExperience, vec![Val::from(16223), Val::from(0)])?;
                                                            } else {
                                                                if ctx.var("BaseLevel").get()?.number()? < 76 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(41227), Val::from(0)],
                                                                    )?;
                                                                } else if ctx.var("BaseLevel").get()?.number()? < 81 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(69073), Val::from(0)],
                                                                    )?;
                                                                } else if ctx.var("BaseLevel").get()?.number()? < 86 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(85102), Val::from(0)],
                                                                    )?;
                                                                } else if ctx.var("BaseLevel").get()?.number()? < 91 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(104615), Val::from(0)],
                                                                    )?;
                                                                } else if ctx.var("BaseLevel").get()?.number()? < 99 {
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(222035), Val::from(0)],
                                                                    )?;
                                                                } else {
                                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["...!? Where is the flute!?"],
                                        )?;
                                        ctx.call(
                                            Function::Emotion,
                                            vec![
                                                ctx.constant("ET_HUK")?,
                                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("rhea_rus_ring").get()? == 8 {
                                        ctx.lines_as("Ryubaba", args!["You give it back to me!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["I have to let your father know the truth! He has to know what you have done!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Ryubaba", args!["W, wait!!!"])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Warp, vec![Val::from("moscovia"), Val::from(203), Val::from(80)])?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ctx.lines_as("Ryubaba", args!["................................", "I was stupid..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Ryubaba",
                args!["I have to repent for my sins and atone for them for the rest of my life."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Ryubaba", args!["................................", "I was stupid..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Ryubaba",
        args!["I have to repent for my sins and atone for them for the rest of my life."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ryubaba_rus08(ctx: &Ctx) -> Script {
    ryubaba_rus08_body(ctx, Vec::new()).map(|_| ())
}

fn little_boy_rus09_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as(
            "Little Boy",
            args![
                "I will be an adventer and will marry the most beautiful bride!",
                "A bride more beautiful than sister Ryubaba!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_ring").get()? == 2 {
            ctx.lines_as("Little Boy", args!["Hehe, you have done by her?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Little Boy",
                args![
                    "Everyone knows her. She is beautiful. My borther told me that he wanted to marry her.",
                    "But her temper makes people hesitate."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Little Boy",
                args!["I don't like her cause she doesn't give me candies or cookies. Although she is beautiful, her temper is so bad..."],
            )?;
            ctx.next()?;
            if (ctx.call(Function::CountItem, vec![Val::from(529)])?.is_true()
                || ctx.call(Function::CountItem, vec![Val::from(538)])?.is_true())
            {
                ctx.lines_as("Little Boy", args!["Ah, do you have candies or cookies?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Give some:Don't give")])?) == 2 {
                    ctx.lines_as("Little Boy", args!["Hm! Hmph! Pish! !", "You are the same as Sister Ryubaba!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Little Boy", args!["Wow, really?", "That is very kind of you!"])?;
                ctx.next()?;
                ctx.lines_as("Little Boy", args!["Ah, I will inform you of something!"])?;
                ctx.next()?;
                ctx.mes("[Little Boy]")?;
                if ctx.call(Function::CountItem, vec![Val::from(529)])?.is_true() {
                    ctx.call(Function::DelItem, vec![Val::from(529), Val::from(1)])?;
                } else if ctx.call(Function::CountItem, vec![Val::from(538)])?.is_true() {
                    ctx.call(Function::DelItem, vec![Val::from(538), Val::from(1)])?;
                } else {
                    ctx.lines(args![
                        "Sister Ryubaba likes.. eh?",
                        "Don't you have candies or cookies? Don't you want to give them to me?"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("Sister Ryubaba likes presents! Especially flowers such as roses. She believes that they resemble her.")?;
                ctx.var("rhea_rus_ring").set(Val::from(3))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Little Boy", args!["I love candy and cookies most."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_ring").get()? == 3 {
            ctx.lines_as(
                "Little Boy",
                args!["Sister Ryubaba likes presents! Especially flowers such as roses. She believes that they resemble her."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Little Boy",
        args![
            "I will be an adventer and will marry the most beautiful bride!",
            "A bride more beautiful than sister Ryubaba!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn little_boy_rus09(ctx: &Ctx) -> Script {
    little_boy_rus09_body(ctx, Vec::new()).map(|_| ())
}

fn shepherdess_rus10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as("Shepherdess", args!["Haaaa, boring, boring! Herding sheep is really boring!"])?;
        ctx.next()?;
        ctx.lines_as("Shepherdess", args!["I wish that I could listen to music all the time."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_ring").get()?.number()? < 5 {
            ctx.lines_as("Shepherdess", args!["Haaaa, boring, boring! Herding sheep is really boring!"])?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["I wish that I could listen to music all the time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_ring").get()? == 5 {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Hmm, did I forget to wear something...?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Shepherdess", args!["Haaaa, boring, boring! Herding sheep is really boring!"])?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["I wish that I could listen to music all the time."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Excuse me...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Excuse me... Have you seen a girl named, Mashenka?",
                    "She was last seen walking towards the marsh."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Shepherdess",
                args![
                    "Hm, I think I might have heard something about her but I don't remember...",
                    "You should check around the marsh yourself to see if you find something."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["Ah, right! While you're searching, can you gather some Pointed Branches for me? Could you please get me ^0000ff25 Pointed Branches^000000?"])?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["I am an excellent flute player! If you get me enough Pointed Branches, I will make a flute from them and play beautiful music for you. Please~!"])?;
            ctx.var("rhea_rus_ring").set(Val::from(6))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_ring").get()? == 6 {
            if ctx.call(Function::CountItem, vec![Val::from(7882)])?.number()? > 24 {
                if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                    && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["...Hmm, did I forget to wear something...?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Shepherdess", args!["Wow, did you get the Pointed Branches!?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Shepherdess",
                    args!["Ok! I will make a flute and play you the most beautiful music you've ever heard!"],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "- She begins to cut and trim -",
                    "- the Pointed Branches and -",
                    "- finally makes a good flute! -"
                ])?;
                ctx.next()?;
                ctx.lines_as("Shepherdess", args!["Ok, good! Let me play it to test its sound!"])?;
                ctx.next()?;
                ctx.lines(args!["- She closes her eyes -", "- and plays the flute -"])?;
                ctx.next()?;
                ctx.lines(args![
                    "^ff0000Red ring that my father gave me^000000",
                    "^ff0000Red ring that my father gave me^000000",
                    "^ff0000The red ring enchanted^000000",
                    "^ff0000For the loveliest daughter^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^ff0000Red ring bringing up jealousy^000000",
                    "^ff0000Red ring bringing up jealousy^000000",
                    "^ff0000Red ring enchanted^000000",
                    "^ff0000My sister envies^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^ff0000Cold hand tightening my neck^000000",
                    "^ff0000Cold eyes tightening my heart^000000",
                    "^ff0000Cold marsh swallowing my body^000000",
                    "^ff0000Red ring taken away^000000",
                    "^ff0000Red ring enchanted^000000"
                ])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_HUK")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shepherdess",
                    args!["W, what is this? Horrible! Why did this flute play a voice?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Shepherdess",
                    args!["I've never seen this before! This doesn't feel right! You! Take this away!"],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "- ^0000ffShepherdess gives me the^000000 -",
                    "- ^0000ff'Pointed Wooden Flute'!!^000000 -"
                ])?;
                ctx.call(Function::DelItem, vec![Val::from(7882), Val::from(25)])?;
                ctx.var("rhea_rus_ring").set(Val::from(7))?;
                ctx.call(Function::GetItem, vec![Val::from(7883), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Shepherdess",
                args![
                    "Hm, I think I might have heard something about her but I don't remember...",
                    "You should check around the marsh yourself to see if you find something."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["Ah, right! While you're searching, can you gather some Pointed Branches for me? Could you please get me ^0000ff50 Pointed Branches^000000?"])?;
            ctx.next()?;
            ctx.lines_as("Shepherdess", args!["I am an excellent flute player! If you get me enough Pointed Branches, I will make a flute from them and play beautiful music for you. Please~!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_ring").get()?.number()? > 6 {
            ctx.lines_as(
                "Shepherdess",
                args!["I will never play a flute made from the Pointed Branches around the marsh ever again!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Shepherdess", args!["Haaaa, boring, boring! Herding sheep is really boring!"])?;
    ctx.next()?;
    ctx.lines_as("Shepherdess", args!["I wish that I could listen to music all the time."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shepherdess_rus10(ctx: &Ctx) -> Script {
    shepherdess_rus10_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PtreeRusmainStep {
    Start,
    OnDisable,
    OnTimer60000,
    OnEnable,
}

fn ptree_rusmain_run(ctx: &Ctx, mut step: PtreeRusmainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PtreeRusmainStep::Start => {
                if ctx.var("rhea_rus_ring").get()? == 6 && ctx.call(Function::CountItem, vec![Val::from(7882)])?.number()? < 50 {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                        ctx.lines(args!["- I avoid the thorns and -", "- cut off a branch!! -"])?;
                        ctx.call(Function::GetItem, vec![Val::from(7882), Val::from(1)])?;
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnDisable"))],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.mes("- My hand is pricked by thorns! -")?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            PtreeRusmainStep::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            PtreeRusmainStep::OnTimer60000 => {
                step = PtreeRusmainStep::OnEnable;
                continue 'machine;
            }
            PtreeRusmainStep::OnEnable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ptree_rusmain(ctx: &Ctx) -> Script {
    ptree_rusmain_run(ctx, PtreeRusmainStep::Start, Vec::new()).map(|_| ())
}

pub fn ptree_rusmain_ondisable(ctx: &Ctx) -> Script {
    ptree_rusmain_run(ctx, PtreeRusmainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ptree_rusmain_ontimer60000(ctx: &Ctx) -> Script {
    ptree_rusmain_run(ctx, PtreeRusmainStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn ptree_rusmain_onenable(ctx: &Ctx) -> Script {
    ptree_rusmain_run(ctx, PtreeRusmainStep::OnEnable, Vec::new()).map(|_| ())
}

fn worried_mother_rus19_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as("Worried Mother", args!["Where is she..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Worried Mother",
            args!["Have you heard of the bridge who drowned to death just before her wedding and became Lusalka, an aqua nymph?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Worried Mother",
            args!["My daughter disappeared some days ago and now i'm really worried. I hope nothing happened to her..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_hair").get()?.number()? < 1 {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Hmm, did I forget to wear something...?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Worried Mother", args!["Where is she..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["Have you heard of the bridge who drowned to death just before her wedding and became Lusalka, an aqua nymph?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["My daughter disappeared some days ago and now i'm really worried. I hope nothing happened to her..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["Did you go to the marsh in the Moscovia field? Did you see my daughter? I don't care even if she became Lusalka. I just want to know where she is."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Calm her down.:I will find her.")])?) == 1 {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["She should be ok.", "She will be back soon."],
                )?;
                ctx.next()?;
                ctx.lines_as("Worried Mother", args!["Will she? I really hope so."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Worried Mother", args!["Ah, will you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["I tried to find her but, I couldn't approach the marsh due to all of the monsters."],
            )?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["Please, please find my daughter."])?;
            ctx.var("rhea_rus_hair").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_hair").get()? == 1 {
            ctx.lines_as("Worried Mother", args!["Ah, there are some things that you have to know."])?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["If my daughter has became the water nymph Lusalka you need to listen to what I know about her."],
            )?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["Lusalka is a good aqua nymph but is not completely harmless. Her stare will seriously hurt anyone who makes eye contact with her."])?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["You must have something to protect yourself from Lusalka. '^0000ffHoly Water^000000' will protect you from Lusalka'a gaze."])?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["And Lusalka is nocturnal. You should be able to find her from ^ff00005 pm to 6 am PST^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args![
                    "Don't forget. You have to bring '^0000ffHoly Water^000000' with you, and find her from ^ff00005 pm to 6 am PST^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["I'm not sure where she would be but it has to be somewhere near water. Please, find where my daughter is. I beg of you."])?;
            ctx.var("rhea_rus_hair").set(Val::from(2))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("rhea_rus_hair").get()?.number()? > 1 && ctx.var("rhea_rus_hair").get()?.number()? < 9) {
            ctx.lines_as("Worried Mother", args!["Lusalka is a good aqua nymph but is not completely harmless. Her stare will seriously hurt anyone who makes eye contact with her."])?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["You must have something to protect yourself from Lusalka. '^0000ffHoly Water^000000' will protect you from Lusalka'a gaze."])?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args!["And Lusalka is nocturnal. You should be able to find her from ^ff00005 pm to 6 am PST^000000."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Worried Mother",
                args![
                    "Don't forget. You have to bring '^0000ffHoly Water^000000' with you, and find her from ^ff00005 pm to 6 am PST^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Worried Mother", args!["I'm not sure where she would be but it has to be somewhere near water. Please, find where my daughter is. I beg of you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Worried Mother", args!["I am able to do nothing recently."])?;
    ctx.next()?;
    ctx.lines_as(
        "Worried Mother",
        args!["It seems that I lost something, but I don't know what it is..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn worried_mother_rus19(ctx: &Ctx) -> Script {
    worried_mother_rus19_body(ctx, Vec::new()).map(|_| ())
}

fn caution_07rus_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![" WARNING !! ", "No Swimming"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn caution_07rus(ctx: &Ctx) -> Script {
    caution_07rus_body(ctx, Vec::new()).map(|_| ())
}

fn lusalka_07russai_22_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lusalka_07russai_22(ctx: &Ctx) -> Script {
    lusalka_07russai_22_body(ctx, Vec::new()).map(|_| ())
}

fn lusalka_07russai_22_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("rhea_rus_main").get()? != 8 || ctx.var("rhea_rus_hair").get()?.number()? < 2) {
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_QUESTION")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        return Err(Stop::End);
    }
    if ctx.var("rhea_rus_hair").get()? == 2 {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            || ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? <= 6)
        {
            ctx.mes("- Splash !! -")?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0 {
                ctx.lines(args!["- You hear a splashing sound -", "- and see something gleaming -"])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnEnable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "- You hear a splashing as -",
                "- something gleaming -",
                "- seems to stare at you!! -"
            ])?;
            ctx.next()?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-60), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if (ctx.var("rhea_rus_hair").get()?.number()? > 2 && ctx.var("rhea_rus_hair").get()?.number()? < 7) {
        ctx.lines_as(
            "Lusalka's Voice",
            args!["His name is 'Igor'.. He has gone to the capital of Rune-Midgarts. Give him my golden earrings."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lusalka's Voice",
            args!["And please, tell him to stop suffering and to be happy. This is my request."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_hair").get()? == 7 {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            || ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? <= 6)
        {
            ctx.mes("- Splash !! -")?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0 {
                ctx.mes("-You hear splashing sound and see something gleaming-")?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnEnable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("-You seem to hear the splash but something gleaming raids on you!!-")?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-60), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("rhea_rus_hair").get()? == 8 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["What was that?!", "Why am I here?"],
        )?;
        ctx.next()?;
        ctx.lines(args!["- After standing up, you see -", "- the wet hair in front of you -"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["...This.. is Lusalka's hair?", "Ah, right.. I came here to get her hair."],
        )?;
        ctx.next()?;
        ctx.mes("- You pick up the hair -")?;
        ctx.next()?;
        ctx.lines(args![
            "- As soon as you touch the -",
            "- cool damp hair, you seem -",
            "- the feel the sadness within -"
        ])?;
        ctx.var("rhea_rus_hair").set(Val::from(9))?;
        ctx.call(Function::GetItem, vec![Val::from(7878), Val::from(2)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_hair").get()?.number()? > 8 {
        ctx.lines(args!["- You feel that someone -", "- is watching you -"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Who's there? Anybody here?"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "- The splashing sound -",
            "- fades away, but you -",
            "- still feel the sadness -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lusalka_07russai_22_ontouch(ctx: &Ctx) -> Script {
    lusalka_07russai_22_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn lusalka_rus23_run(ctx: &Ctx, mut step: LusalkaRus23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LusalkaRus23Step::Start => {
                if ctx.call(Function::CountItem, vec![Val::from(523)])?.is_true() {
                    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
                        ctx.lines_as(
                            "Lusalka",
                            args!["...Your alive...", "My stare didn't...", "What are you doing here...?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rhea_rus_main").get()? == 8 {
                            if ctx.var("rhea_rus_hair").get()?.number()? < 2 {
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["...Your alive...", "My stare didn't...", "What are you doing here...?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("rhea_rus_hair").get()? == 2 {
                                if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                                    && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                                {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["...Hmm, did I forget to wear something...?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines(args!["...Your alive...", "My stare didn't...", "What are you doing here...?"])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["Nighttime for the aqua nymph.", "Haven't you heard that Lusalka's stare can kill people. If you didn't have the 'Holy Water', you would surely have died."])?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("I am an adventurer and...")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args![
                                        "...An adventurer...?",
                                        "If so, you came here from another continent, not Moscovia...?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["I'm from Rune-Midgarts."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["...Rune... Midgarts...", "A few days ago, my lover left for Rune-Midgarts.. Can you find him for me? I want to talk with him..."])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["...I don't know why but, I can't remember how I became Lusalka. When I woke up, I was lying down on the bottom of the river."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["I forgot everything, but not him. His smile and warm hand. I remember everything about him."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["I watched him from the bottom of the river sitting randomly around the river and staring into the marsh. With very sad eyes."])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["I suffered seeing him grieve, but wasn't able to meet him. Ordinary people are killed immediately when they see Lusalka's stare..."])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["I saw him coming to the marsh everyday and heard him mourning. And his assurance to go to Rune-Midgarts."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["Adventurer, can you find him for me? I can't leave this marsh. Please, find him for me?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["I don't want him to come back or to meet me again. But, Please tell him that I hope he'll stop suffering and to be happy."])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["I became Lusalka, but it is ok. I'm not sad. But, I can feel his sadness around this river that surrounds me. I feel sorry about it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["Please, find him and tell him that I am ok and to stop suffering..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Where can I find him?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["The night when he left, he said that he would go to the capital of Rune-Midgarts. I am not sure if he is still there..."])?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Ok, I will do it:I don't think I can do it...")],
                                )?) == 2
                                {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["Rune-Midgarts is large! I don't think I can do it."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["There's nothing we can do. Just give it up."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Lusalka", args!["Ah... but..."])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnDisable")])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Lusalka", args!["Ah! Thank you so much! His name is '^0000ffIgor^000000'. Ah, give these earrings to him. He gave them to me as a present."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "- ^0000ffI receive the^000000 -",
                                    "- ^0000ffgolden earrings^000000 -",
                                    "- ^0000fffrom Lusalka!^000000 -"
                                ])?;
                                ctx.var("rhea_rus_hair").set(Val::from(3))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnDisable")])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("rhea_rus_hair").get()?.number()? > 2 && ctx.var("rhea_rus_hair").get()?.number()? < 7) {
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["His name is 'Igor'.. He has gone to the capital of Rune-Midgarts. Give him my golden earrings."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["And please, tell him to stop suffering and to be happy. This is my request."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("rhea_rus_hair").get()? == 7 {
                                if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                                    && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                                {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["...Hmm, did I forget to wear something...?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Lusalka", args!["Have you seen him? How was he?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "This is a message from Igor.",
                                        "He said that he remembers every moment with you and still loves you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["And he would come back to Moscovia until the full moon and wanted you to meet him. He told me that he will be strong enough not to make you suffer."])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["Igor..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args![
                                        "Thank you very much. I really appreciate it.",
                                        "How can I thank you? Let me know what I can do for you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Can I ask a favor of you?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["If I can, I will do it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["If so!!", "....I need some of your hair."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["...Eh, hair? Is that all?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Actually it's not all.", "I need 2 lockes!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lusalka",
                                    args!["Ah, hahaha.", "Is that what you wanted? Huhu, sure take what you want."],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["- Lusalka cuts some -", "- of her hair with her -", "- sharp nails -"])?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args!["Here, my hair...?!", "Aaaaaaaah!??!?!"])?;
                                ctx.call(
                                    Function::StartStatus,
                                    vec![ctx.constant("SC_CURSE")?, Val::from(60000), Val::from(0)],
                                )?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_HUK")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["W, what was that!?", "Who's there?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Voice unidentified",
                                    args![
                                        "Don't think that you've succeeded in keeping me sealed in this prison you fool!",
                                        "You will never stop me!!!",
                                        "hahahaha!!!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["W, who are you!? Ahkkk!!"],
                                )?;
                                ctx.call(
                                    Function::StartStatus,
                                    vec![ctx.constant("SC_BLIND")?, Val::from(10000), Val::from(0)],
                                )?;
                                ctx.var("rhea_rus_hair").set(Val::from(8))?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnDisable")])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("rhea_rus_hair").get()?.number()? > 7 {
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["...?! What are you? The marsh ghost?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lusalka", args![".........................."])?;
                                ctx.next()?;
                                ctx.mes("- The green thing seems to open its mouth to say something but closes it soon after and watches you with its sad eyes-")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["...?! What are you? The marsh ghost?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Lusalka", args![".........................."])?;
                            ctx.next()?;
                            ctx.mes("- The green thing seems to open its mouth to say something but closes it soon after and watches you with its sad eyes-")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    return Err(Stop::End);
                }
                ctx.mes("-When Lusalka watches you, you are blacked out-")?;
                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
                ctx.call(Function::PercentHeal, vec![Val::from(-60), Val::from(0)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lusalka#rus23::OnDisable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            LusalkaRus23Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Lusalka#rus23")])?;
                return Err(Stop::End);
            }
            LusalkaRus23Step::OnTimer300000 => {
                step = LusalkaRus23Step::OnDisable;
                continue 'machine;
            }
            LusalkaRus23Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = LusalkaRus23Step::OnInit;
                continue 'machine;
            }
            LusalkaRus23Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lusalka#rus23")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lusalka_rus23(ctx: &Ctx) -> Script {
    lusalka_rus23_run(ctx, LusalkaRus23Step::Start, Vec::new()).map(|_| ())
}

pub fn lusalka_rus23_onenable(ctx: &Ctx) -> Script {
    lusalka_rus23_run(ctx, LusalkaRus23Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn lusalka_rus23_ontimer300000(ctx: &Ctx) -> Script {
    lusalka_rus23_run(ctx, LusalkaRus23Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn lusalka_rus23_ondisable(ctx: &Ctx) -> Script {
    lusalka_rus23_run(ctx, LusalkaRus23Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn lusalka_rus23_oninit(ctx: &Ctx) -> Script {
    lusalka_rus23_run(ctx, LusalkaRus23Step::OnInit, Vec::new()).map(|_| ())
}

fn wanderer_rus24_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as(
            "A Wanderer from a strange land",
            args!["Hey, there are so many adventurers around here. I can see why this is the capital of the Rune-Midgarts Kingdom!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "A Wanderer from a strange land",
            args!["In your free time, come to see me! The weather here is even more favorable than our the best summer back home."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_hair").get()?.number()? < 3 {
            ctx.lines_as(
                "A Wanderer from a strange land",
                args!["Hey, there are so many adventurers around here. I can see why this is the capital of the Rune-Midgarts Kingdom!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A Wanderer from a strange land",
                args!["In your free time, come to see me! The weather here is even more favorable than our the best summer back home."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_hair").get()? == 3 {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Hmm, did I forget to wear something...?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "A Wanderer from a strange land",
                args!["Hey, there are so many adventurers around here. I can see why this is the capital of the Rune-Midgarts Kingdom!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A Wanderer from a strange land",
                args!["In your free time, come to see me! The weather here is even more favorable than our the best summer back home."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Back home?", "Excuse me for asking, but are you from Moscovia?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A Wanderer from a strange land",
                args!["Ho, how did you know that?", "Have you been there before?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Ah, yes. I have been there...", "By the way, the important thing is.. are you....."],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Igor" {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Ah, yes. I have been there...",
                        ((Val::from("By the way, the important thing is.. are you..... ") + l_input_s.clone()) + Val::from("?"))
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Ah, yes. I have been there...",
                        ((Val::from("By the way, the important thing is.. are you..... ") + l_input_s.clone()) + Val::from("?"))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "A Wanderer from a strange land",
                    args!["Hmm, no.", "I have never heard that name."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("A Wanderer from a strange land", args!["Hmm, no. But, that name is familiar."])?;
            ctx.next()?;
            ctx.lines_as("A Wanderer from a strange land", args!["Ah, oh yes! How could I forget?"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
            ctx.next()?;
            ctx.lines_as("A Wanderer from a strange land", args!["We came here from Moscovia together. He called himself Igg. I came here with him but parted with him quite a while ago. He was sad because he couldn't forget about his lost lover."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Ah, yes that must be Igor.",
                    "I am looking for him!",
                    "Do you know where he is now?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A Wanderer from a strange land",
                args![
                    "Hmm, I don't know",
                    "exactly where he is.",
                    "Though, he did tell",
                    "me that he wanted",
                    "to travel to a desert."
                ],
            )?;
            ctx.var("rhea_rus_hair").set(Val::from(4))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("rhea_rus_hair").get()?.number()? > 3 && ctx.var("rhea_rus_hair").get()?.number()? < 7) {
            ctx.lines_as("A Wanderer from a strange land", args!["He called himself Igg. I came here with him but parted with him quite a while ago. He was sad because he couldn't forget about his lost lover."])?;
            ctx.next()?;
            ctx.lines_as(
                "A Wanderer from a strange land",
                args![
                    "Hmm, I don't know",
                    "exactly where he is.",
                    "Though, he did tell",
                    "me that he wanted",
                    "to travel to a desert."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "A Wanderer from a strange land",
        args!["Is there anywhere I should go? Where have you been with him?"],
    )?;
    ctx.next()?;
    ctx.lines_as("A Wanderer from a strange land", args!["Let me know somewhere."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wanderer_rus24(ctx: &Ctx) -> Script {
    wanderer_rus24_body(ctx, Vec::new()).map(|_| ())
}

fn morocc_villager_rus25_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as(
            "Morocc Villager",
            args!["Huh, more and more adventurers are settling here recently, so it's hard to tell who is a villager or an adventurer."],
        )?;
        ctx.next()?;
        ctx.lines_as("Morocc Villager", args!["But natives, like me, can tell the difference."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_hair").get()?.number()? < 4 {
            ctx.lines_as(
                "Morocc Villager",
                args![
                    "Huh, more and more adventurers are settling here recently, so it's hard to tell who is a villager or an adventurer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Morocc Villager", args!["But natives, like me, can tell the difference."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_hair").get()? == 4 {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Hmm, did I forget to wear something...?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Morocc Villager",
                args![
                    "Huh, more and more adventurers are settling here recently, so it's hard to tell who is a villager or an adventurer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Morocc Villager", args!["But natives, like me, can tell the difference."])?;
            ctx.next()?;
            ctx.lines_as(
                "Morocc Villager",
                args!["As I was saying, I like this village. The desert is hot and Kafra staff is even hotter!!! *hack* *cough*!!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Morocc Villager", args!["Ah, but don't get me wrong! I'm not coughing because of the Kafra Staffers. It's uhh the heat, it's getting to me... ya..."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "................................",
                    "The hell you talking about..? I'm looking for someone."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Morocc Villager", args!["I see so many adventurers around here, how can I remember everybody? Got any descriptions of this someone..? Or traits..? anything to help me remember."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Hmm... I don't know how he looks like but he is from a land called Moscovia."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Morocc Villager",
                args![
                    "Ah, are you talking about a guy that's always sad and dresses funny? I figured his accent wasn't from around here..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Morocc Villager", args!["He was here a coupla days ago but he left to see the Pyramid. I never saw him after that. I said it would be dangerous but he went anyways."])?;
            ctx.next()?;
            ctx.lines_as(
                "Morocc Villager",
                args!["Yep that's gotta be who you're looking for. I can see his gloomy face right now... that poor sad man."],
            )?;
            ctx.var("rhea_rus_hair").set(Val::from(5))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("rhea_rus_hair").get()?.number()? > 4 && ctx.var("rhea_rus_hair").get()?.number()? < 7) {
            ctx.lines_as("Morocc Villager", args!["He was here a coupla days ago but he left to see the Pyramid. I never saw him after that. I said it would be dangerous but he went anyways."])?;
            ctx.next()?;
            ctx.lines_as(
                "Morocc Villager",
                args!["Yep that's gotta be who you're looking for. I can see his gloomy face right now... that poor sad man."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Morocc Villager", args!["Many travellers in the village and I feel fun to see them. Even though I am here, I feel like that I travel all around the world?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn morocc_villager_rus25(ctx: &Ctx) -> Script {
    morocc_villager_rus25_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_rus26_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_player_name_s = Val::from("");
    if ctx.var("rhea_rus_main").get()?.number()? < 8 {
        ctx.lines_as("A gloomy looking soldier", args!["............................"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rhea_rus_main").get()? == 8 {
        if ctx.var("rhea_rus_hair").get()?.number()? < 5 {
            ctx.lines_as("A gloomy looking soldier", args!["............................"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("rhea_rus_hair").get()? == 4 || ctx.var("rhea_rus_hair").get()? == 5) {
            l_player_name_s = ctx.call(Function::StrCharInfo, vec![Val::from(0)])?;
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(l_player_name_s.clone(), args!["...Hmm, did I forget to wear something...?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("A gloomy looking soldier", args!["............................"])?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["...I don't know who you are. Just get outta my face. I don't want to talk..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["It's pretty dangerous here. There are fierce monsters all over the place."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["... Good. Let them come. I want to die here alone."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["A-Are you Igor? What're you talking about? Have you gone nuts?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["Do you have any idea how much your lover is suffering?"],
            )?;
            ctx.next()?;
            ctx.lines_as("A gloomy looking soldier", args!["My... lover...?"])?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["I brought a message from her. She says not to suffer anymore and to just be happy..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["The more you suffer, the deeper she sinks into sadness. Don't think that you're the only one that's suffering..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args![
                    "...Svetlana?!",
                    "Wait, how could you...? She... how do I know that she really spoke to you?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "-^0000ff Without a word, ^000000-",
                "-^0000ff you give him Lusalka's ^000000-",
                "-^0000ff golden earrings ^000000-"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["T, this is Svetlana's...?!", "H, how did you!?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args!["I met her by accident. She became Lusalka in the marsh of Moscovia."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_player_name_s.clone(),
                args![
                    "She told me that you wouldn't believe me so she gave me these earrings as proof... The present that you gave to her."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["Is that true?", "Did she really become Lusalka?"],
            )?;
            ctx.next()?;
            ctx.lines_as("A gloomy looking soldier", args!["It was near the end of the harsh Moscovia winter... She... didn't realize that the ice on the marsh was already melting..."])?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["By then, it was too late the ice cracked where she was and she fell into the freezing cold marsh."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["She wasn't able to swim. I tried to get her out but it was impossible."],
            )?;
            ctx.next()?;
            ctx.lines_as("A gloomy looking soldier", args!["She sunk quickly into the marsh. I was able to reach her hand but, she realized that it would kill both of us and she released my hand... I regret that moment every single day."])?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["My last memory of her was of her smiling, as she sunk into the cold, wet marsh. As if to say, 'You will be ok.'"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args![
                    "I, I was a coward. I was too afraid of drowning to pull her out of the marsh. Too afraid of....",
                    "Oh... Svetlana!!!"
                ],
            )?;
            ctx.var("rhea_rus_hair").set(Val::from(6))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_hair").get()? == 6 {
            if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
            {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["...Hmm, did I forget to wear something...?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("A gloomy looking soldier", args!["I knew that she had become the cursed nymph Lusalka. But, I still went every night... to find her. I just wanted to see her and talk to her."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "She told me that she also wanted to see you. But, she was afraid that Lusalka's curse would kill you if you saw her.",
                    "And so she hid..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["How could she think of me...", "...I, how should I.. Should I..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Don't be sad. She still loves you. Your suffering is her suffering."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["................................................She is? Thank you. You've encouraged me."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args!["I will go back to Moscovia. But, I'm not well enough at the moment to go back."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "A gloomy looking soldier",
                args![
                    "Could you please go to her and tell her my message?",
                    "I remember every moment I've had with her and... I still love her."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("A gloomy looking soldier", args!["Before the full moon, I will be back to Moscovia and meet her. I will come back so strong and happy that when I go to see her she will not suffer anymore."])?;
            ctx.var("rhea_rus_hair").set(Val::from(7))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("rhea_rus_hair").get()? == 7 {
            ctx.lines_as(
                "A gloomy looking soldier",
                args![
                    "Could you please go to her and tell her my message?",
                    "I remember every moment I've had with her and... I still love her."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("A gloomy looking soldier", args!["Before the full moon, I will be back to Moscovia and meet her. I will come back so strong and happy that when I go to see her she will not suffer anymore."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("A gloomy looking soldier", args!["... I feel like I lost something precious."])?;
    ctx.next()?;
    ctx.mes("- He looks down at the golden earrings and rubs it carefully -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_rus26(ctx: &Ctx) -> Script {
    soldier_rus26_body(ctx, Vec::new()).map(|_| ())
}

fn s_1_rus27_run(ctx: &Ctx, mut step: S1Rus27Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S1Rus27Step::Start => {
                return Err(Stop::End);
            }
            S1Rus27Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("mosk_que"), Val::from(49), Val::from(22)])?;
                return Err(Stop::End);
            }
            S1Rus27Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PORTAL2")?])?;
                ctx.call(Function::EnableNpc, vec![Val::from("1#rus27")])?;
                return Err(Stop::End);
            }
            S1Rus27Step::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = S1Rus27Step::OnInit;
                continue 'machine;
            }
            S1Rus27Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("1#rus27")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1_rus27(ctx: &Ctx) -> Script {
    s_1_rus27_run(ctx, S1Rus27Step::Start, Vec::new()).map(|_| ())
}

pub fn s_1_rus27_ontouch(ctx: &Ctx) -> Script {
    s_1_rus27_run(ctx, S1Rus27Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn s_1_rus27_onenable(ctx: &Ctx) -> Script {
    s_1_rus27_run(ctx, S1Rus27Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn s_1_rus27_ontimer30000(ctx: &Ctx) -> Script {
    s_1_rus27_run(ctx, S1Rus27Step::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn s_1_rus27_oninit(ctx: &Ctx) -> Script {
    s_1_rus27_run(ctx, S1Rus27Step::OnInit, Vec::new()).map(|_| ())
}

fn rus27_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn rus27(ctx: &Ctx) -> Script {
    rus27_body(ctx, Vec::new()).map(|_| ())
}

fn rus27_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rhea_rus_main").get()? == 8 && !(ctx.var("rhea_rus_quiz").get()?.is_true()) {
        ctx.lines_as("Voice unidentified", args!["Who dares to come into my cave!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Are you Marozka? Maria sent me! I need your 'Golden Thread' to make a 'Golden Key' to release her from the wall!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Voice unidentified",
            args!["...To make a 'Golden Key'?", "Do you think that you are right for this job?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Voice unidentified", args!["You must be fighting against 'Koshei' if you are asking about the 'Golden Key' and Maria! Do you think that you are strong enough to fight against him?"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Of course I'm strong enough!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Voice unidentified",
            args![
                "You are very confident.",
                "But, I need to test you to see if you truly are capable."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Voice unidentified", args!["In this cave, there is a 'Stone Furnace' and an 'Apple Tree'.", "You can escape from the cave with ^0000ff100 Well Baked Cookies^000000 from the Stone Furnace and ^0000ff100 Apples^000000 from the Apple Tree."])?;
        ctx.next()?;
        ctx.lines_as(
            "Voice unidentified",
            args!["It won't be easy. Monsters are wandering around the Stone Furnace and the Apple Tree."],
        )?;
        ctx.next()?;
        ctx.lines_as("Voice unidentified", args!["Show me what you can do."])?;
        ctx.var("rhea_rus_quiz").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_QUESTION")?,
            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
        ],
    )?;
    return Err(Stop::End);
}

pub fn rus27_ontouch(ctx: &Ctx) -> Script {
    rus27_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StoneFurnaceRus28Step {
    Start,
    OnMyMobDead,
    OnTimer300000,
    OnInit,
}

fn stone_furnace_rus28_run(ctx: &Ctx, mut step: StoneFurnaceRus28Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StoneFurnaceRus28Step::Start => {
                if ctx.var("rhea_rus_main").get()? == 8 && ctx.var("rhea_rus_quiz").get()? == 1 {
                    if (ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2429
                        && ctx.call(Function::GetEquipId, vec![ctx.constant("EQI_SHOES")?])? != 2430)
                    {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["...Hmm, did I forget to wear something...?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "- You can see the -",
                        "- well baked cookies. -",
                        "- Be careful and -",
                        "- don't get burned -"
                    ])?;
                    ctx.next()?;
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                        ctx.lines(args![
                            "- You quickly stretch out -",
                            "- your hand and pick up -",
                            "- the well baked cookies! -"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["- ^0000ffYou got 100^000000 -", "- ^0000ffWell Baked Cookies!!^000000 -"])?;
                        ctx.next()?;
                    } else {
                        ctx.lines(args![
                            "- You quickly stretch out -",
                            "- your hand and pick up -",
                            "- the well baked cookies! -"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Oops, it's hot, hot!!!"],
                        )?;
                        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_FIREHIT")?])?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_HUK")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Marozka's Guard", args!["How did this rat get inside the cave?!"])?;
                    ctx.call(
                        Function::Announce,
                        vec![
                            Val::from("Marozka's Guard : Invader! Search the whole cave!!"),
                            ctx.constant("BC_MAP")?,
                            Val::from(8900331),
                        ],
                    )?;
                    ctx.var("rhea_rus_quiz").set(Val::from(2))?;
                    if !(ctx.var("$@rus_req01").get()?.is_true()) {
                        ctx.call(Function::InitNpcTimer, vec![])?;
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("mosk_que"),
                                Val::from(49),
                                Val::from(156),
                                Val::from("Marozka's Guard"),
                                Val::from(1889),
                                Val::from(1),
                                Val::from("Stone Furnace#rus28::OnMyMobDead"),
                            ],
                        )?;
                        ctx.var("$@rus_req01").set(Val::from(1))?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_QUESTION")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                return Err(Stop::End);
            }
            StoneFurnaceRus28Step::OnMyMobDead => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.var("$@rus_req01").set(Val::from(0))?;
                ctx.call(
                    Function::Announce,
                    vec![
                        Val::from("Mazroka : You are truly brave. When you get the cookies and apples, come to see me."),
                        ctx.constant("BC_MAP")?,
                        Val::from(8900331),
                    ],
                )?;
                return Err(Stop::End);
            }
            StoneFurnaceRus28Step::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mosk_que"), Val::from("Stone Furnace#rus28::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = StoneFurnaceRus28Step::OnInit;
                continue 'machine;
            }
            StoneFurnaceRus28Step::OnInit => {
                ctx.var("$@rus_req01").set(Val::from(0))?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn stone_furnace_rus28(ctx: &Ctx) -> Script {
    stone_furnace_rus28_run(ctx, StoneFurnaceRus28Step::Start, Vec::new()).map(|_| ())
}

pub fn stone_furnace_rus28_onmymobdead(ctx: &Ctx) -> Script {
    stone_furnace_rus28_run(ctx, StoneFurnaceRus28Step::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn stone_furnace_rus28_ontimer300000(ctx: &Ctx) -> Script {
    stone_furnace_rus28_run(ctx, StoneFurnaceRus28Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn stone_furnace_rus28_oninit(ctx: &Ctx) -> Script {
    stone_furnace_rus28_run(ctx, StoneFurnaceRus28Step::OnInit, Vec::new()).map(|_| ())
}
