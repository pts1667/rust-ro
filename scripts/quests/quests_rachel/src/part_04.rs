use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum NemmaRaTempleStep {
    Start,
    SDonate,
}

fn nemma_ra_temple_run(ctx: &Ctx, mut step: NemmaRaTempleStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_num = Val::from(0);
    let mut l_remaining = Val::from(0);
    'machine: loop {
        match step {
            NemmaRaTempleStep::Start => {
                ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                if ctx.var("$rachel_donate").get()?.number()? < 10000 {
                    if ctx.var("ra_have_donated").get()? == 0 {
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args!["Good day, adventurer.", "May I ask what brings", "you to the temple today?"],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Just sight-seeing.:I came to pray.")])?) == 1 {
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "I guess most foreigners",
                                    "aren't familiar with our",
                                    "faith, so I suppose they",
                                    "wouldn't come here to pray...",
                                    "Well, I hope you enjoy your",
                                    "time here, adventurer."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Oh! I'm glad to see that",
                                "our kind of spirituality is",
                                "practiced in other countries~",
                                "You know, we at the Temple of",
                                "Cheshrumnir will be hosting a",
                                "festival here at the temple."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "We're accepting donations",
                                "for the festival, so we'd be",
                                "grateful if you could make",
                                "a contribution. Regardless,",
                                "we invite you to celebrate the",
                                "grace of the goddess with us."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "There's one High Priest that",
                                "disagrees with collecting money",
                                "from the temple's patrons, but",
                                "it's up to you if you want to",
                                "give or not. If you donate, you",
                                "might be able to win a prize!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "You can use our Lottery Tickets",
                                "to win something nice from our",
                                "Temple Storage. However, you",
                                "donate the 50,000 zeny for each",
                                "Lottery Ticket, which also happens to be the minimum donation amount."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "The prizes that are given to",
                                "donators are also randomly",
                                "chosen, so I have no way of",
                                "telling you what you'd get.",
                                "Would you be interested",
                                "in making a donation?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("No, thanks.:Sure.")])?) == 1 {
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "I understand. Well,",
                                    "maybe some other time.",
                                    "May Freya bless you on",
                                    "all your journeys. May the",
                                    "grace of the goddess always",
                                    "support us in all that we do~"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Great! How much would you",
                                "like to donate? We can only",
                                "accept donations in increments",
                                "of 50,000 zeny, and we can only",
                                "accept up to 150,000 zeny at once. It's a bit complicated, I know..."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "50,000z - 1 Lottery Ticket:100,000z - 2 Lottery Ticket:150,000z - 3 Lottery Ticket:Cancel",
                            )],
                        )? {
                            1 => {
                                nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(1), Val::from("50,000")])?;
                            }
                            2 => {
                                nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(2), Val::from("100,000")])?;
                            }
                            3 => {
                                nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(3), Val::from("150,000")])?;
                            }
                            4 => {
                                ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "I understand that it takes",
                                        "some thought to part with an",
                                        "amount of money like 50,000",
                                        "zeny. Even so, Freya is always",
                                        "protecting you, wherever you go~"
                                    ],
                                )?;
                            }
                            _ => {}
                        }
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                ((Val::from("Oh, you're ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "Welcome back! I remember",
                                "that you donated just a little",
                                "while ago. So what brings you",
                                "to the temple today, hmm?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "I've come to donate again.:Just sight-seeing.:I came to attend the service.:I just wanted to see you again.",
                            )],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "You've come to make another",
                                        "donation? Splendid! How much",
                                        "would you like to donate this",
                                        "time? Remember that we can only",
                                        "accept donations in increments of",
                                        "50,000 zeny up to 150,000 zeny."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from(
                                        "50,000z - 1 Lottery Ticket:100,000z - 2 Lottery Ticket:150,000z - 3 Lottery Ticket:Cancel",
                                    )],
                                )? {
                                    1 => {
                                        nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(1), Val::from("50,000")])?;
                                    }
                                    2 => {
                                        nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(2), Val::from("100,000")])?;
                                    }
                                    3 => {
                                        nemma_ra_temple_run(ctx, NemmaRaTempleStep::SDonate, vec![Val::from(3), Val::from("150,000")])?;
                                    }
                                    4 => {
                                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Priestess Nemma",
                                            args![
                                                "I understand that it takes",
                                                "some thought to part with an",
                                                "amount of money like 50,000",
                                                "zeny. Even so, Freya is always",
                                                "protecting you, wherever you go~"
                                            ],
                                        )?;
                                    }
                                    _ => {}
                                }
                                ctx.close_window()?;
                                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "Oh! If you'd like some",
                                        "information about our temple,",
                                        "why don't you ask Priestess",
                                        "Pano at the Help Desk inside?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "Penno will also redeem",
                                        "your Lottery Tickets for",
                                        "prizes, although she doesn't",
                                        "really seem to enjoy that job.",
                                        "Well, anyway, she'll help",
                                        "you out. See you later!"
                                    ],
                                )?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "You're here for the",
                                        "service? That's great~",
                                        "I hope you find the sense",
                                        "of calm that you can only",
                                        "get from goddess Freya~"
                                    ],
                                )?;
                            }
                            4 => {
                                ctx.lines_as("Priestess Nemma", args!["Oh, how sweet of you~"])?;
                            }
                            _ => {}
                        }
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                } else {
                    if runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true() {
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                        ctx.lines_as("Priestess Nemma", args!["Welcome to our temple!"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    if ctx.var("ra_tem_q").get()?.number()? < 1 {
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Hello, there!",
                                "...............................",
                                "Um, for some reason, the",
                                "temple gate hasn't opened yet",
                                "But it should be open now. Well",
                                "this happens sometimes so..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "See, there's this auto locking",
                                "system that was added to the",
                                "gate when this temple was",
                                "built, but now it's more of",
                                "an annoyance than security."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Every time someone messes",
                                "with the lock, or if it breaks,",
                                "we have to wait until it auto",
                                "resets itself before it works",
                                "again. No one can fix it",
                                "really quickly, you know..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "^333333*Sniff*^000000 What should I do?",
                                "The auto reset never takes",
                                "this long, and I really need",
                                "to go inside. ^333333*Sob*^000000 P-Panno!",
                                "Panno, I neeeed heeeeelp~!"
                            ],
                        )?;
                        ctx.var("ra_tem_q").set(Val::from(1))?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    } else {
                        if (ctx.var("ra_tem_q").get()?.number()? >= 1 && ctx.var("ra_tem_q").get()?.number()? < 10) {
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "If the gate's locked, then",
                                    "I've just got to find that",
                                    "secret entrance. Some kids",
                                    "supposedly use it to enter",
                                    "and play pranks in the temple,",
                                    "but I don't know who they are..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args!["Panno would know", "what to do... I think.", "Ooh, Panno, help me!"],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else if ctx.var("ra_tem_q").get()? == 10 {
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma04"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                    "The gate just opened and",
                                    "I was able to get inside",
                                    "the temple! ^333333*Whew*^000000 I was",
                                    "getting really worried",
                                    "about it for awhile."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "This is the first time that the",
                                    "gate locked up since Panno",
                                    "was assigned to her post in",
                                    "the temple. I thought maybe",
                                    "something happened and",
                                    "she got in trouble, you know?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "Wait, wait...",
                                    "I'm outside. What if",
                                    "the gate locks up again?",
                                    "Oooh, what should I do?!"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else if ctx.var("ra_tem_q").get()? == 11 {
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~!")),
                                    "We reached our target!",
                                    "Isn't that great? We're no",
                                    "longer asking for donations,",
                                    "but you can still redeem your",
                                    "Lottery Tickets inside, okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else if ctx.var("ra_tem_q").get()? == 12 {
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "Mmm... I know what",
                                    "I can do! I can ask for",
                                    "Firecrackers! Yes, that'll",
                                    "be fun for the festival.",
                                    "Hey, do you know where",
                                    "those things are sold?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "I, uh, can't give you",
                                    "much... But if you would",
                                    "bring me some Firecrackers,",
                                    "then I'll pray for your good",
                                    "fortune. Sorry, but that's",
                                    "all I can really offer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma04"), Val::from(2)])?;
                            ctx.lines_as(
                                "Priestess Nemma",
                                args![
                                    "Hey, bring me",
                                    "a lot of Firecrackers,",
                                    "like, ^FF000020 of them^000000! Thank",
                                    "you thank you thank you!"
                                ],
                            )?;
                            ctx.var("ra_tem_q").set(Val::from(13))?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else if ctx.var("ra_tem_q").get()? == 13 {
                            ctx.lines_as(
                                "Priestess Nemma",
                                args!["Hey, did you bring me", "^FF000020 Firecrackers^000000? Mmm?"],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No, not yet.")])?) == 1 {
                                ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                                ctx.lines_as("Priestess Nemma", args!["Let's see..."])?;
                                ctx.next()?;
                                if ctx.call(Function::CountItem, vec![Val::from(12018)])?.number()? >= 20 {
                                    ctx.call(Function::Cutin, vec![Val::from("ra_nemma04"), Val::from(2)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(12018), Val::from(20)])?;
                                    ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                                    ctx.var("ra_tem_q").set(Val::from(14))?;
                                    ctx.lines_as(
                                        "Priestess Nemma",
                                        args!["Yay!", "Firecrackers!", "Oh, let me pray", "for your good fortune!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines(args![
                                        "^3355FFPriestess Nemma prayed",
                                        "fervently for your good",
                                        "fortune. It feels like",
                                        "it's actually working...^000000"
                                    ])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("ra_nemma04"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Priestess Nemma",
                                        args![
                                            "I'm done!",
                                            "Now it's time",
                                            "to play! Hm, maybe",
                                            "our pope would like",
                                            "to see these too?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args![
                                        "Awww...",
                                        "This isn't enough",
                                        "Firecrackers. I mean,",
                                        "I know it's like a donation,",
                                        "but still. How much fun are",
                                        "fireworks if there isn't a lot?"
                                    ],
                                )?;
                            } else {
                                ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Priestess Nemma",
                                    args!["Awww...", "Well, I don't really", "have much to pay you", "back with, anyway~"],
                                )?;
                            }
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        } else {
                            ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                            ctx.lines_as("Priestess Nemma", args!["Welcome to our temple!"])?;
                        }
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                    return Err(Stop::End);
                }
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                return Err(Stop::End);
            }
            NemmaRaTempleStep::SDonate => {
                l_num = runtime::arg(&args, 0, Val::from(0));
                ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                ctx.lines_as(
                    "Priestess Nemma",
                    args![
                        ((Val::from("So, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                        "I just want to make sure:",
                        (Val::from("You want to donate ") + runtime::arg(&args, 1, Val::from(0))),
                        ((Val::from("zeny, and receive ") + l_num.clone()) + Val::from(" Lottery"))
                    ],
                )?;
                if l_num.clone() == 1 {
                    ctx.mes("Ticket. Is that correct?")?;
                } else {
                    ctx.mes("Tickets. Is that correct?")?;
                }
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("No:Yes")])?) == 1 {
                    ctx.lines_as(
                        "Priestess Nemma",
                        args![
                            "Oh, I see. Well, if you",
                            "don't have enough zeny with",
                            "you, then you can just come",
                            "back and donate later. Our",
                            "goddess Freya smiles on the",
                            "generous and rewards the patient!"
                        ],
                    )?;
                    return Ok(Val::from(0));
                }
                if runtime::op(&ctx.var("Zeny").get()?, ">=", &(l_num.clone().try_mul(Val::from(50000))?))?.is_true() {
                    if !(ctx.call(Function::CheckWeight, vec![Val::from(7570), l_num.clone()])?.is_true()) {
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma02"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "I can scarcely believe it...",
                                "You're carrying so much stuff,",
                                "you don't even have enough",
                                "room for a Lottery Ticket.",
                                "You'd better put some of",
                                "your things in Storage, yes?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma01"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Don't worry, I'll still be",
                                "here after you make more",
                                "space available in your",
                                "Inventory. Hurry back, and",
                                "donate if you can, okay?"
                            ],
                        )?;
                        return Ok(Val::from(0));
                    }
                    ctx.var("$rachel_donate").set((ctx.var("$rachel_donate").get()? + l_num.clone()))?;
                    ctx.call(Function::GetItem, vec![Val::from(7570), l_num.clone()])?;
                    ctx.var("Zeny")
                        .set((ctx.var("Zeny").get()?.try_sub((l_num.clone().try_mul(Val::from(50000))?))?))?;
                    ctx.var("ra_have_donated").set(Val::from(1))?;
                    if ctx.var("$rachel_donate").get()?.number()? > 9999 {
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "There you are~",
                                "Thanks so much for",
                                "your donation! I'm sure",
                                "that Freya is smiling down",
                                "upon you, and will reward",
                                "you for your generosity~"
                            ],
                        )?;
                        return Ok(Val::from(0));
                    } else {
                        l_remaining = (Val::from(10000).try_sub(ctx.var("$rachel_donate").get()?)?);
                        ctx.call(Function::Cutin, vec![Val::from("ra_nemma03"), Val::from(2)])?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "Thank you so much! We'll be",
                                "continuing to accept donations",
                                "until we reach our target. Once",
                                ((Val::from("we receive ") + l_remaining.clone()) + Val::from(" more donations")),
                                "in increments of 50,000 zeny,",
                                "our fundraiser will finish."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Priestess Nemma",
                            args![
                                "If you're feeling so",
                                "inclined, come back later",
                                "and make another contribution.",
                                "Thanks again, and may Freya",
                                "always watch over you."
                            ],
                        )?;
                        return Ok(Val::from(0));
                    }
                } else {
                    ctx.lines_as(
                        "Priestess Nemma",
                        args![
                            "I'm sorry, but you have",
                            ((Val::from("less than ") + (l_num.clone().try_mul(Val::from(50000))?)) + Val::from(" zeny...")),
                            "I know it's asking a lot,",
                            "but those are the rules that",
                            "I've been told to follow, so...",
                            "Well, maybe another time, yes?"
                        ],
                    )?;
                    return Ok(Val::from(0));
                }
            }
        }
    }
}

