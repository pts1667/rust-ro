use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn tialfi_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gift = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1301), Val::from(3)])? == 0 {
        ctx.mes("- You are carrying too many items! -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
        ctx.lines_as(
            "Tialfi",
            args![
                "Hmmm...",
                "I can feel a strange force growing stronger and stronger, somewhere in Midgard."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Tialfi", args!["Can you feel it?", "Something must be", "going on!"])?;
    } else {
        if ctx.var("god_mjo_0").get()? == 11 {
            ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
            ctx.lines_as("Tialfi", args!["I'm waiting for the day when I'll finally get to see Thor's thunder for myself. I believe that one of these days, my dream will become reality."])?;
        } else {
            if (((ctx.var("god_mjo_1").get()? == 2 && ctx.var("god_mjo_2").get()? == 2) && ctx.var("god_mjo_3").get()? == 2)
                && ctx.var("god_mjo_4").get()? == 2)
            {
                if ctx.var("god_mjo_0").get()? == 10 {
                    if (ctx.call(Function::CountItem, vec![Val::from(756)])?.number()? > 49
                        && ctx.call(Function::CountItem, vec![Val::from(757)])?.number()? > 49)
                    {
                        l_gift = Val::from(0);
                        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                            l_gift = Val::from(1);
                        } else {
                            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
                                l_gift = Val::from(2);
                            } else {
                                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
                                    l_gift = Val::from(3);
                                } else {
                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?) {
                                        l_gift = Val::from(4);
                                    } else {
                                        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?) {
                                            l_gift = Val::from(5);
                                        } else {
                                            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
                                                l_gift = Val::from(6);
                                            } else {
                                                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?) {
                                                    l_gift = Val::from(7);
                                                } else {
                                                    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?) {
                                                        l_gift = Val::from(8);
                                                    } else {
                                                        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
                                                            l_gift = Val::from(9);
                                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                                                        {
                                                            l_gift = Val::from(10);
                                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
                                                            l_gift = Val::from(11);
                                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_BARD")?) {
                                                            l_gift = Val::from(12);
                                                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_HUNTER")?) {
                                                            l_gift = Val::from(13);
                                                        } else {
                                                            l_gift = ctx.call(Function::Rand, vec![Val::from(1), Val::from(13)])?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "You came back!",
                                "In exchange for the ores that you have brought me, I will give you one of my family treasures."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["You have two options in choosing a treasure. You can have a treasure that will be useful to yourself, or something that may suit one of your friends."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["So which treasure", "would you like to have?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("An item that I can use.:An item that my friend can use.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Tialfi",
                                    args!["I see...", "Give me a moment", "to find a suitable", "item for you."],
                                )?;
                            }
                            2 => {
                                l_gift = ctx.call(Function::Rand, vec![Val::from(1), Val::from(13)])?;
                                ctx.lines_as(
                                    "Tialfi",
                                    args!["I see...", "Give me a moment", "to find a suitable", "item for your friend."],
                                )?;
                            }
                            _ => {}
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "Okay, let's see...",
                                "I seem to recall that I put it somewhere around... Hmm.",
                                "It must be around--Ah!",
                                "Here we are~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(756), Val::from(50)])?;
                        ctx.call(Function::DelItem, vec![Val::from(757), Val::from(50)])?;
                        ctx.var("god_mjo_0").set(Val::from(11))?;
                        if l_gift.clone() == 1 {
                            ctx.call(Function::GetItem, vec![Val::from(1471), Val::from(1)])?;
                        } else {
                            if l_gift.clone() == 2 {
                                ctx.call(Function::GetItem, vec![Val::from(1526), Val::from(1)])?;
                            } else {
                                if l_gift.clone() == 3 {
                                    ctx.call(Function::GetItem, vec![Val::from(1231), Val::from(1)])?;
                                } else {
                                    if l_gift.clone() == 4 {
                                        ctx.call(Function::GetItem, vec![Val::from(1367), Val::from(1)])?;
                                    } else {
                                        if l_gift.clone() == 5 {
                                            ctx.call(Function::GetItem, vec![Val::from(1722), Val::from(1)])?;
                                        } else {
                                            if l_gift.clone() == 6 {
                                                ctx.call(Function::GetItem, vec![Val::from(1230), Val::from(1)])?;
                                            } else {
                                                if l_gift.clone() == 7 {
                                                    ctx.call(Function::GetItem, vec![Val::from(1141), Val::from(1)])?;
                                                } else {
                                                    if l_gift.clone() == 8 {
                                                        ctx.call(Function::GetItem, vec![Val::from(1813), Val::from(1)])?;
                                                    } else {
                                                        if l_gift.clone() == 9 {
                                                            ctx.call(Function::GetItem, vec![Val::from(1557), Val::from(1)])?;
                                                        } else if l_gift.clone() == 10 {
                                                            ctx.call(Function::GetItem, vec![Val::from(1235), Val::from(1)])?;
                                                        } else if l_gift.clone() == 11 {
                                                            ctx.call(Function::GetItem, vec![Val::from(1227), Val::from(1)])?;
                                                        } else if l_gift.clone() == 12 {
                                                            ctx.call(Function::GetItem, vec![Val::from(1913), Val::from(1)])?;
                                                        } else if l_gift.clone() == 13 {
                                                            ctx.call(Function::GetItem, vec![Val::from(1963), Val::from(1)])?;
                                                        } else {
                                                            ctx.mes("Unknown error occurred.")?;
                                                            ctx.close_window()?;
                                                            ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(255)])?;
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
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                        ctx.lines_as("Tialfi", args!["Once again, I thank you for the trouble you've gone through on my behalf. I'm unsure of how this works, but I hope it will be useful to you. From what I know, I believe this is a rare item."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["I'll be waiting for the day when I'll see Thor's thunder for myself. I believe that the dream I've had will come true, one of these days."])?;
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                        ctx.lines_as("Tialfi", args!["I need", "50 Rough Oridecon", "and 50 Rough Elunium."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["I don't have much time, so go ahead and continue on your journeys if you can't bring those to me."],
                        )?;
                    }
                } else {
                    if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                        ctx.lines_as(
                            "Tialfi",
                            args!["I sense a strange energy growing more powerful somewhere on this continent..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["Can you feel it?", "Something must be going on!"])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["I think we'd better wait and see what's happening. Someone will deliver the news to us. Though, I am unsure of whether or not it will be good news or bad..."])?;
                    } else {
                        ctx.var("$god4").set((ctx.var("$god4").get()? + Val::from(1)))?;
                        if ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check1").get()?) {
                            ctx.call(
                                Function::Announce,
                                vec![Val::from("The 4th seal of [Mjolnir] has appeared."), ctx.constant("BC_ALL")?],
                            )?;
                        } else if ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check2").get()?) {
                            if (((ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                                && ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                                && ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                                && ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check2").get()?))
                            {
                                ctx.call(
                                    Function::Announce,
                                    vec![
                                        Val::from("Four seals have been released at the same time with the seal of [Mjolnir]."),
                                        ctx.constant("BC_ALL")?,
                                    ],
                                )?;
                            } else {
                                ctx.call(
                                    Function::Announce,
                                    vec![Val::from("The 4th seal of [Mjolnir] has been released."), ctx.constant("BC_ALL")?],
                                )?;
                            }
                        }
                        ctx.var("god_mjo_0").set(Val::from(10))?;
                        ctx.lines_as("Tialfi", args!["You've met the four Dwarven Blacksmiths. I've heard that they rarely speak to humans. So you must be special if you were able to talk to them."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["Do you think that my dream will come true? Do you believe Thor's Mjolnir will appear before the eyes of humans? I'm afraid great change will come to this world."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["Thank you for going through such trouble on my behalf. I wish to give you one of my family treasures as a token of my gratitude."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["However, I hope you understand that my family would grow suspicious if one of the treasures were to just go missing."])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["Hmm, however, I don't think they'll complain if I exchanged one of the treasures for something else. Let me think..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "I should be able to give you",
                                "one of our family heirlooms if you can bring be 50 Rough Oridecons and 50 Rough Eluniums."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["You don't have to do that if you don't want to. But it seems our family heirlooms would be more useful to an adventurer such as yourself..."])?;
                    }
                }
            } else {
                if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3) || ctx.var("god_mjo_3").get()? == 3)
                    || ctx.var("god_mjo_4").get()? == 3)
                {
                    ctx.lines_as(
                        "Tialfi",
                        args!["Hmm...", "The Dwarven Blacksmiths", "must be upset at you for", "some reason."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tialfi",
                        args![
                            "Remember my sister's",
                            "suggestion and speak to",
                            "them with great courtesy.",
                            "Carefully choose words",
                            "of respect lest they",
                            "be insulted."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("god_mjo_1").set(Val::from(0))?;
                    ctx.var("god_mjo_2").set(Val::from(0))?;
                    ctx.var("god_mjo_3").set(Val::from(0))?;
                    ctx.var("god_mjo_4").set(Val::from(0))?;
                    ctx.lines_as(
                        "Tialfi",
                        args![
                            "You should be okay now.",
                            "By this time, they've probably forgotten the insult. But make sure you speak to my sister for advice first."
                        ],
                    )?;
                } else if (ctx.var("god_mjo_0").get()? == 2 || ctx.var("god_mjo_0").get()? == 1) {
                    ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Tialfi",
                        args![
                            "Sorry for the trouble.",
                            "I wish you good luck",
                            "in finding the Dwarves!",
                            "Just... Don't insult them!"
                        ],
                    )?;
                } else if ctx.var("god_mjo_0").get()? == 0 {
                    if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["I sense a strange energy growing more powerful somewhere on this continent..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["Can you feel it?", "Something must be going on!"])?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["I think we'd better wait and see what's happening. Someone will deliver the news to us. Though, I am unsure of whether or not it will be good news or bad..."])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi02"), Val::from(2)])?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["One of my ancestors supposedly was a servant of Thor. Still, I find it difficult to believe."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["If the gods have all these powers, how could a mere human be of any real assistance?"],
                        )?;
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["Last night, I had the most amazing dream where I was the servant of Thor, god of thunder."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["In this dream, I traveled", "with Thor to Jotunnheim, land of giants. During our journey, he told me many interesting stories about gods and heroes."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args!["Of course, I can't remember everything clearly, but it was truly fantastic."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["For some reason, I can vividly recall what Thor told me about his weapon, Mjolnir. Mjolnir is a Dwarven masterpiece."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "Thor told me that Dwarves are extremely talented artisans, and their works are supreme. So as",
                                "I was thinking about my dream,",
                                "I remembered..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "There is a mountain that",
                                "has the same name as Thor's",
                                "weapon. Surely, the two have",
                                "some relation to each other."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "I've also recently heard a rumor that Dwarven Blacksmiths also reside on Mount Mjolnir.",
                                "I understand the mountain is dangerous, and that I'm in no position to ask such a thing..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Tialfi", args!["I can't help wanting to know for myself whether or not there is truth to my dream. Is it possible for Mjolnir to resurface?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tialfi",
                            args![
                                "If you don't mind, I'd like to ask you to explore this mountain and search for these Dwarven Blacksmiths."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("No.:Okay.")])? {
                            1 => {
                                ctx.call(Function::Cutin, vec![Val::from("god_tialpi02"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Tialfi",
                                    args![
                                        "I see. But I still appreciate that you took the time to listen to me.",
                                        "Hopefully someday I'll learn the truth about my dreams and about Mjolnir itself."
                                    ],
                                )?;
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                                ctx.lines_as("Tialfi", args!["Thank you,", "thank you so much!", "Even though it won't be easy, I have faith that if the Dwarven Blacksmiths do exist, you'll be able to find them."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tialfi",
                                    args!["Oh, and please speak to my sister Roskva first. She is outside the North Entrance of Prontera."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tialfi",
                                    args![
                                        "I'm sure that she can",
                                        "give you useful information if you're fortunate enough to encounter the Dwarves."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.var("god_mjo_0")
                                    .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?)?;
                                ctx.lines_as(
                                    "Tialfi",
                                    args!["I'm truly lucky to meet such an adventurer like yourself. I wish you the best of luck."],
                                )?;
                            }
                            _ => {}
                        }
                    }
                } else {
                    ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(2)])?;
                    ctx.lines_as("Tialfi", args!["I believe in you.", "Just be courageous!"])?;
                }
            }
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from("god_tialpi01"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn tialfi(ctx: &Ctx) -> Script {
    tialfi_body(ctx, Vec::new()).map(|_| ())
}

fn roskva_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("god_mjo_0").get()? == 1 {
        ctx.lines_as(
            "Roskva",
            args!["You should know that Dwarven Blacksmiths are extremely offended if you do not speak to them with the utmost respect."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Roskva",
            args![
                "So it's really",
                "important that you",
                "speak to the Dwarves",
                "as courteously as you can.",
                "The first Dwarf you must visit",
                "can be found to the East."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Roskva", args!["Travel in a clock wise direction around Midgard and seek out the other Dwarves in order. Your final destination will be to the North."])?;
        ctx.next()?;
        ctx.lines_as(
            "Roskva",
            args!["If you happen to speak to them in the wrong order, please go talk to my brother Tialfi again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("god_mjo_0").get()? == 2 {
        ctx.lines_as(
            "Roskva",
            args!["You should know that Dwarven Blacksmiths are extremely offended if you do not speak to them with the utmost respect."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Roskva",
            args![
                "So it's really",
                "important that you",
                "speak to the Dwarves",
                "as courteously as you can.",
                "The first Dwarf you must visit",
                "can be found to the North."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Roskva", args!["Travel in a counter clock wise direction around Midgard and seek out the other Dwarves in order. Your final destination will be to the East."])?;
        ctx.next()?;
        ctx.lines_as(
            "Roskva",
            args!["If you happen to speak to them in the wrong order, please go talk to my brother Tialfi again."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Roskva", args!["A long time ago, many people used to frequent this area. Friends and families would live here, sharing happiness and sadness."])?;
        ctx.next()?;
        ctx.lines_as(
            "Roskva",
            args!["But they're all gone now. My parents and my friends have all gone to a place whose name I don't even know."],
        )?;
        ctx.next()?;
        ctx.lines_as("Roskva", args!["I can't help", "but feel lonesome..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn roskva(ctx: &Ctx) -> Script {
    roskva_body(ctx, Vec::new()).map(|_| ())
}

fn dwarf_blacksmith_east_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_talk_not = Val::from(0);
    let mut l_talk_to = Val::from(0);
    if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as("Austri", args!["Something is happening somewhere on this continent. You might not believe me, but I keep getting visions of the Fenrir-Wolf."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
            ctx.lines_as("Austri", args!["Something is happening somewhere on this continent. You might not believe me, but I keep getting visions of the Fenrir-Wolf."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_mjo_0").get()? == 11 {
                ctx.lines_as("Austri", args!["When my people finally retrieve the memories of their past, we will be able to grant the power of the gods to humans."])?;
                ctx.next()?;
                ctx.lines_as("Austri", args!["I believe that", "time is coming..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_mjo_0").get()? == 10 {
                    ctx.lines_as("Austri", args!["Hm? I sense that you were asked to do a favor for that human. If I were you, I'd finish that task as soon as I could. Somehow, that human's fate, as well as that of the Dwarves, are intertwined..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_mjo_0").get()? == 1 {
                        if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3) || ctx.var("god_mjo_3").get()? == 3)
                            || ctx.var("god_mjo_4").get()? == 3)
                        {
                            ctx.lines_as(
                                "Austri",
                                args![
                                    "What is it?! I refuse to speak to human as rude and contemptible",
                                    "as you! Now, get out of my sight!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("god_mjo_1").get()? == 2 {
                                ctx.lines_as("Austri", args!["What...?", "I have nothing", "to say to you."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ((ctx.var("god_mjo_2").get()? != 0 || ctx.var("god_mjo_3").get()? != 0)
                                    || ctx.var("god_mjo_4").get()? != 0)
                                {
                                    ctx.lines_as("Austri", args!["Hm...?", "Why have you", "come to me, human?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Nothing.:Hey, 'sup!")])? {
                                        1 => {
                                            ctx.lines_as("Austri", args!["..."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.var("god_mjo_1").set(Val::from(3))?;
                                            ctx.lines_as("Austri", args!["What...?!", "Do not greet the", "Dwarves lightly,", "mortal!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    if ctx.var("god_mjo_1").get()? == 1 {
                                        ctx.lines_as("Austri", args!["What has made", "you come to me?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                            1 => {
                                                ctx.lines_as("Austri", args!["..."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                if ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 0 {
                                                    ctx.lines_as("Austri", args!["Ah, I see that the entire human race is not worthy of scorn. Unlike many of your kind, I see that you respect your elders."])?;
                                                    ctx.next()?;
                                                    'l3: loop {
                                                        if !(true) {
                                                            break 'l3;
                                                        }
                                                        'b3: {
                                                            if l_talk_to.clone() == 0 {
                                                                ctx.lines_as("Austri", args!["So what did", "want to ask", "me about?"])?;
                                                                ctx.next()?;
                                                                match runtime::select_values(ctx, &[Val::from("...:About Mjolnir.")])? {
                                                                    1 => {
                                                                        l_talk_not = Val::from(1);
                                                                    }
                                                                    2 => {}
                                                                    _ => {}
                                                                }
                                                            } else {
                                                                if l_talk_to.clone() == 1 {
                                                                    ctx.lines_as("Austri", args!["Mjolnir...?", "Thor's legendary weapon?", "The hammer than can shake the earth and tear the sky asunder? Out of all of legendary weapons, Mjolnir is perhaps greatest."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Austri", args!["As a matter of fact, it was forged by my ancestor. Mjolnir was the perfect weapon, except for one minor flaw."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Austri", args!["The hilt of Mjolnir was forged shorter than intended. Are you still listening to me?"])?;
                                                                    ctx.next()?;
                                                                    match runtime::select_values(ctx, &[Val::from("...:Yes, sir!:Huh?")])? {
                                                                        2 => {}
                                                                        _ => {
                                                                            l_talk_not = Val::from(1);
                                                                        }
                                                                    }
                                                                } else {
                                                                    if l_talk_to.clone() == 2 {
                                                                        ctx.lines_as("Austri", args!["One day, Loki came to our village and showed off his treasures. He boasted that we couldn't possibly create something to surpass their quality."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Austri", args!["Frankly his treasures were", "made by another Dwarf tribe, but we couldn't tolerate his insult. So my ancestors created three treasures of their own."])?;
                                                                        ctx.next()?;
                                                                        match runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from("...:Boooring!:Oh, wow.")],
                                                                        )? {
                                                                            3 => {}
                                                                            _ => {
                                                                                l_talk_not = Val::from(1);
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if l_talk_to.clone() == 3 {
                                                                            ctx.lines_as("Austri", args!["Of the three treasures my ancestors created, Mjolnir was the last and greatest."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Austri", args!["However, while it was created, a strange fly bit my ancestor on the hand. Because of this interruption, Mjolnir's hilt is a little flawed."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Austri", args!["It's very sad. Although Mjolnir is the greatest weapon ever, it was very close to being the epitome of craftsmanship."])?;
                                                                            ctx.next()?;
                                                                            match runtime::select_values(
                                                                                ctx,
                                                                                &[Val::from("...:Epito--what?:Yes sir, I agree.")],
                                                                            )? {
                                                                                3 => {}
                                                                                _ => {
                                                                                    l_talk_not = Val::from(1);
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if l_talk_to.clone() == 4 {
                                                                                ctx.lines_as("Austri", args!["Despite this minor flaw, Mjolnir is still considered the greatest of legendary weapons."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["Mjolnir was the trusted weapon Thor wielded on the battlefield, and every giant feared its power."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["I must say, a Blacksmith's greatest pride comes when he creates the weapon and armor that can be considered his life's work."])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("...:I agree, sir!:Um, yeah.")],
                                                                                )? {
                                                                                    2 => {}
                                                                                    _ => {
                                                                                        l_talk_not = Val::from(1);
                                                                                    }
                                                                                }
                                                                            } else if l_talk_to.clone() == 5 {
                                                                                ctx.lines_as("Austri", args!["The reason this mountain is called Mount Mjolnir is because it was actually created by the hammer."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["In a battle against demons a thousand years ago, Thor struck the earth with Mjolnir. The impact caused the ground to rise, creating this mountain."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["You can imagine just", "how powerful Mjolnir is.", "However, humans can never hope to see or even wield Mjolnir. Only a god can handle that kind of force."])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("...:Wah wah wah~!:Ah, I understand sir!")],
                                                                                )? {
                                                                                    3 => {}
                                                                                    _ => {
                                                                                        l_talk_not = Val::from(1);
                                                                                    }
                                                                                }
                                                                            } else if l_talk_to.clone() == 6 {
                                                                                ctx.lines_as("Austri", args!["Hmm... But perhaps an ambitious dwarf can forge something similar to Mjolnir so that it can actually be used by humans. It would have less power, but it'd be perfectly crafted."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["Yes, it's possible to create", "a Mjolnir suited to humans. Still, it wouldn't be very easy."])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("...:Yes, sir!:Yeah, whatever.")],
                                                                                )? {
                                                                                    2 => {}
                                                                                    _ => {
                                                                                        l_talk_not = Val::from(1);
                                                                                    }
                                                                                }
                                                                            } else if l_talk_to.clone() == 7 {
                                                                                ctx.lines_as("Austri", args!["Well, I happened to speak much longer than I intended. But I hope you learned what you wished to", "know about Mjolnir."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["I feel that a great change is coming. I do not know what kind", "of effect it will have on our world, but something important will happen..."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["Perhaps only the gods can be", "sure as to what the future will bring. In any case, we must prepare ourselves for what will happen."])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("...:Yes?:Yes, sir!")],
                                                                                )? {
                                                                                    3 => {}
                                                                                    _ => {
                                                                                        l_talk_not = Val::from(1);
                                                                                    }
                                                                                }
                                                                            } else if l_talk_to.clone() == 8 {
                                                                                ctx.var("god_mjo_1").set(Val::from(2))?;
                                                                                ctx.lines_as("Austri", args!["Alright then...", "If you wish to learn more, you should speak to my brothers.", "Take care, human."])?;
                                                                                ctx.close_window()?;
                                                                                return Err(Stop::End);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if l_talk_not.clone() == 1 {
                                                                ctx.var("god_mjo_1").set(Val::from(3))?;
                                                                ctx.lines_as(
                                                                    "Austri",
                                                                    args![
                                                                        "Grrr...!",
                                                                        "You're not listening, are you?! What a waste of my time!"
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Austri", args!["This is why I don't want to associate with humans. They always shut me out when I'm talking!"])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                l_talk_to = (l_talk_to.clone() + Val::from(1));
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    ctx.lines_as("Austri", args!["You don't seem to", "understand. If you", "wish to prove to me that you understand blacksmiths, you should bring something related to my work!"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            _ => {}
                                        }
                                    } else if ctx.var("god_mjo_1").get()? == 0 {
                                        ctx.lines_as("Austri", args!["What has made", "you come to me?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                            1 => {
                                                ctx.lines_as("Austri", args!["..."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Austri",
                                                    args![
                                                        "A respectable blacksmith cherishes his tools and crafts with diligence and care."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Austri", args!["When it comes to humans, I believe the ones who can appreciate my line of work are the only ones worth talking to."])?;
                                                ctx.next()?;
                                                ctx.var("god_mjo_1").set(Val::from(1))?;
                                                ctx.lines_as("Austri", args!["Every good blacksmith knows the value of a good hammer. If you can understand that, I shall consider speaking with you."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Austri", args!["Now go, human.", "I wish you safety", "in your travels."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        ctx.lines_as("Austri", args!["Zzzz Zzzz..."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    } else {
                        if ctx.var("god_mjo_0").get()? == 2 {
                            if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3)
                                || ctx.var("god_mjo_3").get()? == 3)
                                || ctx.var("god_mjo_4").get()? == 3)
                            {
                                ctx.lines_as(
                                    "Austri",
                                    args![
                                        "What is it?! I refuse to speak to human as rude and contemptible",
                                        "as you! Now, get out of my sight!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("god_mjo_4").get()? == 2 {
                                    ctx.lines_as("Austri", args!["What is it...?", "I have nothing", "to say to you."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if (((((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                                        || ctx.var("god_mjo_2").get()? == 0)
                                        || ctx.var("god_mjo_2").get()? == 1)
                                        || ctx.var("god_mjo_3").get()? == 0)
                                        || ctx.var("god_mjo_3").get()? == 1)
                                    {
                                        ctx.lines_as("Austri", args!["What made you come to me?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Hey, sup!")])? {
                                            1 => {
                                                ctx.lines_as("Austri", args!["..."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.var("god_mjo_4").set(Val::from(3))?;
                                                ctx.lines_as(
                                                    "Austri",
                                                    args!["What?!", "Leave immediately and go study your english properly!"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        if ctx.var("god_mjo_4").get()? == 1 {
                                            ctx.lines_as("Austri", args!["What has made", "you come to me?"])?;
                                            ctx.next()?;
                                            match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                                1 => {
                                                    ctx.lines_as("Austri", args!["..."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    if ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 0 {
                                                        ctx.lines_as("Austri", args!["Ah, I see that the entire human race is not worthy of scorn. Unlike many of your kind, I see that you respect your elders."])?;
                                                        ctx.next()?;
                                                        'l15: loop {
                                                            if !(true) {
                                                                break 'l15;
                                                            }
                                                            'b15: {
                                                                if l_talk_to.clone() == 0 {
                                                                    ctx.lines_as(
                                                                        "Austri",
                                                                        args!["So what did", "want to ask", "me about?"],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    match runtime::select_values(ctx, &[Val::from("...:About Mjolnir.")])? {
                                                                        1 => {
                                                                            l_talk_not = Val::from(1);
                                                                        }
                                                                        2 => {}
                                                                        _ => {}
                                                                    }
                                                                } else {
                                                                    if l_talk_to.clone() == 1 {
                                                                        ctx.lines_as("Austri", args!["Mjolnir...?", "Thor's legendary weapon?", "The hammer than can shake the earth and tear the sky asunder? Out of all of legendary weapons, Mjolnir is perhaps greatest."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Austri", args!["As a matter of fact, it was forged by my ancestor. Mjolnir was the perfect weapon, except for one minor flaw."])?;
                                                                        ctx.next()?;
                                                                        ctx.lines_as("Austri", args!["The hilt of Mjolnir was forged shorter than intended. Are you still listening to me?"])?;
                                                                        ctx.next()?;
                                                                        match runtime::select_values(
                                                                            ctx,
                                                                            &[Val::from("...:Yes, sir!:Huh?")],
                                                                        )? {
                                                                            2 => {}
                                                                            _ => {
                                                                                l_talk_not = Val::from(1);
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if l_talk_to.clone() == 2 {
                                                                            ctx.lines_as("Austri", args!["One day, Loki came to our village and showed off his treasures. He boasted that we couldn't possibly create something to surpass their quality."])?;
                                                                            ctx.next()?;
                                                                            ctx.lines_as("Austri", args!["Frankly his treasures were", "made by another Dwarf tribe, but we couldn't tolerate his insult. So my ancestors created three treasures of their own."])?;
                                                                            ctx.next()?;
                                                                            match runtime::select_values(
                                                                                ctx,
                                                                                &[Val::from("...:Boooring!:Oh, wow.")],
                                                                            )? {
                                                                                1 => {
                                                                                    l_talk_not = Val::from(1);
                                                                                }
                                                                                2 => {
                                                                                    l_talk_not = Val::from(1);
                                                                                }
                                                                                3 => {}
                                                                                _ => {}
                                                                            }
                                                                        } else {
                                                                            if l_talk_to.clone() == 3 {
                                                                                ctx.lines_as("Austri", args!["Of the three treasures my ancestors created, Mjolnir was the last and greatest."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["However, while it was created, a strange fly bit my ancestor on the hand. Because of this interruption, Mjolnir's hilt is a little flawed."])?;
                                                                                ctx.next()?;
                                                                                ctx.lines_as("Austri", args!["It's very sad. Although Mjolnir is the greatest weapon ever, it was very close to being the epitome of craftsmanship."])?;
                                                                                ctx.next()?;
                                                                                match runtime::select_values(
                                                                                    ctx,
                                                                                    &[Val::from("...:Epito--what?:Yes sir, I agree.")],
                                                                                )? {
                                                                                    3 => {}
                                                                                    _ => {
                                                                                        l_talk_not = Val::from(1);
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if l_talk_to.clone() == 4 {
                                                                                    ctx.lines_as("Austri", args!["Despite this minor flaw, Mjolnir is still considered the greatest of legendary weapons."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["Mjolnir was the trusted weapon Thor wielded on the battlefield, and every giant feared its power."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["I must say, a Blacksmith's greatest pride comes when he creates the weapon and armor that can be considered his life's work."])?;
                                                                                    ctx.next()?;
                                                                                    match runtime::select_values(
                                                                                        ctx,
                                                                                        &[Val::from("...:I agree, sir!:Um, yeah.")],
                                                                                    )? {
                                                                                        2 => {}
                                                                                        _ => {
                                                                                            l_talk_not = Val::from(1);
                                                                                        }
                                                                                    }
                                                                                } else if l_talk_to.clone() == 5 {
                                                                                    ctx.lines_as("Austri", args!["The reason this mountain is called Mount Mjolnir is because it was actually created by the hammer."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["In a battle against demons a thousand years ago, Thor struck the earth with Mjolnir. The impact caused the ground to rise, creating this mountain."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["You can imagine just", "how powerful Mjolnir is.", "However, humans can never hope to see or even wield Mjolnir. Only a god can handle that kind of force."])?;
                                                                                    ctx.next()?;
                                                                                    match runtime::select_values(
                                                                                        ctx,
                                                                                        &[Val::from(
                                                                                            "...:Wah wah wah~!:Ah, I understand sir!",
                                                                                        )],
                                                                                    )? {
                                                                                        3 => {}
                                                                                        _ => {
                                                                                            l_talk_not = Val::from(1);
                                                                                        }
                                                                                    }
                                                                                } else if l_talk_to.clone() == 6 {
                                                                                    ctx.lines_as("Austri", args!["Hmm... But perhaps an ambitious dwarf can forge something similar to Mjolnir so that it can actually be used by humans. It would have less power, but it'd be perfectly crafted."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["Yes, it's possible to create", "a Mjolnir suited to humans. Still, it wouldn't be very easy."])?;
                                                                                    ctx.next()?;
                                                                                    match runtime::select_values(
                                                                                        ctx,
                                                                                        &[Val::from("...:Yes, sir!:Yeah, whatever.")],
                                                                                    )? {
                                                                                        2 => {}
                                                                                        _ => {
                                                                                            l_talk_not = Val::from(1);
                                                                                        }
                                                                                    }
                                                                                } else if l_talk_to.clone() == 7 {
                                                                                    ctx.lines_as("Austri", args!["Well, I happened to speak much longer than I intended. But I hope you learned what you wished to", "know about Mjolnir."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["I feel that a great change is coming. I do not know what kind", "of effect it will have on our world, but something important will happen..."])?;
                                                                                    ctx.next()?;
                                                                                    ctx.lines_as("Austri", args!["Perhaps only the gods can be", "sure as to what the future will bring. In any case, we must prepare ourselves for what will happen."])?;
                                                                                    ctx.next()?;
                                                                                    match runtime::select_values(
                                                                                        ctx,
                                                                                        &[Val::from("...:Yes?:Yes, sir!")],
                                                                                    )? {
                                                                                        3 => {}
                                                                                        _ => {
                                                                                            l_talk_not = Val::from(1);
                                                                                        }
                                                                                    }
                                                                                } else if l_talk_to.clone() == 8 {
                                                                                    ctx.var("god_mjo_4").set(Val::from(2))?;
                                                                                    ctx.lines_as(
                                                                                        "Austri",
                                                                                        args![
                                                                                            "Alright then...",
                                                                                            "Take care of",
                                                                                            "yourself, human."
                                                                                        ],
                                                                                    )?;
                                                                                    ctx.close_window()?;
                                                                                    return Err(Stop::End);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if l_talk_not.clone() == 1 {
                                                                    ctx.var("god_mjo_4").set(Val::from(3))?;
                                                                    ctx.lines_as(
                                                                        "Austri",
                                                                        args![
                                                                            "Grrr...!",
                                                                            "You're not listening, are you?! What a waste of my time!"
                                                                        ],
                                                                    )?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Austri", args!["This is why I don't want to associate with humans. They always shut me out when I'm talking!"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                } else {
                                                                    l_talk_to = (l_talk_to.clone() + Val::from(1));
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        ctx.lines_as("Austri", args!["You don't seem to", "understand. If you", "wish to prove to me that you understand blacksmiths, you should bring something related to my work!"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        } else if ctx.var("god_mjo_4").get()? == 0 {
                                            ctx.lines_as("Austri", args!["What made you come to me?"])?;
                                            ctx.next()?;
                                            match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me, sir.")])? {
                                                1 => {
                                                    ctx.lines_as("Austri", args!["..."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as("Austri", args!["A respectable blacksmith cherishes his tools and crafts with diligence and care."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Austri", args!["When it comes to humans, I believe the ones who can appreciate my line of work are the only ones worth talking to."])?;
                                                    ctx.next()?;
                                                    ctx.var("god_mjo_4").set(Val::from(1))?;
                                                    ctx.lines_as("Austri", args!["Every good blacksmith knows the value of a good hammer. If you can understand that, I shall consider speaking with you."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Austri",
                                                        args!["Now go, human.", "I wish you safety", "in your travels."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            ctx.lines_as("Austri", args!["Zzzz Zzzz..."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                            }
                        } else {
                            if ctx.var("god_mjo_0").get()? == 0 {
                                ctx.lines_as("Austri", args!["Ah...", "It feels like today's going to be a great day."])?;
                                ctx.next()?;
                                ctx.lines_as("Austri", args!["I've got the warm sun, fresh forest air, and my hammer and anvil are at the ready. It's a perfect day for smithing!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Austri", args!["Zzzz Zzzz..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn dwarf_blacksmith_east(ctx: &Ctx) -> Script {
    dwarf_blacksmith_east_body(ctx, Vec::new()).map(|_| ())
}
