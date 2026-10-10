use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn gatan_payon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_relaytime = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(300)])? == 0 {
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
    if (((((ctx.call(Function::CountItem, vec![Val::from(7732)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7733)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7736)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7737)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7740)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7741)])?.number()? > 0)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Say, aren't you forgetting",
                "something? Try to remember...",
                "Earlier, you received some",
                "instructions, didn't you?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_relaytime = ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?;
    if ctx.var("party_relay").get()? == 28 {
        ctx.lines_as(
            "Gatan",
            args![
                "Say, I don't think it's",
                "your turn to meet up with",
                "me. I think you need to ask a",
                "Thief or Acolyte Class member",
                "of your group to bring the",
                "tenth ticket to Bafhail."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 27 {
        ctx.lines_as(
            "Gatan",
            args![
                "Ah, very nice! Please",
                "give this ticket to a Thief",
                "or Acolyte Class member",
                "of your group, and ask him",
                "to bring it to Bafhail. Ah, and",
                "here's a little reward for you~"
            ],
        )?;
        ctx.var("party_relay").set(Val::from(28))?;
        ctx.call(Function::GetItem, vec![Val::from(7739), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.next()?;
        ctx.lines_as("Gatan", args!["Alright, get that done.", "I'll see you around."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("party_relay").get()? == 26
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 14)
        && l_relaytime.clone().number()? < 17)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.var("party_relay").get()? == 26
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 18)
        && l_relaytime.clone().number()? < 21)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 26 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 3 more Base Levels.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I can't tell you exactly",
                "when I work, but my work",
                "hours are in the afternoon...",
                "Pacific Standard Time, anyway.",
                "I might not be here if you",
                "come here too late, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("party_relay").get()? == 25
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 14)
        && l_relaytime.clone().number()? < 17)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.var("party_relay").get()? == 25
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 18)
        && l_relaytime.clone().number()? < 21)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 25 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 2 more Base Levels.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I can't tell you exactly",
                "when I work, but my work",
                "hours are in the afternoon...",
                "Pacific Standard Time, anyway.",
                "I might not be here if you",
                "come here too late, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("party_relay").get()? == 24
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && l_relaytime.clone().number()? >= 14)
        && l_relaytime.clone().number()? < 17)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained a",
                "level, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.var("party_relay").get()? == 24
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && l_relaytime.clone().number()? >= 18)
        && l_relaytime.clone().number()? < 21)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained a",
                "level, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(27))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 24 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 1 more Base Level.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I can't tell you exactly",
                "when I work, but my work",
                "hours are in the afternoon...",
                "Pacific Standard Time, anyway.",
                "I might not be here if you",
                "come here too late, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7738)])?.number()? > 0
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?)))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Oh, um... Is that...?",
                "That's a ticket from",
                "Ledrion, huh? (^666666Nuts! I've",
                "got to work now?^000000) It's nice",
                "to meet you. I'm Gatan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Well, now I've got a little",
                "test for you. You ready?",
                "Your objective is to gain",
                "more levels! Let's see...",
                "What would be fair?"
            ],
        )?;
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? > 94 {
            ctx.lines_as(
                "Gatan",
                args![
                    "Actually, you're already",
                    "pretty strong. I don't feel",
                    "like doing much work either,",
                    "so we'll just say that you",
                    "finished my test, alright?",
                    "Just don't tell anyone!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
            ctx.var("party_relay").set(Val::from(27))?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Hold on a second...",
                    "I'm supposed to tell",
                    "you something... Give",
                    "me a minute, will you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("BaseLevel").get()?.number()? > 89 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "You're pretty strong,",
                        "but it wouldn't kill",
                        "you to gain 1 more",
                        "Base Level. Go ahead",
                        "and do that, alright?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(24))?;
                ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "I'll go ahead and take",
                        "your ticket now. Come",
                        "back after you finish what",
                        "I've asked, and then we can",
                        "move on to the next part."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 79 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(24))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(25))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(25))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(26))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(26))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        if (ctx.call(Function::CountItem, vec![Val::from(7738)])?.number()? > 0
            && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
                || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?)))
        {
            ctx.lines_as(
                "Gatan",
                args![
                    "Oh, um... Is that...?",
                    "That's a ticket from",
                    "Ledrion, huh? (^666666Nuts! I've",
                    "got to work now?^000000) It's nice",
                    "to meet you. I'm Gatan."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Well, now I've got a little",
                    "test for you. You ready?",
                    "Your objective is to gain",
                    "more levels! Let's see...",
                    "What would be fair?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("BaseLevel").get()?.number()? > 94 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Actually, you're already",
                        "pretty strong. I don't feel",
                        "like doing much work either,",
                        "so we'll just say that you",
                        "finished my test, alright?",
                        "Just don't tell anyone!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(27))?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Hold on a second...",
                        "I'm supposed to tell",
                        "you something... Give",
                        "me a minute, will you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 89 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(24))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("BaseLevel").get()?.number()? > 79 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 1 more",
                                "Base Level. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(24))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(25))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(25))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(26))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7738), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(26))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7738)])?.number()? > 0
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?)))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Oh, um... Is that...?",
                "That's a ticket from",
                "Ledrion, huh? It's nice",
                "to meet you. I'm Gatan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I'm not on shift right",
                "now, though: even I need",
                "to take some off time to",
                "avoid getting swamped with",
                "work. Come back to me during",
                "my work hours, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I can't tell you exactly",
                "when I'm working (^666666well,",
                "I just want to avoid working",
                "altogether to be honest^000000) but",
                "come back in the afternoon,",
                "Pacific Standard Time, that is."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 17 {
        ctx.lines_as(
            "Gatan",
            args![
                "Say, it's not time for",
                "you to meet up with me",
                "just yet. I think you need",
                "to ask an Acolyte Class",
                "member in your group to bring",
                "your sixth ticket to Bafhail."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 16 {
        ctx.lines_as(
            "Gatan",
            args![
                "Ah, very nice! Please",
                "give this ticket to an",
                "Acolyte Class member",
                "of your group, and ask him",
                "to bring it to Bafhail. Ah, and",
                "here's a little reward for you~"
            ],
        )?;
        ctx.var("party_relay").set(Val::from(17))?;
        ctx.call(Function::GetItem, vec![Val::from(7735), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.next()?;
        ctx.lines_as("Gatan", args!["Alright, get that done.", "I'll see you around."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("party_relay").get()? == 15
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 8)
        && l_relaytime.clone().number()? < 11)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.var("party_relay").get()? == 15
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 21)
        && l_relaytime.clone().number()? < 1)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 15 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 3 more Base Levels.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Now I work three hours",
                "just before I have lunch,",
                "and three hours around",
                "midnight. Go ahead and",
                "look me up around those",
                "times. A bit confusing, I know~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 14
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 8)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 14
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && l_relaytime.clone().number()? >= 21)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 14 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 2 more Base Levels.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Now I work three hours",
                "just before I have lunch,",
                "and three hours around",
                "midnight. Go ahead and",
                "look me up around those",
                "times. A bit confusing, I know~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.var("party_relay").get()? == 13
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && l_relaytime.clone().number()? >= 8)
        && l_relaytime.clone().number()? < 11)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 13
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && l_relaytime.clone().number()? >= 21)
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Great, you gained some",
                "levels, just like I asked.",
                "Let me find your next--I swore",
                "I left it around somewhere--",
                "and I'll give you your next set",
                "of instructions. Hang on..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(16))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 13 {
        ctx.lines_as(
            "Gatan",
            args![
                "Your mission is to",
                "gain 1 more Base Level.",
                "Come see me during my",
                "working hours once you",
                "accomplish that, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Now I work three hours",
                "just before I have lunch,",
                "and three hours around",
                "midnight. Go ahead and",
                "look me up around those",
                "times. A bit confusing, I know~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7734)])?.number()? > 0)
        && l_relaytime.clone().number()? >= 8)
        && l_relaytime.clone().number()? < 11)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Oh, um... Is that...?",
                "That's a ticket from",
                "Ledrion, huh? (^666666Nuts! I've",
                "got to work now?^000000) It's nice",
                "to meet you. I'm Gatan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Well, now I've got a little",
                "test for you. You ready?",
                "Your objective is to gain",
                "more levels! Let's see...",
                "What would be fair?"
            ],
        )?;
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? > 94 {
            ctx.lines_as(
                "Gatan",
                args![
                    "Actually, you're already",
                    "pretty strong. I don't feel",
                    "like doing much work either,",
                    "so we'll just say that you",
                    "finished my test, alright?",
                    "Just don't tell anyone!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
            ctx.var("party_relay").set(Val::from(16))?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Hold on a second...",
                    "I'm supposed to tell",
                    "you something... Give",
                    "me a minute, will you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("BaseLevel").get()?.number()? > 89 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "You're pretty strong,",
                        "but it wouldn't kill",
                        "you to gain 1 more",
                        "Base Level. Go ahead",
                        "and do that, alright?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(13))?;
                ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "I'll go ahead and take",
                        "your ticket now. Come",
                        "back after you finish what",
                        "I've asked, and then we can",
                        "move on to the next part."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 79 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(13))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(14))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(14))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(15))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(15))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        if (((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7734)])?.number()? > 0)
            && l_relaytime.clone().number()? >= 21)
            && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?))
        {
            ctx.lines_as(
                "Gatan",
                args![
                    "Oh, um... Is that...?",
                    "That's a ticket from",
                    "Ledrion, huh? (^666666Nuts! I've",
                    "got to work now?^000000) It's nice",
                    "to meet you. I'm Gatan."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Well, now I've got a little",
                    "test for you. You ready?",
                    "Your objective is to gain",
                    "more levels! Let's see...",
                    "What would be fair?"
                ],
            )?;
            ctx.next()?;
            if ctx.var("BaseLevel").get()?.number()? > 94 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Actually, you're already",
                        "pretty strong. I don't feel",
                        "like doing much work either,",
                        "so we'll just say that you",
                        "finished my test, alright?",
                        "Just don't tell anyone!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(16))?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Hold on a second...",
                        "I'm supposed to tell",
                        "you something... Give",
                        "me a minute, will you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 89 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(13))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("BaseLevel").get()?.number()? > 79 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 1 more",
                                "Base Level. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(13))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(14))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(14))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(15))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7734), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(15))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    if ((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7734)])?.number()? > 0)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Oh, um... Is that...?",
                "That's a ticket from",
                "Ledrion, huh? It's nice",
                "to meet you. I'm Gatan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I'm not on shift right",
                "now, though: even I need",
                "to take some off time to",
                "avoid getting swamped with",
                "work. Come back to me during",
                "my work hours, alright?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Now I work three hours",
                "just before I have lunch,",
                "and three hours around",
                "midnight. Go ahead and",
                "look me up around those",
                "times. A bit confusing, I know~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 6 {
        ctx.lines_as(
            "Gatan",
            args![
                "Did you give that ticket",
                "to the Thief Class member",
                "of your group? ^666666*Yawn*^000000 If you",
                "wanna finish all of these",
                "challenges, then that's",
                "what you gotta do~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 5 {
        ctx.lines_as(
            "Gatan",
            args![
                "Ah, very nice!",
                "Now please give this",
                "ticket to the Thief Class",
                "member of the your group,",
                "and ask him to deliver",
                "it to Bafhail. Easy, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args!["Here you are...", "Just a little", "something for", "your troubles~"],
        )?;
        ctx.var("party_relay").set(Val::from(6))?;
        ctx.call(Function::GetItem, vec![Val::from(7731), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Don't forget to make",
                "sure that Bafhail gets",
                "that ticket from a Thief",
                "Class character in your",
                "group. I'll see you around."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 4
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && (l_relaytime.clone().number()? >= 11 && l_relaytime.clone().number()? < 14))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 4
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(2)),
        )?
        .is_true())
        && (l_relaytime.clone().number()? >= 0 && l_relaytime.clone().number()? < 3))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 4 {
        ctx.lines_as(
            "Gatan",
            args![
                "You didn't forget that",
                "you had to gain 3 more",
                "Base Levels, did you?",
                "Oh, and come back during",
                "my regular working hours.",
                "Don't bother me when I'm off~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I'll tell you when I work",
                "one more time: come from",
                "11 AM to 2 PM, or 12 AM to",
                "3 AM, Pacific Standard Time.",
                "Good luck to you, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 3
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && (l_relaytime.clone().number()? >= 11 && l_relaytime.clone().number()? < 14))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 3
        && runtime::op(
            &ctx.var("BaseLevel").get()?,
            ">",
            &(ctx.var("party_relay_lv").get()? + Val::from(1)),
        )?
        .is_true())
        && (l_relaytime.clone().number()? >= 0 && l_relaytime.clone().number()? < 3))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 3 {
        ctx.lines_as(
            "Gatan",
            args![
                "You didn't forget that",
                "you had to gain 2 more",
                "Base Levels, did you?",
                "Oh, and come back during",
                "my regular working hours.",
                "Don't bother me when I'm off~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I'll tell you when I work",
                "one more time: come from",
                "11 AM to 2 PM, or 12 AM to",
                "3 AM, Pacific Standard Time.",
                "Good luck to you, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 99
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && (l_relaytime.clone().number()? >= 11 && l_relaytime.clone().number()? < 14))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 99
        && runtime::op(&ctx.var("BaseLevel").get()?, ">", &ctx.var("party_relay_lv").get()?)?.is_true())
        && (l_relaytime.clone().number()? >= 0 && l_relaytime.clone().number()? < 3))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Nice work. I guess",
                "that Ledrion really knows",
                "good adventurers when he",
                "sees them. Now, you mind",
                "waiting a bit? Um, there's",
                "something I have to give you..."
            ],
        )?;
        ctx.var("party_relay").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 99 {
        ctx.lines_as(
            "Gatan",
            args![
                "You didn't forget that",
                "you had to gain 1 more",
                "Base Level, did you?",
                "Oh, and come back during",
                "my regular working hours.",
                "Don't bother me when I'm off~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "I'll tell you when I work",
                "one more time: come from",
                "11 AM to 2 PM, or 12 AM to",
                "3 AM, Pacific Standard Time.",
                "Good luck to you, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7730)])?.number()? > 0)
        && l_relaytime.clone().number()? >= 11)
        && l_relaytime.clone().number()? < 14)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?))
    {
        ctx.lines_as(
            "Gatan",
            args![
                "Oh, um... Is that...?",
                "That's a ticket from",
                "Ledrion, huh? (^666666Nuts! I've",
                "got to work now?^000000) It's nice",
                "to meet you. I'm Gatan."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gatan",
            args![
                "Ledrion wanted me to give",
                "a little test if any Swordman",
                "Class guys brought me a ticket,",
                "and were working with a group.",
                "Ready? Increasing your Base",
                "Level will be my test for you."
            ],
        )?;
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? > 94 {
            ctx.lines_as(
                "Gatan",
                args![
                    "Actually, you're already",
                    "pretty strong. I don't feel",
                    "like doing much work either,",
                    "so we'll just say that you",
                    "finished my test, alright?",
                    "Just don't tell anyone!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
            ctx.var("party_relay").set(Val::from(5))?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Hold on a second...",
                    "I'm supposed to tell",
                    "you something... Give",
                    "me a minute, will you?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("BaseLevel").get()?.number()? > 89 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "You're pretty strong,",
                        "but it wouldn't kill",
                        "you to gain 1 more",
                        "Base Level. Go ahead",
                        "and do that, alright?"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(99))?;
                ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "I'll go ahead and take",
                        "your ticket now. Come",
                        "back after you finish what",
                        "I've asked, and then we can",
                        "move on to the next part."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 79 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(99))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(3))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 2 more",
                            "Base Levels. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(3))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(4))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You know, you'd probably",
                            "benefit from being just",
                            "a little bit stronger. Now",
                            "go out and gain 3 Base",
                            "Levels for me, okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(4))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        if ((((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7730)])?.number()? > 0)
            && l_relaytime.clone().number()? >= 0)
            && l_relaytime.clone().number()? < 3)
            && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?))
        {
            ctx.lines_as(
                "Gatan",
                args![
                    "Oh, um... Is that...?",
                    "That's a ticket from",
                    "Ledrion, huh? (^666666Nuts! I've",
                    "got to work now?^000000) It's nice",
                    "to meet you. I'm Gatan."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Ledrion wanted me to give",
                    "a little test if any Swordman",
                    "Class guys brought me a ticket,",
                    "and were working with a group.",
                    "Ready? Increasing your Base",
                    "Level will be my test for you."
                ],
            )?;
            ctx.next()?;
            if ctx.var("BaseLevel").get()?.number()? > 94 {
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Actually, you're already",
                        "pretty strong. I don't feel",
                        "like doing much work either,",
                        "so we'll just say that you",
                        "finished my test, alright?",
                        "Just don't tell anyone!"
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                ctx.var("party_relay").set(Val::from(5))?;
                ctx.lines_as(
                    "Gatan",
                    args![
                        "Hold on a second...",
                        "I'm supposed to tell",
                        "you something... Give",
                        "me a minute, will you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("BaseLevel").get()?.number()? > 89 {
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "You're pretty strong,",
                            "but it wouldn't kill",
                            "you to gain 1 more",
                            "Base Level. Go ahead",
                            "and do that, alright?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                    ctx.var("party_relay").set(Val::from(99))?;
                    ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                    ctx.lines_as(
                        "Gatan",
                        args![
                            "I'll go ahead and take",
                            "your ticket now. Come",
                            "back after you finish what",
                            "I've asked, and then we can",
                            "move on to the next part."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("BaseLevel").get()?.number()? > 79 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 1 more",
                                "Base Level. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(99))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 69 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(3))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 59 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You're pretty strong,",
                                "but it wouldn't kill",
                                "you to gain 2 more",
                                "Base Levels. Go ahead",
                                "and do that, alright?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(3))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 49 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(4))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "I'll go ahead and take",
                                "your ticket now. Come",
                                "back after you finish what",
                                "I've asked, and then we can",
                                "move on to the next part."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? > 39 {
                        ctx.lines_as(
                            "Gatan",
                            args![
                                "You know, you'd probably",
                                "benefit from being just",
                                "a little bit stronger. Now",
                                "go out and gain 3 Base",
                                "Levels for me, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(7730), Val::from(1)])?;
                        ctx.var("party_relay").set(Val::from(4))?;
                        ctx.var("party_relay_lv").set(ctx.var("BaseLevel").get()?)?;
                        ctx.lines_as(
                            "Gatan",
                            args!["I'll keep your ticket.", "When you're finished, please come back."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        } else if ((ctx.var("BaseLevel").get()?.number()? > 39 && ctx.call(Function::CountItem, vec![Val::from(7730)])?.number()? > 0)
            && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?))
        {
            ctx.lines_as(
                "Gatan",
                args![
                    "Oh, um... Is that...?",
                    "That's a ticket from",
                    "Ledrion, huh? (^666666Nuts! I've",
                    "got to work now?^000000) It's nice",
                    "to meet you. I'm Gatan."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatan",
                args![
                    "Hate to tell you, but",
                    "I'm not on duty right now.",
                    "You should really come back",
                    "and talk to me during my",
                    "work hours, okay? I need",
                    "my rest from work, you know?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gatan",
                args![
                    "I'll tell you when I work,",
                    "just don't forget: come from",
                    "11 AM to 2 PM, or 12 AM to",
                    "3 AM, Pacific Standard Time.",
                    "Good luck to you, alright?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Gatan",
        args![
            "Hey, nice meeting you.",
            "I'm Gatan. I'm not up to",
            "much, just helping out",
            "a friend. He's loaded, but",
            "he's also a really good guy.",
            "Working me to the bone, though."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn gatan_payon(ctx: &Ctx) -> Script {
    gatan_payon_body(ctx, Vec::new()).map(|_| ())
}