pub fn nemma_ra_temple(ctx: &Ctx) -> Script {
    nemma_ra_temple_run(ctx, NemmaRaTempleStep::Start, Vec::new()).map(|_| ())
}

fn kid_candy_addict_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("ra_tem_q").get()?.number()? < 2 || ctx.var("ra_tem_q").get()?.number()? > 9)
        || runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(8192))?.is_true())
    {
        ctx.lines_as(
            "Kid",
            args![
                "Hey! What's that",
                "smile for? You're not",
                "gonna come pat my head,",
                "are you? No! I hate that!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kid",
            args![
                "Don't come here!",
                "I don't want you to",
                "pat my head! Grrr...",
                "I-I'm warning you!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Hey, look at your", "clothes! Where did", "you come from, huh?"])?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Are you from a", "different country?"])?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Huh, huh?", "What country", "are you from?"])?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Eh, whatever."])?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Oh hey, do", "you like candy?"])?;
        if ctx.var("ra_tem_q").get()? == 1 {
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Not at all.:Yeah.:Sure, I love the stuff~:Not much.")])? {
                1 => {
                    ctx.lines_as("Kid", args!["Hmpf! Okay."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {}
                3 => {}
                4 => {
                    ctx.lines_as(
                        "Kid",
                        args!["Oh. It's 'cause you're", "a grown-up. Why don't", "you like candies, anyway?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kid", args!["Why, huh?", "Tell me, tell", "me, how come?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? != 2 {
                ctx.lines_as("Kid", args!["Oh yeah...?"])?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["But you're a grown-up,", "aren't you? Heh heh heh!"])?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["Candy is only for", "kids! Hahahahaah!"])?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["I tricked you!", "You big big dummy!"])?;
            } else {
                ctx.lines_as(
                    "Kid",
                    args!["Oh, you do?", "Does that mean", "that you have", "any candy, then?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["I want one!"])?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["Hurry, give me one!", "Gimmie gimmie~!"])?;
                ctx.next()?;
                if ctx.call(Function::CountItem, vec![Val::from(529)])?.number()? > 0 {
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Mmm...",
                            "If you give me",
                            "some Candy, then I'll",
                            "tell you something",
                            "really cool, yeah?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:No.")])?) == 1 {
                        ctx.lines_as("Kid", args!["Yay~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Hah! Got you!",
                                "I didn't want any",
                                "candy anyway! Nyeh!",
                                "But I like you, so",
                                "I can tell you a secret!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Did you know that",
                                "they always leave one",
                                "of the windows to the temple",
                                "unlocked? You spy on the",
                                "priests inside. They're always",
                                "fighting! Isn't that bad?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Oh well, they're adults,",
                                "so I dunno. Maybe they...",
                                "Mm. Well, I remember that",
                                "fighting is supposed to be bad."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Awww, nuts!",
                                "I just remembered!",
                                "If you have candy, it",
                                "must be from another",
                                "country! I should have",
                                "taken it from you. Dang it!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kid", args!["Oooh, I hate you", "now! Leave me alone!"])?;
                        ctx.var("ra_tem_q").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Kid",
                        args![
                            "You don't wanna",
                            "share your candy?",
                            "Hmpf! I guess you",
                            "must really love it.",
                            "...You big greedy."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Kid",
                    args![
                        "Wait, you don't have",
                        "any Candy, huh? How can",
                        "you tell me you like Candy",
                        "when you don't carry any?",
                        "Look at this! My pockets",
                        "are full of candy!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["See?", "Look at all", "this candy!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kid",
                    args![
                        "Yeah, you have to",
                        "have at least this",
                        "much candy to say",
                        "that you like them.",
                        "Hee hee hee hee!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["Oh hey...", "Would you like to", "have some candy?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kid",
                    args![
                        "Haha! I tricked you!",
                        "I'm not gonna share",
                        "all this candy with you!",
                        "All of it is for me!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["...............................", "Um... Are you mad?"])?;
                ctx.next()?;
                ctx.lines_as("Kid", args!["Oh...", "You are mad", "at me, aren't you?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kid",
                    args![
                        "Okay, okay.",
                        "I'll tell you a really",
                        "cool secret, so promise",
                        "that you won't be mad anymore."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No, thanks.:What's that?")])? {
                    1 => {
                        ctx.lines_as("Kid", args!["Please~"])?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Did you know that",
                                "they always leave one",
                                "of the windows to the temple",
                                "unlocked? You spy on the",
                                "priests inside. They're always",
                                "fighting! Isn't that bad?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Oh well, they're adults,",
                                "so I dunno. Maybe they...",
                                "Mm. Well, I remember that",
                                "fighting is supposed to be bad..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "Now, I hope you",
                                "won't be mad at",
                                "me anymore...",
                                "If you're not,",
                                "then I'm gonna",
                                "be madder at you!"
                            ],
                        )?;
                        ctx.var("ra_tem_q").set(Val::from(2))?;
                    }
                    _ => {}
                }
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ra_tem_q").get()? == 2 {
        ctx.lines_as(
            "Kid",
            args!["You're leaving?", "Don't you want to", "watch me play Rock,", "Paper, Scissors?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kid",
            args![
                "Hey! I don't like",
                "the way you're looking",
                "at me. I don't like you",
                "watching me like that!",
                "Why don't you go away?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kid", args!["Gosh, I said, go away!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Kid",
            args![
                "If you're that bored,",
                "why don't you peep through",
                "the windows in the temple,",
                "huh? Gosh! Some people!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn kid_candy_addict(ctx: &Ctx) -> Script {
    kid_candy_addict_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zawa00Step {
    Start,
    OnTouch,
}

fn zawa00_run(ctx: &Ctx, mut step: Zawa00Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zawa00Step::Start => {
                step = Zawa00Step::OnTouch;
                continue 'machine;
            }
            Zawa00Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 2 {
                    ctx.lines(args![
                        "^3355FFYou notice a slightly",
                        "open window that",
                        "you can easily enter.^000000"
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Stay:Enter Through Window")])?) == 2 {
                        ctx.lines(args![
                            "^3355FFYou pull the window,",
                            "and it smoothly opens.",
                            "This is probably what",
                            "those mischevious hits",
                            "have been using to enter",
                            "unnoticed into this temple.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_rachel"), Val::from(62), Val::from(82)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^3355FFYou decide that it's",
                        "wrong to sneak into",
                        "a place. But what if",
                        "you don't get caught?^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if (ctx.var("ra_tem_q").get()?.number()? >= 3 && ctx.var("ra_tem_q").get()?.number()? < 10) {
                    ctx.lines(args![
                        "^3355FFWould you like to",
                        "enter the temple",
                        "through this window?^000000"
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Enter:Cancel")])?) == 1 {
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_rachel"), Val::from(62), Val::from(82)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^3355FFUsing this window is",
                        "probably the only way",
                        "that you can enter.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zawa00(ctx: &Ctx) -> Script {
    zawa00_run(ctx, Zawa00Step::Start, Vec::new()).map(|_| ())
}

pub fn zawa00_ontouch(ctx: &Ctx) -> Script {
    zawa00_run(ctx, Zawa00Step::OnTouch, Vec::new()).map(|_| ())
}

fn window_ra_temple_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("^3355FFThis window is open.^000000")?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Stay:Exit Through Window")])?) == 2 {
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("ra_temple"), Val::from(73), Val::from(208)])?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn window_ra_temple(ctx: &Ctx) -> Script {
    window_ra_temple_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zawa01Step {
    Start,
    OnTouch,
}

fn zawa01_run(ctx: &Ctx, mut step: Zawa01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zawa01Step::Start => {
                step = Zawa01Step::OnTouch;
                continue 'machine;
            }
            Zawa01Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 2 {
                    ctx.lines(args!["^3355FFIt's strangely", "dark in here.^000000"])?;
                    ctx.var("ra_tem_q").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zawa01(ctx: &Ctx) -> Script {
    zawa01_run(ctx, Zawa01Step::Start, Vec::new()).map(|_| ())
}

pub fn zawa01_ontouch(ctx: &Ctx) -> Script {
    zawa01_run(ctx, Zawa01Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zawa02Step {
    Start,
    OnTouch,
}

fn zawa02_run(ctx: &Ctx, mut step: Zawa02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zawa02Step::Start => {
                step = Zawa02Step::OnTouch;
                continue 'machine;
            }
            Zawa02Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 3 {
                    ctx.lines(args![
                        "^3355FFYou hear some",
                        "noise from the hallway",
                        "towards the chapel. It",
                        "sounds like there are",
                        "several other people here.^000000"
                    ])?;
                    ctx.var("ra_tem_q").set(Val::from(4))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zawa02(ctx: &Ctx) -> Script {
    zawa02_run(ctx, Zawa02Step::Start, Vec::new()).map(|_| ())
}

pub fn zawa02_ontouch(ctx: &Ctx) -> Script {
    zawa02_run(ctx, Zawa02Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zawa03Step {
    Start,
    OnTouch,
}

fn zawa03_run(ctx: &Ctx, mut step: Zawa03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zawa03Step::Start => {
                step = Zawa03Step::OnTouch;
                continue 'machine;
            }
            Zawa03Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 4 {
                    ctx.lines(args!["^3355FFYou hear noises of some", "commotion from the stairs.^000000"])?;
                    ctx.var("ra_tem_q").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zawa03(ctx: &Ctx) -> Script {
    zawa03_run(ctx, Zawa03Step::Start, Vec::new()).map(|_| ())
}

pub fn zawa03_ontouch(ctx: &Ctx) -> Script {
    zawa03_run(ctx, Zawa03Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Zawa04Step {
    Start,
    OnTouch,
}

fn zawa04_run(ctx: &Ctx, mut step: Zawa04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Zawa04Step::Start => {
                step = Zawa04Step::OnTouch;
                continue 'machine;
            }
            Zawa04Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 5 {
                    ctx.lines(args![" ", " ", "^ff0000Crash!^000000"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou hear something",
                        "fall from the stairs,",
                        "followed by noises that",
                        "sound like a scuffle.^000000"
                    ])?;
                    ctx.var("ra_tem_q").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn zawa04(ctx: &Ctx) -> Script {
    zawa04_run(ctx, Zawa04Step::Start, Vec::new()).map(|_| ())
}

pub fn zawa04_ontouch(ctx: &Ctx) -> Script {
    zawa04_run(ctx, Zawa04Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BloodySpotStep {
    Start,
    OnTouch,
}

fn bloody_spot_run(ctx: &Ctx, mut step: BloodySpotStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BloodySpotStep::Start => {
                step = BloodySpotStep::OnTouch;
                continue 'machine;
            }
            BloodySpotStep::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 6 {
                    ctx.lines(args![
                        "^3355FFThere's a spot on the",
                        "ground that's darker than",
                        "the rest of the floor...^000000"
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Investigate:Ignore")])?) == 1 {
                        ctx.lines(args!["^3355FFIt's too dark to", "really see the spot^000000"])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......", "........."])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFIt smells a little bit",
                            "like copper or iron. Blood",
                            "has probably been spilled here.^000000"
                        ])?;
                        ctx.var("ra_tem_q").set(Val::from(7))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "^3355FFYou decide that your",
                            "time would be better spent",
                            "investigating something else.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("ra_tem_q").get()? == 7 {
                    ctx.lines(args![
                        "^3355FFThis dark spot on the",
                        "ground is really creepy,",
                        "no matter how many",
                        "times you look at it.^000000."
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Investigate Again:Ignore")])?) == 1 {
                        ctx.lines(args![
                            "^3355FFThe blood on the ground",
                            "hasn't dried up yet, but",
                            "it's probably cold by now.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......", "........."])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe amount of blood",
                            "on the ground is more",
                            "than can be contained in just",
                            "one person. You'd better get",
                            "out of here before it's too late.^000000"
                        ])?;
                        ctx.var("ra_tem_q").set(Val::from(8))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines(args![
                            "^3355FFYou decide that your",
                            "time would be better spent",
                            "investigating something else.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("ra_tem_q").get()? == 8 {
                    ctx.lines(args![
                        "^3355FFThis blood stain on the",
                        "ground is a pretty bad",
                        "sign. You might have to",
                        "escape this place before",
                        "whatever caused this much",
                        "bleeding does the same to you.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn bloody_spot(ctx: &Ctx) -> Script {
    bloody_spot_run(ctx, BloodySpotStep::Start, Vec::new()).map(|_| ())
}

pub fn bloody_spot_ontouch(ctx: &Ctx) -> Script {
    bloody_spot_run(ctx, BloodySpotStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nemma01Step {
    Start,
    OnTouch,
}

fn nemma01_run(ctx: &Ctx, mut step: Nemma01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nemma01Step::Start => {
                step = Nemma01Step::OnTouch;
                continue 'machine;
            }
            Nemma01Step::OnTouch => {
                if ctx.var("ra_tem_q").get()? == 8 {
                    ctx.lines_as("???", args!["Only the goddess exists."])?;
                    ctx.next()?;
                    ctx.lines_as("???", args!["Everyone must be", "ready for her coming."])?;
                    ctx.next()?;
                    ctx.lines_as("???", args!["Izlude, my hometown,", "I have come to this", "place to build."])?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Th-that's Priestess",
                            "Panno's voice! But isn't",
                            "she just outside the door?",
                            "What exactly was she saying?"
                        ],
                    )?;
                    ctx.var("ra_tem_q").set(Val::from(9))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn nemma01(ctx: &Ctx) -> Script {
    nemma01_run(ctx, Nemma01Step::Start, Vec::new()).map(|_| ())
}

pub fn nemma01_ontouch(ctx: &Ctx) -> Script {
    nemma01_run(ctx, Nemma01Step::OnTouch, Vec::new()).map(|_| ())
}

fn quest_temple_exit_ra_tem_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("^3355FFThe gate is closed.^000000")?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Push Gate:Examine Gate:Kick Gate:Smash Gate with Weapon")])? {
        1 => {
            ctx.lines(args!["^3355FFYou push the gate", "with all of your might..."])?;
            if ctx.var("ra_tem_q").get()? == 9 {
                ctx.close_window()?;
                ctx.var("ra_tem_q").set(Val::from(10))?;
                ctx.call(Function::Warp, vec![Val::from("ra_temple"), Val::from(119), Val::from(175)])?;
                return Err(Stop::End);
            }
            ctx.mes("But it won't even budge.^000000")?;
        }
        2 => {
            ctx.lines(args![
                "^3355FFThere's some strange",
                "machinery installed on",
                "the gate, and a slot where",
                "it looks you you can insert",
                "a card or permit. The lights",
                "are on, so it must be working.^000000"
            ])?;
        }
        3 => {
            ctx.mes("^3355FFYou angrily kick the gate...")?;
            if ctx.var("ra_tem_q").get()? == 9 {
                ctx.close_window()?;
                ctx.var("ra_tem_q").set(Val::from(10))?;
                ctx.call(Function::Warp, vec![Val::from("ra_temple"), Val::from(119), Val::from(175)])?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "But no matter how much rage",
                "you put into your kick, the",
                "gate refuses to open for you.",
                "Oh, and your foot hurts too.^000000."
            ])?;
        }
        4 => {
            ctx.lines(args![
                "^3355FFWait! That's not",
                "a good idea. You can't",
                "smash down the gate to",
                "a holy place: heroes don't",
                "specialize in desecration.^000000"
            ])?;
        }
        _ => {}
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn quest_temple_exit_ra_tem(ctx: &Ctx) -> Script {
    quest_temple_exit_ra_tem_body(ctx, Vec::new()).map(|_| ())
}
