use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn poison_king_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer_poet = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_question_poet = Val::from(0);
    if (ctx.var("ch_poison").get()? == 0 && ctx.var("cl_poisonking").get()? != 0) {
        if ctx.var("ql_poisonking").get()?.number()? <= 12 {
            ctx.var("ch_poison").set(ctx.var("ql_poisonking").get()?)?;
        } else if ctx.var("ql_poisonking").get()? == 16 {
            ctx.var("ch_poison").set(Val::from(19))?;
        } else if ctx.var("ql_poisonking").get()?.number()? >= 17 {
            ctx.var("ch_poison").set(Val::from(20))?;
        }
        ctx.var("ql_poisonking").set(Val::from(0))?;
    }
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FF * Wait a minute! *",
            "Right now, you are carrying too many items with you. Please place some of your items into Kafra Storage and try again.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_poison").get()?.number()? < 6 {
        ctx.lines_as(
            "Nagash Arses",
            args!["It's been 40 years since I came here. Hahaha, but it doesn't feel like it's been that long."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nagash Arses",
            args![
                "In the past, I was one of the",
                "most renowned experts in the use",
                "of poison. I even created a martial art based on its use, and formed my own martial arts organization."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nagash Arses",
            args![
                "Now, those memories",
                "don't even seem real",
                "anymore. This poem is",
                "all I can remember..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nagash Arses",
            args![
                "As I lay in bed looking",
                "up at the moonlight",
                " ",
                "It looks like the",
                "frost on the ground."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nagash Arses",
            args![
                "I lift head up to look",
                "at the bright moon,",
                " ",
                "I lower my head",
                "feeling homesick."
            ],
        )?;
        ctx.next()?;
        if (((ctx.call(Function::CountItem, vec![Val::from(506)])?.number()? > 0
            || ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0)
            || ctx.call(Function::CountItem, vec![Val::from(716)])?.number()? > 0)
            && (ctx.var("ch_poison").get()?.number()? > 0 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(300)])?.number()? > 99))
        {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Ask about the poem.:Ask about his hometown.:Ask about use of Poison.:Ask about his situation.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "Ah, have you",
                            "heard of this poem?",
                            "As I grow older, my",
                            "memory also grows worse,",
                            "but I really like this poem",
                            "and don't want to forget it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args!["I hope you don't", "mind helping me", "memorize this poem..."],
                    )?;
                    ctx.next()?;
                    l_question_poet = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if l_question_poet.clone() == 1 {
                        ctx.lines_as("Nagash Arses", args!["'^3355FFAs I lay^000000 [ ] ^3355FFlooking up at the moonlight^000000.' In this first line, what word should be in [ ]?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("on the ground:with you:in bed:in the stars")],
                        )?) == 3
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as("Nagash Arses", args!["'^3355FFIt looks like the^000000 [ ] ^3355FFon the ground^000000.' In the second line, which word should be in [ ]?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("frost:dew:pebbles:snow")])?) == 1 {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as("Nagash Arses", args!["Now to see if you see really understand the poem. It's no use to just know the words. They must be a part of you as well."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["How would you describe the overall mood and tone of the speaker of this poem?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Romantic:Wistful:Regretful:Passionate")],
                        )?) == 2
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as("Nagash Arses", args!["What do you think is the", "major theme of this poem?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Tragedy:Separation:Love:Revenge:Buddy Cop Film")],
                        )?) == 2
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Ha ha ha! You understand this poem well! Now, would you repeat the first line for me again?"],
                        )?;
                        ctx.next()?;
                        if l_answer_poet.clone().number()? > 30 {
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "When I lay in bed looking up at the moon light:When I lay in bed thinking of the moon light:As I lay in bed looking up at the moonlight:As I lay in bed thinking of the moonlight",
                                )],
                            )?) == 3
                            {
                                ctx.var("ch_poison").set(Val::from(2))?;
                            }
                        } else {
                            let choice = runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "When I lay in bed looking up at the moon light:When I lay in bed thinking of the moon light:As I lay in bed looking up at the moonlight:As I lay in bed thinking of the moon light",
                                )],
                            )?;
                            ctx.var("@menu").set(choice)?;
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Thank you for your time,",
                                "youngster. Oh, and this is",
                                "an old, famous poem written",
                                "by Li Tai Bai. You know that,",
                                "don't you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_question_poet.clone() == 2 {
                        ctx.lines_as("Nagash Arses", args!["'^3355FFIt looks like the^000000 [ ] ^3355FFon the ground^000000.' In the second line, which word should be in [ ]?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("frost:dew:pebbles:snow")])?) == 1 {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["'^3355FFI lower my head feeling^000000 [ ].' Which word should be in [ ]?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("homesick.:drowsy:loneliness.:heartbroken.")],
                        )?) == 1
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Now to see if you see really understand the poem. It's no use to just know the words. To know this poem by heart is to truly take it to heart."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "According to the poem,",
                                "where is the location",
                                "of the speaker as he is",
                                "gazing at the moon?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("In his hometown.:In jail.:In the depths of the cosmos.:In bed.")],
                        )?) == 4
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as("Nagash Arses", args!["Although this poem is only four lines long, its structure can be easily classified. How would you describe this poem's structure?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Why, it's a sonnet.:It's prose with erratic caesuras.:It's a quatrain, of course.:Iambic pentameter?",
                            )],
                        )?) == 3
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Ha ha ha! You understand this poem well! Now, would you repeat the first line for me again?"],
                        )?;
                        ctx.next()?;
                        if l_answer_poet.clone().number()? > 30 {
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "It looks like the frost on the ground:It looks like an icicle on the ground:It looks as though shining:It looks like the frost in the sky",
                                )],
                            )?) == 1
                            {
                                ctx.var("ch_poison").set(Val::from(2))?;
                            }
                        } else {
                            let choice = runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "It looks like the frost on the ground:It looks like an icicle on the ground:It looks as though shining:It looks like the frost in the sky",
                                )],
                            )?;
                            ctx.var("@menu").set(choice)?;
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Thank you for your time,",
                                "youngster. Oh, and this is",
                                "an old, famous poem written",
                                "by Li Tai Bai. You know that,",
                                "don't you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_question_poet.clone() == 3 {
                        ctx.lines_as("Nagash Arses", args!["'^3355FFIt looks like the^000000 [ ] ^3355FFon the ground^000000.' In the second line, which word should be in [ ]?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("frost:dew:pebbles:snow")])?) == 1 {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["'^3355FFI lower my head feeling^000000 [ ].' Which word should be in [ ]?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("homesick.:drowsy:loneliness.:heartbroken.")],
                        )?) == 1
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Now to see if you see really understand the poem. It's not enough to just know the words. You must know what they truly mean."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["In the first two lines, what two images are being linked by the poet?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Bed and ground:Frost and hometown:Gloomy:Smokey:Moonlight and frost")],
                        )?) == 5
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.lines_as("Nagash Arses", args!["In this land, the image of the moon often appears in poems expressing separation, longing and homesickness. Why would gazing at the moon offer comfort?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "The rabbit on the moon grants wishes.:Its sheer beauty eases any anxiety.:Because it wanes and waxes.:All places and peoples share the same moon.",
                            )],
                        )?) == 4
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Ha ha ha! You understand this poem well! Now, would you repeat the third line for me again?"],
                        )?;
                        ctx.next()?;
                        if l_answer_poet.clone().number()? > 30 {
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "I look up at the bright moon:I lift my head to look at the bright moon:I turn my head to look at the bright moon:I face the bright moon",
                                )],
                            )?) == 2
                            {
                                ctx.var("ch_poison").set(Val::from(2))?;
                            }
                        } else {
                            let choice = runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "I look up at the bright moon:I lift my head to look at the bright moon:I turn my head to look at the bright moon:I face the bright moon",
                                )],
                            )?;
                            ctx.var("@menu").set(choice)?;
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Thank you for your time,",
                                "youngster. Oh, and this is",
                                "an old, famous poem written",
                                "by Li Tai Bai. You know that,",
                                "don't you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "'^3355FFIt looks like the^000000 [ ] ^3355FFon the ground^000000.' In the second line,",
                                "which word should be in [ ]?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("frost:dew:pebbles:snow")])?) == 1 {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["'^3355FFI lift my^000000 [ ] ^3355FFto look at the bright moon^000000.' In the third line, which word should be in [ ]?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("eyes:head:gaze:sights")])?) == 2 {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Now to see if you see really understand the poem. It's no",
                                "use to just know the words. They must be a part of you as well."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["What do you think is the major theme of this poem?"])?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Tragedy:Separation:Love:Revenge:Buddy Cop Film")],
                        )?) == 2
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Why might be one reason why the bright moonlight looks like frost on the ground to the poet?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "He's looking through a frosty window.:A bright moon glimmers like icicles.:He's homesick, so the moonlight looks cold:It's called 'poetic license.'",
                            )],
                        )?) == 3
                        {
                            l_answer_poet = (l_answer_poet.clone() + Val::from(10));
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Ha ha ha! You understand this poem well! Now, would you repeat the last line for me again?"],
                        )?;
                        ctx.next()?;
                        if l_answer_poet.clone().number()? > 30 {
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "I cry for my home town.:I lower my head feeling homesick.:I miss my home town.:I sob feeling homesick.",
                                )],
                            )?) == 2
                            {
                                ctx.var("ch_poison").set(Val::from(2))?;
                            }
                        } else {
                            let choice = runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "I cry for my home town.:I lower my head feeling homesick.:I miss my home town.:I sob feeling homesick.",
                                )],
                            )?;
                            ctx.var("@menu").set(choice)?;
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Thank you for your time,",
                                "youngster. Oh, and this is",
                                "an old, famous poem written",
                                "by Li Tai Bai. You know that,",
                                "don't you?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "Are you asking me",
                            "about my hometown...?",
                            "As a Midgardian,",
                            "I'm sure you've at least",
                            "heard of Morocc..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Nagash Arses", args!["A city built in the middle of the desert, its founders had to combat the harsh and unforgiving forces of nature everyday."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "Yeah, the blazing sun never",
                            "seems to leave, and it's a dry,",
                            "desert area, but people still",
                            "manage to live there."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.var("ch_poison").get()? == 2 {
                        ctx.lines_as("Nagash Arses", args!["Ah~ I miss Morocc, my hometown.", "I used to be a member of the Assassin organization known as the 'Canine of Desert.' Long ago, I was their poison expert."])?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Tell him news of Morocc.:Info about 'Canine of Desert'.:Just listen.")],
                            )?);
                            let mut matched2 = false;
                            let no_case2 = !subject2.loosely_equals(&Val::from(1))
                                && !subject2.loosely_equals(&Val::from(2))
                                && !subject2.loosely_equals(&Val::from(3));
                            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Nagash Arses", args!["Hm? Do you have recent news of Morocc?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args!["Yeah, I've got some news you might be interested in."],
                                )?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![((Val::from("") + l_input_s.clone()) + Val::from(""))],
                                )?;
                                ctx.next()?;
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(50)])?.number()? > 25 {
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args!["Ah, I see. Thank you for the news. Now, let me continue my story."],
                                    )?;
                                    ctx.next()?;
                                } else {
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args!["I see! You just said,", ((Val::from("") + l_input_s.clone()) + Val::from("."))],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Nagash Arses", args!["Thank you for", "telling me the news."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                break 'b2;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Nagash Arses", args!["I just told you what 'Canine of Desert' is. Now you tell me what it is. If you were listening, you would be able to."])?;
                                ctx.next()?;
                                let (input, status) = runtime::input_text(ctx, None, None)?;
                                l_input_s = input;
                                if ((l_input_s.clone() == "Assassin Organization" || l_input_s.clone() == "Assassin")
                                    || l_input_s.clone() == "Assassins")
                                {
                                    ctx.lines_as("Nagash Arses", args!["Correct. You listened to me very well. They are Assassins. Assassins that were abandoned by society."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args!["'Canine of Desert' is the name of that Assassin organization. Don't forget that."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Nagash Arses", args!["..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Nagash Arses", args!["...", "......"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args![
                                            "Bah! I forgot what I was just talking about! This bad memory of mine frustrates me so much!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                        ctx.lines_as("Nagash Arses", args!["How could you not know what the 'Canine of Desert' is?! Just how long have you been an Assassin?!"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Nagash Arses",
                                            args!["You're supposed to be aware of your origins and your colleagues! Oh, the shame..."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as("Nagash Arses", args!["Well, I suppose I can't blame you for not knowing or remembering. When you get the chance, find someone wearing a purple suit..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args!["Any Assassin worth his salt should know what the 'Canine of Desert' is."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                matched2 = true;
                            }
                            if matched2 && ctx.call(Function::Rand, vec![Val::from(1), Val::from(33)])?.number()? < 12 {
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args![
                                        "Wait...",
                                        "Let me think...",
                                        "Now how did I becoming interested in the use of poison...?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "I tried many things to develop my poison skills when I was young.",
                                "I tried to extract poison from Muka's needles and from purple mushrooms. Eventually, I became",
                                "an expert of toxins from my efforts."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "One time I even injected",
                                "a poison into my own body",
                                "to fully test it. Yeah...",
                                "It almost killed me."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["I was recognized as the man who", "was most highly skilled in the use of poison in the 'Canine of Desert,' and I was sent on the most crucial and dangerous missions."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["One day, I was hired by the Alberta Merchant Guild to assassinate an enemy that had been threatening them. However, I never got to complete that mission."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["While I was on the ship to procceed with my mission, we encountered heavy wind and waves and the ship sank. I believe I was the only survivor of that accident."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "I was floated helplessly on the ocean and somehow managed",
                                "to arrive here, in Luoyang. That was forty years ago."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["It seemed that my arrival was rather timely. Luoyang was intruded by huge mobs of dangerous monsters that were even able to infiltrate the Castle of the Dragon."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Since I was one of the best Assassins of the 'Canine of Desert,' I did far more than my share of monster killing."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "It was on the battlefield that",
                                "I met Bai Long who is now lord of this town. But back then, he was known as the 'Street Knight.'"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["I remember seeing him surrounded", "by enemies, and I dove into thick of battle to keep him from getting killed. We fought back to back and managed to stay alive back then."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "You have to understand that it's not easy to let someone back you",
                                "up in battle unless there's a solid trust. Those fights... were the greatest moments in my life."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Sadly, many of our comrades,", "all of them respectable and highly skilled martial artists, fell in battle. The number of monsters we had to contend with was just overwhelming."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Still, I managed to continue my poison research, even during those tough times. I studied medicine in this town and was able use that knowledge to enhance my understanding of poisons."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "I created a new skill based on all of my knowledge, and learned how",
                                "to put poison on weapons."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "I even learned martial arts in Luoyang, and combined that",
                                "knowledge with my poison expertise to create my own unique fighting style."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Well, that's my story.",
                                "But now, I just miss my",
                                "home town. I miss the heat",
                                "of the desert and the glare",
                                "of the blazing sun..."
                            ],
                        )?;
                        ctx.var("ch_poison").set(Val::from(3))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Ah, Morocc.",
                                "That name brings",
                                "back memories of",
                                "being an Assassin",
                                "in the 'Canine of Desert.'"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["^666666*Sigh...*^000000", "I wish I could see", "Morocc just once before I die."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Tell him news of Morocc.:Ask about 'Canine of Desert.'")],
                        )?) == 1
                        {
                            ctx.lines_as("Nagash Arses", args!["Hm...?", "Is there anything", "you want to say?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I have news", "of Morocc..."],
                            )?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            l_input_s = input;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![((Val::from("") + l_input_s.clone()) + Val::from(""))],
                            )?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(50)])?.number()? > 30 {
                                ctx.lines_as("Nagash Arses", args!["I see...", "Thank you for", "telling me that."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "I see!",
                                    ((Val::from("You just said, ") + l_input_s.clone()) + Val::from(". Thank you."))
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Nagash Arses", args!["I just told you what 'Canine of Desert.' If you can't remember what I just told you, it's no use for me to explain further. You go ahead and tell me what it is."])?;
                        ctx.next()?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        l_input_s = input;
                        if ((l_input_s.clone() == "Assassin Organization" || l_input_s.clone() == "Assassin")
                            || l_input_s.clone() == "Assassins")
                        {
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "Correct. You listened to me very well. They are Assassins. Assassins that were abandoned by society."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "Canine of Desert",
                                    "is the name of an",
                                    "Assassin organization.",
                                    "Do not forget that."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args![
                                        "How could you not know what the 'Canine of Desert' is?! Just how long have you been an Assassin?!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args!["You're supposed to be aware of your origins and your colleagues! Oh, the shame..."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Nagash Arses", args!["Well, I suppose I can't blame you for not knowing or remembering. When you get the chance, find someone wearing a purple suit..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args!["Any Assassin worth his salt should know what the 'Canine of Desert' is."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                3 => {
                    if (ctx.var("ch_poison").get()? == 4 || ctx.var("ch_poison").get()? == 5) {
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Did you just say you", "want to learn about the", "use of poison?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Why do you want",
                                "to learn about it?",
                                "Even if I wanted to",
                                "teach, I'm too old to",
                                "remember everything",
                                "clearly..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Go find my last disciple,", "^0000FFSong Zhi Du^000000, as he may tell you something useful. He's working at the doctor's office. If it weren't for him, I'd be starving now."])?;
                        ctx.var("ch_poison").set(Val::from(5))?;
                        ctx.call(Function::SetQuest, vec![Val::from(11070)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Did you just say you", "want to learn about the", "use of poison?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Why do you want",
                                "to learn about it?",
                                "Even if I wanted to",
                                "teach, I'm too old to",
                                "remember everything",
                                "clearly..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                4 => {
                    if ctx.var("ch_poison").get()? == 3 {
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["You're asking me how", "things came to be like", "this? Life used to be good."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["As I told you, I distinguished myself in the battles with monsters. I was rewarded with Luoyang citizenship and given", "gifts of money."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["I was invited to join a martial arts organization that specialized in the use of poison. I joined them, excited about further broadening my knowledge."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["I was stunned by the enormous body of knowledge about toxins that they provided. I tried to learn as much as I could so I could develop my own, unique poisoning skills."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["As I studied with them, together we started to clear the town of the remaining monsters. Can you guess the results of our efforts to clean up the city?"])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "...??:It must have been good.:I guess it was okay?:I don't know, but how did it go?",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as("Nagash Arses", args!["Did you even listen to me? It's no use talking to you if you don't even care about what I have to say."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args!["I'm sorry for keeping you from what you wanted to do. Take care of yourself, youngster."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args!["You're right!", "We got rid of every", "single monster in the city!"],
                                )?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args!["Just okay...?", "I don't think you realized how powerful we were back then! Eh..."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args!["Sorry for keeping you from what you wanted to do. Take care of yourself, youngster."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            4 => {
                                ctx.lines_as("Nagash Arses", args!["We got rid of every single monster in the city!"])?;
                                ctx.next()?;
                            }
                            _ => {}
                        }
                        ctx.lines_as("Nagash Arses", args!["We managed to eliminate every monster that was wandering around Luoyang. At the time, we were the only people brave enough to take on this sort of task."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Unfortunately, other martial arts organizations grew envious of our success, insisting that we were dishonorable for using poison to attack our enemies."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Although we can accept criticism for our use of poison, we were finally blamed for something we never would have done."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["Someone put poison into a meal which was eaten by a son of the lord and then falsely accused us!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "In the end, our organization was disbanded and I was put in jail.",
                                "I suffered through much to escape from prison..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "However, because of all the injuries I've had to endure in jail and in my escape attempts,",
                                "I can no longer use my",
                                "martial arts."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["Even to this day, the police are hounding after me. I really want to tell the lord of Luoyang that I'm innocent, but it may be", "too late now..."])?;
                        ctx.var("ch_poison").set(Val::from(4))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["I...", "I don't want", "to talk about that.", "Let's not talk about it."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                _ => {}
            }
        } else {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Grin at him.:Lament for his grief.:Reprove him.:Listen to the poem again.:Show him a sad look.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "Yeah, I don't blame you.",
                            "I know I look stupid. Legally,",
                            "I'm a criminal after all.",
                            "I don't any friends and",
                            "there's no one I can trust."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "How can you sympathize?",
                            "I doubt anyone has had",
                            "experiences that are",
                            "much worse than mine..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Nagash Arses", args!["Wha--? I haven't wronged you in any way! Why must you be so mean to an old man? You don't even know half of what I've had to go through."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "As I lay in bed looking",
                            "up at the moonlight",
                            " ",
                            "It looks like the",
                            "frost on the ground."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "I lift head up to look",
                            "at the bright moon,",
                            " ",
                            "I lower my head",
                            "feeling homesick."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                5 => {
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "Do you miss your hometown",
                            "as much as I do? I'm envious",
                            "of you, youngster. You have",
                            "the freedom to go",
                            "wherever you want."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args![
                            "I'm a wanted criminal.",
                            "Even if I were free to",
                            "travel, I may not have",
                            "the strength to try."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args!["Let's not talk about this any longer. It's reminding me of", "my worst memories."],
                    )?;
                    ctx.next()?;
                    ctx.var("ch_poison").set(Val::from(1))?;
                    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Ah, you look just like I did when",
                                "I was young. It seems you know",
                                "a little something about poison."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "It seems you haven't",
                                "perfected your knowledge yet,",
                                "though. Oh well, I doubt I can",
                                "pinpoint what you need to master."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["I'm too old to remember anything. Ha ha ha. Maybe if I saw something related to poison, I might remember something..."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["No, no...", "I think I'm too old", "to remember anything.", "Anything at all."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Nagash Arses", args!["I'm too old to remember anything. Ha ha ha. Maybe if I saw something related to poison, I might remember something..."])?;
                    ctx.next()?;
                    ctx.lines_as("Nagash Arses", args!["..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Nagash Arses",
                        args!["No, no...", "I think I'm too old", "to remember anything.", "Anything at all."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    } else {
        if (ctx.var("ch_poison").get()?.number()? > 4 && ctx.var("ch_poison").get()?.number()? < 12) {
            ctx.lines_as(
                "Nagash Arses",
                args![
                    "If you wish to talk about poison, you'd better go to the doctor's office. My last disciple is",
                    "working there."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nagash Arses",
                args![
                    "Now, let me rest.",
                    "I am very tired.",
                    "I... I hope he doesn't",
                    "have any bad intentions..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ch_poison").get()? == 12 {
                ctx.lines_as(
                    "Nagash Arses",
                    args![
                        "Aren't you the youngster",
                        "I talked to a while ago?",
                        "Did you get a chance",
                        "to meet my disciple?",
                        "How was he?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Nagash Arses", args!["He was sick all the time when he was young, so he worried me a lot. But now I'm proud of him and he's doing as well just any other fine young man."])?;
                ctx.next()?;
                ctx.lines_as("Nagash Arses", args!["I am living only for him,", "and hope that he becomes a great healer someday. Recently, I've found that my sole comfort is in treating diseases for other people..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Nagash Arses",
                    args!["By the way...", "You look pale...", "Did something happen?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Nothing.:I want to talk about your diciple.:I don't feel good.")],
                )? {
                    1 => {
                        ctx.lines_as("Nagash Arses", args!["Oh...", "I see...", "But take care", "of yourself."])?;
                        ctx.next()?;
                        ctx.lines_as("Nagash Arses", args!["You better enjoy your physical strength when you're young. When you're my age, it's tough to regain your health once you lose it."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Did something happen to him?",
                                "I've been worried since he hasn't come to visit me since you last came to speak with me."
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.call(Function::CountItem, vec![Val::from(678)])?.number()? > 1 {
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Give him Poison Bottle.:Cancel and go to 'Song Zhi Du.")],
                            )?) == 1
                            {
                                ctx.call(Function::DelItem, vec![Val::from(678), Val::from(1)])?;
                                ctx.lines_as("Nagash Arses", args!["Huh!?", "Isn't this...!?"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args![
                                        "I've dreamed of creating",
                                        "a poison of such potency!",
                                        "W-where did you find this?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("It's common nowadays.:Explain to him what happened.")],
                                )?) == 1
                                {
                                    ctx.lines_as("Nagash Arses", args!["Really...?", "Then someone", "finally figured it out."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Nagash Arses", args!["I shared all of my knowledge with others, so I guess a doctor might have figured how to create this."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Nagash Arses",
                                        args!["^666666*Sigh...*^000000", "Now I feel depressed", "for some reason..."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines(args!["^3355FFYou tell Nagash Arses how", "Song Zhi Du created this Deadly Poison Bottle and his plan to get his revenge on the lord of Luoyang.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Nagash Arses", args!["^666666*Sigh...*^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Nagash Arses", args!["Will you...", "Will you excuse", "me for a second...?"])?;
                                ctx.var("ch_poison").set(Val::from(15))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(11077), Val::from(11078)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Nothing much."])?;
                            ctx.next()?;
                            ctx.lines_as("Nagash Arses", args!["Hmm...?", "I guess you don't know what is going on. I've heard some bad rumors about him, which is why I'm asking."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args!["I hope he'll be fine.", "After all, he's truly", "kind at heart."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Hmm...?",
                                "I guess you don't know what is going on. I've heard some bad rumors about him, which is why I'm asking."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Nagash Arses",
                            args!["I hope he'll be fine.", "After all, he's truly", "kind at heart."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Nagash Arses",
                            args![
                                "Oh, you don't?",
                                "Well, that can happen",
                                "sometimes when you're",
                                "in a foreign land for",
                                "too long. I hope you",
                                "feel better soon."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("ch_poison").get()? == 13 {
                    ctx.lines_as("Nagash Arses", args!["Recently...", "I've been getting", "a bad feeling..."])?;
                    ctx.next()?;
                    ctx.lines_as("Nagash Arses", args!["I want to blame it", "on my solitude, but..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ch_poison").get()? == 14 {
                        ctx.mes("^3355FFNagash Arses. He is no longer a well-known Assassin or the martial arts organization of which he was once master has long since disbanded.^000000")?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFNow he is but a poor old man", "who is in anguish over his disciple. No matter what you say, he continues to talk about something totally unrelated, as if in a dream.^000000"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ch_poison").get()? == 15 || ctx.var("ch_poison").get()? == 16) {
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "Youngster...",
                                    "I beg of you.",
                                    "Please do not let him",
                                    "bring any harm to the lord."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "If you cannot stop him,",
                                    "then please interrupt him",
                                    "from carrying out his plan."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "I tried...",
                                    "To convince myself",
                                    "that what I had heard",
                                    "were just ugly rumors.",
                                    "I was wrong..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Nagash Arses", args!["I am pretty sure he's been asking you, as well as others, for aid. Even if you do not agree to carry out his plan, I'm sure he will find someone to poison Lord Bai Long."])?;
                            ctx.next()?;
                            ctx.lines_as("Nagash Arses", args!["There is only one way he can poison", "the lord. He likes drinking, and has a favorite drink bottle he keeps with him all the time. You must get rid of his drink."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Nagash Arses",
                                args![
                                    "Please, I beg you.",
                                    "For my sake, as well",
                                    "as that of my disciple,",
                                    "Luoyang's leader must live..."
                                ],
                            )?;
                            ctx.var("ch_poison").set(Val::from(16))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11078), Val::from(11079)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ch_poison").get()? == 17 {
                                ctx.lines_as("Nagash Arses", args!["So...", "What...", "What happened?"])?;
                                ctx.next()?;
                                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I made it."])?;
                                ctx.next()?;
                                ctx.lines_as("Nagash Arses", args!["You did?!", "Oh~ thank you,", "thank you so much!"])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nagash Arses",
                                    args![
                                        "Thank God I've",
                                        "lived this long.",
                                        "And... I just realized",
                                        "something while you away."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Nagash Arses", args!["This is a journal that I have been writing for 15 years about my feelings of guilt, and what really happened in the past. I hope you can deliver this to the lord for me."])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou obtained",
                                    "^0000FFPoison King",
                                    "Nagash Arses' Journal^000000."
                                ])?;
                                ctx.var("ch_poison").set(Val::from(18))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(11080), Val::from(11081)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("ch_poison").get()? == 18 {
                                    ctx.lines_as("Nagash Arses", args!["I hope you", "can deliver this", "to Lord Bai Long..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("ch_poison").get()? == 19 {
                                        ctx.lines(args!["^3355FFYou gave him the", "^0000FFLetter from Bai Long^000000."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Nagash Arses", args!["What's this...?"])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Nagash Arses",
                                            args![
                                                "Why, he's invited me",
                                                "to take the position of",
                                                "a government official!",
                                                "I'm truly grateful!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Nagash Arses", args!["However, please let him", "know that I must respectfully decline his offer. I'm afraid that I may be a burden to him once again."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Nagash Arses", args!["Please tell him that I wish to enjoy the rest of my life here. Also, there are people here", "who still need me..."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Nagash Arses",
                                            args![
                                                "Thank you, youngster.",
                                                "You've stopped my disciple",
                                                "and helped me make up",
                                                "with an old friend..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.mes("^3355FFNagash Arses put his palm on your back, and you can feel his strength flowing into you.^000000")?;
                                        ctx.next()?;
                                        ctx.mes("^3355FFYou grow dizzy, but you also feel like you're becoming more powerful and gaining experience.^000000")?;
                                        ctx.next()?;
                                        ctx.var("ch_poison").set(Val::from(20))?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(11082), Val::from(11083)])?;
                                        {
                                            if ctx.var("BaseLevel").get()?.number()? < 56 {
                                                ctx.call(Function::GetExperience, vec![Val::from(9000), Val::from(0)])?;
                                            } else {
                                                if (ctx.var("BaseLevel").get()?.number()? > 55
                                                    && ctx.var("BaseLevel").get()?.number()? < 61)
                                                {
                                                    ctx.call(Function::GetExperience, vec![Val::from(10500), Val::from(0)])?;
                                                } else {
                                                    if (ctx.var("BaseLevel").get()?.number()? > 60
                                                        && ctx.var("BaseLevel").get()?.number()? < 66)
                                                    {
                                                        ctx.call(Function::GetExperience, vec![Val::from(18684), Val::from(0)])?;
                                                    } else {
                                                        if (ctx.var("BaseLevel").get()?.number()? > 65
                                                            && ctx.var("BaseLevel").get()?.number()? < 71)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(27411), Val::from(0)])?;
                                                        } else if (ctx.var("BaseLevel").get()?.number()? > 70
                                                            && ctx.var("BaseLevel").get()?.number()? < 76)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(70757), Val::from(0)])?;
                                                        } else if (ctx.var("BaseLevel").get()?.number()? > 75
                                                            && ctx.var("BaseLevel").get()?.number()? < 81)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(130246), Val::from(0)])?;
                                                        } else if (ctx.var("BaseLevel").get()?.number()? > 80
                                                            && ctx.var("BaseLevel").get()?.number()? < 86)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(150340), Val::from(0)])?;
                                                        } else if (ctx.var("BaseLevel").get()?.number()? > 85
                                                            && ctx.var("BaseLevel").get()?.number()? < 91)
                                                        {
                                                            ctx.call(Function::GetExperience, vec![Val::from(182052), Val::from(0)])?;
                                                        } else {
                                                            ctx.call(Function::GetExperience, vec![Val::from(406786), Val::from(0)])?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ctx.lines(args!["^3355FFThen...", "Everything blacks out...^000000"])?;
                                        ctx.close_window()?;
                                        ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(270), Val::from(136)])?;
                                        return Err(Stop::End);
                                    } else if ctx.var("ch_poison").get()?.number()? > 20 {
                                        ctx.lines(args![
                                            "^3355FFHe looks tired",
                                            "for some reason,",
                                            "as if he's thinking",
                                            "of Morocc, his hometown.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.mes("^3355FFYou reflect on when he gave you his remaining strength and remember that he told you that he used to be one of the greatest Assassins...^000000")?;
                                        ctx.next()?;
                                        ctx.lines(args!["^3355FFBut after all that's happened,", "he just looks like an old man who likes treating other people. It seems that he's finally at peace with himself.^000000"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as("Nagash Arses", args!["My name, Nagash Arses,", "is also the name of the poison king in the legend of Arcturus. I have good reason to be proud of my name!"])?;
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

pub fn poison_king_lou(ctx: &Ctx) -> Script {
    poison_king_lou_body(ctx, Vec::new()).map(|_| ())
}
