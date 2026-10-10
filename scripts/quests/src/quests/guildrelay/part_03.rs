use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guilddummy4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gid = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_name_s = Val::from("");
    let mut l_partymembercount = Val::from(0);
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
            if ctx.call(Function::CountItem, vec![Val::from(7246)])?.number()? > 0 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hm? That spirit that",
                        "you've brought with you...",
                        "I'm sorry, but it's useless",
                        "for you to carry it around."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give that to",
                        "a ^FF0000Rogue^000000 or ^FF0000Stalker^000000",
                        "in your guild, and then",
                        "ask him bring it to me."
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
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?)
                && ctx.call(Function::CountItem, vec![Val::from(7246)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "So you're the one that's",
                        "been entrusted with the",
                        "Spirit of Peace? Well then...",
                        "My task for you is to ^FF0000form"
                    ],
                )?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.lines(args!["a party with 6 members^000000.", "No more and no less."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "This is a strange test,",
                            "but if you can do this, it",
                            "will clearly demonstrate",
                            "to me that your people",
                            "skills are up to par."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7246), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(91))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.lines(args!["a party with 8 members^000000.", "No more and no less."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "This is a strange test,",
                            "but if you can do this, it",
                            "will clearly demonstrate",
                            "to me that your people",
                            "skills are up to par."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7246), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(92))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.lines(args!["a party with 10 members^000000.", "No more and no less."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "This is a strange test,",
                            "but if you can do this, it",
                            "will clearly demonstrate",
                            "to me that your people",
                            "skills are up to par."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7246), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(93))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
            l_partymembercount = ctx.var("$@partymembercount").get()?;
            if ctx.var("guildrelay_q").get()? == 91 {
                if l_partymembercount.clone() == 6 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I see that you've",
                            "formed a party with",
                            "the exact number of",
                            "people that I asked. Hmm.",
                            "I guess you can be trusted",
                            "to lead when you must."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Here. I want you to take",
                            "this spirit, and then give",
                            "it to a Priest or High",
                            "Priest in your guild."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(95))?;
                    ctx.call(Function::GetItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I thought I asked you to",
                            "^FF0000form a party of 6 members^000000?",
                            "No more and no less. Hmm.",
                            "Come back to me after you've",
                            "finished this simple task."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if ctx.var("guildrelay_q").get()? == 92 {
                if l_partymembercount.clone() == 8 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I see that you've",
                            "formed a party with",
                            "the exact number of",
                            "people that I asked. Hmm.",
                            "I guess you can be trusted",
                            "to lead when you must."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Here. I want you to take",
                            "this spirit, and then give",
                            "it to a Priest or High",
                            "Priest in your guild."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(95))?;
                    ctx.call(Function::GetItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I thought I asked you to",
                            "^FF0000form a party of 8 members^000000?",
                            "No more and no less. Hmm.",
                            "Come back to me after you've",
                            "finished this simple task."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if ctx.var("guildrelay_q").get()? == 93 {
                if l_partymembercount.clone() == 10 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I see that you've",
                            "formed a party with",
                            "the exact number of",
                            "people that I asked. Hmm.",
                            "I guess you can be trusted",
                            "to lead when you must."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Here. I want you to take",
                            "this spirit, and then give",
                            "it to a Priest or High",
                            "Priest in your guild."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(95))?;
                    ctx.call(Function::GetItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I thought I asked you to",
                            "^FF0000form a party of 10 members^000000?",
                            "No more and no less. Hmm.",
                            "Come back to me after you've",
                            "finished this simple task."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?)
                && ctx.call(Function::CountItem, vec![Val::from(7247)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Good work getting the",
                        "Spirit of Determination,",
                        "but I want you to work on",
                        "getting stronger. If you",
                        "intend to protect others,",
                        "you can't afford to lose."
                    ],
                )?;
                ctx.next()?;
                if (ctx.var("BaseLevel").get()?.number()? > 1 && ctx.var("BaseLevel").get()?.number()? < 61) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I want you to gain",
                            "^FF00003 Base Levels^000000 for the",
                            "sake of strengthening",
                            "yourself and your guild.",
                            "This is my test for you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Try not to worry",
                            "over this task so much.",
                            "I'm certain you can do",
                            "this. Come back to me",
                            "when you are ready."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(18))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 76) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I want you to gain",
                            "^FF00002 Base Levels^000000 for the",
                            "sake of strengthening",
                            "yourself and your guild.",
                            "This is my test for you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Try not to worry",
                            "over this task so much.",
                            "I'm certain you can do",
                            "this. Come back to me",
                            "when you are ready."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(19))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 97) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "I want you to gain",
                            "^FF00001 Base Level^000000 for the",
                            "sake of strengthening",
                            "yourself and your guild.",
                            "This is my test for you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Try not to worry",
                            "over this task so much.",
                            "I'm certain you can do",
                            "this. Come back to me",
                            "when you are ready."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(20))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseLevel").get()?.number()? > 96 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Hum, You are stronger than",
                            "I was expected.",
                            "You don't need any more quests",
                            "for becoming stronger.",
                            "But, do not be so proud of yourself."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Alright then...",
                            "I entrust you with",
                            "this spirit. Please be",
                            "sure to give it to a",
                            "^4D4DFFCrusader^000000 or ^4D4DFFPaladin^000000",
                            "in your guild."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7247), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(98))?;
                    ctx.call(Function::GetItem, vec![Val::from(7249), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(2)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 18)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I see that you've",
                        "become much stronger,",
                        "just as I asked. No wonder",
                        "your guild mates can rely",
                        "on you. Congratulations",
                        "on a job well done."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Alright then...",
                        "I entrust you with",
                        "this spirit. Please be",
                        "sure to give it to a",
                        "^4D4DFFCrusader^000000 or ^4D4DFFPaladin^000000",
                        "in your guild."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(98))?;
                ctx.call(Function::GetItem, vec![Val::from(7249), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (runtime::op(&(ctx.var("BaseLevel").get()? + Val::from(1)), "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 19)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I see that you've",
                        "become much stronger,",
                        "just as I asked. No wonder",
                        "your guild mates can rely",
                        "on you. Congratulations",
                        "on a job well done."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Alright then...",
                        "I entrust you with",
                        "this spirit. Please be",
                        "sure to give it to a",
                        "^4D4DFFCrusader^000000 or ^4D4DFFPaladin^000000",
                        "in your guild."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(98))?;
                ctx.call(Function::GetItem, vec![Val::from(7249), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (runtime::op(&ctx.var("BaseLevel").get()?, "<", &ctx.var("BaseLevel").get()?)?.is_true()
                && ctx.var("guildrelay_q").get()? == 20)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Huh. I see that you've",
                        "become much stronger,",
                        "just as I asked. No wonder",
                        "your guild mates can rely",
                        "on you. Congratulations",
                        "on a job well done."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Ok, take this and give to",
                        "Crusader or Paladin",
                        "who is in our guild.",
                        "Good luck!"
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(98))?;
                ctx.call(Function::GetItem, vec![Val::from(7249), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?)
                && ctx.call(Function::CountItem, vec![Val::from(7249)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you possess",
                        "the Spirit of Service. Hmm.",
                        "Good job. Now, your next",
                        "task will test your patience.",
                        "All you have to do is ^FF0000wait^000000.",
                        "Wait until the time is right."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I want you to spend this",
                        "time to look into yourself,",
                        "and examine your relationship",
                        "with your guild members.",
                        "When you feel that the time",
                        "is right, come talk to me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I'm not going to tell you",
                        "when you should come back.",
                        "I'll merely confirm whether",
                        "you've come early or on time.",
                        "You'll need to really listen",
                        "to your feelings this time..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7249), Val::from(1)])?;
                ctx.var("guildrelay_q").set(Val::from(21))?;
                ctx.var("guildtime").set(runtime::atoi(
                    &ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?,
                ))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_time = runtime::atoi(&ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?);
            if ((ctx.var("guildtime").get()?.number()? > 2259 && ctx.var("guildrelay_q").get()? == 21)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
            {
                if (l_time.clone().number()? > 129 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Yes. You've come at just",
                            "the right time. Remember",
                            "that feeling in your heart,",
                            "and trust it when it comes",
                            "again. Now give this spirit to",
                            "a ^FF0000Monk^000000 or ^FF0000Champion^000000 for me..."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(96))?;
                    ctx.call(Function::GetItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Too early.",
                            "Still too early.",
                            "Have you been listening",
                            "to your heart? Listen harder."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if ((ctx.var("guildtime").get()?.number()? > 2159 && ctx.var("guildrelay_q").get()? == 21)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
            {
                if (l_time.clone().number()? > 65 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Yes. You've come at just",
                            "the right time. Remember",
                            "that feeling in your heart,",
                            "and trust it when it comes",
                            "again. Now give this spirit to",
                            "a ^FF0000Monk^000000 or ^FF0000Champion^000000 for me..."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(96))?;
                    ctx.call(Function::GetItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Too early.",
                            "Still too early.",
                            "Have you been listening",
                            "to your heart? Listen harder."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if ((ctx.var("guildtime").get()?.number()? > 2059 && ctx.var("guildrelay_q").get()? == 21)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
            {
                if (l_time.clone().number()? > 1 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Yes. You've come at just",
                            "the right time. Remember",
                            "that feeling in your heart,",
                            "and trust it when it comes",
                            "again. Now give this spirit to",
                            "a ^FF0000Monk^000000 or ^FF0000Champion^000000 for me..."
                        ],
                    )?;
                    ctx.var("guildrelay_q").set(Val::from(96))?;
                    ctx.call(Function::GetItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Too early.",
                            "Still too early.",
                            "Have you been listening",
                            "to your heart? Listen harder."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (((l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 192 && ctx.var("guildrelay_q").get()? == 21)
                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Yes. You've come at just",
                        "the right time. Remember",
                        "that feeling in your heart,",
                        "and trust it when it comes",
                        "again. Now give this spirit to",
                        "a ^FF0000Monk^000000 or ^FF0000Champion^000000 for me..."
                    ],
                )?;
                ctx.var("guildrelay_q").set(Val::from(96))?;
                ctx.call(Function::GetItem, vec![Val::from(7250), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 21 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?)) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args!["Not yet. Just wait", "a little longer. Relax,", "and come back later."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?)
                && ctx.call(Function::CountItem, vec![Val::from(7250)])?.number()? > 0)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Most of the souls have",
                        "been shared, and there is",
                        "but one thing I want to ask",
                        "you to do. Please bring me",
                        "some items. Having your friends",
                        "help you do this is acceptable."
                    ],
                )?;
                ctx.next()?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject2 == 1 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Please bring",
                            "^FF000030 Dokebi Horns^000000,",
                            "^FF000030 Fish Tails^000000, and",
                            "^FF000030 Celestial Robes^000000.",
                            "I shall be waiting",
                            "for you right here."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(23))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 2 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Please bring",
                            "^FF000030 Rainbow Shells^000000,",
                            "^FF000030 Elastic Bands^000000, and",
                            "^FF000030 Horrendous Hairs^000000.",
                            "I shall be waiting",
                            "for you right here."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(24))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 3 {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Please bring",
                            "^FF000030 Worn-out Kimonos^000000,",
                            "^FF000030 Anolian Skins^000000, and",
                            "^FF000030 PecoPeco Feathers^000000.",
                            "I shall be waiting",
                            "for you right here."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7250), Val::from(1)])?;
                    ctx.var("guildrelay_q").set(Val::from(94))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(7165)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(1021)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(1023)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 23)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Well, well. You've",
                        "gathered those items",
                        "more quickly that I thought",
                        "you would. Good job. Now,",
                        "please give this to your ^FF0000Guild",
                        "Master^000000 as soon as you can."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7165), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1021), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1023), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(97))?;
                ctx.call(Function::GetItem, vec![Val::from(7251), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?) && ctx.var("guildrelay_q").get()? == 23) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You have to give ^FF0000Transparent Celestial Robe ^00000030ea,",
                        "^FF0000Dokebi Horn ^00000030ea, ^FF0000Fish Tail ^00000030ea.",
                        "You know that, right?",
                        "Good luck~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(1048)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(1013)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7200)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 24)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Well, well. You've",
                        "gathered those items",
                        "more quickly that I thought",
                        "you would. Good job. Now,",
                        "please give this to your ^FF0000Guild",
                        "Master^000000 as soon as you can."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(1048), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(1013), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7200), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(97))?;
                ctx.call(Function::GetItem, vec![Val::from(7251), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?) && ctx.var("guildrelay_q").get()? == 24) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please bring",
                        "^FF000030 Rainbow Shells^000000,",
                        "^FF000030 Elastic Bands^000000, and",
                        "^FF000030 Horrendous Hairs^000000.",
                        "I shall be waiting",
                        "for you right here."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (((ctx.call(Function::CountItem, vec![Val::from(7003)])?.number()? > 29
                && ctx.call(Function::CountItem, vec![Val::from(7101)])?.number()? > 29)
                && ctx.call(Function::CountItem, vec![Val::from(7153)])?.number()? > 29)
                && ctx.var("guildrelay_q").get()? == 94)
            {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Well, well. You've",
                        "gathered those items",
                        "more quickly that I thought",
                        "you would. Good job. Now,",
                        "please give this to your ^FF0000Guild",
                        "Master^000000 as soon as you can."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7003), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7101), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7153), Val::from(30)])?;
                ctx.var("guildrelay_q").set(Val::from(97))?;
                ctx.call(Function::GetItem, vec![Val::from(7251), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?) && ctx.var("guildrelay_q").get()? == 94) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please bring",
                        "^FF000030 Worn-out Kimonos^000000,",
                        "^FF000030 Anolian Skins^000000, and",
                        "^FF000030 PecoPeco Feathers^000000.",
                        "I shall be waiting",
                        "for you right here."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 95 && ctx.call(Function::CountItem, vec![Val::from(7247)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Determination",
                        "to a ^FF0000Priest^000000 or ^FF0000High Priest^000000.",
                        "You knew that already,",
                        "didn't you? Please hurry",
                        "and deliver it soon."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 95 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "For now, it would be",
                        "best for you to rest and",
                        "recoup your strength. Your",
                        "chance to help your guild",
                        "will come soon enough so",
                        "there's no need to rush."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 98 && ctx.call(Function::CountItem, vec![Val::from(7249)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Service to",
                        "a ^FF0000Crusader^000000 or ^FF0000Paladin^000000.",
                        "You knew that already,",
                        "didn't you? Please hurry",
                        "and deliver it soon."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 98 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Hmm. You need to wait",
                        "for the right time to act.",
                        "Why don't you help out",
                        "your guild in the meantime?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 96 && ctx.call(Function::CountItem, vec![Val::from(7250)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "Please give the",
                        "Spirit of Glory to",
                        "a ^FF0000Monk^000000 or ^FF0000Champion^000000.",
                        "You knew that already,",
                        "didn't you? Please hurry",
                        "and deliver it soon."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 96 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "I see that you have",
                        "the potential to become",
                        "a great leader. Be sure",
                        "not to waste it, and lead",
                        "your guild as well as you can."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if (ctx.var("guildrelay_q").get()? == 97 && ctx.call(Function::CountItem, vec![Val::from(7251)])?.number()? > 0) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You need to give",
                        "the ^FF0000Spirit of Victory^000000",
                        "to your Guild Master.",
                        "Please make sure that",
                        "it gets delivered soon."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("guildrelay_q").get()? == 97 {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "You're a nenown explorer,",
                        "and I see in you the potential",
                        "to accomplish even more",
                        "great things. However,",
                        "I doubt there is anything",
                        "more that you can do here."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Hmm. You have no business",
                    "being here. Why don't you",
                    "find something productive",
                    "to do? There is nothing",
                    "for you here, I assure you."
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
                    "You might be the master",
                    "of a guild, but you and I",
                    "have nothing to do with",
                    "each other. I'm sorry."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Then again, I suppose",
                    "you came here to see",
                    "how a real guild operates.",
                    "If that's the case, you're",
                    "welcome to stay and observe."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(l_name_s.clone(), args!["........................."])?;
            ctx.next()?;
            ctx.lines_as(l_name_s.clone(), args!["........................."])?;
            ctx.next()?;
            ctx.lines_as(
                l_name_s.clone(),
                args!["What brings you here?", "I don't think I've seen", "you around before. Hm."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn guilddummy4(ctx: &Ctx) -> Script {
    guilddummy4_body(ctx, Vec::new()).map(|_| ())
}
