use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn crusader_god1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if (ctx.var("god_eremes").get()?.number()? > 17 && ctx.var("god_megin_1").get()?.number()? < 2) {
            ctx.lines_as(
                "Zan.Huadoku",
                args![
                    "^333333*Phew...*^000000",
                    "This work is really getting to me. Going on a mission with my war buddies sounds a lot better than this."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zan.Huadoku",
                args!["At least in those days, I felt like I was actually doing something useful!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Zan.Huadoku", args!["Hey...", "Can I help you", "with anything?"])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Ask him about the 1st Squad.:Ask him how he's been doing.:Ask him about the 1st Squad's last mission.",
                    )],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    if ctx.var("god_eremes").get()? == 18 {
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["Yeah, I was a member of the 1st Squad in the 3rd Platoon a long time ago. How did you know that?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["I miss the guys back in the squad. I wonder how our leader's been doing recently..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFYou tell Zan about Rebarev Doug, and about how he is now an instructor for Crusader Boot Camp. He seems to be absorbed in his thoughts of the past.^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("god_eremes").get()?.number()? > 18 {
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["Yeah, I was a member of the 1st Squad in the 3rd Platoon a long time ago. How did you know that?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["I miss the guys back in the squad. I wonder how our leader's been doing recently..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFYou tell Zan about Rebarev Doug, and about how he is now an instructor for Crusader Boot Camp. He seems to be absorbed in his thoughts of the past.^000000")?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["Before we went out on our last mission, the seven of us were like brothers and sisters. If ^FF0000he^000000 didn't disobey the order, we'd still be together today."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args![
                                "Wait...",
                                "What was his name...?",
                                "There's no way I could forget something like that..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["H-how can I not", "remember his name!", "He's the reason", "my life is...!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["^333333*Groan...*^000000", "I.... My head...", "My head hurts..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["^3355FFZan looks very confused and his eyes begin to glaze with a dazed look. You try speaking to him again, but he doesn't respond at all.^000000"])?;
                        if !(ctx.var("god_megin_1").get()?.is_true()) {
                            ctx.var("god_megin_1").set(Val::from(1))?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Zan.Huadoku",
                        args![
                            "How am I been doing?",
                            "Well, I work for the Blacksmith Guild, picking out good weapons",
                            "to supply the Crusaders."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Zan.Huadoku", args!["Sometimes, I get really", "bored with what I'm doing. I keep trying to convince myself that my job's important to future Crusaders, but..."])?;
                    ctx.next()?;
                    ctx.lines_as("Zan.Huadoku", args!["You know what?", "A lot of people have been asking me weird questions about me recently. I feel like I'm getting spied on, but maybe I'm just getting paranoid."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    if ctx.var("god_eremes").get()? == 18 {
                        ctx.lines_as("Zan.Huadoku", args!["Yeah, on our", "final mission..."])?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["Final...? What the? I know it happened, but for some reason, all I can envision is a complete blank when I try to think about it."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args![
                                "I...",
                                "I guess I need something to remind me? I know it happened, but... I'm so confused."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFZan seems to be having a very difficult time recalling that specific memory of his past.^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("god_eremes").get()?.number()? > 18 && ctx.var("god_megin_1").get()?.number()? > 0) {
                        ctx.lines_as("Zan.Huadoku", args!["The last mission..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args![
                                "Umm...",
                                "Huh. I don't",
                                "remember anything.",
                                "That's weird. Maybe",
                                "I need a bit of a clue?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFZan.Huadoku seemed to", "have a hard time remembering what had happened in the past. You begin to share with him what you had read in the library about the 1st Squad...^000000"])?;
                        ctx.next()?;
                        ctx.mes("...")?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.mes(".....")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args!["Y-yeah that's right! And then three days after we started the mission, we found some kind of..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["Well, I'm not sure what it was. But we found an ^0000FFunknown fragment^000000 that was a sign from God! And then...!"])?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["And then...", "Oh. Oh God.", "I can't remember..."])?;
                        ctx.next()?;
                        ctx.lines_as("Zan.Huadoku", args!["I can't think about anything further than that. I can't even remember what we found. But I'm sure it was damned important."])?;
                        ctx.next()?;
                        ctx.mes("^3355FFZan stood still in silence, with a pained look on his face.^000000")?;
                        if ctx.var("god_megin_1").get()? == 1 {
                            ctx.var("god_megin_1").set(Val::from(2))?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as("Zan.Huadoku", args!["The last mission..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zan.Huadoku",
                            args![
                                "Umm...",
                                "Huh. I don't",
                                "remember anything.",
                                "That's weird. Maybe",
                                "I need a bit of a clue?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        } else if (ctx.var("god_megin_1").get()?.number()? > 1 && ctx.var("god_megin_1").get()?.number()? < 3) {
            ctx.mes("...")?;
            ctx.next()?;
            ctx.lines(args!["...", "....."])?;
            ctx.next()?;
            ctx.lines_as(
                "Zan.Huadoku",
                args!["Y-yeah that's right! And then three days after we started the mission, we found some kind of..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Zan.Huadoku",
                args![
                    "Well, I'm not sure what it was. But we found an ^0000FFunknown fragment^000000 that was a sign from God! And then...!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Zan.Huadoku", args!["And then...", "Oh. Oh God.", "I can't remember..."])?;
            ctx.next()?;
            ctx.lines_as("Zan.Huadoku", args!["I can't think about anything further than that. I can't even remember what we found. Three months after that all happened, I've had these head problems..."])?;
            ctx.next()?;
            ctx.mes("^3355FFZan stood still in silence, with a pained look on his face.^000000")?;
            ctx.var("god_megin_1").set(Val::from(3))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("god_megin_1").get()?.number()? > 2 {
            ctx.lines(args![
                "^3355FFGrabbing his head,",
                "tearing his hair and writhing in Agony, Zan kept repeating the same words over and over again...^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Zan.Huadoku",
                args![
                    "3 days later!",
                    "We f-found some ^0000FFfragment^000000!",
                    "It was G-God's sign! But why",
                    "can't I remember?! Why?!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Zan.Huadoku", args!["Hey yo.", "Can I help you?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Zan.Huadoku", args!["Good day!", "Do you know the", "importance of supply?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("No.:Yes!")])? {
            1 => {
                ctx.lines_as("Zan.Huadoku", args!["Any military force or party needs their supplies to be replenished if they are to continue battling for a prolonged period of time."])?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["For instance, let's say your group go on hunting without a healer. In this case, everyone would consume health restoration items, like potions."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["Since you can only carry so many items at once, you'll eventually run out of potions."],
                )?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["But if one of your party members traveled between town and the hunting area, he could supply your party with new healing items."])?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["Supplying your party in this way would let you hunt with less worry. For that duty, the Merchant class would be the best by using their Carts."])?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["I suppose that's why I'm working here in the Blacksmith Guild. As weapon quartermaster, I'm obligated to provide supplies for the Crusader forces."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["The Blacksmiths in this guild also help me provide high quality weapons for the Crusades as well."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["If you happen to run out of health items while hunting, why don't you ask a Merchant for help?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["Most likely, they will have some spare items and would be willing to share with you if you ask nicely enough."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["I hope to see you later again! May Odin protect you on your journeys."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Zan.Huadoku", args!["Oh...", "Well then.", "In that case."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Zan.Huadoku",
                    args!["Let me tell you about some useful knowledge I learned while working with the Blacksmiths."],
                )?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["A Blacksmith character's forging skills are affected by the DEX and LUK stats. DEX and LUK also affect the item creation skills for Alchemists."])?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["Since I've been staying here in the Blacksmith guild, I'm trying to better understand Blacksmiths and develop a deeper appreciation of their work."])?;
                ctx.next()?;
                ctx.lines_as("Zan.Huadoku", args!["Alright then...", "Take it easy."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn crusader_god1(ctx: &Ctx) -> Script {
    crusader_god1_body(ctx, Vec::new()).map(|_| ())
}

fn employee_megin1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if (ctx.var("god_eremes").get()?.number()? > 17 && ctx.var("god_megin_2").get()?.number()? < 1) {
            ctx.lines(args!["^3355FFThe Inn Employee", "eyes you suspiciously.^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                "Inn Employee",
                args!["Excuse me.", "How can I help you?", "Are you looking for", "someone...?"],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            ctx.var("@str$").set(input)?;
            if ((ctx.var("@str$").get()? == "Cuaque Donon" || ctx.var("@str$").get()? == "Cuaque") || ctx.var("@str$").get()? == "Donon") {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![((Val::from("Do you happen to know a person named ") + ctx.var("@str$").get()?) + Val::from(" ...?"))],
                )?;
                if ctx.var("god_eremes").get()? == 18 {
                    ctx.lines_as(
                        "Inn Employee",
                        args![
                            "I don't think",
                            "I know that person.",
                            "I guess that guy left",
                            "before I started working",
                            "here, maybe?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("god_eremes").get()?.number()? > 18 {
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFOnce you said that name, she immediately drew closer to you",
                        "and began speaking in a low, threatening tone.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Scary Inn Employee",
                        args![
                            "Who the hell are you?",
                            "If you try anything funny,",
                            "I'll rip your heart out!",
                            "Why are you so curious?!",
                            "Are you one of them?!"
                        ],
                    )?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Rebarev Doug sent me!:Wait, is he in hiding?:Just... curious.")],
                        )?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1))
                            && !subject1.loosely_equals(&Val::from(2))
                            && !subject1.loosely_equals(&Val::from(3));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 3 {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Rebarev Doug...?!",
                                        "That old coot must be afraid",
                                        "of the rumors we're spreading around. Is already deperate enough to send his men?!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args!["But...", "I'm not in the mood", "to guide you over to", "Cuaque Donon..."],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFThe Inn Employee", "knocks you out~^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Rebarev Doug...?!",
                                        "That old coot must be afraid",
                                        "of the rumors we're spreading around. Is already deperate enough to send his men?!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Ms. Scary Inn Employee", args!["Hmm...", "It might not be a bad idea to let someone like you talk to Cuaque Donon. You don't seem like the bad sort..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "But you only get a hint,",
                                        "and I'm saying it just once...",
                                        "^0000FFAragham never hoarded",
                                        "upgrade items.^000000"
                                    ],
                                )?;
                                ctx.var("god_megin_2").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 4 {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Right.",
                                        "If you're here",
                                        "looking for him,",
                                        "you definitely know",
                                        "why he's hiding."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args!["How dare you...", "How dare you play", "dumb with me?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFThe Inn Employee", "knocks you out~^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                            } else {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Right.",
                                        "If you're here",
                                        "looking for him,",
                                        "you definitely know",
                                        "why he's hiding."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Well, maybe not.",
                                        "Who knows what kind",
                                        "of friends Cuaque made",
                                        "when he was a Crusader.",
                                        "Alright, but listen..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "I'm only giving",
                                        "you this hint once...",
                                        "^0000FFAragham never hoarded",
                                        "upgrade items.^000000"
                                    ],
                                )?;
                                ctx.var("god_megin_2").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                            matched1 = true;
                        }
                        if matched1 {
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 3 {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Just curious?",
                                        "Huh. You've got a lot of nerve, don't you? I'm sorry to tell you this but..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args!["I'm not in the mood", "to guide you over to", "Cuaque Donon...", "Heh heh heh..."],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFThe Inn Employee", "knocks you out~^000000"])?;
                                ctx.close_window()?;
                                ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "Just curious?",
                                        "Huh. You've got a lot of nerve, don't you? I'm sorry to tell you this but..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args![
                                        "I can't really tell",
                                        "you exactly where he",
                                        "is. The most I can do",
                                        "is give you a small hint.",
                                        "Listen carefully now...",
                                        "I'll only say it once."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ms. Scary Inn Employee",
                                    args!["^0000FFAragham never", "hoarded upgrade items.^000000", "Now, don't forget!"],
                                )?;
                                ctx.var("god_megin_2").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            } else {
                ctx.lines_as(
                    "Inn Employee",
                    args!["What...?", "I don't know", "anyone named that.", "If that's even a name..."],
                )?;
                ctx.close_window()?;
            }
        } else if ctx.var("god_megin_2").get()?.number()? > 0 {
            ctx.lines_as(
                "Inn Employee",
                args![
                    "Welcome to the Inn.",
                    "When you move to the entrance, you can also enter a PvP zone though a PvP doorman."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Inn Employee", args!["Usually, you can visit inns in big towns. If you want to take a rest or compete with others, an Inn is the place to go."])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFShe welcomed you",
                "very professionally,",
                "as if nothing",
                "had happened.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Inn Employee",
                args![
                    "Welcome to the Inn.",
                    "When you move to the entrance, you can also enter a PvP zone though a PvP doorman."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Inn Employee", args!["Usually, you can visit inns in big towns. If you want to take a rest or compete with others, an Inn is the place to go."])?;
            ctx.close_window()?;
        }
    } else {
        ctx.lines_as(
            "Inn Employee",
            args![
                "Welcome to the Inn.",
                "When you move to the entrance, you can also enter a PvP zone though a PvP doorman."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Inn Employee",
            args![
                "Usually, you can visit inns in big towns. If you want to take a rest or compete with others, an Inn is the place to go."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Inn Employee", args!["Oh, and I have a bit of a secret...! I hear the Inn's owner is planning some kind of shady business. It's some sort of major project, but don't let anyone know I told you!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn employee_megin1(ctx: &Ctx) -> Script {
    employee_megin1_body(ctx, Vec::new()).map(|_| ())
}

fn suspicious_man_megin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_toy_s = Val::from("");
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_eremes").get()? == 18 {
            ctx.lines_as(
                "Cuaque Donon",
                args![
                    "Wh-who are you?!",
                    "How the hell did",
                    "you get in here?!",
                    "Get away from me!",
                    "Geeeet awwwway!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_eremes").get()?.number()? > 18 {
                if (ctx.var("god_megin_2").get()?.number()? > 0 && ctx.var("god_megin_2").get()?.number()? < 4) {
                    ctx.lines_as(
                        "Cuaque Donon",
                        args![
                            "Wh-who are you?!",
                            "How the hell did",
                            "you get in here?!",
                            "Get away from me!",
                            "Geeeet awwwway!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FFYou try your best to calm him down and to tell him what you've found in the document.^000000")?;
                    ctx.next()?;
                    if ((((((((((ctx.call(Function::CountItem, vec![Val::from(740)])?.number()? > 0
                        || ctx.call(Function::CountItem, vec![Val::from(741)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(742)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(743)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(750)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(751)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(752)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(753)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(754)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(7206)])?.number()? > 0)
                        || ctx.call(Function::CountItem, vec![Val::from(7212)])?.number()? > 0)
                    {
                        if ctx.call(Function::CountItem, vec![Val::from(740)])?.number()? > 0 {
                            l_toy_s = Val::from("Puppet");
                        } else {
                            if ctx.call(Function::CountItem, vec![Val::from(741)])?.number()? > 0 {
                                l_toy_s = Val::from("Poring Doll");
                            } else {
                                if ctx.call(Function::CountItem, vec![Val::from(742)])?.number()? > 0 {
                                    l_toy_s = Val::from("Chonchon Doll");
                                } else {
                                    if ctx.call(Function::CountItem, vec![Val::from(743)])?.number()? > 0 {
                                        l_toy_s = Val::from("Spore Doll");
                                    } else {
                                        if ctx.call(Function::CountItem, vec![Val::from(750)])?.number()? > 0 {
                                            l_toy_s = Val::from("Baphomet Doll");
                                        } else {
                                            if ctx.call(Function::CountItem, vec![Val::from(751)])?.number()? > 0 {
                                                l_toy_s = Val::from("Osiris Doll");
                                            } else if ctx.call(Function::CountItem, vec![Val::from(752)])?.number()? > 0 {
                                                l_toy_s = Val::from("Rocker Doll");
                                            } else if ctx.call(Function::CountItem, vec![Val::from(753)])?.number()? > 0 {
                                                l_toy_s = Val::from("Yoyo Doll");
                                            } else if ctx.call(Function::CountItem, vec![Val::from(754)])?.number()? > 0 {
                                                l_toy_s = Val::from("Racoon Doll");
                                            } else if ctx.call(Function::CountItem, vec![Val::from(7206)])?.number()? > 0 {
                                                l_toy_s = Val::from("Black Cat Doll");
                                            } else {
                                                l_toy_s = Val::from("Hung Doll");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ctx.lines(args![
                            ((Val::from("^3355FFYou pulled out a ") + l_toy_s.clone()) + Val::from("")),
                            "to cover your face, and wiggled its arms as if it were talking.^000000"
                        ])?;
                    } else {
                        ctx.lines(args!["^3355FFYou must find something", "that can calm Cuaque Donon.^000000"])?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFSadly, Chaque Donon seems", "to have regressed to a very childish mentality. Perhaps if you brought something that children like. Something that would offer them comfort...^000000"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFChaque Donon nodded his head",
                        "and happily sucked his thumb. It saddens you to see a former Crusader reduced to this state.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Ask him about the Inn Maid.:Ask him how he's been doing.:Ask about 1st Squad's Final Mission.",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "S-she's my sister.",
                                    "And she's also a Rogue.",
                                    "When I was retired from service...."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["...", "......"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "My family always expected",
                                    "too much of me. That's why I used to be a Crusader. I mean, they're paid well and respected."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["When I retired, my sister was so mad. She's the only one I can depend upon, but now that I've messed up my life like this..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "But even after yelling at me, and saying I was worthless, she got ",
                                    "me a job at her Inn anyway."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["I've been working there,", "living a quiet life. But now, for some reason, a lot of people want to talk to me about something important."])?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["I don't know what I said or did, but I think when I gave them the right answer, after I could finally remember something, they tried to hurt me."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "So now my sister's been hiding",
                                    "me here. It's boring and lonely, and the guy downstairs is always yelling at me to leave."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["Even the Rogue Guild that", "my sister's a part of comes in sometimes to yell at me. They all want me to tell them something I'm supposed to know!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "But I can't remember anything!",
                                    "This place is safe from dangerous people, but I don't know for how long."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["Even my sister says I have to tell the Rogue Guild what I should know, or they won't protect us anymore."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "She looked really sad when she",
                                    "said that. I don't want to let her down or make her upset. She's",
                                    "all I have left..."
                                ],
                            )?;
                            if ctx.var("god_megin_2").get()? == 2 {
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cuaque Donon",
                                    args![
                                        "Oh, I do remember something!",
                                        "My sister told me not to tell anyone, but I'm gonna tell you..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args!["^3355FFCuaque Donon", "whispers into your ears.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Cuaque Donon", args!["^666666She said the Rogue Guild wants something from me. So she's gonna send to me to good people that we can trust and can help me.^000000"])?;
                                ctx.next()?;
                                ctx.lines_as("Cuaque Donon", args!["^666666Still, this is the safest place for me for a while. But I'm always ready to run away if I have to.^000000"])?;
                                ctx.var("god_megin_2").set(Val::from(3))?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "How have I been doing?",
                                    "Lots of people have been coming",
                                    "to see me. They all want me to tell them something I don't even know!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args![
                                    "The Rogue Guild promised to",
                                    "protect me, but they scare the hell out of me sometimes by coming out of the floor all of a sudden."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["Oh, and my sister said she's", "gonna complain to my old boss about something. She said I used to be fine, but now that I've retired, I'm not the same anymore."])?;
                            ctx.next()?;
                            ctx.lines_as("Cuaque Donon", args!["But my old boss from when I was a Crusader doesn't wanna meet my sister. So she's gonna ask help from other Rogues to spread some rumors..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cuaque Donon",
                                args!["Like anyone who works as a Crusader under my old boss will end up like me. You know... broken."],
                            )?;
                            if ctx.var("god_megin_2").get()? == 1 {
                                ctx.var("god_megin_2").set(Val::from(2))?;
                            }
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as("Cuaque Donon", args!["I don't know...", "I can't even remember."])?;
                            if ctx.var("god_megin_2").get()? == 3 {
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cuaque Donon",
                                    args![
                                        "Wait, that's right!",
                                        "^0000FFMegingjard^000000! Right.",
                                        "We found it, and",
                                        "we knew what it was",
                                        "just by looking at it!"
                                    ],
                                )?;
                                ctx.next()?;
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? > 4 {
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("^FF0000Megingjard^000000!:^FFFFFFListen to him quietly^000000.")],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                "Cuaque Donon",
                                                args!["Waaaaaaaah~!", "You're the same", "as all the others!", "Go away from me!"],
                                            )?;
                                            ctx.var("god_megin_2").set(Val::from(0))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as("Cuaque Donon", args!["I remember after we found it. One of us in the squad had a argument with our old leader. I was keeping night watch and happened to hear it. I think our leader was out of line..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Cuaque Donon", args!["Ergh...", "I can't remember", "more than that..."])?;
                                            ctx.var("god_megin_2").set(Val::from(4))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    match runtime::select_values(
                                        ctx,
                                        &[
                                            Val::from("^FF0000Megingjard^000000!"),
                                            Val::from("^FFFFFFMegingjard!^000000."),
                                            Val::from("^FFFFFFListen to him quitely^000000"),
                                        ],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                "Cuaque Donon",
                                                args!["Waaaaaaaah~!", "You're the same", "as all the others!", "Go away from me!"],
                                            )?;
                                            ctx.var("god_megin_2").set(Val::from(0))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Cuaque Donon",
                                                args!["Waaaaaaaah~!", "You're the same", "as all the others!", "Go away from me!"],
                                            )?;
                                            ctx.var("god_megin_2").set(Val::from(0))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        3 => {
                                            ctx.lines_as("Cuaque Donon", args!["I remember after we found it. One of us in the squad had a argument with our old leader. I was keeping night watch and happened to hear it. I think our leader was out of line..."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Cuaque Donon", args!["Ergh...", "I can't remember", "more than that..."])?;
                                            ctx.var("god_megin_2").set(Val::from(4))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            ctx.close_window()?;
                        }
                        _ => {}
                    }
                } else if ctx.var("god_megin_2").get()? == 4 {
                    ctx.lines_as("Cuaque Donon", args!["I remember after we found Megingjard. One of us in the squad had a argument with our old leader. I was keeping night watch and happened to hear it. I think our leader was out of line..."])?;
                    ctx.next()?;
                    ctx.lines_as("Cuaque Donon", args!["Ergh...", "I can't remember", "more than that..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cuaque Donon",
                        args!["I'm getting sleepy", "and my head hurts.", "Let me rest now..."],
                    )?;
                    ctx.close_window()?;
                } else {
                    ctx.lines_as(
                        "Cuaque Donon",
                        args![
                            "Wh-who are you?!",
                            "How the hell did",
                            "you get in here?!",
                            "Get away from me!",
                            "Geeeet awwwway!"
                        ],
                    )?;
                    ctx.close_window()?;
                }
            }
        }
    } else {
        ctx.lines_as(
            "Cuaque Donon",
            args!["Wh-who are you?!", "How the hell did", "you get in here?!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cuaque Donon",
            args!["Wahhhhh...!", "My head!", "It's hurting so bad!", "I'm so s-scared!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn suspicious_man_megin(ctx: &Ctx) -> Script {
    suspicious_man_megin_body(ctx, Vec::new()).map(|_| ())
}

fn crusader_megin2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_eremes").get()? == 18 {
            ctx.lines_as(
                "Jack O",
                args![
                    "^333333*Yawn...*^000000",
                    "It's quiet and boring, as per usual. Let's see if there's any Swordmen I can recruit today."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jack O", args!["Eh...?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Ask him about Rebarev Doug.:Ask him how he's been doing.:Ask him about the last mission of the 1st Squad.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Jack O",
                        args!["Ah! Yeah, our old leader back in the 1st Squad. Heh. I haven't seen him since we were disbanded."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Jack O",
                        args![
                            "Me? Yeah, I'm always busy recruiting future Crusaders.",
                            "We always welcome Swordmen",
                            "of great ability!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jack O",
                        args!["How does it sound? Why don't you volunteer to become a Crusader?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Jack O", args!["Last mission?", "Eh, I don't really remember it."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("god_eremes").get()?.number()? > 18 {
            if (ctx.var("god_megin_3").get()? == 0 || ctx.var("god_megin_3").get()? == 1) {
                ctx.lines_as(
                    "Jack O",
                    args![
                        "^333333*Yawn...*^000000",
                        "It's quiet and boring, as per usual. Let's see if there's any Swordmen I can recruit today."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Jack O", args!["Eh...?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Ask him about Rebarev Doug.:Ask him how he's been doing.:Ask about 1st Squad's final mission.",
                    )],
                )? {
                    1 => {
                        if ctx.var("god_megin_3").get()? == 1 {
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "Ah right. Our old leader.",
                                    "Now I remember: He was",
                                    "a pretty self righteous jerk",
                                    "now that I think about it!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Jack O", args!["Can you believe he used to say that ^0000FFno one is more religious than him in this world^000000?! That's egoism right there. And maybe insanity."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "Anyway, everyone in the squad",
                                    "took pride in their faith. When",
                                    "you're having a rough time in the",
                                    "real world, you depend on religion, you know?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "Anyways, in the records about",
                                    "our squad, it says that one of us was punished for insubordination, rebelling or something. That's",
                                    "a complete lie."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Jack O", args!["During our final mission,", "our old leader had a huge argument with one us over something we found. I remember it being some kind of godly artifact, but I can't clearly remember what it was."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "Although there was an obvious dispute, I think it was our squad leader who was out of line.",
                                    "I... I can't really say..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "I can't for the life of me",
                                    "remember how the guy who",
                                    "argued with the squad leader",
                                    "looked like. The higher-ups",
                                    "musta did something to me..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Jack O", args!["Anyway, I know for sure that whatever that guy did, it wasn't insubordination. In fact, I think he might've been right!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Jack O",
                                args![
                                    "That's all I can remember.",
                                    "I better take some Green Herbs now. My head throbs like crazy whenever I think about that time."
                                ],
                            )?;
                            ctx.var("god_megin_3").set(Val::from(2))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Jack O",
                                args!["Ah! Yeah, our old leader back in the 1st Squad. Heh. I haven't seen him since we were disbanded."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            "Jack O",
                            args![
                                "Me? Yeah, I'm not doing so bad. Keeping busy recruiting future Crusaders. We welcome all",
                                "Swordmen if they show potential~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Jack O", args!["Huh...?", "Why would", "you want to know?"])?;
                        ctx.next()?;
                        ctx.mes("^3355FFYou tell him what you've read in the records for the 1st Squad. Afterwards, Jack O looks a little confused.^000000")?;
                        ctx.next()?;
                        ctx.lines_as("Jack O", args!["Huh...?", "I might not be able to remember", "a whole lot from back then, but the part about ^0000FFinsubordination^000000 can't be right. I'm sure of that."])?;
                        if ctx.var("god_megin_3").get()? == 0 {
                            ctx.var("god_megin_3").set(Val::from(1))?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else if ctx.var("god_megin_3").get()? == 2 {
                ctx.lines_as(
                    "Jack O",
                    args![
                        "Our old leader was",
                        "so arrogant to the point",
                        "of being a little off his rocker."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Jack O", args!["I mean, normal people", "don't say things like '^0000FFI am the most religious man in the universe! Kneel before me!^000000' Yeah. Not a normal thing to say."])?;
                ctx.next()?;
                ctx.lines_as("Jack O", args!["Anyway, if the squad", "leader punished anyone for insubordination, I'm sure that it was unwarranted. I don't remember too much, but I'm sure of that."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jack O",
                    args![
                        "Excuse me, I need to take some Green Herbs. Taking these seems",
                        "to be the only thing that works for my headache."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFJack O busily chewed",
                    "on some Green Herbs.",
                    "It seemed to greatly relieve him from the pain of his headaches.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Jack O",
                    args![
                        "^333333*Yawn...*^000000",
                        "It's quiet and boring, as per usual. Let's see if there's any Swordmen I can recruit today."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as("Jack O", args!["Hey kid!", "Ever think about", "bein' a Crusader?"])?;
            ctx.close_window()?;
        }
    } else {
        if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?) {
            ctx.lines_as("Jack O", args!["Hey kid!", "Ever think about", "bein' a Crusader?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jack O",
                args!["Basically, you'd be volunteering to be part of our military, as well as train for the Holy War."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jack O",
                args!["If you have some faith and know how to wield a sword, you'll be more than welcome here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jack O",
                args![
                    "Just think about my suggestion",
                    "and come back if you're interested. Alright then, see you later~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Jack O",
                args![
                    "^333333*Yawn...*^000000",
                    "It's quiet and boring, as per usual. Let's see if there's any Swordmen I can recruit today."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jack O",
                args![
                    "Huh...?",
                    "Are you interested in knowing more about Crusaders? Hahaha, I guess that must be the case."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jack O", args!["By the way, bring a doll to", "the kid in front of me. If you're lucky, she'll give you something neat in return. What it is, I don't know. I've got no dolls."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jack O",
                args!["In accordance with the Crusader code, I'm not supposed to lie. So yeah, you can trust me!"],
            )?;
            ctx.close_window()?;
        }
    }
    Ok(Val::from(0))
}

pub fn crusader_megin2(ctx: &Ctx) -> Script {
    crusader_megin2_body(ctx, Vec::new()).map(|_| ())
}

fn lady_megin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1301), Val::from(3)])? == 0 {
        ctx.mes("- You are carrying too many items!")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_eremes").get()? == 18 {
            ctx.lines_as(
                "Emma Searth",
                args![
                    "^333333*Sigh...*^000000 I haven't gotten any response from them. I don't know",
                    "if I can wait much longer to join the Kafra Corporation."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("god_eremes").get()?.number()? > 18 && ctx.var("god_eremes").get()?.number()? < 26) {
                if ctx.var("god_megin_4").get()?.number()? < 2 {
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "^333333*Sigh...*^000000 I haven't gotten any response from them. I don't know",
                            "if I can wait much longer to join the Kafra Corporation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Emma Searth", args!["Oh, an adventurer! I actually used to work in the same field that you do. More specifically, I used to be a Crusader."])?;
                    ctx.next()?;
                    ctx.lines_as("Emma Searth", args!["My name is Emma Searth. Right now, I'm following my dream and applying to be a Kafra Lady. So my days of wielding a sword are over."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "Sadly, I haven't succeeded yet because I was diagnosed with",
                            "some weird ^0000FFamnesia^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "Of course, I understand that",
                            "Kafra Corporation is a large, professional company, but",
                            "if I can work for them even if it's just part-time..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "Is it that big of a problem?",
                            "I suppose they're worried",
                            "that I'd forget important customer information once I'm hired."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "It's strange. When I was",
                            "a Crusader, I remembered",
                            "everything lucidly, and I never had any problems recalling the past."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "But now I get these migraines whenever I try to focus on my memories from those times",
                            "when I used to fight in the Prontera Military."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Emma Searth", args!["^333333*Sigh...*^000000"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("About her past.:Why be a Kafra Lady?:About memories she can remember.")],
                    )? {
                        1 => {
                            if ctx.call(Function::CountItem, vec![Val::from(7015)])?.number()? > 0 {
                                ctx.lines(args![
                                    "^3355FFThe scent of your",
                                    "Memory Bookmark",
                                    "seems to bring clarity",
                                    "to her thoughts.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["I was born in a wealthy family where I was raised to learn etiquette, fencing, Peco riding, and music lessons since I was", "born. I suppose you can say I'm pretty well educated."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "However, my parents weren't",
                                        "pleased when they learned that I wanted to become a Kafra Lady."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "My parents insisted that",
                                        "I become a Sage or a Scholar.",
                                        "In the end, I ended up running",
                                        "away from them by joining the Crusaders."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["Luckily, the Crusaders were advertising their recruitment at the time, so I took that chance. Plus, having military experience might even help me join Kafra!"])?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["Since I did so well in Boot Camp,", "I got the chance to join an elite squad. My squad was often assigned difficult missions, and we never failed to complete those."])?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["Eventually, my reputation as a female Crusader, and being in a respectable military position, was enough to satisfy my family."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "But after that day,",
                                        "everything went wrong.",
                                        "I only remember that it",
                                        "was very sad and depressing.",
                                        "Somehow, I feel like we were",
                                        "even betrayed..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["I think I can", "try to remember", "my comrades..."])?;
                                ctx.var("god_megin_4").set(Val::from(1))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Emma Searth", args!["Ah, let's not talk about it. It's too depressing. Maybe if you had some sort of ^0000FFmemory^000000 aid, I might be able to recall my forgotten past. Anything, like a reminder, a scheduler, a bookmark, a string..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as("Emma Searth", args!["One day, I happened to see Ms. Leilah at work here in Al De Baran. She was shining with confidence and commanded respect, even from her customers!"])?;
                            ctx.next()?;
                            ctx.lines_as("Emma Searth", args!["She carried herself like a professional, no matter what the situation. Even though I knew she was tired from working in the summer heat, she made everything look effortless!"])?;
                            ctx.next()?;
                            ctx.lines_as("Emma Searth", args!["When the male customers try to flirt with her, she never loses her poise. She just adjusts her glasses and does what she needs to do."])?;
                            ctx.next()?;
                            ctx.lines_as("Emma Searth", args!["To me, she's like a lone, guardian angel. At that moment, it seemed the world existed just to be mocked by Ms. Leila."])?;
                            ctx.next()?;
                            ctx.lines_as("Emma Searth", args!["I've also heard that Kafra Ladies are always traveling around the world, which is something I've always dreamed of doing."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Emma Searth",
                                args![
                                    "Ms. Leilah is my idol,",
                                    "and she's the reason why",
                                    "I want to become a Kafra Lady."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            if (ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0
                                && ctx.var("god_megin_3").get()?.number()? > 1)
                            {
                                ctx.mes("^3355FFEmma Searth clutches her head and winces as she suffers from another migraine.^000000")?;
                                ctx.next()?;
                                ctx.mes("^3355FFRemembering that a Green Herb was able to relieve Jack O, you gave a Green Herb to Emma. She takes it, and looks a little more relaxed.^000000")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "I'm not even sure",
                                        "how much I can remember",
                                        "and how much I've actually",
                                        "forgotten."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Emma Searth", args!["Let me...", "Try to remember every one in our squad. Let's see, there was Jack O, The Nineball. Z-Zan. Zan.H-Huadoku. C-Cuaque Donon. Myself. Um, E... Eni... Egni... Um..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "Egni...",
                                        "Egnigem?^000000",
                                        "Who was that?",
                                        "Oh no. I'm sure he might",
                                        "have, no, was there someone",
                                        "named that with us?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.var("god_megin_4").set(Val::from(2))?;
                                return Err(Stop::End);
                            } else {
                                ctx.mes("^3355FFEmma Searth clutches her head and winces as she suffers from another migraine.^000000")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Emma Searth",
                                    args![
                                        "I'm not even sure",
                                        "how much I can remember",
                                        "and how much I've actually",
                                        "forgotten."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        _ => {}
                    }
                } else if ctx.var("god_megin_4").get()?.number()? > 1 {
                    ctx.lines_as(
                        "Emma Searth",
                        args!["^0000FFJack O, The Nineball, Zan.Huadoku, Cuaque Donon, Egnigem.^000000 Oh, I really miss those guys..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Emma Searth",
                        args![
                            "^333333*Sigh...*^000000 I haven't gotten any response from them. I don't know",
                            "if I can wait much longer to join the Kafra Corporation."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("god_eremes").get()? == 25 {
                    ctx.lines_as(
                        "Emma Searth",
                        args!["Egnigem...", "That name makes", "me so sad. I'm not", "quite sure why, but..."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if (ctx.var("god_eremes").get()? == 26 || ctx.var("god_eremes").get()? == 27) {
                        ctx.lines_as(
                            "Emma Searth",
                            args!["Egnigem...", "That name makes", "me so sad. I'm not", "quite sure why, but..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFEmma turns away before you can see her cry. Still, her shoulders heave with each sob as you imagine the hardship she's had to endure.^000000")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Emma Searth",
                            args!["I wish I knew what that name really means. I'm sure it's linked to an important memory from my past."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Emma Searth",
                            args!["I really appreciate you coming here to talk to me about the old days. I, I want you to have this."],
                        )?;
                        ctx.var("god_eremes").set(Val::from(28))?;
                        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                        {
                            if ctx.var("BaseLevel").get()?.number()? < 56 {
                                ctx.call(Function::GetExperience, vec![Val::from(27000), Val::from(0)])?;
                            } else {
                                if (ctx.var("BaseLevel").get()?.number()? > 55 && ctx.var("BaseLevel").get()?.number()? < 61) {
                                    ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                                } else {
                                    if (ctx.var("BaseLevel").get()?.number()? > 60 && ctx.var("BaseLevel").get()?.number()? < 66) {
                                        ctx.call(Function::GetExperience, vec![Val::from(56052), Val::from(0)])?;
                                    } else {
                                        if (ctx.var("BaseLevel").get()?.number()? > 65 && ctx.var("BaseLevel").get()?.number()? < 71) {
                                            ctx.call(Function::GetExperience, vec![Val::from(82233), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 70 && ctx.var("BaseLevel").get()?.number()? < 76)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(212271), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 75 && ctx.var("BaseLevel").get()?.number()? < 81)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(390738), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 80 && ctx.var("BaseLevel").get()?.number()? < 86)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(451020), Val::from(0)])?;
                                        } else if (ctx.var("BaseLevel").get()?.number()? > 85 && ctx.var("BaseLevel").get()?.number()? < 91)
                                        {
                                            ctx.call(Function::GetExperience, vec![Val::from(546156), Val::from(0)])?;
                                        } else {
                                            ctx.call(Function::GetExperience, vec![Val::from(1220358), Val::from(0)])?;
                                        }
                                    }
                                }
                            }
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("god_eremes").get()? == 28 {
                        ctx.lines_as(
                            "Emma Searth",
                            args!["Egnigem...", "That name makes", "me so sad. I'm not", "quite sure why, but..."],
                        )?;
                        ctx.next()?;
                        ctx.mes("^3355FFEmma turns away before you can see her cry. Still, her shoulders heave with each sob as you imagine the hardship she's had to endure.^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Emma Searth",
                            args![
                                "^333333*Sigh...*^000000 I haven't gotten any response from them. I don't know",
                                "if I can wait much longer to join the Kafra Corporation."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    } else {
        ctx.lines_as("Emma Searth", args!["I hear the Kafra Ladies always travel around the world, something I've always dreamed of doing! Hopefully, one of these days, I'll be able to work for the Kafra Corporation too."])?;
        ctx.next()?;
        ctx.lines_as(
            "Emma Searth",
            args![
                "^333333*Sigh...*^000000",
                "Still, it's discouraging. I haven't heard anything from them..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lady_megin(ctx: &Ctx) -> Script {
    lady_megin_body(ctx, Vec::new()).map(|_| ())
}

fn man_megin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_eremes").get()? == 18 {
            ctx.lines_as("Royal Myst", args!["Wha...?", "You wanna talk?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Royal Myst",
                args![
                    "What am I, your personal psychiatrist?! Lemme alone,",
                    "I'm busy here! Awright, now",
                    "which one do I bet on...?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Speak of Rebarev Doug.:Speak of the 1st Squad.:Talk about gambling.:Discuss hobbies.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Royal Myst",
                        args![
                            "Wha--? Him again?! Tell him I'm fine! Dandy, even! Why's he gotta send all these people just to ask",
                            "a silly question?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Royal Myst",
                        args!["1st Squad? Yeah, I used to be in that. All of us in the squad used to be real good buddies too."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Royal Myst",
                        args![
                            "Zan.Huadoku, Cuaque Donon,",
                            "Jack O, Emma Searth and The Nineball. Yeah... I wonder what they're all up to?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as("Royal Myst", args!["Gambling? I love it, you", "know? Eh, I don't win all the time, but I love it the most when I leave with more Zeny than I came in with! Hahaha!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                4 => {
                    ctx.lines_as("Royal Myst", args!["Hobbies?", "What, you comin'", "on to me?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Royal Myst",
                        args!["A man's hobby is drinking, or didn't you know that? I happen to be real good at it too! Mwahahaha!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if (ctx.var("god_eremes").get()?.number()? > 18 && ctx.var("god_eremes").get()?.number()? < 21) {
            if (((((ctx.var("god_megin_6").get()? == 0 && ctx.var("god_megin_5").get()?.number()? > 4)
                && ctx.var("god_megin_4").get()?.number()? > 1)
                && ctx.var("god_megin_3").get()?.number()? > 1)
                && ctx.var("god_megin_2").get()?.number()? > 3)
                && ctx.var("god_megin_1").get()?.number()? > 2)
            {
                ctx.lines_as("Royal Myst", args!["Wha...?", "You wanna talk?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Royal Myst",
                    args![
                        "What am I, your personal psychiatrist?! Lemme alone,",
                        "I'm busy here! Awright, now",
                        "which one do I bet on...?"
                    ],
                )?;
                ctx.next()?;
            } else if ctx.var("god_megin_6").get()?.number()? > 0 {
                ctx.lines_as("Royal Myst", args!["Hmm?", "What's up?"])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Royal Myst", args!["Wha...?", "You wanna talk?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Royal Myst",
                    args![
                        "What am I, your personal psychiatrist?! Lemme alone,",
                        "I'm busy here! Awright, now",
                        "which one do I bet on...?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if ctx.var("god_megin_6").get()?.number()? < 16 {
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[
                            Val::from("Speak of Rebarev Doug."),
                            Val::from("Speak of the 3rd squad."),
                            Val::from("Talk about gambling."),
                            Val::from("Discuss hobbies."),
                        ],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                        && !subject2.loosely_equals(&Val::from(2))
                        && !subject2.loosely_equals(&Val::from(3))
                        && !subject2.loosely_equals(&Val::from(4));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        if ctx.var("god_megin_6").get()? == 0 {
                            ctx.lines_as("Royal Myst", args!["Huh? What's that old geezer want this time? Tell 'em I'm fine, dandy even! Why the hell does he keep sending people..."])?;
                            ctx.var("god_megin_6").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("god_megin_6").get()?.number()? > 0 && ctx.var("god_megin_6").get()?.number()? < 15) {
                            ctx.lines_as("Royal Myst", args!["What the hell!? Stop talking about him! I never wanna see his face again! Tell him to leave me the hell alone!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("god_megin_6").get()?.number()? > 14 {
                            ctx.lines_as(
                                "Royal Myst",
                                args![
                                    "Damn geezer...",
                                    "How much is he being paid for studying what we all found?! Eh, somehow, I don't care as long as",
                                    "he pays me..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Royal Myst", args!["Yeah...", "I just don't", "care anymore."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        if ctx.var("god_megin_6").get()?.number()? < 15 {
                            ctx.lines_as(
                                "Royal Myst",
                                args!["1st Squad? Yeah, I used to be in that. All of us in the squad used to be real good buddies too."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Royal Myst",
                                args![
                                    "Zan.Huadoku, Cuaque Donon,",
                                    "Jack O, Emma Searth and The Nineball. Yeah... I wonder what they're all up to?"
                                ],
                            )?;
                        } else if ctx.var("god_megin_6").get()?.number()? > 14 {
                            ctx.lines_as(
                                "Royal Myst",
                                args!["1st Squad?", "Don't know, don't care~", "Do I look like a stupid Crusader?"],
                            )?;
                        }
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                        matched2 = true;
                    }
                    if matched2 {
                        if ctx.var("god_megin_6").get()?.number()? < 15 {
                            ctx.lines_as(
                                "Royal Myst",
                                args![
                                    "Gambling? ! Oh man...",
                                    "I love gambling, you know!",
                                    "Heh, but I haven't been lucky enough to win yet. Hahaha~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("god_megin_6").get()?.number()? > 14 {
                            ctx.lines_as(
                                "Royal Myst",
                                args![
                                    "Gambling...!",
                                    "Heh heh! Some risks you take,",
                                    "and others you really shouldn't.",
                                    "I can't help but feel sorry for that guy, E--"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Royal Myst", args!["Ergh...?", "Damn, I can never remember his name. You'd think I wouldn't forget the guy whose rap I'm taking but... Eh, I'll remember once I sober up."])?;
                            ctx.next()?;
                            ctx.lines_as("Royal Myst", args!["Hey, what do you think happens to Crusaders when they're framed and killed, huh? Where exactly do they go? Niflheim, Vahalla...?"])?;
                            ctx.var("god_eremes").set(Val::from(20))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                        matched2 = true;
                    }
                    if matched2 {
                        if ctx.var("god_megin_6").get()? == 0 {
                            ctx.lines(args!["Hobbies?", "What, you comin'", "on to me?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Royal Myst",
                                args!["A man's hobby is drinking, or didn't you know that? I happen to be real good at it too! Mwahahaha!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("god_megin_6").get()?.number()? > 0 && ctx.var("god_megin_6").get()?.number()? < 15) {
                            if ctx.call(Function::CountItem, vec![Val::from(970)])?.number()? > 0 {
                                ctx.lines_as(
                                    "Royal Myst",
                                    args![
                                        "Speaking of which,",
                                        "I haven't had a drink",
                                        "for a looong time. Almost",
                                        "a couple hours now."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Royal Myst",
                                    args!["Oooh, looks like you've got a tasty beverage I can enjoy. Bwahahaha! Gimmie~!"],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFBefore you can even think,",
                                    "Royal Myst dips his hand into your inventory and helps himself to an Alcohol.^000000"
                                ])?;
                                ctx.call(Function::DelItem, vec![Val::from(970), Val::from(1)])?;
                                ctx.var("god_megin_6").set((ctx.var("god_megin_6").get()? + Val::from(2)))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Royal Myst", args!["Why mention it? You gonna bring me something I'll like? Cuz I'm more than willing to take it! Bwahahaha!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else if ctx.var("god_megin_6").get()?.number()? > 14 {
                            ctx.lines_as("Royal Myst", args!["^333333*Hiccup*^000000 Oh yeah, this is the stuff. Not like that imitation junk they've been serving at the Bars nowadays..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Royal Myst",
                                args![
                                    "Hey! There anything you wanna",
                                    "know about me? You did me a favor, so I'll tell you anything! Eeeeeverythiiing~~!!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        } else {
            ctx.lines_as("Royal Myst", args!["Eh heh heh~", "I just know", "I'm gonna win", "this time!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Royal Myst", args!["Wha...?", "You wanna talk?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Royal Myst",
            args![
                "What am I, your personal psychiatrist?! Lemme alone,",
                "I'm busy here! Awright, now",
                "which one do I bet on...?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn man_megin(ctx: &Ctx) -> Script {
    man_megin_body(ctx, Vec::new()).map(|_| ())
}

fn security_officer_megin_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check1").get()?)?.is_true()
        && runtime::op(&ctx.var("$god2").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true())
    {
        if ctx.var("god_eremes").get()? == 18 {
            ctx.lines_as("The Nineball", args!["Welcome to Jawaii,", "the paradise resort!"])?;
            ctx.next()?;
            ctx.lines_as("The Nineball", args!["I'm the 'The Nineball', the security officer of Jawaii! If you encounter any trouble, or find any Singles, please don't hesitate to report to me as soon as you can~"])?;
            ctx.next()?;
            ctx.lines_as(
                "The Nineball",
                args![
                    "I've been told that there have",
                    "been many unruly drunkards here",
                    "lately, but it is in our best interest to make your experience here as enjoyable as possible."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("god_eremes").get()?.number()? > 18 {
            if ctx.var("god_megin_5").get()?.number()? < 5 {
                ctx.lines_as("The Nineball", args!["Welcome to Jawaii,", "the paradise resort!"])?;
                ctx.next()?;
                ctx.lines_as("The Nineball", args!["I'm the 'The Nineball, the security officer of Jawaii! If you encounter any trouble, or find any Singles, please don't hesitate to report to me as soon as you can~"])?;
                ctx.next()?;
                ctx.lines_as("The Nineball", args!["I've been told that there have", "been many unruly drunkards here lately, but it is in our best interest to make your experience here as enjoyable as possible."])?;
                ctx.var("god_megin_5").set((ctx.var("god_megin_5").get()? + Val::from(1)))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("god_megin_5").get()? == 5 {
                ctx.mes("^3355FFHe acted very bright and friendly, but for a fleeting moment, you were able to glimpse a hint of sadness in his eyes.^000000")?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFIt doesn't seem that",
                    "you'll be able to get him",
                    "to talk about what exactly",
                    "happened in the 1st Squad...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as("The Nineball", args!["Welcome to Jawaii,", "the paradise resort!"])?;
        ctx.next()?;
        ctx.lines_as("The Nineball", args!["I'm the 'The Nineball, the security officer of Jawaii! If you encounter any trouble, or find any Singles, please don't hesitate to report to me as soon as you can~"])?;
        ctx.next()?;
        ctx.lines_as("The Nineball", args!["I've been told that there have", "been many unruly drunkards here lately, but it is in our best interest to make your experience here as enjoyable as possible."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn security_officer_megin(ctx: &Ctx) -> Script {
    security_officer_megin_body(ctx, Vec::new()).map(|_| ())
}
