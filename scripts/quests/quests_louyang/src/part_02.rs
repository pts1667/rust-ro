use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn tool_shop_master_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_par").get()?.number()? < 2 {
        ctx.lines_as(
            "Wang Chuiyi",
            args![
                "My business hasn't been doing",
                "well recently. And what is wrong with this weather? I don't know what's going on with the world..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()? == 2 {
        ctx.lines_as("Wang Chuiyi", args!["Darn it!", "I hate this", "weather...!"])?;
        ctx.next()?;
        ctx.lines_as("Wang Chuiyi", args!["Um? Can I help", "you with anything?"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "I'm here to get something for the doctor...:I agree, the weather really is bad.",
            )],
        )?) == 1
        {
            ctx.lines_as(
                "Wang Chuiyi",
                args![
                    "Huh?",
                    "An errand for the doctor?",
                    "She must have run out of",
                    "medicine again. Go ahead",
                    "and check the storage."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Wang Chuiyi", args!["You can find the storage on", "the opposite side of this building. There, you'll see a guy named Jiang Xiayou. Go ahead and ask him for the stuff you need."])?;
            ctx.var("ch_par").set(Val::from(3))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(11044), Val::from(11045)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Wang Chuiyi",
            args![
                "Tell me about it. This weather keeps stressing me out. Damn,",
                "I don't think I'll live very long if I keep getting aggravated like",
                "this by the weather..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()? == 3 {
        ctx.lines_as("Wang Chuiyi", args!["I can't understand why outsiders continue to travel here to Luoyang despite the weather. I'm also getting tired of keeping my business here. Sooner or later,", "I may have to leave."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()? == 4 {
        ctx.lines_as("Wang Chuiyi", args!["What? He didn't give you the stuff you need? Huh, I guess you did something he didn't like. I can't help you if that's the case."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()?.number()? < 10 {
        ctx.lines_as("Wang Chuiyi", args!["I can't understand why outsiders continue to travel here to Luoyang despite the weather. I'm also getting tired of keeping my business here. Sooner or later,", "I may have to leave."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Wang Chuiyi",
            args![
                "God, what is wrong with this weather?! I wish the Cloud God would bring the rainy season, but it doesn't seem possible."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn tool_shop_master_lou(ctx: &Ctx) -> Script {
    tool_shop_master_lou_body(ctx, Vec::new()).map(|_| ())
}

fn storage_keeper_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_j = Val::from(0);
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FF * Wait a moment!! *", "Currently you're over weight, so you cannot receive more items into your inventory. Please store some of your items into Kafra Storage and try again.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()?.number()? < 3 {
        ctx.lines_as(
            "Jiang Xiayou",
            args![
                "^666666*Yawn...*^000000",
                "This is boring...",
                "So boring, it's ridiculous. I don't wanna waste any more time here, I've got important things to do..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ch_par").get()? == 3 {
            ctx.lines_as(
                "Jiang Xiayou",
                args![
                    "Huh?",
                    "What, what are you doing here?",
                    "If you don't need anything, you better get a move on."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Sorry about that.:It's hot, isn't it?:I'm on an errand for the doctor.")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args![
                            "That's right!",
                            "You don't mess with a person in this hot weather! Now, go away. Can't you see I'm busy!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args![
                            "You don't have to",
                            "ask me that. I can feel",
                            "it for myself! Now, I got",
                            "a bunch of things to take",
                            "care of, so quit bugging me.",
                            "Damn, it's hot!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args!["Oh yeah?", "Well, why didn't you", "say so? Let's see.", "Hmmm..."],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args![
                            "Actually, could you help me",
                            "out first? Don't worry, it's not so hard but it's pretty important for me. Once you help me out, I'll",
                            "get you what you need."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Um, what is it?:Sorry, I'm busy.")])?) == 1 {
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "Cool, thanks. I have to go get",
                                "some official documents from",
                                "City Hall, but I can't leave this storage area since no one can take over my shift."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "So I want you to go get the documents from City Hall for me.",
                                "I don't think it'll take much of your time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "Just go East from here",
                                "and look for the building that looks sort of like it was made in gauge form. That's City Hall."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Jiang Xiayou", args!["I don't get why the government spent so much money making that building, but anyway, I hope you can do that for me."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "Oh, I almost forgot.",
                                "You have to ask for a specific type of document, so let me tell you right now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Jiang Xiayou", args!["This is important,", "So don't forget this."])?;
                        l_paper_j = ctx.call(Function::Rand, vec![Val::from(10), Val::from(13)])?;
                        if l_paper_j.clone() == 10 {
                            ctx.mes("When the guy asks what you need, you tell him: '^ff0000Residence Transfer Application^000000.' Just like that.")?;
                            ctx.var("ch_par").set(Val::from(5))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11045), Val::from(11046)])?;
                        } else if l_paper_j.clone() == 11 {
                            ctx.mes(
                                "When the guy asks what you need, you tell him: '^ff0000Summer SAT Class Application^000000.' Got it?",
                            )?;
                            ctx.var("ch_par").set(Val::from(6))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11045), Val::from(11047)])?;
                        } else if l_paper_j.clone() == 12 {
                            ctx.mes(
                                "When the guy asks what you need, you tell him: '^ff0000Sandy Dust Phenomenon Report^000000.' Easy, right?",
                            )?;
                            ctx.var("ch_par").set(Val::from(7))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11045), Val::from(11048)])?;
                        } else if l_paper_j.clone() == 13 {
                            ctx.mes(
                                "When the guy asks what you need, you tell him: '^ff0000Communication Proposal^000000.' Just like that?",
                            )?;
                            ctx.var("ch_par").set(Val::from(8))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11045), Val::from(11049)])?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "If you don't specify the",
                                "documents you need, they won't give you anything. So be careful and don't forget!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Jiang Xiayou", args!["Bah~!", "Forget it, then!"])?;
                    ctx.next()?;
                    ctx.lines_as("Jiang Xiayou", args!["The medicines you're looking for might be around here, so look around. If it weren't for the doctor, I wouldn't even let you hang around, you know that?"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                    ctx.var("ch_par").set(Val::from(4))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(11045), Val::from(11050)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("ch_par").get()? == 4 {
                ctx.lines_as("Jiang Xiayou", args!["Hah...", "Sorry pal, you'll have to find the medicines on your own. A man's life is at stake, so I guess you oughta get a move on."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ch_par").get()?.number()? < 9 {
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args!["Huh?", "Haven't you gone", "to City Hall yet?", "Whaaaat a lazy ass."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jiang Xiayou", args!["Shouldn't you hurry to get that medicine to the doc? We're talking a man's life at stake, that mean anything to you?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jiang Xiayou",
                        args![
                            "I guess you're",
                            "the forgetful type...",
                            "When the guy asks what",
                            "you need, you tell him:"
                        ],
                    )?;
                    if ctx.var("ch_par").get()? == 5 {
                        ctx.mes("'^ff0000Residence Transfer Application^000000.'")?;
                    } else if ctx.var("ch_par").get()? == 6 {
                        ctx.mes("'^ff0000Summer SAT Class Application^000000.'")?;
                    } else if ctx.var("ch_par").get()? == 7 {
                        ctx.mes("'^ff0000Sandy Dust Phenomenon Report^000000'.")?;
                    } else if ctx.var("ch_par").get()? == 8 {
                        ctx.mes("^ff0000Communication Proposal^000000.'")?;
                    }
                    ctx.next()?;
                    ctx.lines_as("Jiang Xiayou", args!["Just like that.", "Now, go to it, tiger."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ch_par").get()? == 9 {
                        ctx.lines_as(
                            "Jiang Xiayou",
                            args![
                                "Hmm...",
                                "You're lucky.",
                                "Alright, go ahead.",
                                "Still, things have",
                                "become complicated..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ch_par").get()? == 10 {
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args![
                                    "^666666*Yawn~*^000000 I really hope the",
                                    "weather gets better. Man, now why did I have to think about the heat? Now I'm all depressed..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args!["Huh. I guess I should", "take care of these things", "as soon as I can..."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ch_par").get()?.number()? < 15 {
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args!["Huh?", "Haven't you gone to City Hall yet? Whaaaaat a lazy ass."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Jiang Xiayou", args!["If I were you, I'd hurry so I could get that medicine to the doc. A man's life is at stake here, that's gotta mean something to you!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args![
                                    "I guess you're",
                                    "the forgetful type...",
                                    "When the guy asks what",
                                    "you need, you tell him:"
                                ],
                            )?;
                            if ctx.var("ch_par").get()? == 11 {
                                ctx.mes("'^ff0000Residence Transfer Application^000000.'")?;
                            } else if ctx.var("ch_par").get()? == 12 {
                                ctx.mes("'^ff0000Summer SAT Class Application^000000.'")?;
                            } else if ctx.var("ch_par").get()? == 13 {
                                ctx.mes("'^ff0000Sandy Dust Phenomenon Report^000000.'")?;
                            } else if ctx.var("ch_par").get()? == 14 {
                                ctx.mes("'^ff0000Communication Proposal^000000.'")?;
                            }
                            ctx.next()?;
                            ctx.lines_as("Jiang Xiayou", args!["Just like that.", "Now, go to it, tiger."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("ch_par").get()? == 15 {
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args![
                                    "You finally brought it! Haha,",
                                    "I just got everything you need, too. Let me see...",
                                    "Yup, this is it!",
                                    "Good, good..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["^3355FF*Rummage rummage*", "......^000000"])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Excuse me.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args!["Huh?", "Ah....", "Haha...", "Sorry about", "that. Hahaha~"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jiang Xiayou",
                                args![
                                    "Ah! Here you go. Even if it was kind of annoying to do, I guess",
                                    "we gotta help each other, right? Alright then, I'll see ya around."
                                ],
                            )?;
                            ctx.var("ch_par").set(Val::from(16))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11055), Val::from(11056)])?;
                            ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
    ctx.lines_as(
        "Jiang Xiayou",
        args![
            "^666666*Yawn~*^000000 I really hope the",
            "weather gets better. Man, now why did I have to think about the heat? Now I'm all depressed..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiang Xiayou",
        args!["Huh. I guess I should", "take care of these things", "as soon as I can..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn storage_keeper_lou(ctx: &Ctx) -> Script {
    storage_keeper_lou_body(ctx, Vec::new()).map(|_| ())
}

fn city_hall_officer_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    if ctx.var("ch_par").get()?.number()? < 5 {
        ctx.lines_as(
            "Jin Chiyuan",
            args![
                "^666666*Yawn~*^000000",
                "Gosh, this hot weather is such",
                "a pain. Oh? You look like a tourist. Are you enjoying your stay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jin Chiyuan",
            args![
                "Although we've been having bad weather recently, there are many good places to visit in Luoyang.",
                "I hope you have a good time."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ch_par").get()?.number()? < 9 {
            ctx.lines_as("Jin Chiyuan", args!["Hm? A Midgardian?", "Now, how may I help you?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I need a document.:I just dropped by.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Jin Chiyuan",
                    args![
                        "Ah, you do?",
                        "Unfortunately, there are many people waiting in line to procure government forms, so it will take",
                        "a while to handle your request."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jin Chiyuan",
                    args![
                        "^333333*Whispers*",
                        "Well, there is a way that you can, shall we say, expediate our processing of your request...^000000"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Huh? Come again?:A little zeny to cut the red tape, eh?")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args![
                            "^666666*Ahem!*^000000 Nothing.",
                            "Nothing of importance. Now, I'm very busy, so if you would go fill the application over there..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (input, status) = runtime::input_number(ctx, None, None)?;
                l_input = input;
                if l_input.clone().number()? < 10000 {
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args![
                            "Good lord,",
                            "what are you thinking?",
                            "What am I, a beggar?!",
                            "That's a poor excuse",
                            "for a bribe!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args![
                            "I mean...",
                            "How dare you bribe an officer of the law! I hope other outsiders are not like you! Please leave immediately!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if runtime::op(&ctx.var("Zeny").get()?, ">", &l_input.clone())?.is_true() {
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args![
                            "What...?!",
                            "That's not what I meant,",
                            "but if you insist on donating to our government..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jin Chiyuan", args!["^666666*Whispers*^000000", "^333333When you go up stairs, another officer will give you the document you want.^000000 ^666666*Ahem*^000000 Luoyang thanks you!"])?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_input.clone())?))?;
                    if ctx.var("ch_par").get()? == 5 {
                        ctx.var("ch_par").set(Val::from(11))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11046), Val::from(11051)])?;
                    } else if ctx.var("ch_par").get()? == 6 {
                        ctx.var("ch_par").set(Val::from(12))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11047), Val::from(11052)])?;
                    } else if ctx.var("ch_par").get()? == 7 {
                        ctx.var("ch_par").set(Val::from(13))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11048), Val::from(11053)])?;
                    } else if ctx.var("ch_par").get()? == 8 {
                        ctx.var("ch_par").set(Val::from(14))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(11049), Val::from(11054)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args!["Good lord, what are you thinking? What am I, a beggar?! That's a poor excuse for a bribe!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jin Chiyuan",
                        args![
                            "I mean...",
                            "How dare you bribe an officer of the law! I hope other outsiders are not like you! Please leave immediately!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Oh well...",
                    "Let me tell you that this is not a good time for tourists. I hope you don't wander into places you're",
                    "not supposed to be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Quite frankly,",
                    "it's a dangerous climate for curiosity right now. Please,",
                    "be careful."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ch_par").get()? == 9 {
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Oh well...",
                    "Let me tell you that this is not a good time for tourists. I hope you don't wander into places you're",
                    "not supposed to be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Quite frankly,",
                    "it's a dangerous climate for curiosity right now. Please,",
                    "be careful."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ch_par").get()? == 10 {
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Oh well...",
                    "Let me tell you that this is not a good time for tourists. I hope you don't wander into places you're",
                    "not supposed to be."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jin Chiyuan",
                args![
                    "Quite frankly,",
                    "it's a dangerous climate for curiosity right now. Please,",
                    "be careful."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ch_par").get()?.number()? < 15 {
            ctx.lines_as(
                "Jin Chiyuan",
                args!["Haven't you met the officer I told you about? I've contacted him about the matter, so you may meet him upstairs."],
            )?;
            ctx.next()?;
            ctx.lines_as("Jin Chiyuan", args!["Hahahaha...", "Take care,", "Midgardian."])?;
        } else if ctx.var("ch_par").get()? == 15 {
            ctx.lines_as(
                "Jin Chiyuan",
                args!["Ah~", "You met him,", "didn't you?", "Hahaha...", "Enjoy your stay", "in Luoyang!"],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as("Jin Chiyuan", args!["Welcome.", "How may I help you?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Jin Chiyuan",
        args![
            "If you have nothing to ask of me, please leave. This place is not",
            "a playground for adventurers."
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn city_hall_officer_lou(ctx: &Ctx) -> Script {
    city_hall_officer_lou_body(ctx, Vec::new()).map(|_| ())
}

fn studying_officer_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_paper = Val::from(0);
    if ctx.var("ch_par").get()?.number()? < 5 {
        ctx.lines_as("Huang Zhishu", args![".....", "^666666*Mumble mumble*^000000"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFHe appears to be",
            "rummaging around for",
            "some documents and takes",
            "no notice of you.^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Excuse me.:Pass him.")])?) == 1 {
            ctx.lines_as(
                "Huang Zhishu",
                args!["Hmmm...?", "Well...", "...", "I see...", "^666666*Mumble mumble...*^000000"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Huang Zhishu", args![".........", "^666666*Mumble mumble*^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()?.number()? < 9 {
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "Books contain the spirit and",
                "ideas of their authors. Any work of art can be considered a window into the soul of its creator."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "^666666*Mumble mumble...*^000000",
                "Hmm... I see...",
                "Ah, I see....",
                "^666666*Mumble mumble...*^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()? == 9 {
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "A book is more than a mere collection of pages and words are more than simple arrangements of letters that mean nothing."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Huang Zhishu", args!["For humans seeking to enrich their lives, books are an important part of their education, as books can impart various kinds of knowledge."])?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "Read as many books as you can. Without books and the knowledge",
                "of others, you'll never grasp the essense of life."
            ],
        )?;
        ctx.next()?;
        ctx.mes("^3355FF*Rummage rummage...*^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()? == 10 {
        ctx.lines_as(
            "Huang Zhishu",
            args!["A book is more than a mere collection of pages and words are more than simple arrangements of letters."],
        )?;
        ctx.next()?;
        ctx.lines_as("Huang Zhishu", args!["For humans seeking to enrich their lives, books are an important part of their education, as they can impart various kinds of knowledge."])?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Zhishu",
            args!["Read as many books as you can. Without books and the knowledge of others, you'll never grasp the essense of life."],
        )?;
        ctx.next()?;
        ctx.lines_as("Huang Zhishu", args!["^3355FF*Rummage rummage...*^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()?.number()? < 15 {
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "Huh?",
                "What are you doing here?",
                "Please don't disturb me,",
                "I'm trying to study."
            ],
        )?;
        ctx.next()?;
        ctx.mes("^3355FF*Rummage rummage...*^000000")?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sorry about that.:I came to pick up my documents.")],
        )?) == 1
        {
            ctx.lines_as("Huang Zhishu", args!["Apology accepted.", "Hmmm....", "Oh I see, I see..."])?;
            ctx.next()?;
            ctx.lines(args!["^3355FF*Rummage rummage...*", ".........^000000"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "Huh?",
                "Ah...",
                "You're the one I'm waiting for.",
                "So what kind of document",
                "did you need?"
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        let subject1 = ctx.var("ch_par").get()?;
        if subject1 == 11 {
            if l_input_s.clone() == "Residence Transfer Application" {
                l_paper = Val::from(1);
            }
        } else if subject1 == 12 {
            if l_input_s.clone() == "Summer SAT Class Application" {
                l_paper = Val::from(1);
            }
        } else if subject1 == 13 {
            if l_input_s.clone() == "Sandy Dust Phenomenon Report" {
                l_paper = Val::from(1);
            }
        } else if subject1 == 14 {
            if l_input_s.clone() == "Communication Proposal" {
                l_paper = Val::from(1);
            }
        }
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "Let's see, now.",
                "You want...",
                ((Val::from("a ") + l_input_s.clone()) + Val::from("?"))
            ],
        )?;
        ctx.next()?;
        if l_paper.clone() == 1 {
            ctx.lines_as(
                "Huang Zhishu",
                args![
                    ((Val::from("Now where did I put that? You want a ") + l_input_s.clone())
                        + Val::from(", huh? I think I'll need some time to find it. Ah, right, I think I know where I put that."))
                ],
            )?;
            ctx.next()?;
            ctx.mes("...")?;
            ctx.next()?;
            ctx.lines(args!["...", "......"])?;
            ctx.next()?;
            ctx.lines(args!["...", "......", "........."])?;
            ctx.next()?;
            ctx.lines(args!["...", "......", ".........", "............"])?;
            ctx.next()?;
            ctx.lines(args!["...", "......", ".........", "............", "..............."])?;
            ctx.next()?;
            ctx.lines_as(
                "Huang Zhishu",
                args![
                    "Here it is. A few days ago, someone asked me for the same document,",
                    "so I was able to find it again pretty easily. I hope it's useful to you."
                ],
            )?;
            ctx.var("ch_par").set(Val::from(15))?;
            l_i = Val::from(11051);
            'l2: loop {
                if !(l_i.clone().number()? <= 11054) {
                    break 'l2;
                }
                'b2: {
                    if (ctx.call(Function::CheckQuest, vec![l_i.clone()])?.number()? > -1
                        && ctx
                            .call(Function::CheckQuest, vec![Val::from(l_i.clone().number()? < 2)])?
                            .is_true())
                    {
                        ctx.call(Function::CompleteQuest, vec![l_i.clone()])?;
                    }
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            ctx.call(Function::SetQuest, vec![Val::from(11055)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Huang Zhishu", args!["Huh?", "Wha...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Zhishu",
            args![
                "I'm sorry but I don't think we have that one. You might want to check the name of the document once",
                "more, and then ask me again."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Huang Zhishu", args!["Hmmm...?", "Do you think I need to go outside more often? Well, I guess for some reason, I don't feel well. I guess I really should get some fresh air. ^666666*Yawn...*^000000"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_OK")?])?;
    ctx.next()?;
    ctx.lines_as("Huang Zhishu", args!["But you should get out more often yourself! It's not a good idea to always stay home. If you don't get some exercise when you're young,", "it could affect your health later."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn studying_officer_lou(ctx: &Ctx) -> Script {
    studying_officer_lou_body(ctx, Vec::new()).map(|_| ())
}

fn supply_stack_1lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "^3355FF* Wait a minute! *",
            "You're carrying too many items with you. Please put some of your items into your Kafra Storage and try again. ^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 4 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 7 {
            ctx.lines(args!["^3355FF*Rummage rummage*", "It seems there's something inside of this stack. It might be a good idea to rummage around to see if you can find something.^000000"])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])? == 67 {
                ctx.lines(args![
                    "^3355FF*Rummage rummage*",
                    "*Rummage rummage*",
                    "*Rummage rummage*",
                    "You found a medicine.^000000"
                ])?;
                ctx.var("ch_par").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11050), Val::from(11056)])?;
                ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiang Xiayou",
                    args![
                        "What...!",
                        "You found it...?!",
                        "Bah, I can't believe it!",
                        "You're just lucked out!",
                        "^666666*Grumble grumble*^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "*Rummage rummage*",
                "*Rummage rummage*",
                "You didn't find anything.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiang Xiayou",
                args![
                    "Hey, hey...",
                    "I told you! It's not too late, why don't you do me the favor I asked? I'm only asking you once!"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_FRET")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Storage Keeper#lou")])?,
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FF*Rummage rummage*",
            "You didn't find anything. You decided to give up your search.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFThere are many things piled up to the ceiling. Since there's too many things jumbled together, it doesn't look like it's possible that you'll find anything here.^000000")?;
    ctx.next()?;
    ctx.lines_as("Jiang Xiayou", args!["Hey, don't touch anything! If you mess anything up, you've gotta pile it up again, got it? If you don't want to clean up after yourself, don't make a mess!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supply_stack_1lou(ctx: &Ctx) -> Script {
    supply_stack_1lou_body(ctx, Vec::new()).map(|_| ())
}

fn supply_stack_5lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "^3355FF* Wait a minute !! *",
            "You're carrying too many items with you. Please put some of your items into your Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 4 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 13 {
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "It seems there's something inside of this stack. You decide to rummage around to see if you can find anything.^000000"
            ])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])? == 43 {
                ctx.lines(args![
                    "^3355FF*Rummage rummage*",
                    "*Rummage rummage*",
                    "*Rummage rummage*",
                    "You found a medicine.^000000"
                ])?;
                ctx.var("ch_par").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11050), Val::from(11056)])?;
                ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiang Xiayou",
                    args!["Err...", "You found it...?!", "I can't believe it!", "You just lucked out."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "*Rummage rummage*",
                "*Rummage rummage*",
                "You didn't find anything.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiang Xiayou",
                args![
                    "Hey, hey...",
                    "I told you!",
                    "It's not too late for you",
                    "to do the favor I asked.",
                    "I'm only asking you once!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FF*Rummage rummage*",
            "You didn't find anything. You decided to give up your search.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFThere are many things piled up to the ceiling. Since there is too much stuff jumbled together, it doesn't seem possible that you'll find anything in this stack.^000000")?;
    ctx.next()?;
    ctx.lines_as("Jiang Xiayou", args!["Hey, don't touch anything! If you mess anything up, you've gotta pile it up again, got it? If you don't want to clean up after yourself, don't make a mess!"])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_ROCK")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Storage Keeper#lou")])?,
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supply_stack_5lou(ctx: &Ctx) -> Script {
    supply_stack_5lou_body(ctx, Vec::new()).map(|_| ())
}

fn supply_stack_4lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "^3355FF* Wait a minute !! *",
            "You're carrying too many items with you. Please put some of your items into your Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 4 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 4 {
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "It seems there's something inside of this stack. You decide to rummage around to see if you can find anything.^000000"
            ])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])? == 11 {
                ctx.lines(args![
                    "^3355FF*Rummage rummage*",
                    "*Rummage rummage*",
                    "*Rummage rummage*",
                    "You found a medicine.^000000"
                ])?;
                ctx.var("ch_par").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11050), Val::from(11056)])?;
                ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiang Xiayou",
                    args!["Err...", "You found it...?!", "I can't believe it!", "You just lucked out."],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_KEK")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Storage Keeper#lou")])?,
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "*Rummage rummage*",
                "*Rummage rummage*",
                "You didn't find anything.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiang Xiayou",
                args![
                    "Hey, hey...",
                    "I told you!",
                    "It's not too late for you",
                    "to do the favor I asked.",
                    "I'm only asking you once!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FF*Rummage rummage*",
            "You didn't find anything. You decided to give up your search.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFThere are many things piled up to the ceiling. Since there is too much stuff jumbled together, it doesn't seem possible that you'll find anything in this stack.^000000")?;
    ctx.next()?;
    ctx.lines_as("Jiang Xiayou", args!["Hey, don't touch anything! If you mess anything up, you've gotta pile it up again, got it? If you don't want to clean up after yourself, don't make a mess!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supply_stack_4lou(ctx: &Ctx) -> Script {
    supply_stack_4lou_body(ctx, Vec::new()).map(|_| ())
}

fn supply_stack_3lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "^3355FF* Wait a minute !! *",
            "You're carrying too many items with you. Please put some of your items into your Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 4 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 18 {
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "It seems there's something inside of this stack. You decide to rummage around to see if you can find anything.^000000"
            ])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])? == 96 {
                ctx.lines(args![
                    "^3355FF*Rummage rummage*",
                    "*Rummage rummage*",
                    "*Rummage rummage*",
                    "You found a medicine.^000000"
                ])?;
                ctx.var("ch_par").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11050), Val::from(11056)])?;
                ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiang Xiayou",
                    args!["Err...", "You found it...?!", "I can't believe it!", "You just lucked out."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "*Rummage rummage*",
                "*Rummage rummage*",
                "You didn't find anything.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiang Xiayou",
                args![
                    "Hey, hey...",
                    "I told you!",
                    "It's not too late for you",
                    "to do the favor I asked.",
                    "I'm only asking you once!"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_KIK")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Storage Keeper#lou")])?,
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FF*Rummage rummage*",
            "You didn't find anything. You decided to give up your search.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFThere are many things piled up to the ceiling. Since there is too much stuff jumbled together, it doesn't seem possible that you'll find anything in this stack.^000000")?;
    ctx.next()?;
    ctx.lines_as("Jiang Xiayou", args!["Hey, don't touch anything! If you mess anything up, you've gotta pile it up again, got it? If you don't want to clean up after yourself, don't make a mess!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supply_stack_3lou(ctx: &Ctx) -> Script {
    supply_stack_3lou_body(ctx, Vec::new()).map(|_| ())
}

fn supply_stack_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines(args![
            "^3355FF* Wait a minute !! *",
            "You're carrying too many items with you. Please put some of your items into your Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 4 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])? == 10 {
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "It seems there's something inside of this stack. You decide to rummage around to see if you can find anything.^000000"
            ])?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])? == 87 {
                ctx.lines(args![
                    "^3355FF*Rummage rummage*",
                    "*Rummage rummage*",
                    "*Rummage rummage*",
                    "You found a medicine.^000000"
                ])?;
                ctx.var("ch_par").set(Val::from(9))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(11050), Val::from(11056)])?;
                ctx.call(Function::GetItem, vec![Val::from(7252), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiang Xiayou",
                    args!["Err...", "You found it...?!", "I can't believe it!", "You just lucked out."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "^3355FF*Rummage rummage*",
                "*Rummage rummage*",
                "*Rummage rummage*",
                "You didn't find anything.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiang Xiayou",
                args!["I told you! Now stop bugging me and leave now! I don't wanna deal with someone who won't trust me."],
            )?;
            ctx.call(
                Function::Emotion,
                vec![
                    ctx.constant("ET_HNG")?,
                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Storage Keeper#lou")])?,
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FF*Rummage rummage*",
            "You didn't find anything. You decided to give up your search.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("^3355FFThere are many things piled up to the ceiling. Since there is too much stuff jumbled together, it doesn't seem possible that you'll find anything in this stack.^000000")?;
    ctx.next()?;
    ctx.lines_as("Jiang Xiayou", args!["Hey, don't touch anything! If you mess anything up, you've gotta pile it up again, got it? If you don't want to clean up after yourself, don't make a mess!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn supply_stack_2(ctx: &Ctx) -> Script {
    supply_stack_2_body(ctx, Vec::new()).map(|_| ())
}
