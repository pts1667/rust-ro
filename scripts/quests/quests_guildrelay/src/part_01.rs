use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn relaydummy1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gid = Val::from(0);
    let mut l_incen_item = Val::from(0);
    let mut l_name2_s = Val::from("");
    let mut l_name3_s = Val::from("");
    let mut l_name4_s = Val::from("");
    let mut l_name_s = Val::from("");
    let mut l_time = Val::from(0);
    l_name_s = ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?;
    if l_name_s.clone() == "Buzz" {
        l_name2_s = Val::from("Lenya");
        l_name3_s = Val::from("Gealuve");
        l_name4_s = Val::from("Pariz");
        l_gid = ctx.call(
            Function::GetCastleData,
            vec![
                (Val::from("aldeg_cas") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?),
                Val::from(1),
            ],
        )?;
    } else if l_name_s.clone() == "Jody" {
        l_name2_s = Val::from("Ron Haware");
        l_name3_s = Val::from("Vers");
        l_name4_s = Val::from("Gen Garish");
        l_gid = ctx.call(
            Function::GetCastleData,
            vec![
                (Val::from("gefg_cas") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?),
                Val::from(1),
            ],
        )?;
    } else if l_name_s.clone() == "Chungye" {
        l_name2_s = Val::from("Dosuhlji");
        l_name3_s = Val::from("Yayula");
        l_name4_s = Val::from("Ashin");
        l_gid = ctx.call(
            Function::GetCastleData,
            vec![
                (Val::from("payg_cas") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?),
                Val::from(1),
            ],
        )?;
    } else if l_name_s.clone() == "Hermod" {
        l_name2_s = Val::from("Atila");
        l_name3_s = Val::from("Cecil");
        l_name4_s = Val::from("Diligo");
        l_gid = ctx.call(
            Function::GetCastleData,
            vec![
                (Val::from("prtg_cas") + ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?),
                Val::from(1),
            ],
        )?;
    }
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(630)])? == 0 {
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
            l_time = runtime::atoi(&ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?);
            if ctx.var("guildrelay_q").get()? == 100 {
                if ctx.var("guildtime").get()?.number()? > 2299 {
                    if (l_time.clone().number()? > 65 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Oh, you're back. So did you",
                                "rest up enough? I'm sure the",
                                "other guild members are",
                                "feeling refreshed by now.",
                                "From the looks of it, you're",
                                "ready for your next mission."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "The next step for",
                                "you is to take this, the",
                                "''Spirit of Trust.'' If this",
                                "guild is going to be solid,",
                                "you need to think how much",
                                "trust there is in the guild."
                            ],
                        )?;
                        ctx.var("guildrelay_q").set(Val::from(8))?;
                        ctx.call(Function::GetItem, vec![Val::from(7240), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Your second test will be",
                                "to give that ^4D4DFFSpirit of Trust^000000",
                                "to a sage that can manipulate",
                                "nature's attributes. In other",
                                "words, a Sage or Scholar",
                                "must carry out this task."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "The Spirit of Trust must",
                                ((Val::from("be delivered to ^4D4DFF") + l_name3_s.clone()) + Val::from("^000000,")),
                                "so don't forget to relay",
                                "that information to your",
                                "Sage or Scholar. Very well,",
                                "good luck on your journey."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Ah, did you rest we-- Oh.",
                                "Why do you look so pale?",
                                "You can't accomplish great",
                                "deeds when you're overworked!",
                                "Rest. Your guild must be able",
                                "to depend on your strength."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Your determination and",
                                "spirit is commendable, but",
                                "have the patience to recollect",
                                "yourself when you must. I will",
                                "be waiting here, so please",
                                "come back to me later."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("guildtime").get()?.number()? > 2200 {
                        if (l_time.clone().number()? > 1 && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()) {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Oh, you're back. So did you",
                                    "rest up enough? I'm sure the",
                                    "other guild members are",
                                    "feeling refreshed by now.",
                                    "From the looks of it, you're",
                                    "ready for your next mission."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "The next step for",
                                    "you is to take this, the",
                                    "''Spirit of Trust.'' If this",
                                    "guild is going to be solid,",
                                    "you need to think how much",
                                    "trust there is in the guild."
                                ],
                            )?;
                            ctx.var("guildrelay_q").set(Val::from(8))?;
                            ctx.call(Function::GetItem, vec![Val::from(7240), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Your second test will be",
                                    "to give that ^4D4DFFSpirit of Trust^000000",
                                    "to a sage that can manipulate",
                                    "nature's attributes. In other",
                                    "words, a Sage or Scholar",
                                    "must carry out this task."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "The Spirit of Trust must",
                                    ((Val::from("be delivered to ^4D4DFF") + l_name3_s.clone()) + Val::from("^000000,")),
                                    "so don't forget to relay",
                                    "that information to your",
                                    "Sage or Scholar. Very well,",
                                    "good luck on your journey."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Ah, did you rest we-- Oh.",
                                    "Why do you look so pale?",
                                    "You can't accomplish great",
                                    "deeds when you're overworked!",
                                    "Rest. Your guild must be able",
                                    "to depend on your strength."
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Your determination and",
                                    "spirit is commendable, but",
                                    "have the patience to recollect",
                                    "yourself when you must. I will",
                                    "be waiting here, so please",
                                    "come back to me later."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if (l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 128 {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Oh, you're back. So did you",
                                    "rest up enough? I'm sure the",
                                    "other guild members are",
                                    "feeling refreshed by now.",
                                    "From the looks of it, you're",
                                    "ready for your next mission."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "The next step for",
                                    "you is to take this, the",
                                    "''Spirit of Trust.'' If this",
                                    "guild is going to be solid,",
                                    "you need to think how much",
                                    "trust there is in the guild."
                                ],
                            )?;
                            ctx.var("guildrelay_q").set(Val::from(8))?;
                            ctx.call(Function::GetItem, vec![Val::from(7240), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Your second test will be",
                                    "to give that ^4D4DFFSpirit of Trust^000000",
                                    "to a sage that can manipulate",
                                    "nature's attributes. In other",
                                    "words, a Sage or Scholar",
                                    "must carry out this task."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "The Spirit of Trust must",
                                    ((Val::from("be delivered to ^4D4DFFY") + l_name3_s.clone()) + Val::from("^000000,")),
                                    "so don't forget to relay",
                                    "that information to your",
                                    "Sage or Scholar. Very well,",
                                    "good luck on your journey."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Ah, did you rest we-- Oh.",
                                    "Why do you look so pale?",
                                    "You can't accomplish great",
                                    "deeds when you're overworked!",
                                    "Rest. Your guild must be able",
                                    "to depend on your strength."
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Your determination and",
                                    "spirit is commendable, but",
                                    "have the patience to recollect",
                                    "yourself when you must. I will",
                                    "be waiting here, so please",
                                    "come back to me later."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            } else {
                if ctx.var("guildrelay_q").get()? == 150 {
                    if ((((ctx.var("guildtime").get()?.number()? > 2299 && l_time.clone().number()? > 65)
                        && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true())
                        || ((ctx.var("guildtime").get()?.number()? > 2199 && l_time.clone().number()? > 1)
                            && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()))
                        || (l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 128)
                    {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Ah, you look well rested,",
                                "master. It is now time for",
                                "you to proceed with the third",
                                "test. Let me remind you that",
                                "these spirits are incredibly",
                                "precious. Do not lose them."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Like all the other spirits,",
                                "please keep this one safely.",
                                "This is the ^4D4DFFSpirit of Peace^000000.",
                                "It seems contradictive that",
                                "strongholds and might can",
                                "bring peace, but it's true."
                            ],
                        )?;
                        ctx.var("guildrelay_q").set(Val::from(15))?;
                        ctx.call(Function::GetItem, vec![Val::from(7246), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "If power isn't used to",
                                "protect the weak, then",
                                "some bully, in one form or",
                                "another, will always come",
                                "to exploit them. That is why",
                                "Tristan III built the strongholds."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Please dispatch a Rogue",
                                "or Stalker to bring this",
                                ((Val::from("Spirit of Peace to ^4D4DFF") + l_name4_s.clone()) + Val::from("^000000.")),
                                "If you don't know any Rogues",
                                "or Stalkers, then it would be",
                                "prudent for you to meet one."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Yes, there's no substitute",
                                "for the subterfuge and intel",
                                "gathering that a Rogue can",
                                ((Val::from("provide. Anyway, ") + l_name4_s.clone()) + Val::from(" will")),
                                "guide you on your third test."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Ah, did you rest we-- Oh.",
                                "Why do you look so pale?",
                                "You can't accomplish great",
                                "deeds when you're overworked!",
                                "Rest. Your guild must be able",
                                "to depend on your strength."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "Your determination and",
                                "spirit is commendable, but",
                                "have the patience to recollect",
                                "yourself when you must. I will",
                                "be waiting here, so please",
                                "come back to me later."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("guildrelay_q").get()? == 25 {
                        if (((((((ctx.var("guildtime").get()?.number()? > 2299 && l_time.clone().number()? > 257)
                            && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true())
                            || ((ctx.var("guildtime").get()?.number()? > 2199 && l_time.clone().number()? > 193)
                                && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()))
                            || ((ctx.var("guildtime").get()?.number()? > 2059 && l_time.clone().number()? > 129)
                                && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()))
                            || ((ctx.var("guildtime").get()?.number()? > 1999 && l_time.clone().number()? > 65)
                                && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()))
                            || ((ctx.var("guildtime").get()?.number()? > 1899 && l_time.clone().number()? > 1)
                                && runtime::op(&l_time.clone(), "<", &ctx.var("guildtime").get()?)?.is_true()))
                            || (l_time.clone().try_sub(ctx.var("guildtime").get()?)?).number()? > 320)
                        {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Ah, have you rested well,",
                                    "master? Please excuse my",
                                    "manners a while ago. I had",
                                    "to report your trial results,",
                                    "and lost my composure for a",
                                    "moment. It won't happen again."
                                ],
                            )?;
                            ctx.var("guildrelay_q").set(Val::from(999))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines(args!["...............................", "..............................."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "...............................",
                                "...............................",
                                "..............................."
                            ])?;
                            ctx.next()?;
                            ctx.lines(args![((Val::from("[") + l_name_s.clone()) + Val::from("]"))])?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                            ctx.lines(args![
                                "Oh! M-master? Wh-when",
                                "did you...? I'm so sorry.",
                                "I was busy working."
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "It's only been a few days",
                                    "since you finished the trials.",
                                    "For now, you should rest and",
                                    "take care of your guild, okay?",
                                    "Please come back later.",
                                    "I've got much to attend to..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("guildrelay_q").get()? == 999 {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "Good day, master.",
                                    "Is something wrong?",
                                    "You look as though",
                                    "something is on your mind."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("N-no, nothing.:I want to take a lesson.")])? {
                                1 => {
                                    ctx.lines_as(l_name_s.clone(), args!["Hm? That's strange."])?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                                    ctx.lines(args!["You didn't want to take", "the trials again, did you?"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "I see. You know it won't",
                                            "be easy, but I suppose you",
                                            "are prepared. Here, take",
                                            "the ^4D4DFFSpirit of Guild^000000, and",
                                            "give it to your most trusted",
                                            "Knight or Lord Knight."
                                        ],
                                    )?;
                                    ctx.var("guildrelay_q").set(Val::from(1))?;
                                    ctx.call(Function::GetItem, vec![Val::from(7234), Val::from(1)])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "This Knight should take",
                                            "the Spirit of Guild over",
                                            ((Val::from("to ^4D4DFF") + l_name2_s.clone()) + Val::from("^000000. Good luck, and")),
                                            "may the gods be with you."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if ctx.var("guildrelay_q").get()? == 1 {
                                if ctx.call(Function::CountItem, vec![Val::from(7234)])?.number()? > 0 {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "You still have the Spirit of",
                                            "Guild I gave you? I suppose",
                                            "you haven't found a Knight or",
                                            "Lord Knight to which it can",
                                            "be entrusted. It is imperative",
                                            "that you find someone soon."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "For the sake of the guild, it",
                                            "will be advantageous to have",
                                            "a Knight or Lord Knight on",
                                            "your side: their combat skills",
                                            "can turn the tide of battles."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "When you do find a Knight,",
                                            "give him the Spirit of Guild",
                                            ((Val::from("and ask him to find ") + l_name2_s.clone()) + Val::from("")),
                                            "since he will conducting",
                                            "the trial. Good luck",
                                            "to you, master."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.call(Function::CountItem, vec![Val::from(7239)])?.number()? > 0 {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "Ah, is this the Spirit",
                                            "of Advance? This must mean",
                                            "that you completed the first",
                                            "test. Keep up the good work.",
                                            "Hand me the spirit, and allow",
                                            "me to give you your guild's reward."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7239), Val::from(1)])?;
                                    ctx.var("guildtime").set(l_time.clone())?;
                                    ctx.var("guildrelay_q").set(Val::from(100))?;
                                    l_incen_item = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                                    if (l_incen_item.clone().number()? > 0 && l_incen_item.clone().number()? < 25) {
                                        ctx.call(Function::GetItem, vec![Val::from(608), Val::from(20)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(2)])?;
                                    } else if (l_incen_item.clone().number()? > 24 && l_incen_item.clone().number()? < 50) {
                                        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(10)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(2)])?;
                                    } else if (l_incen_item.clone().number()? > 50 && l_incen_item.clone().number()? < 75) {
                                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(5)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(2)])?;
                                    } else if (l_incen_item.clone().number()? > 74 && l_incen_item.clone().number()? < 101) {
                                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(3)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(2)])?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "You've done well, but",
                                            "there are more trials",
                                            "ahead of you. For now,",
                                            "you should rest before",
                                            "undertaking the second test.",
                                            "Please come when you are ready."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "If you're not busy, then",
                                            "why don't you spend your",
                                            "time increasing morale",
                                            "among your guild members?",
                                            "Perhaps some team building",
                                            "exercise can be of help."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.var("guildrelay_q").get()? == 8 {
                                    if ctx.call(Function::CountItem, vec![Val::from(7240)])?.number()? > 0 {
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "You must be having",
                                                ((Val::from("trouble finding ") + l_name3_s.clone()) + Val::from(".")),
                                                "Make sure that you have",
                                                "a Sage or Scholar friend",
                                                "give that Spirit of Trust to",
                                                "him once you locate him."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        if ctx.call(Function::CountItem, vec![Val::from(7245)])?.number()? > 0 {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "Ah, so you already completed",
                                                    "the test and earned the Spirit",
                                                    "of Friendship? Good work.",
                                                    "You must now recognize the",
                                                    "value of teamwork. Please give",
                                                    "the Spirit of Friendship to me."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "Now, please take this",
                                                    "reward and share it with",
                                                    "your guild members. This",
                                                    "may come in handy in future",
                                                    "challenges that you will all",
                                                    "face together. Good work!"
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(7245), Val::from(1)])?;
                                            ctx.var("guildtime").set(runtime::atoi(
                                                &ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?,
                                            ))?;
                                            ctx.var("guildrelay_q").set(Val::from(150))?;
                                            l_incen_item = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                                            if (l_incen_item.clone().number()? > 0 && l_incen_item.clone().number()? < 16) {
                                                ctx.call(Function::GetItem, vec![Val::from(607), Val::from(10)])?;
                                                ctx.call(Function::GetItem, vec![Val::from(644), Val::from(5)])?;
                                                ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                            } else {
                                                if (l_incen_item.clone().number()? > 14 && l_incen_item.clone().number()? < 31) {
                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(10)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(3)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                } else {
                                                    if (l_incen_item.clone().number()? > 29 && l_incen_item.clone().number()? < 46) {
                                                        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(10)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(3)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                    } else if (l_incen_item.clone().number()? > 44 && l_incen_item.clone().number()? < 61) {
                                                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(4)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(2)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                    } else if (l_incen_item.clone().number()? > 59 && l_incen_item.clone().number()? < 76) {
                                                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(3)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(2)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                    } else if (l_incen_item.clone().number()? > 74 && l_incen_item.clone().number()? < 91) {
                                                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(2)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(2)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(678), Val::from(3)])?;
                                                    } else if (l_incen_item.clone().number()? > 89 && l_incen_item.clone().number()? < 101)
                                                    {
                                                        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(10)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(644), Val::from(3)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(2)])?;
                                                        ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                                    }
                                                }
                                            }
                                            ctx.next()?;
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "You should rest and",
                                                    "recuperate before you",
                                                    "undertake the third test.",
                                                    "I too would benefit from",
                                                    "a brief respite. Please come",
                                                    "back to me when you are ready."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "If you're not busy, then",
                                                    "why don't you spend your",
                                                    "time increasing morale",
                                                    "among your guild members?",
                                                    "Perhaps some team building",
                                                    "exercise can be of help."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "If it suits your fancy,",
                                                    "you may simply come here",
                                                    "and join me for a cup of tea."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                } else {
                                    if ctx.var("guildrelay_q").get()? == 15 {
                                        if ctx.call(Function::CountItem, vec![Val::from(7246)])?.number()? > 0 {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "You still have the",
                                                    "Spirit of Peace? If you",
                                                    "don't have any Rogues or",
                                                    "Stalkers in your guild, now",
                                                    "would be the time to recruit",
                                                    "them. Heed my advice..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.call(Function::CountItem, vec![Val::from(7251)])?.number()? > 0 {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "I see that you've completed",
                                                        "the last test and received",
                                                        "the Spirit of Victory. Heh.",
                                                        "Victory is the natural result",
                                                        "when your guild works together",
                                                        "in harmony as a united team."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "Congratulations on",
                                                        "completing all the trials.",
                                                        "Please accept this reward,",
                                                        "given on the behalf of King",
                                                        "Tristan III, and share it with",
                                                        "guild. Once again, good work."
                                                    ],
                                                )?;
                                                ctx.call(Function::DelItem, vec![Val::from(7251), Val::from(1)])?;
                                                ctx.var("guildtime").set(runtime::atoi(
                                                    &ctx.call(Function::GetTimeStr, vec![Val::from("%H%M"), Val::from(5)])?,
                                                ))?;
                                                ctx.var("guildrelay_q").set(Val::from(25))?;
                                                l_incen_item = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                                                if (l_incen_item.clone().number()? > 0 && l_incen_item.clone().number()? < 26) {
                                                    ctx.call(Function::GetItem, vec![Val::from(608), Val::from(10)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(5)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(4)])?;
                                                } else if (l_incen_item.clone().number()? > 25 && l_incen_item.clone().number()? < 51) {
                                                    ctx.call(Function::GetItem, vec![Val::from(608), Val::from(10)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(5)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(3)])?;
                                                } else if (l_incen_item.clone().number()? > 50 && l_incen_item.clone().number()? < 76) {
                                                    ctx.call(Function::GetItem, vec![Val::from(608), Val::from(10)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(617), Val::from(2)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(607), Val::from(5)])?;
                                                } else if (l_incen_item.clone().number()? > 75 && l_incen_item.clone().number()? < 91) {
                                                    ctx.call(Function::GetItem, vec![Val::from(608), Val::from(10)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(644), Val::from(4)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(2)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                                } else if (l_incen_item.clone().number()? > 90 && l_incen_item.clone().number()? < 101) {
                                                    ctx.call(Function::GetItem, vec![Val::from(5074), Val::from(1)])?;
                                                }
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "You and your guild must be",
                                                        "quite tired now. Your rooms",
                                                        "are ready for you if you decide",
                                                        "to rest. Please visit me again",
                                                        "if you wish to take the trials",
                                                        "again. I'll see you later."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "If you aren't particularly",
                                                        "busy, then why don't you",
                                                        "spend some time with your",
                                                        "guild members? Building",
                                                        "strong camaraderie never",
                                                        "fails to pay off. Never."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else {
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "Greetings, master.",
                                                ((Val::from("I am ") + l_name_s.clone()) + Val::from(", one of the four")),
                                                "Great Sages, and I am here",
                                                "to serve you under the orders",
                                                "of wise and benevolent",
                                                "King Tristan III."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "We test guilds that own",
                                                "strongholds, and see if they",
                                                "are qualified to be considered",
                                                "elite guilds. Our goal is to",
                                                "train guilds to strengthen",
                                                "our military forces."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "Although your guild has the",
                                                "strength and courage to conquer",
                                                "a stronghold, we want you to",
                                                "prove that your guild has",
                                                "a strong sense of justice,",
                                                "honor, and compassion."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "It is up to you. Will",
                                                "you take the test I have",
                                                "for you? I will do my best to",
                                                "help your guild grow stronger",
                                                "so that you will be better",
                                                "able to protect the weak."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("I want to take the test.:Let me think about it.")])?
                                        {
                                            1 => {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "Is that so? In this test,",
                                                        "you will be given orders",
                                                        "and special spirits. These",
                                                        "spirits will only be given",
                                                        "once: you must not lose them."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "If you lose the spirit,",
                                                        "you will be judged as",
                                                        "irresponsible, and will be",
                                                        "unable to complete the tests.",
                                                        "You must find any lost spirit to",
                                                        "proceed. There's no second chance."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "There are a few things you",
                                                        "should know. Firstly, you can",
                                                        "only take care of one spirit",
                                                        "at a time. Secondly, you need",
                                                        "to do the tests in order and",
                                                        "follow the guide's instructions."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "Thirdly, the spirit should",
                                                        "be given to the type of person",
                                                        "specified in the test. Those",
                                                        "are the rules. Remember them."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "Okay, this is the first",
                                                        "spirit that will be entrusted",
                                                        "to you. Make sure to give this",
                                                        "to a Knight or Lord Knight.",
                                                        "The test has now officially",
                                                        "begun. Good luck to you."
                                                    ],
                                                )?;
                                                ctx.var("guildrelay_q").set(Val::from(1))?;
                                                ctx.call(Function::GetItem, vec![Val::from(7234), Val::from(1)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "I suppose it would be",
                                                        "a good idea to discuss",
                                                        "this with your guild before",
                                                        "you decide to commit to",
                                                        "taking the test. Feel free",
                                                        "free to visit me again later."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if ctx.call(Function::CountItem, vec![Val::from(7234)])?.number()? > 0 {
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                        "So you were the one chosen",
                        "by your guild master? I see.",
                        "You should deliver that Spirit",
                        ((Val::from("of Guild over to ") + l_name2_s.clone()) + Val::from(".")),
                        "He'll instruct you further."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    l_name_s.clone(),
                    args![
                        "The Spirit of Guild is",
                        "useless unless it is in the",
                        "hands of a Knight or Lord",
                        "Knight. You should speak",
                        ((Val::from("to ") + l_name2_s.clone()) + Val::from(" to learn more."))
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.call(Function::CountItem, vec![Val::from(7235)])?.number()? > 0 {
                if (ctx.var("guildrelay_q").get()? == 88 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            ((Val::from("Ah,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                            "Congratulations. It looks",
                            "like you did a good job.",
                            "Please give the Spirit of",
                            "Charge to the next person",
                            "to continue the testing."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            ((Val::from("") + l_name2_s.clone()) + Val::from(" should have")),
                            "explained everything, but",
                            "if you forgot, then please",
                            "go and ask him again."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?) {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                            "Ah, I see that you have",
                            "the Spirit of Charge.",
                            "Heh heh, it's always exciting",
                            "to charge into battle, isn't",
                            "it? Well then, do your best."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "That spirit won't be very",
                            "useful if it's not in the",
                            "hands of the right person.",
                            ((Val::from("") + l_name2_s.clone()) + Val::from(" knows more about the")),
                            "Spirit of Charge, so you should",
                            "ask him more about that spirit."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.call(Function::CountItem, vec![Val::from(7237)])?.number()? > 0 {
                    if (ctx.var("guildrelay_q").get()? == 87 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
                    {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                "I commend you on your work.",
                                "Please give that spirit to",
                                "the next person so that the",
                                "testing can continue."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                ((Val::from("") + l_name2_s.clone()) + Val::from(" should have")),
                                "explained everything, but",
                                "if you forgot, then please",
                                "go and ask him again."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?) {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                "I see that you posess",
                                "the Spirit of Association.",
                                "Good luck with your test."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            l_name_s.clone(),
                            args![
                                "That spirit won't be very",
                                "useful if it's not in the",
                                "hands of the right person.",
                                ((Val::from("Talk to ") + l_name2_s.clone()) + Val::from(" if you want")),
                                "to know more about the",
                                "Spirit of Association."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.call(Function::CountItem, vec![Val::from(7238)])?.number()? > 0 {
                        if (ctx.var("guildrelay_q").get()? == 86
                            && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?))
                        {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                    "I commend you on your work.",
                                    "Please give that spirit to",
                                    "the next person so that the",
                                    "testing can continue."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    ((Val::from("") + l_name2_s.clone()) + Val::from(" should have")),
                                    "explained everything, but",
                                    "if you forgot, then please",
                                    "go and ask him again."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?) {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                    "I see that you posess",
                                    "the Spirit of Coordination.",
                                    "Good luck on your test."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                l_name_s.clone(),
                                args![
                                    "That spirit won't be very",
                                    "useful if it's not in the",
                                    "hands of the right person.",
                                    ((Val::from("") + l_name2_s.clone()) + Val::from(" will know more about")),
                                    "the Spirit of Coordination",
                                    "so you should consult him."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.call(Function::CountItem, vec![Val::from(7239)])?.number()? > 0 {
                            if (ctx.var("guildrelay_q").get()? == 85
                                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?))
                            {
                                ctx.lines_as(
                                    l_name_s.clone(),
                                    args![
                                        ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                                        "Congratulations, it looks",
                                        "like you finished the test.",
                                        "You may now give the Spirit of",
                                        "Advance to your guild master."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    l_name_s.clone(),
                                    args![
                                        "That spirit won't be very",
                                        "useful if it's not in the",
                                        "hands of the right person.",
                                        ((Val::from("") + l_name2_s.clone()) + Val::from(" will know more about")),
                                        "the Spirit of Advance so",
                                        "you should consult him."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.call(Function::CountItem, vec![Val::from(7240)])?.number()? > 0 {
                                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from(".")),
                                            "I see that you possess",
                                            "the Spirit of Trust.",
                                            "Good luck, and do not",
                                            "fail the trust placed in",
                                            "you by your guild."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        l_name_s.clone(),
                                        args![
                                            "That spirit won't be very",
                                            "useful if it's not in the",
                                            "hands of the right person.",
                                            ((Val::from("") + l_name3_s.clone()) + Val::from(" will know more")),
                                            "about the Spirit of Trust",
                                            "so you should consult him."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                if ctx.call(Function::CountItem, vec![Val::from(7241)])?.number()? > 0 {
                                    if (ctx.var("guildrelay_q").get()? == 71
                                        && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?))
                                    {
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from(".")),
                                                "I commend you on your work.",
                                                "Please give that spirit to",
                                                "the next person so that the",
                                                "testing can continue."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "If you don't remember",
                                                ((Val::from("") + l_name3_s.clone()) + Val::from("'s explanation,")),
                                                "then you might want to",
                                                "go back to him and ask",
                                                "him to tell you again."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?))
                                    {
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from(",")),
                                                "I see that you have the",
                                                "Spirit of Union. Always keep",
                                                "in mind that the strength of",
                                                "your guild is directly",
                                                "related to its unity."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            l_name_s.clone(),
                                            args![
                                                "That spirit won't be very",
                                                "useful if it's not in the",
                                                "hands of the right person.",
                                                ((Val::from("") + l_name3_s.clone()) + Val::from(" will know more")),
                                                "about the Spirit of Union",
                                                "so you should consult him."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.call(Function::CountItem, vec![Val::from(7242)])?.number()? > 0 {
                                        if (ctx.var("guildrelay_q").get()? == 72
                                            && (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BARD")?)
                                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_DANCER")?)))
                                        {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    ((Val::from("Hello,") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from(".")),
                                                    "I commend you on your work.",
                                                    "Please give that spirit to",
                                                    "the next person so that the",
                                                    "testing can continue."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "If you don't remember",
                                                    ((Val::from("") + l_name3_s.clone()) + Val::from("'s explanation,")),
                                                    "then you might want to",
                                                    "go back to him and ask",
                                                    "him to tell you again."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from(",")),
                                                    "I see that you have the Spirit",
                                                    "of Combination. Remember that",
                                                    "working in tandem, combining",
                                                    "your guild's skills and talents,",
                                                    "will realize your true potential."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines_as(
                                                l_name_s.clone(),
                                                args![
                                                    "That spirit won't be very",
                                                    "useful if it's not in the",
                                                    "hands of the right person.",
                                                    ((Val::from("") + l_name3_s.clone()) + Val::from(" will know more about")),
                                                    "the Spirit of Combination so",
                                                    "you should consult him."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.call(Function::CountItem, vec![Val::from(7244)])?.number()? > 0 {
                                            if (ctx.var("guildrelay_q").get()? == 74
                                                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?))
                                            {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        ((Val::from("Greetings, ")
                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(",")),
                                                        "did you rest well? Please",
                                                        "give that spirit to the next",
                                                        "person so that the testing",
                                                        "of your guild may continue."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "If you don't remember",
                                                        ((Val::from("") + l_name3_s.clone()) + Val::from("'s explanation,")),
                                                        "then you might want to",
                                                        "go back to him and ask",
                                                        "him to tell you again."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from(",")),
                                                        "I see that you've been",
                                                        "entrusted with the Spirit",
                                                        "of Solidarity. Do your best",
                                                        "on this test for the sake",
                                                        "of your guild, alright?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    l_name_s.clone(),
                                                    args![
                                                        "That spirit won't be very",
                                                        "useful if it's not in the",
                                                        "hands of the right person.",
                                                        ((Val::from("") + l_name3_s.clone()) + Val::from(" will know more")),
                                                        "about the Spirit of Solidarity",
                                                        "so you should consult him."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            if ctx.call(Function::CountItem, vec![Val::from(7245)])?.number()? > 0 {
                                                if (ctx.var("guildrelay_q").get()? == 75
                                                    && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?))
                                                {
                                                    ctx.lines_as(
                                                        l_name_s.clone(),
                                                        args![
                                                            ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from(",")),
                                                            "congratulations on a job",
                                                            "well done. Please give the",
                                                            "Spirit of Friendship to your",
                                                            "guild master to continue",
                                                            "the guild testing."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as(
                                                        l_name_s.clone(),
                                                        args![
                                                            "That spirit won't be very",
                                                            "useful if it's not in the",
                                                            "hands of the right person.",
                                                            ((Val::from("") + l_name3_s.clone()) + Val::from(" will know more")),
                                                            "about the Spirit of Friendship",
                                                            "so you should consult him."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                if ctx.call(Function::CountItem, vec![Val::from(7246)])?.number()? > 0 {
                                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
                                                        ctx.lines_as(
                                                            l_name_s.clone(),
                                                            args![
                                                                ((Val::from("Ah, ")
                                                                    + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                    + Val::from(",")),
                                                                "I see that you have the",
                                                                "Spirit of Peace. Please",
                                                                "do your best for the sake",
                                                                "of the guild, though I do not",
                                                                "doubt you'll pass this test."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        ctx.lines_as(
                                                            l_name_s.clone(),
                                                            args![
                                                                "That spirit won't be very",
                                                                "useful if it's not in the",
                                                                "hands of the right person.",
                                                                ((Val::from("") + l_name4_s.clone()) + Val::from(" will know more")),
                                                                "about the Spirit of Peace",
                                                                "so you should consult him."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                } else {
                                                    if ctx.call(Function::CountItem, vec![Val::from(7247)])?.number()? > 0 {
                                                        if (ctx.var("guildrelay_q").get()? == 95
                                                            && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?))
                                                        {
                                                            ctx.lines_as(
                                                                l_name_s.clone(),
                                                                args![
                                                                    ((Val::from("Hello,")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from(".")),
                                                                    "I commend you on your work.",
                                                                    "Please give that spirit to",
                                                                    "the next person so that the",
                                                                    "testing can continue."
                                                                ],
                                                            )?;
                                                            ctx.next()?;
                                                            ctx.lines_as(
                                                                l_name_s.clone(),
                                                                args![
                                                                    "If you don't remember",
                                                                    ((Val::from("") + l_name4_s.clone()) + Val::from("'s explanation,")),
                                                                    "then you might want to",
                                                                    "go back to him and ask",
                                                                    "him to tell you again."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                                                            ctx.lines_as(
                                                                l_name_s.clone(),
                                                                args![
                                                                    ((Val::from("Ah, ")
                                                                        + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                        + Val::from(",")),
                                                                    "please take good care of",
                                                                    "that Spirit of Determination."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            ctx.lines_as(
                                                                l_name_s.clone(),
                                                                args![
                                                                    "That spirit won't be very",
                                                                    "useful if it's not in the",
                                                                    "hands of the right person.",
                                                                    ((Val::from("") + l_name4_s.clone())
                                                                        + Val::from(" will know more about")),
                                                                    "the Spirit of Determination",
                                                                    "so you should consult him."
                                                                ],
                                                            )?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    } else {
                                                        if ctx.call(Function::CountItem, vec![Val::from(7249)])?.number()? > 0 {
                                                            if (ctx.var("guildrelay_q").get()? == 98
                                                                && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?))
                                                            {
                                                                ctx.lines_as(
                                                                    l_name_s.clone(),
                                                                    args![
                                                                        ((Val::from("Hello,")
                                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                            + Val::from(".")),
                                                                        "I commend you on your work.",
                                                                        "Please give that spirit to",
                                                                        "the next person so that the",
                                                                        "testing can continue."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    l_name_s.clone(),
                                                                    args![
                                                                        "If you don't remember",
                                                                        ((Val::from("") + l_name4_s.clone())
                                                                            + Val::from("'s explanation,")),
                                                                        "then you might want to",
                                                                        "go back to him and ask",
                                                                        "him to tell you again."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx
                                                                .var("BaseJob")
                                                                .get()?
                                                                .loosely_equals(&ctx.constant("JOB_CRUSADER")?)
                                                            {
                                                                ctx.lines_as(
                                                                    l_name_s.clone(),
                                                                    args![
                                                                        ((Val::from("Ah, ")
                                                                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                            + Val::from(",")),
                                                                        "please take good care",
                                                                        "of that Spirit of Service."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                ctx.lines_as(
                                                                    l_name_s.clone(),
                                                                    args![
                                                                        "That spirit won't be very",
                                                                        "useful if it's not in the",
                                                                        "hands of the right person.",
                                                                        ((Val::from("") + l_name4_s.clone())
                                                                            + Val::from(" will know more about")),
                                                                        "the Spirit of Service so",
                                                                        "you should consult him."
                                                                    ],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            }
                                                        } else {
                                                            if ctx.call(Function::CountItem, vec![Val::from(7250)])?.number()? > 0 {
                                                                if (ctx.var("guildrelay_q").get()? == 96
                                                                    && ctx
                                                                        .var("BaseJob")
                                                                        .get()?
                                                                        .loosely_equals(&ctx.constant("JOB_CRUSADER")?))
                                                                {
                                                                    ctx.lines_as(
                                                                        l_name_s.clone(),
                                                                        args![
                                                                            ((Val::from("Hello,")
                                                                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                                + Val::from(".")),
                                                                            "I commend you on your work.",
                                                                            "Please give that spirit to",
                                                                            "the next person so that the",
                                                                            "testing can continue."
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        l_name_s.clone(),
                                                                        args![
                                                                            "If you don't remember",
                                                                            ((Val::from("") + l_name4_s.clone())
                                                                                + Val::from("'s explanation,")),
                                                                            "then you might want to",
                                                                            "go back to him and ask",
                                                                            "him to tell you again."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else if ctx
                                                                    .var("BaseJob")
                                                                    .get()?
                                                                    .loosely_equals(&ctx.constant("JOB_MONK")?)
                                                                {
                                                                    ctx.lines_as(
                                                                        l_name_s.clone(),
                                                                        args![
                                                                            ((Val::from("Ah, ")
                                                                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                                + Val::from(",")),
                                                                            "please take good care",
                                                                            "of that Spirit of Glory."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    ctx.lines_as(
                                                                        l_name_s.clone(),
                                                                        args![
                                                                            "That spirit won't be very",
                                                                            "useful if it's not in the",
                                                                            "hands of the right person.",
                                                                            ((Val::from("") + l_name4_s.clone())
                                                                                + Val::from(" will know more about")),
                                                                            "the Spirit of Glory so you",
                                                                            "should consult him."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                            } else {
                                                                if ctx.call(Function::CountItem, vec![Val::from(7251)])?.number()? > 0 {
                                                                    if (ctx.var("guildrelay_q").get()? == 97
                                                                        && ctx
                                                                            .var("Class")
                                                                            .get()?
                                                                            .loosely_equals(&ctx.constant("JOB_MONK")?))
                                                                    {
                                                                        ctx.lines_as(
                                                                            l_name_s.clone(),
                                                                            args![
                                                                                ((Val::from("Ah, ")
                                                                                    + ctx.call(
                                                                                        Function::StrCharInfo,
                                                                                        vec![Val::from(0)]
                                                                                    )?)
                                                                                    + Val::from(",")),
                                                                                "congratulations on a job",
                                                                                "well done. Please give",
                                                                                "the Spirit of Victory to your",
                                                                                "guild master. That's it for now.",
                                                                                "Good luck to you in the future."
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        ctx.lines_as(
                                                                            l_name_s.clone(),
                                                                            args![
                                                                                "That spirit won't be very",
                                                                                "useful if it's not in the",
                                                                                "hands of the right person.",
                                                                                ((Val::from("") + l_name4_s.clone())
                                                                                    + Val::from(" will know more about")),
                                                                                "the Spirit of Victory so you",
                                                                                "should consult him."
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                } else {
                                                                    ctx.lines_as(
                                                                        l_name_s.clone(),
                                                                        args![
                                                                            "I'm sorry, but I'm too",
                                                                            "busy with my work to offer",
                                                                            "you any help right now.",
                                                                            "You'll have to forgive me."
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
                            }
                        }
                    }
                }
            }
        }
    } else {
        if ctx.call(Function::GetCharacterId, vec![Val::from(2)])? == 0 {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "You haven't joined",
                    "a guild yet? Why don't",
                    "you think about joining",
                    "one? Well, it's your decision,",
                    "but I don't think you'd regret",
                    "being part of a strong guild."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                l_name_s.clone(),
                args![
                    "Ah, I see that you are",
                    "affiliated with a guild.",
                    "Have you come here",
                    "as an invited guest?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("No, not really...:I was invited by the guild master.")])? {
                1 => {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Oh, really?",
                            "That's too bad...",
                            "My apologies, but",
                            "I can't allow strangers",
                            "to simply come and go",
                            "around here. Farewell, then."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Warp, vec![Val::from("alde_gld"), Val::from(186), Val::from(157)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        l_name_s.clone(),
                        args![
                            "Oh, really? Ah, now",
                            ((Val::from("I recognize you, ^4d4dff") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("^000000.")),
                            "Please come in, and make",
                            "yourself comfortable."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn relaydummy1(ctx: &Ctx) -> Script {
    relaydummy1_body(ctx, Vec::new()).map(|_| ())
}
