use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn relaydummy2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gid = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_name_s = Val::from("");
    let mut l_time = Val::from(0);
    let mut l_x = Val::from(0);
    l_name_s = ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?;
    let position = ctx
        .call(Function::GetMapXy, vec![ctx.constant("BL_NPC")?])?
        .into_array()
        .ok_or_else(|| Stop::Error("Invalid position".into()))?;
    l_m_s = position[0].clone();
    l_x = position[1].clone();
    l_x = position[2].clone();
    l_gid = ctx.call(Function::GetCastleData, vec![l_m_s.clone(), Val::from(1)])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a minute! You're",
            "carrying too many items",
            "right now: store some of",
            "your extra things in Kafra",
            "Storage, and then come back.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if l_gid.clone() == 0 {
        ctx.lines(args![
            "^3355FFYou're not sure why, but",
            "this guy seems to be pretty",
            "depressed. He briefly makes",
            "eye contact with you, but then",
            "breaks it off. Apparently,",
            "he wants to be left alone.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThere's no reason for you",
            "to stick around an ownerless",
            "stronghold. You may as well",
            "head on your way.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(2)])?
        .loosely_equals(&l_gid.clone())
    {
        if ctx
            .call(
                Function::GetGuildInfo,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(2)],
            )?
            .loosely_equals(&Val::from(1))
        {
            if ctx.call(Function::CountItem, vec![Val::from(7234)])?.number()? > 0 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hm. You can't just keep",
                        "that spirit to yourself.",
                        "Give it to a ^FF0000Knight^000000 or",
                        "a ^FF0000Lord Knight^000000. Hurry it up!",
                        "You need to work fast to",
                        "expand your guild."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hm. You seem awfully",
                        "busy. Why don't you just",
                        "take a break, and relax?",
                        "It's alright to have fun",
                        "if you're feeling a lot of",
                        "pressure. Heh heh heh~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
                && ctx.call(Function::CountItem, vec![Val::from(7234)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "For your first test,",
                        "I'll start you off with",
                        "something pretty simple.",
                        "Just collect some items,",
                        "and bring them to me. Ah, and",
                        "your guild can help you too."
                    ],
                )?;
                ctx.next()?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^FF000030 Tongues^000000,",
                            "^FF000030 Dark Masks^000000, and",
                            "^FF000030 Shoulder Protectors^000000.",
                            "That shouldn't be too",
                            "hard now, right?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7234), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^FF000030 Worn Out Pages^000000,",
                            "^FF000030 Round Shells^000000, and",
                            "^FF000030 Mole Whiskers^000000.",
                            "That shouldn't be too",
                            "hard now, right?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7234), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^FF000030 Frills^000000,",
                            "^FF000030 Sharp Papers^000000, and",
                            "^FF000030 Elder Pixie's Moustaches^000000.",
                            "That shouldn't be too",
                            "hard now, right?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7234), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(89))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(1015)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7196)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7157)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 2)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I didn't actually",
                        "expect that you'd collect",
                        "all these items. Good job.",
                        "Now, take this and give it to",
                        "a Blacksmith or Mastersmith.",
                        "Your guild's pretty good..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(1015), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7196), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7157), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(88))?;
                ctx.call(Function::GetItem, vec![Val::from(7235), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) && ctx.var("guildrelay_q").get()? == 2) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Did you forget what",
                        "to bring me? I wanted",
                        "^FF000030 Tongues^000000,",
                        "^FF000030 Dark Masks^000000, and",
                        "^FF000030 Shoulder Protectors^000000.",
                        "Don't forget this time."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(1097)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(1017)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(1096)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 3)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I didn't actually",
                        "expect that you'd collect",
                        "all these items. Good job.",
                        "Now, take this and give it to",
                        "a Blacksmith or Mastersmith.",
                        "Your guild's pretty good..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(1097), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1017), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1096), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(88))?;
                ctx.call(Function::GetItem, vec![Val::from(7235), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) && ctx.var("guildrelay_q").get()? == 3) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Did you forget what",
                        "to bring me? I wanted",
                        "^FF000030 Worn Out Pages^000000,",
                        "^FF000030 Round Shells^000000, and",
                        "^FF000030 Mole Whiskers^000000.",
                        "Don't forget this time."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(7112)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(1012)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(1040)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 89)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I didn't actually",
                        "expect that you'd collect",
                        "all these items. Good job.",
                        "Now, take this and give it to",
                        "a Blacksmith or Mastersmith.",
                        "Your guild's pretty good..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7112), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1012), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1040), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(88))?;
                ctx.call(Function::GetItem, vec![Val::from(7235), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) && ctx.var("guildrelay_q").get()? == 89) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Did you forget what",
                        "to bring me? I wanted",
                        "^FF000030 Frills^000000,",
                        "^FF000030 Sharp Papers^000000, and",
                        "^FF000030 Elder Pixie's Moustaches^000000.",
                        "Don't forget this time."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)
                && ctx.call(Function::CountItem, vec![Val::from(7235)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Wow, you brought the",
                        "items sooner than I had",
                        "expected. Great! Well then,",
                        "your next task for me will",
                        "be to... Wait. Wait and",
                        "kill some time. Easy, huh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "^FF0000just waiting.^000000",
                        "Justing wait and kill",
                        "some time. You can do",
                        "whatever you want to",
                        "do during that time."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Come back after you've",
                        "relaxed and enjoyed yourself.",
                        "We can continue the testing",
                        "when the time is right so",
                        "don't you worry about it."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7235), Val::from(1)])?;
                ctx.var("guildrelay_q").set(Val::from(4))?;
                ctx.var("guildtime").set(runtime::atoi(
                    &ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?,
                ))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_time = runtime::atoi(&ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?);
            if ((ctx.var("guildtime").get()?.number()? > 2259 && ctx.var("guildrelay_q").get()? == 4)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
                && (l_time.clone().number()? > 129 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true())
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I guess enough time",
                        "has passed. You ready",
                        "to resume the testing?",
                        "Please give this to an",
                        "Alchemist or Biochemist.",
                        "Your work here is done."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(87))?;
                ctx.call(Function::GetItem, vec![Val::from(7237), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ((ctx.var("guildtime").get()?.number()? > 2159 && ctx.var("guildrelay_q").get()? == 4)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
                && (l_time.clone().number()? > 65 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true())
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I guess enough time",
                        "has passed. You ready",
                        "to resume the testing?",
                        "Please give this to an",
                        "Alchemist or Biochemist.",
                        "Your work here is done."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(87))?;
                ctx.call(Function::GetItem, vec![Val::from(7237), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ((ctx.var("guildtime").get()?.number()? > 2059 && ctx.var("guildrelay_q").get()? == 4)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
                && (l_time.clone().number()? > 1 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true())
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I guess enough time",
                        "has passed. You ready",
                        "to resume the testing?",
                        "Please give this to an",
                        "Alchemist or Biochemist.",
                        "Your work here is done."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(87))?;
                ctx.call(Function::GetItem, vec![Val::from(7237), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 192 && ctx.var("guildrelay_q").get()? == 4)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I guess enough time",
                        "has passed. You ready",
                        "to resume the testing?",
                        "Please give this to an",
                        "Alchemist or Biochemist.",
                        "Your work here is done."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(87))?;
                ctx.call(Function::GetItem, vec![Val::from(7237), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 4 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You must be rarin'",
                        "to take on the next",
                        "test. Please be patient,",
                        "kill some time, and just",
                        "come back to me later."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                && ctx.call(Function::CountItem, vec![Val::from(7237)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Oh, you got it, eh?",
                        "You might not think this",
                        "will be so hard, but I'm",
                        "going to give you a much",
                        "different challenge now.",
                        "Are you ready for it?"
                    ],
                )?;
                ctx.next()?;
                if (ctx.var("BaseLevel").get()?.number()? > 1 && ctx.var("BaseLevel").get()?.number()? < 58) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Your test will be to",
                            "grow a little stronger!",
                            "You'll pass once you",
                            "gain 3 more levels.",
                            "How about that, eh?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7237), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseLevel").get()?.number()? > 57 && ctx.var("BaseLevel").get()?.number()? < 76) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Your test will be to",
                            "grow a little stronger!",
                            "You'll pass once you",
                            "gain 2 more levels.",
                            "How about that, eh?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7237), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 94) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Your test will be to",
                            "grow a little stronger!",
                            "You'll pass once you",
                            "gain 1 more level.",
                            "How about that, eh?"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7237), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(7))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseLevel").get()?.number()? > 93 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Wait a minute. Forget it!",
                            "I was going to ask you to",
                            "raise your Base Level, but",
                            "you're tough enough as it is.",
                            "Fine, fine. You pass! Give this",
                            "to a ^FF0000Hunter^000000 or ^FF0000Sniper^000000 now~"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7237), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(86))?;
                    ctx.call(Function::GetItem, vec![Val::from(7238), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(2)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 5)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You got a little stronger,",
                        "eh? Fine, fine. You pass.",
                        "Here, now take this and",
                        "give it to a ^FF0000Hunter^000000 or ^FF0000Sniper^000000",
                        "in your guild. Nice work,",
                        "and I'll see you around."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(86))?;
                ctx.call(Function::GetItem, vec![Val::from(7238), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(1)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 6)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You got a little stronger,",
                        "eh? Fine, fine. You pass.",
                        "Here, now take this and",
                        "give it to a ^FF0000Hunter^000000 or ^FF0000Sniper^000000",
                        "in your guild. Nice work,",
                        "and I'll see you around."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(86))?;
                ctx.call(Function::GetItem, vec![Val::from(7238), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (runtime::op(&ctx.var("BaseLevel").get()?, "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 7)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You got a little stronger,",
                        "eh? Fine, fine. You pass.",
                        "Here, now take this and",
                        "give it to a ^FF0000Hunter^000000 or ^FF0000Sniper^000000",
                        "in your guild. Nice work,",
                        "and I'll see you around."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(86))?;
                ctx.call(Function::GetItem, vec![Val::from(7238), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ((ctx.var("guildrelay_q").get()? == 5 || ctx.var("guildrelay_q").get()? == 6) || ctx.var("guildrelay_q").get()? == 7) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You're still not strong",
                        "enough to pass this test.",
                        "Hurry up, kill some monsters,",
                        "and gain some ^FF0000Base Levels^000000.",
                        "You have to become stronger!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?)
                && ctx.call(Function::CountItem, vec![Val::from(7238)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Ah, good, you're here.",
                        "Now, my request for you",
                        "is this: please donate",
                        "your Falcon for the",
                        "sake of your guild."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Donate Falcon:No way!")])? {
                    1 => {
                        if ctx.call(Function::CheckFalcon, vec![])?.is_true() {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Great! Don't worry,",
                                    "your Falcon will be safe",
                                    "under our care, and will",
                                    "be use to scout areas and",
                                    "deliver mail. That's why",
                                    "I asked you for it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Now, please take this",
                                    "spirit, and give it to",
                                    "your guild master as",
                                    "soon as you can, alright?"
                                ],
                            )?;
                            ctx.call(Function::SetFalcon, vec![])?;
                            ctx.call(Function::DelItem, vec![Val::from(7238), Val::from(1)])?;
                            ctx.var("guildrelay_q").set(Val::from(85))?;
                            ctx.call(Function::GetItem, vec![Val::from(7239), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "I'm glad that you're",
                                    "willing to part with your",
                                    "Falcon, but it doesn't seem",
                                    "to be with you right now.",
                                    "Go get one, and come back."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Well, I can't help it if you",
                                "really want to keep your",
                                "Falcon, but please come",
                                "back if you change your mind.",
                                "I can't do anything for you",
                                "unless you cooperate."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if (ctx.var("guildrelay_q").get()? == 88 && ctx.call(Function::CountItem, vec![Val::from(7235)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Charge to",
                        "a ^FF0000Blacksmith^000000 or ^FF0000Mastersmith^000000.",
                        "It won't do any good in the",
                        "hands of anybody else."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 88 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You don't have",
                        "anything to do?",
                        "Why don't you help",
                        "your guild hunt monsters?",
                        "It'll be a good chance to",
                        "show them your skills~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 87 && ctx.call(Function::CountItem, vec![Val::from(7237)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Association to",
                        "an ^FF0000Alchemist^000000 or ^FF0000Biochemist^000000.",
                        "It won't do any good in the",
                        "hands of anybody else."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 87 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hm. Don't you have",
                        "anything to do? I suppose",
                        "it's better to be bored",
                        "than to be overwhelmed.",
                        "Oh well, you'll figure",
                        "something out."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 86 && ctx.call(Function::CountItem, vec![Val::from(7238)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Coordination",
                        "to a ^FF0000Hunter^000000 or ^FF0000Sniper^000000.",
                        "It won't do any good in the",
                        "hands of anybody else."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 86 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Good work. Please",
                        "go ahead and take a rest,",
                        "and then come back to me",
                        "later. I'll have something",
                        "to give you by then."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 85 && ctx.call(Function::CountItem, vec![Val::from(7239)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Advance",
                        "to your ^FF0000guild master^000000.",
                        "It won't do any good in the",
                        "hands of anybody else."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 85 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Your falcon's been",
                        "a great help in the",
                        "guild. Hm? You already",
                        "miss it? Hahahaha!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Hm? What brings you",
                    "here? I don't think we",
                    "have any business to",
                    "conduct. Am I mistaken?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx
            .call(Function::GetGuildInfo, vec![l_gid.clone(), Val::from(2)])?
            .loosely_equals(&Val::from(1))
        {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Hm? What brings you",
                    "here? Feel free to take",
                    "your time and look around",
                    "if that's what pleases you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "There isn't much to see",
                    "around here, but you're",
                    "welcome to stay and look",
                    "around here if you wish."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn relaydummy2(ctx: &Ctx) -> Script {
    relaydummy2_body(ctx, Vec::new()).map(|_| ())
}

fn relaydummy3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gid = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_name_s = Val::from("");
    let mut l_time = Val::from(0);
    let mut l_x = Val::from(0);
    l_name_s = ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?;
    let position = ctx
        .call(Function::GetMapXy, vec![ctx.constant("BL_NPC")?])?
        .into_array()
        .ok_or_else(|| Stop::Error("Invalid position".into()))?;
    l_m_s = position[0].clone();
    l_x = position[1].clone();
    l_x = position[2].clone();
    l_gid = ctx.call(Function::GetCastleData, vec![l_m_s.clone(), Val::from(1)])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFWait a minute! You're",
            "carrying too many items",
            "right now: store some of",
            "your extra things in Kafra",
            "Storage, and then come back.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if l_gid.clone() == 0 {
        ctx.lines(args![
            "^3355FFThis exhausted man",
            "notices you staring at",
            "him, but chooses to leave",
            "you alone. There's no one",
            "else in this stronghold so",
            "there's no reason to be here.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(2)])?
        .loosely_equals(&l_gid.clone())
    {
        if ctx
            .call(
                Function::GetGuildInfo,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(2)],
            )?
            .loosely_equals(&Val::from(1))
        {
            if ctx.call(Function::CountItem, vec![Val::from(7240)])?.number()? > 0 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hm? What are you doing?",
                        "You're not supposed to",
                        "be the one holding onto",
                        "this soul. A Sage or a",
                        "Professor was supposed",
                        "to come here with it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Make sure you give",
                        "that soul you're holding",
                        "to a ^FF0000Sage^000000 or ^FF0000Professor^000000, and",
                        "then have him bring it to me."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Our business together",
                        "has concluded. If you're",
                        "not busy, why don't you try",
                        "helping out your guild? Yes,",
                        "I'm sure they'd appreciate it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?)
                && ctx.call(Function::CountItem, vec![Val::from(7240)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you've brought",
                        "the ^42426FSpirit of Trust^000000. Now, the",
                        "first thing I want you to do is",
                        "to build the trust between you",
                        "and your guild members.",
                        "Spend time with them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I want you to build",
                        "up some friendly bonds",
                        "within your guild before",
                        "coming back to me. Your",
                        "guild must have solidarity",
                        "in order to be successful."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7240), Val::from(1)])?;
                ctx.var("guildrelay_q").set(Val::from(9))?;
                ctx.var("guildtime").set(runtime::atoi(
                    &ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?,
                ))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_time = runtime::atoi(&ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?);
            if ((ctx.var("guildtime").get()?.number()? > 2259 && ctx.var("guildrelay_q").get()? == 9)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?))
            {
                if (l_time.clone().number()? > 129 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "So did you spend some",
                            "quality time with your",
                            "guild members, and get",
                            "a chance to really learn",
                            "who they are? You must",
                            "love your comrades."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Now you know how",
                            "important bonding truly",
                            "is. Please give this spirit",
                            "to a ^42426FBard^000000, ^42426FMinstrel^000000, ^42426FDancer^000000,",
                            "or ^42426FGypsy^000000. Good luck to you."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(81))?;
                    ctx.call(Function::GetItem, vec![Val::from(7241), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hm? Back so soon?",
                            "That hasn't been enough",
                            "time for you to really bond",
                            "with your guild members.",
                            "Go back, ask them about their",
                            "dreams, passions, and goals!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ((ctx.var("guildtime").get()?.number()? > 2159 && ctx.var("guildrelay_q").get()? == 9)
                    && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?))
                {
                    if (l_time.clone().number()? > 65 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "So did you spend some",
                                "quality time with your",
                                "guild members, and get",
                                "a chance to really learn",
                                "who they are? You must",
                                "love your comrades."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Now you know how",
                                "important bonding truly",
                                "is. Please give this spirit",
                                "to a ^42426FBard^000000, ^42426FMinstrel^000000, ^42426FDancer^000000,",
                                "or ^42426FGypsy^000000. Good luck to you."
                            ],
                        )?;
                        ctx.var("guildrelay_q").set(Val::from(71))?;
                        ctx.call(Function::GetItem, vec![Val::from(7241), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Hm? Back so soon?",
                                "That hasn't been enough",
                                "time for you to really bond",
                                "with your guild members.",
                                "Go back, ask them about their",
                                "dreams, passions, and goals!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ((ctx.var("guildtime").get()?.number()? > 2059 && ctx.var("guildrelay_q").get()? == 9)
                        && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?))
                    {
                        if (l_time.clone().number()? > 1 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "So did you spend some",
                                    "quality time with your",
                                    "guild members, and get",
                                    "a chance to really learn",
                                    "who they are? You must",
                                    "love your comrades."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Now you know how",
                                    "important bonding truly",
                                    "is. Please give this spirit",
                                    "to a ^42426FBard^000000, ^42426FMinstrel^000000, ^42426FDancer^000000,",
                                    "or ^42426FGypsy^000000. Good luck to you."
                                ],
                            )?;
                            ctx.var("guildrelay_q").set(Val::from(71))?;
                            ctx.call(Function::GetItem, vec![Val::from(7241), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Hm? Back so soon?",
                                    "That hasn't been enough",
                                    "time for you to really bond",
                                    "with your guild members.",
                                    "Go back, ask them about their",
                                    "dreams, passions, and goals!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if (((l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 192 && ctx.var("guildrelay_q").get()? == 9)
                            && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?))
                        {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "So did you spend some",
                                    "quality time with your",
                                    "guild members, and get",
                                    "a chance to really learn",
                                    "who they are? You must",
                                    "love your comrades."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "You have done",
                                    "what you had to do for now.",
                                    "Please give it to a ^42426FBard^000000 or ^42426FClown^000000",
                                    "You can also give it to ^42426FDancer^000000 or ^42426FGypsy^000000",
                                    "Good luck."
                                ],
                            )?;
                            ctx.var("guildrelay_q").set(Val::from(71))?;
                            ctx.call(Function::GetItem, vec![Val::from(7241), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?)
                            && ctx.var("guildrelay_q").get()? == 9)
                        {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Hm? Back so soon?",
                                    "That hasn't been enough",
                                    "time for you to really bond",
                                    "with your guild members.",
                                    "Go back, ask them about their",
                                    "dreams, passions, and goals!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
            if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?))
                && ctx.call(Function::CountItem, vec![Val::from(7241)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "For the sake of promoting",
                        "teamwork amongst your guild,",
                        "my test will be for you to",
                        "gather specific items for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "This is a difficult task",
                        "to complete alone, but it",
                        "will be much easier if you",
                        "and your guild cooperate to",
                        "get all the items. Now listen,",
                        "this is what I want you to get."
                    ],
                )?;
                ctx.next()?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Burning Hearts^000000,",
                            "^8C171730 Wolf Claws^000000, and",
                            "^8C171730 Leopard Claws^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7241), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(10))?;
                } else if subject1 == 2 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Soft Blades of Grass^000000,",
                            "^8C171730 Wooden Hearts^000000, and",
                            "^8C171730 Poisonous Toad Skins^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7241), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(11))?;
                } else if subject1 == 3 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Antelope Horns^000000,",
                            "^8C171730 Honey Pots^000000, and",
                            "^8C171730 Porcupine Quills^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7241), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(80))?;
                }
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You should be bonding",
                        "with your guild while you",
                        "gather those items I asked",
                        "you to bring. I know enough",
                        "time hasn't passed for your",
                        "guild to work together on this."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(7097)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7172)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(920)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 10)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Oh, perfect! You brought",
                        "all the items. Well then,",
                        "I guess you're ready to",
                        "take this spirit now. Please",
                        "give it to an ^42426FAssassin^000000 or an",
                        "^42426FAssassin Cross^000000. Thank you."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7097), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7172), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(920), Val::from(30)])?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?) {
                    ctx.var("guildrelay_q").set(Val::from(72))?;
                } else {
                    ctx.var("guildrelay_q").set(Val::from(72))?;
                }
                ctx.call(Function::GetItem, vec![Val::from(7242), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if (((ctx.call(Function::CountItem, vec![Val::from(7189)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7194)])?.number()? > 29)
                    && ctx.call(Function::CountItem, vec![Val::from(7155)])?.number()? > 29)
                    && ctx.var("guildrelay_q").get()? == 11)
                {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Oh, perfect! You brought",
                            "all the items. Well then,",
                            "I guess you're ready to",
                            "take this spirit now. Please",
                            "give it to an ^42426FAssassin^000000 or an",
                            "^42426FAssassin Cross^000000. Thank you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7189), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7194), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7155), Val::from(30)])?;
                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?) {
                        ctx.var("guildrelay_q").set(Val::from(72))?;
                    } else {
                        ctx.var("guildrelay_q").set(Val::from(72))?;
                    }
                    ctx.call(Function::GetItem, vec![Val::from(7242), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (((ctx.call(Function::CountItem, vec![Val::from(7106)])?.number()? > 29
                    && ctx.call(Function::CountItem, vec![Val::from(7121)])?.number()? > 29)
                    && ctx.call(Function::CountItem, vec![Val::from(1027)])?.number()? > 29)
                    && ctx.var("guildrelay_q").get()? == 80)
                {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Oh, perfect! You brought",
                            "all the items. Well then,",
                            "I guess you're ready to",
                            "take this spirit now. Please",
                            "give it to an ^42426FAssassin^000000 or an",
                            "^42426FAssassin Cross^000000. Thank you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7106), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7121), Val::from(30)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1027), Val::from(30)])?;
                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?) {
                        ctx.var("guildrelay_q").set(Val::from(72))?;
                    } else {
                        ctx.var("guildrelay_q").set(Val::from(72))?;
                    }
                    ctx.call(Function::GetItem, vec![Val::from(7242), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("guildrelay_q").get()? == 10 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hm? You still haven't",
                            "gathered all the items",
                            "with your guild yet?",
                            "Let me remind you what",
                            "you need to bring me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Burning Hearts^000000,",
                            "^8C171730 Wolf Claws^000000, and",
                            "^8C171730 Leopard Claws^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("guildrelay_q").get()? == 11 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hm? You still haven't",
                            "gathered all the items",
                            "with your guild yet?",
                            "Let me remind you what",
                            "you need to bring me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Soft Blades of Grass^000000,",
                            "^8C171730 Wooden Hearts^000000, and",
                            "^8C171730 Poisonous Toad Skins^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("guildrelay_q").get()? == 80 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hm? You still haven't",
                            "gathered all the items",
                            "with your guild yet?",
                            "Let me remind you what",
                            "you need to bring me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "^8C171730 Antelope Horns^000000,",
                            "^8C171730 Honey Pots^000000, and",
                            "^8C171730 Porcupine Quills^000000.",
                            "You might want to write",
                            "these down so you don't",
                            "forget. Good luck to you."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                && ctx.call(Function::CountItem, vec![Val::from(7242)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You brought the",
                        "Spirit of Combination?",
                        "Make sure that you give",
                        "that to a ^2F4F2FWizard^000000 or",
                        "a ^2F4F2FHigh Wizard^000000."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7242), Val::from(1)])?;
                ctx.var("guildrelay_q").set(Val::from(74))?;
                ctx.call(Function::GetItem, vec![Val::from(7244), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?)
                && ctx.call(Function::CountItem, vec![Val::from(7244)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "For the sake of your",
                        "guild, you must become",
                        "stronger. Sometimes",
                        "your spells will make the",
                        "difference between victory",
                        "and defeat. Remember that."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        ((Val::from("Hey,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                        "if you really want to become",
                        "strong enough to protect your",
                        "guild, then you must level up.",
                        "Come back to me once you've",
                        "raised your Base Level."
                    ],
                )?;
                ctx.next()?;
                if (ctx.var("BaseLevel").get()?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? < 61) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "You will be ready",
                            "for your task after you",
                            "gain ^FF00003 Base Levels^000000.",
                            "Don't despair: I know",
                            "you'll be able to reach",
                            "this goal. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7244), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(12))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 76) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "You will be ready",
                            "for your task after you",
                            "gain ^FF00002 Base Levels^000000.",
                            "Don't despair: I know",
                            "you'll be able to reach",
                            "this goal. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7244), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(13))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 97) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "You will be ready",
                            "for your task after you",
                            "gain ^FF00001 Base Level^000000.",
                            "Don't despair: I know",
                            "you'll be able to reach",
                            "this goal. Good luck to you."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7244), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(14))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 96 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hm. You're much stronger",
                            "that I expected. There's no",
                            "need for me to encourage",
                            "you to develop your strength.",
                            "I admit that you are strong."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Acknowledge the extent and",
                            "limits of your strength, but",
                            "never allow conceit to pollute",
                            "your heart. Pride will always",
                            "shackle your power if you",
                            "let it. Remember humility."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "You qualified enough",
                            "for me to trust you.",
                            "Please give this spirit",
                            "to your ^FF0000Guild Master^000000.",
                            "You're done for now."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7244), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(75))?;
                    ctx.call(Function::GetItem, vec![Val::from(7245), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(2)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 12)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you've",
                        "completed the task",
                        "I have given you. It may",
                        "have been difficult, but",
                        "you'll see that I had your",
                        "guild's best interests in mind."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You've earned my trust.",
                        "Please give this spirit",
                        "to your ^FF0000Guild Master^000000.",
                        "You've done well."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(75))?;
                ctx.call(Function::GetItem, vec![Val::from(7245), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(1)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 13)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you've",
                        "completed the task",
                        "I have given you. It may",
                        "have been difficult, but",
                        "you'll see that I had your",
                        "guild's best interests in mind."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You've earned my trust.",
                        "Please give this spirit",
                        "to your ^FF0000Guild Master^000000.",
                        "You've done well."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(75))?;
                ctx.call(Function::GetItem, vec![Val::from(7245), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (runtime::op(&ctx.var("BaseLevel").get()?, "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 14)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you've",
                        "completed the task",
                        "I have given you. It may",
                        "have been difficult, but",
                        "you'll see that I had your",
                        "guild's best interests in mind."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You've earned my trust.",
                        "Please give this spirit",
                        "to your ^FF0000Guild Master^000000.",
                        "You've done well."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(75))?;
                ctx.call(Function::GetItem, vec![Val::from(7245), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 12 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?)) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You're not strong enough",
                        "yet for your guild to fully",
                        "rely on you in a crisis.",
                        "You must level up!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("guildrelay_q").get()? == 13 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?)) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You're not strong enough",
                        "yet for your guild to fully",
                        "rely on you in a crisis.",
                        "You must level up!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("guildrelay_q").get()? == 14 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?)) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hmm... I don't think you've",
                        "spent enough time leveling",
                        "up yet. Keep working on it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if ctx
            .call(Function::GetGuildInfo, vec![l_gid.clone(), Val::from(2)])?
            .loosely_equals(&Val::from(1))
        {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Hm. You're the master",
                    "of another guild, aren't",
                    "you? I have no loyalty",
                    "towards you. Please...",
                    "Leave this place."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Hm? You have no reason",
                    "to linger in this stronghold.",
                    "Please leave this place now."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("guildrelay_q").get()? == 71 && ctx.call(Function::CountItem, vec![Val::from(7241)])?.number()? > 0) {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "There's no need for you",
                "to hold onto that Spirit",
                "of Union. Please give it to",
                "a Bard, Minstrel, Dancer",
                "or Gypsy in your guild."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("guildrelay_q").get()? == 71 {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "When you improve yourself,",
                "you'll also be strengthening",
                "your guild. Always devote some",
                "time for yourself and for your",
                "team. You cannot have one",
                "without the other."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I suppose you really",
                "don't have much use",
                "for me anymore... But I'll",
                "always offer my support."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("guildrelay_q").get()? == 72 && ctx.call(Function::CountItem, vec![Val::from(7242)])?.number()? > 0) {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "There's no need for you",
                "to hold onto that Spirit of",
                "Combination. Please give",
                "it to an Assassin or Assassin",
                "Cross in your guild."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("guildrelay_q").get()? == 72 {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "Your songs bolster your",
                "guild's morale, and will",
                "help them stand and fight,",
                "no matter how desperate",
                "the situation may seem.",
                "Your voice can make miracles."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I suppose you really",
                "don't have much use",
                "for me anymore... But I'll",
                "always offer my support."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("guildrelay_q").get()? == 73 && ctx.call(Function::CountItem, vec![Val::from(7242)])?.number()? > 0) {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "There's no need for you",
                "to hold onto that Spirit of",
                "Combination. Please give",
                "it to an Assassin or Assassin",
                "Cross in your guild."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("guildrelay_q").get()? == 73 {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "Your dances bolster your",
                "guild's morale, and will",
                "help them stand and fight,",
                "no matter how desperate",
                "the situation may seem.",
                "Your voice can make miracles."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I suppose you really",
                "don't have much use",
                "for me anymore... But I'll",
                "always offer my support."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("guildrelay_q").get()? == 74 && ctx.call(Function::CountItem, vec![Val::from(7244)])?.number()? > 0) {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "There's no need for you",
                "to hold onto that Spirit",
                "of Solidarity. Please give",
                "it to a Wizard or High",
                "Wizard in your guild."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("guildrelay_q").get()? == 74 {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I suppose you really",
                "don't have much use",
                "for me anymore... But I'll",
                "always offer my support."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "Know who your enemies are.",
                "If anybody opposes your guild,",
                "you must crush them without",
                "any hesitation. Your justice",
                "must be meted swiftly!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("guildrelay_q").get()? == 75 && ctx.call(Function::CountItem, vec![Val::from(7245)])?.number()? > 0) {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "There's no need for you",
                "to hold onto that Spirit of",
                "Friendship. Please give",
                "it to your Guild Master."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("guildrelay_q").get()? == 75 {
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I can tell that you're",
                "always trying to help all",
                "the members of your guild.",
                "Your loyalty will bring them",
                "to your side in times of",
                "joy and of tribulation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            l_name_s.clone(),
            args![
                "I suppose you really",
                "don't have much use",
                "for me anymore... But I'll",
                "always offer my support."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        l_name_s.clone(),
        args![
            "We have nothing to",
            "do with each other.",
            "Leave me be, and",
            "just go on your way."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn relaydummy3(ctx: &Ctx) -> Script {
    relaydummy3_body(ctx, Vec::new()).map(|_| ())
}
