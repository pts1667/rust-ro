use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bafhail_payon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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
    if (((((ctx.call(Function::CountItem, vec![Val::from(7730)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7733)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7734)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7737)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7738)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7741)])?.number()? > 0)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "You must be confused...",
                "It's not time for you",
                "to see me just yet.",
                "Have you tried meeting",
                "with any of the other guys?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_relaytime = ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?;
    if ctx.var("party_relay").get()? == 30 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Did you give that ticket",
                "to an Archer or Merchant",
                "Class member of your group?",
                "Just a reminder in case",
                "you've already forgotten."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((((ctx.var("party_relay").get()? == 29 && l_relaytime.clone().number()? >= 0) && l_relaytime.clone().number()? < 3)
        && ctx.call(Function::CountItem, vec![Val::from(1012)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(1048)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(7003)])?.number()? > 19)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1012), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(1048), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(7003), Val::from(20)])?;
        ctx.var("party_relay").set(Val::from(30))?;
        ctx.call(Function::GetItem, vec![Val::from(7740), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Make sure that you give",
                "that ticket to an Archer",
                "or Merchant Class member",
                "of your group, and have him",
                "bring it to a boy named Lospii.",
                "Goodbye now, and good luck~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("party_relay").get()? == 29 && l_relaytime.clone().number()? >= 11) && l_relaytime.clone().number()? < 14)
        && ctx.call(Function::CountItem, vec![Val::from(1012)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(1048)])?.number()? > 19)
        && ctx.call(Function::CountItem, vec![Val::from(7003)])?.number()? > 19)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1012), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(1048), Val::from(20)])?;
        ctx.call(Function::DelItem, vec![Val::from(7003), Val::from(20)])?;
        ctx.var("party_relay").set(Val::from(30))?;
        ctx.call(Function::GetItem, vec![Val::from(7740), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Make sure that you give",
                "that ticket to an Archer",
                "or Merchant Class member",
                "of your group, and have him",
                "bring it to a boy named Lospii.",
                "Goodbye now, and good luck~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 29 {
        ctx.lines_as(
            "Bafhail",
            args![
                "My mission for you is",
                "to bring me some items",
                "at the right time. I'll only",
                "accept your items for 3 hours",
                "once the day starts, and for 3",
                "hours in the middle of the day."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "I need",
                "^4D4DFF20 Frills^000000,",
                "^4D4DFF20 Anolian Skins^000000, and",
                "^4D4DFF20 Horrendous Hairs^000000.",
                "Don't forget now, alright?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7739)])?.number()? > 0 && l_relaytime.clone().number()? >= 11)
        && l_relaytime.clone().number()? < 14)
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~",
                "Now, let's get to business."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "My mission is",
                "for you to collect",
                "^4D4DFF20 Frills^000000,",
                "^4D4DFF20 Anolian Skins^000000, and",
                "^4D4DFF20 Horrendous Hairs^000000."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7739), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(29))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Understood?",
                "Alright then, if you'll",
                "just hand me your ticket,",
                "you can start gathering",
                "those items I listed.",
                "I'll see you later~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7739)])?.number()? > 0 && l_relaytime.clone().number()? >= 0)
        && l_relaytime.clone().number()? < 3)
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~",
                "Now, let's get to business."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "My mission is",
                "for you to collect",
                "^4D4DFF20 Frills^000000,",
                "^4D4DFF20 Anolian Skins^000000, and",
                "^4D4DFF20 Horrendous Hairs^000000."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7739), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(29))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Understood?",
                "Understood?",
                "Alright then, if you'll",
                "just hand me your ticket,",
                "you can start gathering",
                "those items I listed.",
                "I'll see you later~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7739)])?.number()? > 0
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Would you mind coming ",
                "back to me later? Now",
                "isn't the right time to talk",
                "to me. I'm available for 3",
                "hours at the start of the day,",
                "and 3 hours in midday."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 20 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Did you give that ticket",
                "to an Archer or Merchant",
                "Class member of your group?",
                "Just a reminder in case",
                "you've already forgotten."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((((ctx.var("party_relay").get()? == 19 && l_relaytime.clone().number()? >= 14) && l_relaytime.clone().number()? < 17)
        && ctx.call(Function::CountItem, vec![Val::from(1015)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7172)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7155)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1015), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7172), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7155), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(20))?;
        ctx.call(Function::GetItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Do me a favor and make",
                "sure that the Merchant",
                "Class character in your",
                "group delivers that ticket",
                "to Lospii. Alright, I guess",
                "I'll see you later, then."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("party_relay").get()? == 19 && l_relaytime.clone().number()? >= 18) && l_relaytime.clone().number()? < 21)
        && ctx.call(Function::CountItem, vec![Val::from(1015)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7172)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7155)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1015), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7172), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7155), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(20))?;
        ctx.call(Function::GetItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Do me a favor and make",
                "sure that the Merchant",
                "Class character in your",
                "group delivers that ticket",
                "to Lospii. Alright, I guess",
                "I'll see you later, then."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 19 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember to bring me",
                "the items I requested",
                "at the right time, during",
                "the 3 hours after noon or",
                "3 hours in the early evening.",
                "Now, I want you to bring..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Tongues^000000,",
                "^4D4DFF10 Leopard Claws^000000, and",
                "^4D4DFF10 Poisonous Toad Skins^000000.",
                "I'll be waiting for you here",
                "and will accept those items",
                "when the time is right."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((((ctx.var("party_relay").get()? == 18 && l_relaytime.clone().number()? >= 14) && l_relaytime.clone().number()? < 17)
        && ctx.call(Function::CountItem, vec![Val::from(7157)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(1021)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7101)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7157), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1021), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7101), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(20))?;
        ctx.call(Function::GetItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Do me a favor and make",
                "sure that the Merchant",
                "Class character in your",
                "group delivers that ticket",
                "to Lospii. Alright, I guess",
                "I'll see you later, then."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("party_relay").get()? == 18 && l_relaytime.clone().number()? >= 18) && l_relaytime.clone().number()? < 21)
        && ctx.call(Function::CountItem, vec![Val::from(7157)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(1021)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7101)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7157), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1021), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7101), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(20))?;
        ctx.call(Function::GetItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Do me a favor and make",
                "sure that the Merchant",
                "Class character in your",
                "group delivers that ticket",
                "to Lospii. Alright, I guess",
                "I'll see you later, then."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 18 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember that your",
                "mission from me is",
                "to collect some items,",
                "and to bring them to",
                "me at the right time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "I'll only accept your",
                "items for 3 hours after",
                "the day starts, and for",
                "3 hours during midday.",
                "These are the items",
                "that I want you to bring..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Dark Masks^000000,",
                "^4D4DFF10 Dokebi Horns^000000, and",
                "^4D4DFF10 PecoPeco Feathers^000000.",
                "I'll be here waiting for",
                "you during the hours",
                "I just described, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7735)])?.number()? > 0 && l_relaytime.clone().number()? >= 14)
        && l_relaytime.clone().number()? < 17)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Alright, let's get down",
                "to what you came for.",
                "My mission is for you to",
                "collect some items, and",
                "for you to bring them at",
                "the right time. Bring me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Dark Masks^000000,",
                "^4D4DFF10 Dokebi Horns^000000, and",
                "^4D4DFF10 PecoPeco Feathers^000000."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7735), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(18))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Understood?",
                "Alright then, if you'll",
                "just hand me your ticket,",
                "you can start gathering",
                "those items I listed.",
                "I'll see you later~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7735)])?.number()? > 0 && l_relaytime.clone().number()? >= 18)
        && l_relaytime.clone().number()? < 21)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Alright, let's get down",
                "to what you came for.",
                "My mission is for you to",
                "collect some items, and",
                "for you to bring them at",
                "the right time. Bring me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Tongues^000000,",
                "^4D4DFF10 Leopard Claws^000000, and",
                "^4D4DFF10 Poisonous Toad Skins^000000."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7735), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(19))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Understood?",
                "Alright then, if you'll",
                "just hand me your ticket,",
                "you can start gathering",
                "those items I listed.",
                "I'll see you later~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7735)])?.number()? > 0
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Would you mind coming ",
                "back to me later? Now",
                "isn't the right time to talk",
                "to me. I'm available for 3",
                "hours at the start of the day,",
                "and 3 hours in midday."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 9 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, did you already",
                "deliver the ticket that",
                "I gave you? If that's the",
                "case, we won't have any",
                "business with each other",
                "for a while, I suppose."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("party_relay").get()? == 8 && (l_relaytime.clone().number()? >= 8 && l_relaytime.clone().number()? < 11))
        && ctx.call(Function::CountItem, vec![Val::from(7196)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(7184)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(920)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7196), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7189), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(920), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(9))?;
        ctx.call(Function::GetItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Please give that",
                "ticket to the Archer",
                "Class member of your",
                "group, and have him",
                "deliver it to Lospii, okay?",
                "Good luck on your travels~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("party_relay").get()? == 8
        && (((l_relaytime.clone().number()? >= 21 && ctx.call(Function::CountItem, vec![Val::from(7196)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(7184)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(920)])?.number()? > 9))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7196), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(7189), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(920), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(9))?;
        ctx.call(Function::GetItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Please give that",
                "ticket to the Archer",
                "Class member of your",
                "group, and have him",
                "deliver it to Lospii, okay?",
                "Good luck on your travels~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 8 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Your mission",
                "is to bring me",
                "^4D4DFF10 Wooden Hearts^000000,",
                "^4D4DFF10 Wolf Claws^000000, and",
                "^4D4DFF10 Shoulder Protectors^000000",
                "during hours I accept items."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember that I only",
                "accept items from 8 AM",
                "to 11 AM, and from 9 PM",
                "to 12 AM. That's Pacific",
                "Standard Time. Don't forget!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("party_relay").get()? == 7 && (l_relaytime.clone().number()? >= 8 && l_relaytime.clone().number()? < 11))
        && ctx.call(Function::CountItem, vec![Val::from(1027)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(1040)])?.number()? > 9)
        && ctx.call(Function::CountItem, vec![Val::from(1023)])?.number()? > 9)
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1027), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1040), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1023), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(9))?;
        ctx.call(Function::GetItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Please give that",
                "ticket to the Archer",
                "Class member of your",
                "group, and have him",
                "deliver it to Lospii, okay?",
                "Good luck on your travels~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("party_relay").get()? == 7
        && (((l_relaytime.clone().number()? >= 21 && ctx.call(Function::CountItem, vec![Val::from(1027)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(1040)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(1023)])?.number()? > 9))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Well, it looks like you came",
                "at the right time and brought",
                "everything I asked. Good work.",
                "Please take your next ticket",
                "and this small reward for you."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(1027), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1040), Val::from(10)])?;
        ctx.call(Function::DelItem, vec![Val::from(1023), Val::from(10)])?;
        ctx.var("party_relay").set(Val::from(9))?;
        ctx.call(Function::GetItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Please give that",
                "ticket to the Archer",
                "Class member of your",
                "group, and have him",
                "deliver it to Lospii, okay?",
                "Good luck on your travels~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 7 {
        ctx.lines_as(
            "Bafhail",
            args![
                "Your mission",
                "is to bring me",
                "^4D4DFF10 Fish Tails^000000,",
                "^4D4DFF10 Porcupine Quills^000000, and",
                "^4D4DFF10 Elder Pixie's Moustaches^000000",
                "during hours I accept items."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember that I only",
                "accept items from 8 AM",
                "to 11 AM, and from 9 PM",
                "to 12 AM. That's Pacific",
                "Standard Time. Don't forget!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7731)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && (l_relaytime.clone().number()? >= 8 && l_relaytime.clone().number()? < 11))
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Alright, let's get down",
                "to what you came for.",
                "My mission is for you to",
                "collect some items, and",
                "for you to bring them at",
                "the right time. Bring me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Wooden Hearts^000000,",
                "^4D4DFF10 Wolf Claws^000000, and",
                "^4D4DFF10 Shoulder Protectors^000000"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7731), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(8))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember that I won't",
                "accept your items, even",
                "if you have everything,",
                "if you don't come during",
                "the hours when I accept",
                "items for missions, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.call(Function::CountItem, vec![Val::from(7731)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && (l_relaytime.clone().number()? >= 21 && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Alright, let's get down",
                "to what you came for.",
                "My mission is for you to",
                "collect some items, and",
                "for you to bring them at",
                "the right time. Bring me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "^4D4DFF10 Fish Tails^000000,",
                "^4D4DFF10 Porcupine Quills^000000, and",
                "^4D4DFF10 Elder Pixie's Moustaches^000000"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7731), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(7))?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Remember that I won't",
                "accept your items, even",
                "if you have everything,",
                "if you don't come during",
                "the hours when I accept",
                "items for missions, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.call(Function::CountItem, vec![Val::from(7731)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?))
    {
        ctx.lines_as(
            "Bafhail",
            args![
                "Oh, isn't that ticket...?",
                "Ah, good, good. Judging",
                "from your Job, Gatan must",
                "have sent you, right? Nice",
                "to meet you, I'm Bafhail~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bafhail",
            args![
                "Would you mind coming",
                "back to me later? Now",
                "isn't the right time to",
                "talk to me. I'm available",
                "from 8 AM to 11 AM, and",
                "from 9 PM to 12 AM."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Bafhail",
        args![
            "...Hm? I don't think",
            "we have any business",
            "with each other. You mind",
            "leaving me alone? I'm pretty",
            "busy administering a bunch",
            "of these missions for a friend."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bafhail",
        args![
            "If we're lucky, maybe",
            "you'll be one of those",
            "adventurers that'll carry",
            "out these fun little missions",
            "for my friend Ledrion."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bafhail_payon(ctx: &Ctx) -> Script {
    bafhail_payon_body(ctx, Vec::new()).map(|_| ())
}

fn lospii_payon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_juwi = Val::from(0);
    let mut l_m_s = Val::from("");
    let mut l_relaytime = Val::from(0);
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
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
    if (((((ctx.call(Function::CountItem, vec![Val::from(7730)])?.number()? > 0
        || ctx.call(Function::CountItem, vec![Val::from(7731)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7734)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7735)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7738)])?.number()? > 0)
        || ctx.call(Function::CountItem, vec![Val::from(7739)])?.number()? > 0)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hey, didn't you figure",
                "out what you need to do?",
                "You're not supposed to",
                "talk to me now! Mmm...",
                "I don't know who you",
                "have to meet either, but..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_relaytime = ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?;
    let position = ctx
        .call(
            Function::GetMapXy,
            vec![ctx.constant("BL_NPC")?, ctx.call(Function::StrNpcInfo, vec![Val::from(3)])?],
        )?
        .into_array()
        .ok_or_else(|| Stop::Error("Invalid position".into()))?;
    l_m_s = position[0].clone();
    l_x = position[1].clone();
    l_y = position[2].clone();
    l_juwi = ctx.call(
        Function::GetAreaUsers,
        vec![
            l_m_s.clone(),
            (l_x.clone().try_sub(Val::from(8))?),
            (l_y.clone().try_sub(Val::from(8))?),
            (l_x.clone() + Val::from(8)),
            (l_y.clone() + Val::from(8)),
        ],
    )?;
    if ctx.var("party_relay").get()? == 32 {
        ctx.lines_as(
            "Lospii",
            args![
                "Hey! Did you give",
                "that ticket to the",
                "leader of your group",
                "yet? The guy that started",
                "this whole relay thing!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 31 && (l_relaytime.clone().number()? >= 8 && l_relaytime.clone().number()? < 11))
        && l_juwi.clone().number()? > 13)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(32))?;
        ctx.call(Function::GetItem, vec![Val::from(7741), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("party_relay").get()? == 31 && (l_relaytime.clone().number()? >= 21 && l_juwi.clone().number()? > 13)) {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(32))?;
        ctx.call(Function::GetItem, vec![Val::from(7741), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 31 {
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, you have to bring",
                "me 10 of your friends when",
                "I'm working, okay? That's, uh,",
                "3 hours before noon, and then",
                "another 3 hours after midnight.",
                "Just only those times, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7740)])?.number()? > 0 && l_relaytime.clone().number()? >= 8)
        && l_relaytime.clone().number()? < 11)
        && (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "14 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7740), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(31))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.call(Function::CountItem, vec![Val::from(7740)])?.number()? > 0 && l_relaytime.clone().number()? >= 21)
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "14 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7740), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(31))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7740)])?.number()? > 0
        && (ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?)
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Uh oh... I forgot",
                "that I'm not supposed",
                "to be working now. Come",
                "back when I'm working, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I... I work before",
                "noon for 3 hours, and",
                "then... 3 hours after",
                "midnight? Yes, those",
                "are the times when",
                "I'm working!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 22 {
        ctx.lines_as(
            "Lospii",
            args![
                "Huh? No, no, you're",
                "not supposed to be here!",
                "Give the eighth ticket to",
                "your leader so he can give",
                "it to Ledrion! Your leader...",
                "You know him, right?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 21 && (l_relaytime.clone().number()? >= 11 && l_relaytime.clone().number()? < 14))
        && l_juwi.clone().number()? > 11)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(22))?;
        ctx.call(Function::GetItem, vec![Val::from(7737), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 21 && (l_relaytime.clone().number()? >= 0 && l_relaytime.clone().number()? < 3))
        && l_juwi.clone().number()? > 11)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(22))?;
        ctx.call(Function::GetItem, vec![Val::from(7737), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(610), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 21 {
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, you have to bring",
                "me 12 of your friends when",
                "I'm working, okay? That's, uh,",
                "3 hours before noon, and then",
                "another 3 hours after midnight.",
                "Just only those times, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7736)])?.number()? > 0 && l_relaytime.clone().number()? >= 11)
        && l_relaytime.clone().number()? < 14)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "12 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(21))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7736)])?.number()? > 0 && l_relaytime.clone().number()? >= 0)
        && l_relaytime.clone().number()? < 3)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "12 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7736), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(21))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7736)])?.number()? > 0
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Uh oh... I forgot",
                "that I'm not supposed",
                "to be working now. Come",
                "back when I'm working, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I... I work before",
                "noon for 3 hours, and",
                "then... 3 hours after",
                "midnight? Yes, those",
                "are the times when",
                "I'm working!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 11 {
        ctx.lines_as(
            "Lospii",
            args![
                "Umm... Did you bring",
                "the fourth ticket to your",
                "leader yet? You know,",
                "the leader of the group.",
                "The one that started this",
                "whole relay test thingee."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.var("party_relay").get()? == 10 && (l_relaytime.clone().number()? >= 14 && l_relaytime.clone().number()? < 17))
        && l_juwi.clone().number()? > 9)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(11))?;
        ctx.call(Function::GetItem, vec![Val::from(7733), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("party_relay").get()? == 10 && (l_relaytime.clone().number()? >= 18 && l_relaytime.clone().number()? < 21))
        && l_juwi.clone().number()? > 9)
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Oh! You brought so many",
                "friends! One... T-two...",
                ((Val::from("You brought ") + l_juwi.clone()) + Val::from("?! Hmpf.")),
                "Maybe I gave you something",
                "too easy to do. I didn't know",
                "you knew this many people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Oh well, thanks for",
                "bringing everyone here.",
                "For that, you get this",
                "gift from me. Cool, huh?"
            ],
        )?;
        ctx.next()?;
        ctx.var("party_relay").set(Val::from(11))?;
        ctx.call(Function::GetItem, vec![Val::from(7733), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(2)])?;
        shared::quests_partyrelay::f_partyrelay_exp(ctx, vec![])?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, now take this",
                "ticket, and give it to",
                "your leader, the guy that",
                "started this whole relay",
                "thing. He needs to give",
                "the ticket to Ledrion, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "You're almost done!",
                "...I think. Um, I'm not",
                "really sure what will",
                "happen next. I can only",
                "remember the parts I have",
                "to do. C'mon! I'm just a kid!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("party_relay").get()? == 10 {
        ctx.lines_as(
            "Lospii",
            args![
                "Bring 10 of your friends",
                "here to me while I'm working,",
                "okay? Umm... I work frooom...",
                "2 PM to 5 PM, and 6 PM to 9 PM.",
                "I think those are the times.",
                "I... I can't read watches..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7732)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && (l_relaytime.clone().number()? >= 14 && l_relaytime.clone().number()? < 17))
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "10 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(10))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7732)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && (l_relaytime.clone().number()? >= 18 && l_relaytime.clone().number()? < 21))
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I'm working now for this",
                "mission thing so you came",
                "at a good time! Let's see...",
                "I need to give you... Some",
                "mission for you to doooo...",
                "Oh! I know! I got it!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "We're seeing if you",
                "know about teamwork, right?",
                "Why don't you show me a lot",
                "of your friends? Bring me...",
                "10 of them! Yes, that's good!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I need to be able to",
                "see them, you know, so",
                "I know you're not lying",
                "to me. Bring them reeeally",
                "close so I know they're your",
                "friends, not some other guys."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7732), Val::from(1)])?;
        ctx.var("party_relay").set(Val::from(10))?;
        ctx.lines_as(
            "Lospii",
            args![
                "Okay, I'll be right",
                "here! Oh! And you have",
                "to come with your friends",
                "while I'm at work! That's",
                "important to know!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.call(Function::CountItem, vec![Val::from(7732)])?.number()? > 0 && ctx.var("BaseLevel").get()?.number()? > 39)
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ARCHER")?))
    {
        ctx.lines_as(
            "Lospii",
            args![
                "Hi, I'm Lospii! Maybe",
                "I'm younger than you, but",
                "I'm in charge of this mission!",
                "So you have to listen, okay?",
                "Heh heh! Don't be scared~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "Uh oh... I forgot",
                "that I'm not supposed",
                "to be working now. Come",
                "back when I'm working, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lospii",
            args![
                "I work frooom... Uh...",
                "2 PM to 5 PM? Oh!",
                "And also 6 PM to 9 PM.",
                "I think those are the",
                "times... I mean, clocks",
                "are hard to read, man!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Lospii",
        args![
            "Hi! I'm Lospii!",
            "Hey, does it look like",
            "I'm at work? Ha ha!",
            "I got a job! I'm helping",
            "my friends a lot!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lospii_payon(ctx: &Ctx) -> Script {
    lospii_payon_body(ctx, Vec::new()).map(|_| ())
}
