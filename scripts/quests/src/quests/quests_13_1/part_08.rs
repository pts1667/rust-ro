use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn staff_officer_abidal_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_ep13_edq_wrong = Val::from(0);
    if (ctx.var("ep13_1_edq").get()? == 13 || ctx.var("ep13_1_edq").get()? == 14) {
        ctx.lines_as(
            "Staff Officer Abidal",
            args!["Phew, I'm glad the mission is over. Thank you so much for your help."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff Officer Abidal",
            args![
                "We'll contact you again when we have another mission for you. I hope we can count on your help again when the time comes."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ep13_1_edq").get()? == 0 {
            ctx.lines_as(
                "Staff Officer Abidal",
                args!["I'm Staff Officer Abidal of the Midgard Expedition. How may I help you?"],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Midgard Expedition Introduction:Midgard Expedition's Achievement:Situation in the Ash-Vacuum:Quit",
                )],
            )? {
                1 => {
                    ctx.lines_as("Staff Officer Abidal", args!["The Midgard Expedition is conducted by a group of explorers from the three countries of Midgard and the Mercenary Association. We're here to explore the Ash-Vacuum."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["Command Hibba Agip behind me is our commander. He may appear unreliable, but he has extensive knowledge and experience in exploration."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Staff Officer Abidal",
                        args!["We have many aides and scholars to support our expedition."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["Oh, you may know this already, but Commander Agip isn't from any of the three countries. Neither am I or Instructor Igrid."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["There's an obvious power struggle among the three countries to assign their officers to the commander position."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["As a last resort, they had to select Commander Agip since he's unaffiliated with any of those countries, but is talented enough to lead the expedition."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["We, the commanders, Igrid, and I aren't used to the leadership role. We're having trouble taking care of everything that's happening. *Sigh*"])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["The members of the Midgard Expedition are conducting various kinds of research about the Ash-Vacuum searching for possible living creatures and studying what might be beneficial for the development of the mainland."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["If you look around, you'll find many people in need of your help. Enjoy your stay in the otherworld, and feel free to ask me if you have questions."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Staff Officer Abidal", args!["We're still at an early stage of the expedition, so we haven't made any major achievements. That's why we need help from adventurers like you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Staff Officer Abidal",
                        args![
                            "If you look around this camp, you'll find many opportunities to help us. We always welcome every bit of help."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Staff Officer Abidal",
                        args!["So far, only a few truths about the Ash-Vacuum have been revealed."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["We now know there's a demihuman race in this otherworld, but we're too busy figuring out the true nature of the time-space gap, let alone study the demihuman race in depth."])?;
                    ctx.next()?;
                    ctx.lines_as("Staff Officer Abidal", args!["My advice for you will be to be careful when you encounter members of that race. Who knows? They might be extremely hostile toward other races."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Staff Officer Abidal",
                        args![
                            "Oh, and take heed, unidentified creatures appear outside the camp during nighttime. Be careful of them, too."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            if ctx.var("ep13_1_edq").get()? == 1 {
                ctx.lines_as(
                    "Staff Officer Abidal",
                    args![
                        "Oh, welcome back. I apologize for troubling you like this. I didn't feel to comfortable talking to you back there."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff Officer Abidal",
                    args![
                        "In fact, I have a favor to ask of you. From what I know of your accomplishments you're the right person for this."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff Officer Abidal",
                    args!["On behalf of the expedition, I'd like to give you a mission. Would you like to accept it?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
                    1 => {
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["Thank you so much. I had a good feeling about you, and I was so sure that you'd be willing to help."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["Let's cut to the chase. The mission is simple, deliver our expedition report to the three countries of Midgard."])?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["But I have to tell you... We sent out three adventurers on this mission before you, and they all dissapeared. Please don't disclose this information to anyone."])?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["The commander or I cannot leave this camp, even though delivering the report is one of our most important duties. That's why I need someone reliable and trustworthy like you."])?;
                        ctx.next()?;
                        ctx.lines_as("Staff Officer Abidal", args!["Phew, I'm so glad that you've accepted this mission. Now, please go receive the report from the commander. He'll officially assign you to the mission."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["I'll prepare for your departure.", "Please go speak to the commander."],
                        )?;
                        ctx.var("ep13_1_edq").set(Val::from(2))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(3085), Val::from(3086)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["Oh, I see... Then can you help me whenever you have some time?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["This is urgent, please think it over and come back if you change your mind."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("ep13_1_edq").get()? == 2 {
                    ctx.lines_as(
                        "Staff Officer Abidal",
                        args![
                            "I've reported to the commander about your decision to help us.",
                            "Please go speak to him."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ep13_1_edq").get()? == 3 {
                        ctx.lines_as(
                            "Staff Officer Abidal",
                            args!["You've been assinged to the mission officially. Good luck, and I hope you'll come back safely."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ep13_1_edq").get()? == 4 || ctx.var("ep13_1_edq").get()? == 5) {
                            ctx.lines_as(
                                "Staff Officer Abidal",
                                args!["I've heard the story. Why don't you go speak to the commander first?"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ep13_1_edq").get()? == 61 {
                                ctx.lines_as("Staff Officer Abidal", args!["It's as what the commander said. I still remember the contents of the report, even though it might not all be accurate without any references."])?;
                                ctx.next()?;
                                ctx.lines_as("Staff Officer Abidal", args!["If you bring me the pages of the lost report, I'll fill the missing parts and be able to complete the report again."])?;
                                ctx.next()?;
                                ctx.lines_as("Staff Officer Abidal", args!["Why don't you go check the site of the accident? You might find some clues of their whereabouts. and..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Staff Officer Abidal",
                                    args!["This is my opinion, but otherworldly creatures might have taken an interest in the pages."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Staff Officer Abidal", args!["I'm thinking this because the report contains information about the ecology of the otherworldly creatures. It's also made of paper and ink, materials from the mainland that might seem interesting to those creatures."])?;
                                ctx.next()?;
                                ctx.lines_as("Staff Officer Abidal", args!["Anyways, please bring the pages of the report in ^0000FFincrements of 10^000000 whenever you find them. I'll try to put them in order."])?;
                                ctx.var("ep13_1_edq").set(Val::from(71))?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(3089), Val::from(3090)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("ep13_1_edq").get()? == 62 {
                                    ctx.lines_as("Staff Officer Abidal", args!["It's as what the commander said. I still remember the contents of the report, even though it might not all be accurate without any references."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Staff Officer Abidal", args!["If you bring me the pages of the lost report, I'll fill the missing parts and be able to complete the report again."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Staff Officer Abidal", args!["Why don't you go check the site of the accident? You might find some clues of their whereabouts. and..."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Staff Officer Abidal",
                                        args!["This is my opinion, but otherworldly creatures might have taken an interest in the pages."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Staff Officer Abidal", args!["I'm thinking this because the report contains information about the ecology of the otherworldly creatures. It's also made of paper and ink, materials from the mainland that might seem interesting to those creatures."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Staff Officer Abidal", args!["Anyways, please bring the pages of the report in ^0000FFincrements of 10^000000 whenever you find them. I'll try to put them in order."])?;
                                    ctx.var("ep13_1_edq").set(Val::from(72))?;
                                    ctx.call(Function::ChangeQuest, vec![Val::from(3089), Val::from(3090)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72) {
                                        ctx.lines_as(
                                            "Staff Officer Abidal",
                                            args![
                                                "How have you been doing with finding the report's lost pages?",
                                                "Feel free to tell me whenever you're ready."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        'b3: {
                                            let subject3 = Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("Submit found pages.:Check the report's resoration status.:Quit.")],
                                            )?);
                                            let mut matched3 = false;
                                            let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                                && !subject3.loosely_equals(&Val::from(2))
                                                && !subject3.loosely_equals(&Val::from(3));
                                            if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                                matched3 = true;
                                            }
                                            if matched3 {
                                                if (((ctx.call(Function::CountItem, vec![Val::from(11013)])?.number()? > 0
                                                    && ctx.call(Function::CountItem, vec![Val::from(11014)])?.number()? > 0)
                                                    && ctx.call(Function::CountItem, vec![Val::from(11015)])?.number()? > 0)
                                                    && ctx.call(Function::CountItem, vec![Val::from(11016)])?.number()? > 0)
                                                {
                                                    ctx.lines_as("Staff Officer Abidal", args!["Oh, you've collected enough pages..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Staff Officer Abidal",
                                                        args!["Why don't you check the report's restoration status again?"],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as("Staff Officer Abidal", args!["Have you brought me pages of the lost report? Please bring them to me in increments of 10."])?;
                                                    ctx.next()?;
                                                    if ctx.call(Function::CountItem, vec![Val::from(6040)])?.number()? > 9 {
                                                        ctx.lines_as(
                                                            "Staff Officer Abidal",
                                                            args!["Thank you. Let me try to put them in order."],
                                                        )?;
                                                        ctx.next()?;
                                                        if ctx.var("ep13_1_edq").get()? == 71 {
                                                            ctx.lines_as(
                                                                "Staff Officer Abidal",
                                                                args!["Umm... This page should go here and.."],
                                                            )?;
                                                            ctx.next()?;
                                                            l_ep13_edq_wrong =
                                                                ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                                                            if (ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                                && l_ep13_edq_wrong.clone() == 1)
                                                            {
                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                ctx.next()?;
                                                                ctx.call(Function::DelItem, vec![Val::from(6040), Val::from(10)])?;
                                                                ctx.call(Function::GetItem, vec![Val::from(11013), Val::from(1)])?;
                                                                ctx.lines_as("Staff Officer Abidal", args!["This is the first volume of the report. I'm glad that we've restored atleast the first part."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Staff Officer Abidal",
                                                                    args!["Keep up the good work on finding the rest of the volumes."],
                                                                )?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if (ctx.call(Function::CountItem, vec![Val::from(11013)])? == 1
                                                                    && l_ep13_edq_wrong.clone() == 1)
                                                                {
                                                                    ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the first volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(6040), Val::from(10)])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if (ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0
                                                                        && (l_ep13_edq_wrong.clone() == 3 || l_ep13_edq_wrong.clone() == 2))
                                                                    {
                                                                        ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                        ctx.next()?;
                                                                        ctx.call(Function::DelItem, vec![Val::from(6040), Val::from(10)])?;
                                                                        ctx.call(Function::GetItem, vec![Val::from(11014), Val::from(1)])?;
                                                                        ctx.lines_as("Staff Officer Abidal", args!["This is the second volume of the report. I'm glad that we've restored atleast the second part."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as(
                                                                            "Staff Officer Abidal",
                                                                            args![
                                                                                "Keep up the good work on finding the rest of the volumes."
                                                                            ],
                                                                        )?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if (ctx.call(Function::CountItem, vec![Val::from(11014)])? == 1
                                                                            && (l_ep13_edq_wrong.clone() == 3
                                                                                || l_ep13_edq_wrong.clone() == 2))
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the second volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if (ctx.call(Function::CountItem, vec![Val::from(11015)])?
                                                                            == 0
                                                                            && (l_ep13_edq_wrong.clone() == 5
                                                                                || l_ep13_edq_wrong.clone() == 6))
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(11015), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["This is the third volume of the report. I'm glad that we've restored atleast the third part."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Keep up the good work on finding the rest of the volumes."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if (ctx.call(Function::CountItem, vec![Val::from(11015)])?
                                                                            == 1
                                                                            && (l_ep13_edq_wrong.clone() == 5
                                                                                || l_ep13_edq_wrong.clone() == 6))
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the third volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if (ctx.call(Function::CountItem, vec![Val::from(11016)])?
                                                                            == 0
                                                                            && l_ep13_edq_wrong.clone() == 7)
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(11016), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["This is the fourth volume of the report. I'm glad that we've restored atleast the fourth part."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Keep up the good work on finding the rest of the volumes."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else if (ctx.call(Function::CountItem, vec![Val::from(11016)])?
                                                                            == 1
                                                                            && l_ep13_edq_wrong.clone() == 7)
                                                                        {
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the fourth volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            ctx.mes("[Staff Officer Abidal]")?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.mes("I'm sorry, but you've brought pages from different volumes of the report, I was unable to put them in order.")?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["I'm sorry, but please go try to find pages that can be bound into one volume."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if ctx.var("ep13_1_edq").get()? == 72 {
                                                                ctx.lines_as(
                                                                    "Staff Officer Abidal",
                                                                    args!["Umm... This page should go here and.."],
                                                                )?;
                                                                ctx.next()?;
                                                                l_ep13_edq_wrong =
                                                                    ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                                                                if (ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                                    && l_ep13_edq_wrong.clone() == 1)
                                                                {
                                                                    ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                    ctx.next()?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(6040), Val::from(10)])?;
                                                                    ctx.call(Function::GetItem, vec![Val::from(11013), Val::from(1)])?;
                                                                    ctx.lines_as("Staff Officer Abidal", args!["This is the first volume of the report. I'm glad that we've restored atleast the first part."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Staff Officer Abidal",
                                                                        args!["Keep up the good work on finding the rest of the volumes."],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if (ctx.call(Function::CountItem, vec![Val::from(11013)])? == 1
                                                                        && l_ep13_edq_wrong.clone() == 1)
                                                                    {
                                                                        ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the first volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                        ctx.call(Function::DelItem, vec![Val::from(6040), Val::from(10)])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if (ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0
                                                                            && l_ep13_edq_wrong.clone() == 2)
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                            ctx.next()?;
                                                                            ctx.call(
                                                                                Function::DelItem,
                                                                                vec![Val::from(6040), Val::from(10)],
                                                                            )?;
                                                                            ctx.call(
                                                                                Function::GetItem,
                                                                                vec![Val::from(11014), Val::from(1)],
                                                                            )?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["This is the second volume of the report. I'm glad that we've restored atleast the second part."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Staff Officer Abidal", args!["Keep up the good work on finding the rest of the volumes."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            if (ctx.call(Function::CountItem, vec![Val::from(11014)])? == 1
                                                                                && l_ep13_edq_wrong.clone() == 2)
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the second volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else if (ctx
                                                                                .call(Function::CountItem, vec![Val::from(11015)])?
                                                                                == 0
                                                                                && (l_ep13_edq_wrong.clone() == 3
                                                                                    || l_ep13_edq_wrong.clone() == 4))
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(11015), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["This is the third volume of the report. I'm glad that we've restored atleast the third part."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Keep up the good work on finding the rest of the volumes."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else if (ctx
                                                                                .call(Function::CountItem, vec![Val::from(11015)])?
                                                                                == 1
                                                                                && (l_ep13_edq_wrong.clone() == 3
                                                                                    || l_ep13_edq_wrong.clone() == 4))
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the third volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else if (ctx
                                                                                .call(Function::CountItem, vec![Val::from(11016)])?
                                                                                == 0
                                                                                && l_ep13_edq_wrong.clone() == 5)
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.call(
                                                                                    Function::GetItem,
                                                                                    vec![Val::from(11016), Val::from(1)],
                                                                                )?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["This is the fourth volume of the report. I'm glad that we've restored atleast the fourth part."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Keep up the good work on finding the rest of the volumes."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else if (ctx
                                                                                .call(Function::CountItem, vec![Val::from(11016)])?
                                                                                == 1
                                                                                && l_ep13_edq_wrong.clone() == 5)
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Thank you for your hard work. We were able to restore one volume of the report."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["Unfortunately, this is the fourth volume of the report which we've restored already. I'm sorry, but we need to find pages for the rest of the volumes."])?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["I'm sorry, but you've brought pages from different volumes of the report, I was unable to put them in order."])?;
                                                                                ctx.next()?;
                                                                                ctx.call(
                                                                                    Function::DelItem,
                                                                                    vec![Val::from(6040), Val::from(10)],
                                                                                )?;
                                                                                ctx.lines_as("Staff Officer Abidal", args!["I'm sorry, but please go try to find pages that can be bound into one volume."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        ctx.lines_as("Staff Officer Abidal", args!["You didn't bring enough pages, please bring them to me in increments of 10."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                            }
                                            if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                                matched3 = true;
                                            }
                                            if matched3 {
                                                ctx.lines_as(
                                                    "Staff Officer Abidal",
                                                    args!["Let's see how far we've progressed on the report's restoration."],
                                                )?;
                                                if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 1
                                                    && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 1)
                                                    && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 1)
                                                    && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 1)
                                                {
                                                    ctx.next()?;
                                                    ctx.lines_as("Staff Officer Abidal", args!["Oh, great! Every volume of the report has been restored. I'll bind them into one book for you."])?;
                                                    ctx.next()?;
                                                    ctx.call(Function::DelItem, vec![Val::from(11013), Val::from(1)])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(11014), Val::from(1)])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(11015), Val::from(1)])?;
                                                    ctx.call(Function::DelItem, vec![Val::from(11016), Val::from(1)])?;
                                                    ctx.call(Function::GetItem, vec![Val::from(11012), Val::from(1)])?;
                                                    ctx.var("ep13_1_edq").set(Val::from(8))?;
                                                    if ctx.call(Function::IsBeginQuest, vec![Val::from(3090)])?.number()? > 0 {
                                                        ctx.call(Function::EraseQuest, vec![Val::from(3090)])?;
                                                    }
                                                    if ctx.call(Function::IsBeginQuest, vec![Val::from(3091)])?.number()? > 0 {
                                                        ctx.call(Function::EraseQuest, vec![Val::from(3091)])?;
                                                    }
                                                    ctx.call(Function::SetQuest, vec![Val::from(3092)])?;
                                                    ctx.lines_as("Staff Officer Abidal", args!["I'm glad that we were able to make this again. Please bring this report to the commander."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                        && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0)
                                                        && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 0)
                                                        && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 0)
                                                    {
                                                        ctx.lines_as("Staff Officer Abidal", args!["Well, you haven't restored any volumes of the report yet. Please try harder."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 1
                                                            && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0)
                                                            && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 0)
                                                            && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 0)
                                                        {
                                                            ctx.lines_as("Staff Officer Abidal", args!["You have restored the first volume of the report. There are 3 volumes left to restore."])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                                && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 1)
                                                                && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 0)
                                                                && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 0)
                                                            {
                                                                ctx.lines_as("Staff Officer Abidal", args!["You have restored the second volume of the report. There are 3 volumes left to restore."])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                                    && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0)
                                                                    && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 1)
                                                                    && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 0)
                                                                {
                                                                    ctx.lines_as("Staff Officer Abidal", args!["You have restored the third volume of the report. There are 3 volumes left to restore."])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 0
                                                                        && ctx.call(Function::CountItem, vec![Val::from(11014)])? == 0)
                                                                        && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 0)
                                                                        && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 1)
                                                                    {
                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the fourth volume of the report. There are 3 volumes left to restore."])?;
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    } else {
                                                                        if (((ctx.call(Function::CountItem, vec![Val::from(11013)])? == 1
                                                                            && ctx.call(Function::CountItem, vec![Val::from(11014)])?
                                                                                == 1)
                                                                            && ctx.call(Function::CountItem, vec![Val::from(11015)])? == 0)
                                                                            && ctx.call(Function::CountItem, vec![Val::from(11016)])? == 0)
                                                                        {
                                                                            ctx.lines_as("Staff Officer Abidal", args!["You have restored the first and second volume of the report. There are 2 volumes left to restore."])?;
                                                                            ctx.close_window()?;
                                                                            return Err(Stop::End);
                                                                        } else {
                                                                            if (((ctx
                                                                                .call(Function::CountItem, vec![Val::from(11013)])?
                                                                                == 1
                                                                                && ctx
                                                                                    .call(Function::CountItem, vec![Val::from(11014)])?
                                                                                    == 0)
                                                                                && ctx
                                                                                    .call(Function::CountItem, vec![Val::from(11015)])?
                                                                                    == 1)
                                                                                && ctx.call(Function::CountItem, vec![Val::from(11016)])?
                                                                                    == 0)
                                                                            {
                                                                                ctx.lines_as("Staff Officer Abidal", args!["You have restored the first and third volume of the report. There are 2 volumes left to restore."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            } else {
                                                                                if (((ctx
                                                                                    .call(Function::CountItem, vec![Val::from(11013)])?
                                                                                    == 1
                                                                                    && ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11014)],
                                                                                    )? == 0)
                                                                                    && ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11015)],
                                                                                    )? == 0)
                                                                                    && ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11016)],
                                                                                    )? == 1)
                                                                                {
                                                                                    ctx.lines_as("Staff Officer Abidal", args!["You have restored the first and fourth volume of the report. There are 2 volumes left to restore."])?;
                                                                                    ctx.close_window()?;
                                                                                    return Err(Stop::End);
                                                                                } else {
                                                                                    if (((ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11013)],
                                                                                    )? == 0
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11014)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11015)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11016)],
                                                                                        )? == 0)
                                                                                    {
                                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the second and third volume of the report. There are 2 volumes left to restore."])?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if (((ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11013)],
                                                                                    )? == 0
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11014)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11015)],
                                                                                        )? == 0)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11016)],
                                                                                        )? == 1)
                                                                                    {
                                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the second and fourth volume of the report. There are 2 volumes left to restore."])?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if (((ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11013)],
                                                                                    )? == 0
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11014)],
                                                                                        )? == 0)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11015)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11016)],
                                                                                        )? == 1)
                                                                                    {
                                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the third and fourth volume of the report. There are 2 volumes left to restore."])?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if (((ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11013)],
                                                                                    )? == 1
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11014)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11015)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11016)],
                                                                                        )? == 0)
                                                                                    {
                                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the first, second and third volume of the report. There is 1 volume left to restore."])?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else if (((ctx.call(
                                                                                        Function::CountItem,
                                                                                        vec![Val::from(11013)],
                                                                                    )? == 0
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11014)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11015)],
                                                                                        )? == 1)
                                                                                        && ctx.call(
                                                                                            Function::CountItem,
                                                                                            vec![Val::from(11016)],
                                                                                        )? == 1)
                                                                                    {
                                                                                        ctx.lines_as("Staff Officer Abidal", args!["You have restored the second, third and fourth volume of the report. There is 1 volume left to restore."])?;
                                                                                        ctx.close_window()?;
                                                                                        return Err(Stop::End);
                                                                                    } else {
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
                                            if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                                matched3 = true;
                                            }
                                            if matched3 {
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else if (ctx.var("ep13_1_edq").get()? == 8
                                        || (ctx.var("ep13_1_edq").get()?.number()? > 8 && ctx.var("ep13_1_edq").get()?.number()? < 13))
                                    {
                                        ctx.lines_as("Staff Officer Abidal", args!["Phew, I'm glad we can resume the mission again."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Staff Officer Abidal", args!["...Aren't you leaving? I think you should."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("ins_nyd").get()?.number()? > 0 {
                                        ctx.lines_as(
                                            "Staff Officer Abidal",
                                            args!["I heard that you got a new duty from Commander Agip."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Staff Officer Abidal", args!["...Aren't you leaving? I think you should."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Staff Officer Abidal",
                                            args!["Hello, I'm Staff Officer Abidal. Safe travels, Adventurer."],
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
    Ok(Val::from(0))
}

pub fn staff_officer_abidal(ctx: &Ctx) -> Script {
    staff_officer_abidal_body(ctx, Vec::new()).map(|_| ())
}

fn instructor_igrid_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ep13_1_edq").get()? == 13 || ctx.var("ep13_1_edq").get()? == 14) {
        ctx.lines_as(
            "Instructor Igrid",
            args!["Hey, rookies. I guess you were lucky enough to complete the mission."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args!["This is not the end. Keep yourself in one piece until we need your services again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_1_edq").get()? == 0 {
        ctx.lines_as(
            "Instructor Igrid",
            args!["I'm Instructor Igrid of the Midgard Expedition. I'm in charge of training and commanding soldiers."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args![
                "You must want to become an official member of our expedition.",
                "I'm sorry, but you're too weak to endure my intensive training.",
                "Give up before you get hurt."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Instructor Igrid", args!["If you have too much time on your hands, look around. There's plenty of chores available for so-called adventurers's like you.", "If you do your best to do them, I might accept you as a trainee on this expedition."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ep13_1_edq").get()?.number()? > 0 && ctx.var("ep13_1_edq").get()?.number()? < 4) {
        ctx.lines_as(
            "Instructor Igrid",
            args![
                "I don't trust you. Hm. Sorry. I don't really have the right to say that if the commander has assigned you to the mission."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Instructor Igrid",
            args!["If you're going to give up, give up now. That'll be better for both of us."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ep13_1_edq").get()? == 4 || ctx.var("ep13_1_edq").get()? == 5) {
        ctx.lines_as(
            "Instructor Igrid",
            args!["I knew you couldn't do it. Didn't I tell you to give up when you had the chance?!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_1_edq").get()?.number()? > 5 {
        ctx.lines_as(
            "Instructor Igrid",
            args!["Do you think I'm doing this for you? No, I'm just using this chance to discipline idle soldiers."],
        )?;
        ctx.next()?;
        ctx.lines_as("Instructor Igrid", args!["What are you looking at? Go mind your own business!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Instructor Igrid",
            args!["I am Expedition Training Instructor Igrid, and the commander responsible for training soldiers. "],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn instructor_igrid(ctx: &Ctx) -> Script {
    instructor_igrid_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ExpeditionMessengerStep {
    Start,
    OnInit,
}

fn expedition_messenger_run(ctx: &Ctx, mut step: ExpeditionMessengerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExpeditionMessengerStep::Start => {
                if (ctx.var("ep13_1_edq").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(11012)])?.number()? > 0) {
                    ctx.lines_as(
                        "Expedition Messenger",
                        args![
                            "Welcome, I've been waiting for you.",
                            ((Val::from("You must be ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(". Nice to meet you."))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Expedition Messenger",
                        args![
                            "Let me check something quicly. You received the report directly from the commander, didn't you? May I see it?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["...", "......", "........."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Expedition Messenger",
                        args![
                            ((Val::from("Excellent. Thank you. Then we'll go back to the mainland and let the leaders know that you, ")
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", have departed."))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Expedition Messenger", args!["Guys, it's time to go."])?;
                    ctx.next()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::Ongo")])?;
                    ctx.lines_as(
                        "Expedition Messenger",
                        args![
                            "You, head to Schwaltzval Republic.",
                            "And you, take Arunafeltz. I'll visit Prontera Palace."
                        ],
                    )?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_OK")?,
                            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Expedition Messenger#2")])?,
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Expedition Messenger", args!["I'll see you guys later."])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnDisable")])?;
                    ctx.next()?;
                    ctx.lines_as("Expedition Messenger", args!["What's up? Why haven't you left...?"])?;
                    ctx.call(
                        Function::NpcSpecialEffect,
                        vec![
                            ctx.constant("EF_SOULBREAKER")?,
                            ctx.constant("AREA")?,
                            Val::from("Expedition Messenger"),
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Expedition Messenger", args!["Argh... Why are you doing this?!"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ep13_shadow_edq"), Val::from(2)])?;
                    ctx.lines_as("???", args!["......"])?;
                    ctx.next()?;
                    ctx.lines_as("???", args!["Give me the report."])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Give the report.:Protect the report.")])? {
                        1 => {
                            ctx.lines_as(
                                "Expedition Messenger",
                                args!["Wha... What are you doing? You can't give up the report...", "Awww..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("???", args!["Give it to me, now!"])?;
                            ctx.next()?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnEnable")])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_SURPRISE")?,
                                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Expedition Messenger#3")])?,
                                ],
                            )?;
                            ctx.call(Function::Cutin, vec![Val::from("ep13_shadow_edq"), Val::from(2)])?;
                            ctx.lines_as("???", args!["Argh..."])?;
                            ctx.next()?;
                            ctx.call(
                                Function::NpcSpecialEffect,
                                vec![
                                    ctx.constant("EF_SOULBREAKER")?,
                                    ctx.constant("AREA")?,
                                    Val::from("Expedition Messenger#3"),
                                ],
                            )?;
                            ctx.call(
                                Function::NpcSpecialEffect,
                                vec![
                                    ctx.constant("EF_SOULBREAKER")?,
                                    ctx.constant("AREA")?,
                                    Val::from("Expedition Messenger#3"),
                                ],
                            )?;
                            ctx.lines_as("???", args!["Argh... You..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    ((Val::from("*Cough* Oh, thank god! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", please take the report and run!"))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("???", args!["No... Noooo!"])?;
                            ctx.next()?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SOULBREAKER")?])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SONICBLOWHIT")?])?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    ((Val::from("Argh... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", are you alright? The report... The report..."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
                            ctx.var("ep13_1_edq").set(Val::from(4))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3087), Val::from(3088)])?;
                            ctx.lines_as("Expedition Agent", args!["We lost the report. The pages are blowing away!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "???",
                                args!["Haha, I may have failed to take the report, but it's better destroyed than in your hands!"],
                            )?;
                            ctx.call(Function::Cutin, vec![Val::from("ep13_shadow_edq"), Val::from(255)])?;
                            ctx.next()?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#3::OnDisable")])?;
                            ctx.lines_as("Expedition Agent", args!["Are you alright? What about the report... ?"])?;
                            ctx.next()?;
                            ctx.lines_as("Expedition Messenger", args!["*Cough* The report is out of our reach now..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Agent",
                                args!["If we mobilize all our troops to search this entire area..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    "We must report this to the commander before anything else.",
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", please report to the commander immidiatly. Let us take care of the rest."))
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnDisable")])?;
                            ctx.next()?;
                            ctx.lines_as("Expedition Agent", args!["Yes, please report to the commander..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("???", args!["Oh, yeah? Then I'll have to use force!"])?;
                            ctx.next()?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnEnable")])?;
                            ctx.call(
                                Function::Emotion,
                                vec![
                                    ctx.constant("ET_SURPRISE")?,
                                    ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Expedition Messenger#3")])?,
                                ],
                            )?;
                            ctx.lines_as("???", args!["Argh!"])?;
                            ctx.next()?;
                            ctx.call(
                                Function::NpcSpecialEffect,
                                vec![
                                    ctx.constant("EF_SOULBREAKER")?,
                                    ctx.constant("AREA")?,
                                    Val::from("Expedition Messenger#3"),
                                ],
                            )?;
                            ctx.call(
                                Function::NpcSpecialEffect,
                                vec![
                                    ctx.constant("EF_SOULBREAKER")?,
                                    ctx.constant("AREA")?,
                                    Val::from("Expedition Messenger#3"),
                                ],
                            )?;
                            ctx.lines_as("???", args!["Argh... You..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    ((Val::from("*Cough* Oh, thank god! ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", please take the report and run!"))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("???", args!["No... Noooo!"])?;
                            ctx.next()?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SOULBREAKER")?])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SONICBLOWHIT")?])?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    ((Val::from("Argh... ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", are you alright? The report... The report..."))
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Expedition Agent", args!["We lost the report. The pages are blowing away!"])?;
                            ctx.next()?;
                            ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
                            ctx.var("ep13_1_edq").set(Val::from(5))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(3087), Val::from(3088)])?;
                            ctx.lines_as(
                                "???",
                                args!["Haha, I may have failed to take the report, but it's better destroyed than in your hands!"],
                            )?;
                            ctx.call(Function::Cutin, vec![Val::from("ep13_shadow_edq"), Val::from(255)])?;
                            ctx.next()?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#3::OnDisable")])?;
                            ctx.lines_as("Expedition Agent", args!["Are you alright? What about the report... ?"])?;
                            ctx.next()?;
                            ctx.lines_as("Expedition Messenger", args!["*Cough* The report is out of our reach now..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Agent",
                                args!["If we mobilize all our troops to search this entire area..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Expedition Messenger",
                                args![
                                    "We must report this to the commander before anything else.",
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", please report to the commander immidiatly. Let us take care of the rest."))
                                ],
                            )?;
                            ctx.var("ep13_1_edq").set(Val::from(5))?;
                            ctx.next()?;
                            ctx.lines_as("Expedition Agent", args!["Yes, please report to the commander..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if (ctx.var("ep13_1_edq").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(11012)])? == 0) {
                    ctx.lines_as(
                        "Expedition Messenger",
                        args!["Were you assigned to deliver the expedition report?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Expedition Messenger", args!["I'm sorry, but I don't see any report in your hands. Please bring me the report, so I can confirm your identification."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ep13_1_edq").get()?.number()? > 3 && ctx.var("ep13_1_edq").get()?.number()? < 6) {
                    ctx.lines_as(
                        "Expedition Messenger",
                        args!["Please hurry up and report this incident to the commander."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Expedition Messenger",
                        args!["Good day! I'm here, just waiting for someone. Well, that's my duty."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ExpeditionMessengerStep::OnInit;
                continue 'machine;
            }
            ExpeditionMessengerStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Command Timer#edq::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn expedition_messenger(ctx: &Ctx) -> Script {
    expedition_messenger_run(ctx, ExpeditionMessengerStep::Start, Vec::new()).map(|_| ())
}

pub fn expedition_messenger_oninit(ctx: &Ctx) -> Script {
    expedition_messenger_run(ctx, ExpeditionMessengerStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CommandTimerEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer3600000,
}

fn command_timer_edq_run(ctx: &Ctx, mut step: CommandTimerEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CommandTimerEdqStep::Start => {
                step = CommandTimerEdqStep::OnInit;
                continue 'machine;
            }
            CommandTimerEdqStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Command Timer#edq")])?;
                return Err(Stop::End);
            }
            CommandTimerEdqStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Command Timer#edq")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CommandTimerEdqStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Command Timer#edq")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CommandTimerEdqStep::OnTimer3600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Expedition Messenger#3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Command Timer#edq::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn command_timer_edq(ctx: &Ctx) -> Script {
    command_timer_edq_run(ctx, CommandTimerEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn command_timer_edq_oninit(ctx: &Ctx) -> Script {
    command_timer_edq_run(ctx, CommandTimerEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn command_timer_edq_onenable(ctx: &Ctx) -> Script {
    command_timer_edq_run(ctx, CommandTimerEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn command_timer_edq_ondisable(ctx: &Ctx) -> Script {
    command_timer_edq_run(ctx, CommandTimerEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn command_timer_edq_ontimer3600000(ctx: &Ctx) -> Script {
    command_timer_edq_run(ctx, CommandTimerEdqStep::OnTimer3600000, Vec::new()).map(|_| ())
}

fn expedition_messenger_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Expedition Agent", args!["......"])?;
    ctx.next()?;
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#2")])?;
    ctx.mes("- When you tried to talk to him, he disappeared into thin air. He looked like someone on a very important mission... -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn expedition_messenger_2(ctx: &Ctx) -> Script {
    expedition_messenger_2_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#2")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_2_oninit(ctx: &Ctx) -> Script {
    expedition_messenger_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Expedition Messenger#2")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_2_onenable(ctx: &Ctx) -> Script {
    expedition_messenger_2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_2_ongo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Expedition Messenger#2")])?;
    ctx.call(Function::EnableNpc, vec![Val::from("Expedition Messenger#3")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_2_ongo(ctx: &Ctx) -> Script {
    expedition_messenger_2_ongo_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#2")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_2_ondisable(ctx: &Ctx) -> Script {
    expedition_messenger_2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Expedition Agent", args!["..."])?;
    ctx.next()?;
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#3")])?;
    ctx.mes("- When you tried to talk to him, he disappeared into thin air. He looked like someone on a very important mission... -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn expedition_messenger_3(ctx: &Ctx) -> Script {
    expedition_messenger_3_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#3")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_3_oninit(ctx: &Ctx) -> Script {
    expedition_messenger_3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_3_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Expedition Messenger#3")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_3_onenable(ctx: &Ctx) -> Script {
    expedition_messenger_3_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_messenger_3_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Expedition Messenger#3")])?;
    return Err(Stop::End);
}

pub fn expedition_messenger_3_ondisable(ctx: &Ctx) -> Script {
    expedition_messenger_3_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn expedition_scout_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_playtime = Val::from(0);
    ctx.lines_as(
        "Expedition Scout",
        args!["Good day. I'm from the Third Scout Party under direct command of the commander of the Midgard Expedition."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Expedition Scout",
        args!["I've been ordered by Instructor Igrid to conduct a search for something."],
    )?;
    l_playtime = ctx.call(Function::CheckQuest, vec![Val::from(3091), ctx.constant("PLAYTIME")?])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Ask about search results.:Chitchat.:Quit.")])? {
        1 => {
            if (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72) {
                ctx.lines_as(
                    "Expedition Scout",
                    args![
                        ((Val::from("Oh, you must be") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(". I heard that you'll be delivering the report."))
                    ],
                )?;
                ctx.next()?;
                if (l_playtime.clone() == 0 || l_playtime.clone() == 1) {
                    ctx.lines_as("Expedition Scout", args!["Searching for lost pages was tougher then I expected. Please come back later when I'm finished making one round."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?;
                    if subject2 == 1 || subject2 == 6 {
                        l_i = Val::from(2);
                    } else if subject2 == 3 {
                        l_i = Val::from(3);
                    } else if subject2 == 4 {
                        l_i = Val::from(5);
                    } else if subject2 == 2 || subject2 == 5 {
                        l_i = Val::from(0);
                    }
                    ctx.mes("[Expedition Scout]")?;
                    if l_i.clone().is_true() {
                        ctx.lines(args![
                            ((Val::from("Good news! I've found ") + l_i.clone())
                                + Val::from(" pieces of paper that appear to be part of the report."))
                        ])?;
                        ctx.next()?;
                        ctx.call(Function::GetItem, vec![Val::from(6040), l_i.clone()])?;
                    } else {
                        ctx.mes("I'm doing my best, but I haven't found any pages yet.")?;
                        ctx.next()?;
                    }
                    if l_playtime.clone() == -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(3090), Val::from(3091)])?;
                    } else {
                        ctx.call(Function::EraseQuest, vec![Val::from(3091)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(3091)])?;
                    }
                    ctx.lines_as("Expedition Scout", args!["Well then, keep up the good work!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as(
                "Expedition Scout",
                args!["Who are you? Why do you want to know about our search results? Are you from the army?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Expedition Scout",
                args!["Man, don't you know that you can be arrested for interrupting a military operation?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Expedition Scout", args!["Please leave immediately."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Expedition Scout",
                args!["I'm sorry, but I'm on duty. Why don't we talk later when I'm off-duty?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn expedition_scout_1(ctx: &Ctx) -> Script {
    expedition_scout_1_body(ctx, Vec::new()).map(|_| ())
}

fn laur_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Laur]")?;
    if ((((ctx.var("ep13_1_edq").get()? == 9 || ctx.var("ep13_1_edq").get()? == 111) || ctx.var("ep13_1_edq").get()? == 121)
        || ctx.var("ep13_1_edq").get()? == 113)
        || ctx.var("ep13_1_edq").get()? == 123)
    {
        ctx.mes("I'm Laur, the aide of the Home Minister of the Rune-Midgarts Kingdom. Do you have any business with me?")?;
        ctx.next()?;
        ctx.lines_as("Laur", args!["Oh, you brought me the report from the Midgard Expedition. What took you so long? I received a message about your departure a long time ago."])?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["Well, tell the commander that I'm not happy with your tardiness. Now, give me the report."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["I knew it was the wrong idea to out a mercenary in the commander position. See, nothing is done on time. Gosh!"],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
        if ctx.var("ep13_1_edq").get()? == 9 {
            ctx.var("ep13_1_edq").set(Val::from(101))?;
        } else if ctx.var("ep13_1_edq").get()? == 111 {
            ctx.var("ep13_1_edq").set(Val::from(112))?;
        } else if ctx.var("ep13_1_edq").get()? == 121 {
            ctx.var("ep13_1_edq").set(Val::from(122))?;
        } else if (ctx.var("ep13_1_edq").get()? == 113 || ctx.var("ep13_1_edq").get()? == 123) {
            ctx.var("ep13_1_edq").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3093), Val::from(3094)])?;
        }
        ctx.mes("...")?;
        ctx.next()?;
        ctx.mes("... ...")?;
        ctx.next()?;
        ctx.mes("... ... ...")?;
        ctx.next()?;
        ctx.lines_as("Laur", args!["*Ahem Ahem* Thank you for bringing me the report. I guess I was wrong about the commander. This report is actually very well done."])?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["But don't forget to tell him that I need the expedition reports on time from now on. Alright? *Ahem*"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("ep13_1_edq").get()? == 101 || ctx.var("ep13_1_edq").get()? == 102) || ctx.var("ep13_1_edq").get()? == 103)
        || ctx.var("ep13_1_edq").get()? == 112)
        || ctx.var("ep13_1_edq").get()? == 122)
        || ctx.var("ep13_1_edq").get()? == 13)
    {
        ctx.mes("Why are you back? You've delivered the report to me already. Are you suffering from amnesia or something?")?;
        ctx.next()?;
        ctx.lines_as("Laur", args!["You should leave and go back to your work."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("I'm Laur, the aide of the Home Minister of the Rune-Midgarts Kingdom. Do you have any business with me?")?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["Can you sense the great disorder occuring on the Midgard Continent and the Rune-Midgarts' Kingdom?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["Still, there always a silver lining, no matter how dangerous this world may become."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Laur",
            args!["There is hope and opportunity beyond the chaos, and I see the world beyond the time-space gap as a source of new hope."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn laur(ctx: &Ctx) -> Script {
    laur_body(ctx, Vec::new()).map(|_| ())
}

fn nuria_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Nuria]")?;
    if ((((ctx.var("ep13_1_edq").get()? == 9 || ctx.var("ep13_1_edq").get()? == 101) || ctx.var("ep13_1_edq").get()? == 121)
        || ctx.var("ep13_1_edq").get()? == 122)
        || ctx.var("ep13_1_edq").get()? == 103)
    {
        ctx.mes("Welcome, stranger. How can I help you?")?;
        ctx.next()?;
        ctx.lines_as(
            "Nuria",
            args!["Oh, I see. You've brought me a report from the expedition beyond the time-space gap."],
        )?;
        ctx.next()?;
        ctx.lines_as("Nuria", args!["I regret admitting that you've brought it much later than I expected, but... I'm glad it is here safely. Please give it to me."])?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
        if ctx.var("ep13_1_edq").get()? == 9 {
            ctx.var("ep13_1_edq").set(Val::from(111))?;
        } else if ctx.var("ep13_1_edq").get()? == 101 {
            ctx.var("ep13_1_edq").set(Val::from(102))?;
        } else if ctx.var("ep13_1_edq").get()? == 121 {
            ctx.var("ep13_1_edq").set(Val::from(123))?;
        } else if (ctx.var("ep13_1_edq").get()? == 122 || ctx.var("ep13_1_edq").get()? == 103) {
            ctx.var("ep13_1_edq").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3093), Val::from(3094)])?;
        }
        ctx.mes("...")?;
        ctx.next()?;
        ctx.mes("... ...")?;
        ctx.next()?;
        ctx.mes("... ... ...")?;
        ctx.next()?;
        ctx.lines_as(
            "Nuria",
            args!["I'm happy to see that they've made good progress so far. Please tell the commander to keep up the good work."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nuria",
            args!["Excuse me, but it's time to pray now. Please be safe on your travels."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("ep13_1_edq").get()? == 111 || ctx.var("ep13_1_edq").get()? == 102) || ctx.var("ep13_1_edq").get()? == 112)
        || ctx.var("ep13_1_edq").get()? == 123)
        || ctx.var("ep13_1_edq").get()? == 113)
        || ctx.var("ep13_1_edq").get()? == 13)
    {
        ctx.mes("Oh, right, I already received the report from you. Don't worry, you didn't forget.")?;
        ctx.next()?;
        ctx.lines_as("Nuria", args!["Please be safe on your way back to the expedition camp."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("Good day, stranger. May Freya bless you.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn nuria(ctx: &Ctx) -> Script {
    nuria_body(ctx, Vec::new()).map(|_| ())
}

fn gerhart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Gerhart]")?;
    if ((((ctx.var("ep13_1_edq").get()? == 9 || ctx.var("ep13_1_edq").get()? == 101) || ctx.var("ep13_1_edq").get()? == 111)
        || ctx.var("ep13_1_edq").get()? == 102)
        || ctx.var("ep13_1_edq").get()? == 112)
    {
        ctx.lines(args![
            ((Val::from("Welcome, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". What took you so long?"))
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args![
                "Oh, who am I? Let's say... That you can make ends meet because of me.",
                "Make sense?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args!["Do you know how long I waited for this report? *Sigh* ...Fine, just give it to me."],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(11012), Val::from(1)])?;
        if ctx.var("ep13_1_edq").get()? == 9 {
            ctx.var("ep13_1_edq").set(Val::from(121))?;
        } else if ctx.var("ep13_1_edq").get()? == 101 {
            ctx.var("ep13_1_edq").set(Val::from(103))?;
        } else if ctx.var("ep13_1_edq").get()? == 111 {
            ctx.var("ep13_1_edq").set(Val::from(113))?;
        } else if (ctx.var("ep13_1_edq").get()? == 102 || ctx.var("ep13_1_edq").get()? == 112) {
            ctx.var("ep13_1_edq").set(Val::from(13))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3093), Val::from(3094)])?;
        }
        ctx.mes("...")?;
        ctx.next()?;
        ctx.mes("... ...")?;
        ctx.next()?;
        ctx.mes("... ... ...")?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args!["Hmmm... This report is a clear demonstration of Commander Hibba Agip's competence... Well, that's great."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args![
                "Thank you for bringing the report.",
                "Please tell the commander to keep up the good work."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Gerhart", args!["Now excuse me, I must go back to work."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((((ctx.var("ep13_1_edq").get()? == 121 || ctx.var("ep13_1_edq").get()? == 103) || ctx.var("ep13_1_edq").get()? == 113)
        || ctx.var("ep13_1_edq").get()? == 123)
        || ctx.var("ep13_1_edq").get()? == 122)
        || ctx.var("ep13_1_edq").get()? == 13)
    {
        ctx.mes("What is it? Do you still have business with me?")?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args![
                "I've received the report safely.",
                "Please go back to the commander, and tell him to keep up the good work."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("What is it? I don't have time for chitchat.")?;
        ctx.next()?;
        ctx.lines_as(
            "Gerhart",
            args![
                "If you have business with me. please talk to my secretary and schedule an appointment. My office is on the second floor."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn gerhart(ctx: &Ctx) -> Script {
    gerhart_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ep13SplFild02MonEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer600000,
    OnMyMobDead,
}

fn ep13_spl_fild02_mon_edq_run(ctx: &Ctx, mut step: Ep13SplFild02MonEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13SplFild02MonEdqStep::Start => {
                step = Ep13SplFild02MonEdqStep::OnInit;
                continue 'machine;
            }
            Ep13SplFild02MonEdqStep::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_spl_fild02_mon_edq")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_spl_fild02_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13SplFild02MonEdqStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild02"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Luciola Vespa"),
                        Val::from(1994),
                        Val::from(7),
                        Val::from("ep13_spl_fild02_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild02"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Pinguicula"),
                        Val::from(1995),
                        Val::from(7),
                        Val::from("ep13_spl_fild02_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ep13SplFild02MonEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("spl_fild02"), Val::from("ep13_spl_fild02_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_spl_fild02_mon_edq")])?;
                return Err(Stop::End);
            }
            Ep13SplFild02MonEdqStep::OnTimer600000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("spl_fild02"), Val::from("ep13_spl_fild02_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_spl_fild02_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13SplFild02MonEdqStep::OnMyMobDead => {
                if (ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("spl_fild02"), Val::from("ep13_spl_fild02_mon_edq::OnMyMobDead")],
                    )?
                    .number()?
                    < 14
                    && (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72))
                {
                    ctx.call(Function::GetItem, vec![Val::from(6040), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_spl_fild02_mon_edq(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild02_mon_edq_oninit(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild02_mon_edq_onenable(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild02_mon_edq_ondisable(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild02_mon_edq_ontimer600000(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild02_mon_edq_onmymobdead(ctx: &Ctx) -> Script {
    ep13_spl_fild02_mon_edq_run(ctx, Ep13SplFild02MonEdqStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ep13SplFild03MonEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer600000,
    OnMyMobDead,
}

fn ep13_spl_fild03_mon_edq_run(ctx: &Ctx, mut step: Ep13SplFild03MonEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13SplFild03MonEdqStep::Start => {
                step = Ep13SplFild03MonEdqStep::OnInit;
                continue 'machine;
            }
            Ep13SplFild03MonEdqStep::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_spl_fild03_mon_edq")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_spl_fild03_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13SplFild03MonEdqStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild03"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Luciola Vespa"),
                        Val::from(1994),
                        Val::from(4),
                        Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild03"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Cornus"),
                        Val::from(1992),
                        Val::from(5),
                        Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild03"),
                        Val::from(0),
                        Val::from(0),
                        Val::from("Naga"),
                        Val::from(1993),
                        Val::from(5),
                        Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ep13SplFild03MonEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("spl_fild03"), Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_spl_fild03_mon_edq")])?;
                return Err(Stop::End);
            }
            Ep13SplFild03MonEdqStep::OnTimer600000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("spl_fild03"), Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_spl_fild03_mon_edq::OnEnable")])?;
                return Err(Stop::End);
            }
            Ep13SplFild03MonEdqStep::OnMyMobDead => {
                if (ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("spl_fild03"), Val::from("ep13_spl_fild03_mon_edq::OnMyMobDead")],
                    )?
                    .number()?
                    < 14
                    && (ctx.var("ep13_1_edq").get()? == 71 || ctx.var("ep13_1_edq").get()? == 72))
                {
                    ctx.call(Function::GetItem, vec![Val::from(6040), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_spl_fild03_mon_edq(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::Start, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild03_mon_edq_oninit(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild03_mon_edq_onenable(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild03_mon_edq_ondisable(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild03_mon_edq_ontimer600000(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn ep13_spl_fild03_mon_edq_onmymobdead(ctx: &Ctx) -> Script {
    ep13_spl_fild03_mon_edq_run(ctx, Ep13SplFild03MonEdqStep::OnMyMobDead, Vec::new()).map(|_| ())
}
