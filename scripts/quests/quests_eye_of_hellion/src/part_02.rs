use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn grout_he_tuccok_hellion_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("hellionq").get()?.number()? < 47 {
        ctx.lines_as(
            "Grout'he",
            args![
                "Hey...",
                "Hey you!",
                "What are you",
                "looking at?!",
                "Get outta my",
                "face, ya jerkwad."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hellionq").get()? == 47 {
            ctx.lines_as(
                "Grout'he",
                args![
                    "What...?",
                    "You got something",
                    "to tell me or are you",
                    "just bein' a rude prick",
                    "and staring at me like",
                    "this is your turf?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I'm sorry, but well...",
                    "Chilias'Tyus told me that",
                    "you might know something",
                    "about a piece of a tablet--"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grout'he",
                args![
                    "What? Oh, so Chilias'Tyus",
                    "is lookin for the gem too, eh?",
                    "Ooh, that's right. He wants to",
                    "seal its evil or something.",
                    "Yeah, yeah. That's a good",
                    "cause. Real noble and all..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grout'he",
                args![
                    "Tell you what. The only reason",
                    "I want that gem is to sell it so I can pay off all my debts. But",
                    "if I sell you the clues I found, I'll get the zeny I need and the",
                    "gem will be in good hands too."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grout'he",
                args![
                    "So what do you say?",
                    "It's not a bad deal if",
                    "you think about it and",
                    "we might as well kill",
                    "two birds with one stone..."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Alright, I'll buy your clues.:Why the hell should I pay you?!")],
            )?) == 1
            {
                ctx.lines_as(
                    "Grout'he",
                    args![
                        "Oh man, thanks a lot.",
                        "Alright, all I'm asking",
                        "for is 10,000 zeny. You get",
                        "my clues and I'll let you know",
                        "where I found 'em, in case",
                        "you can find something new."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("Zeny").get()?.number()? > 9999 {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 1000 {
                        ctx.lines_as(
                            "Grout'he",
                            args![
                                "There you go!",
                                "Thanks for the cash~",
                                "(Now I won't have to",
                                "get my knees all broken!)",
                                "Oh, and let me mark your",
                                "Mini-Map for you real quick..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grout'he",
                            args![
                                "You see this here?",
                                "It's where I found the",
                                "Skirt of Virgin I'm about",
                                "to give you. So check that",
                                "spot out and see if you find",
                                "anything new and interesting."
                            ],
                        )?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(101), Val::from(190), Val::from(1), Val::from(16776960)],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grout'he",
                            args![
                                "Here you go, pal...",
                                "A Stone Heart, Green Herb,",
                                "some Grape Juice and one",
                                "clean and pure Skirt of Virgin.",
                                "If you got any more questions,",
                                "I guess you can ask me later~"
                            ],
                        )?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                        ctx.var("hellionq").set(Val::from(48))?;
                        ctx.call(Function::GetItem, vec![Val::from(953), Val::from(1)])?;
                        ctx.call(Function::GetItem, vec![Val::from(511), Val::from(1)])?;
                        ctx.call(Function::GetItem, vec![Val::from(533), Val::from(1)])?;
                        ctx.call(Function::GetItem, vec![Val::from(1049), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Grout'he",
                        args![
                            "Whoa whoa...",
                            "But you don't got any room",
                            "in your inventory to hold",
                            "what I wanna give you. It'd",
                            "probably be best to stash your",
                            "stuff in Kafra Storage, yeah?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Grout'he",
                    args![
                        "Oh hey...",
                        "You don't got the zeny.",
                        "Sorry pal, but I've really got",
                        "to pay my debts somehow.",
                        "I don't know if I'm allowed",
                        "to really talk about it...",
                        "^333333*Sob*^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Grout'he",
                args![
                    "Why the hell should you",
                    "pay me?! Hey, aren't you",
                    "looking for the gem to help",
                    "the world? Well, I happen to",
                    "be a little part of this world,",
                    "so why don't you help me first?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Grout'he",
                args![
                    "Man, you might not",
                    "know it, but the money",
                    "you give me will save",
                    "a life. My own! Dude, how",
                    "can you be so insensitive?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hellionq").get()? == 48 {
                ctx.lines_as(
                    "Grout'he",
                    args![
                        "You forgot where I told",
                        "you to look for more clues?",
                        "Eh, I'll let you know once",
                        "again, sure. Keep your eye",
                        "on your Mini-Map, got it?"
                    ],
                )?;
                ctx.call(
                    Function::ViewPoint,
                    vec![Val::from(1), Val::from(101), Val::from(190), Val::from(1), Val::from(16776960)],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grout'he",
                    args![
                        "Alright...",
                        "Good luck out there.",
                        "I really hope you can",
                        "keep the gem out of the",
                        "wrong hands. Of course, I was gonna just sell it, but..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hellionq").get()? == 49 {
                    ctx.lines_as(
                        "Grout'he",
                        args![
                            "You've been to the",
                            "first location? Good.",
                            "Okay, now look over here.",
                            "Check your Mini-Map to",
                            "see where I found that",
                            "Stone Heart, got it?"
                        ],
                    )?;
                    ctx.call(
                        Function::ViewPoint,
                        vec![Val::from(1), Val::from(82), Val::from(109), Val::from(2), Val::from(16776960)],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hellionq").get()? == 50 {
                        ctx.lines_as(
                            "Grout'he",
                            args![
                                "Alright, here's where",
                                "I found that Green Herb.",
                                "You know you can use that",
                                "to counter poison, right?",
                                "Check that place to see if",
                                "there's anything I missed."
                            ],
                        )?;
                        ctx.call(
                            Function::ViewPoint,
                            vec![Val::from(1), Val::from(239), Val::from(56), Val::from(3), Val::from(16776960)],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hellionq").get()? == 51 {
                            ctx.lines_as(
                                "Grout'he",
                                args![
                                    "Okay, here's the last",
                                    "place you should double",
                                    "check, the place where",
                                    "I nabbed that Grape",
                                    "Juice. Good luck~"
                                ],
                            )?;
                            ctx.call(
                                Function::ViewPoint,
                                vec![Val::from(1), Val::from(243), Val::from(160), Val::from(4), Val::from(16776960)],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("hellionq").get()? == 52 {
                                ctx.lines_as(
                                    "Grout'he",
                                    args![
                                        "Oh, you need some help",
                                        "in figuring out the clues?",
                                        "Heh. Alright, let's see.",
                                        "You want me to help",
                                        "you in making sense of",
                                        "this weird riddle?"
                                    ],
                                )?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("Yes, please!:No, thanks~")])?) == 1 {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Alright now. All those",
                                            "messages said stuff about",
                                            "one sized clothing, dried",
                                            "fish and green herbs, wine...",
                                            "Huh. It doesn't make sense."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["What...?", "I could have", "told you that!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Wait, wait.",
                                            "Let's mark all the",
                                            "clue locations again",
                                            "on your Mini-Map.",
                                            "Okay, we've got four",
                                            "dots around Payon."
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(243), Val::from(160), Val::from(4), Val::from(16776960)],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(239), Val::from(56), Val::from(3), Val::from(16776960)],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(82), Val::from(109), Val::from(2), Val::from(16776960)],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(101), Val::from(190), Val::from(1), Val::from(16776960)],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Alright, now if",
                                            "I connect and intersect",
                                            "all the lines, there's",
                                            "a point in the middle.",
                                            "Why don't you check",
                                            "this middle point, eh?"
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::ViewPoint,
                                        vec![Val::from(1), Val::from(159), Val::from(129), Val::from(5), Val::from(16776960)],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Now, this location",
                                            "is a house, and I know",
                                            "it's pretty random, but",
                                            "it's all we got. Heck,",
                                            "all these clues make",
                                            "less sense than this!"
                                        ],
                                    )?;
                                    ctx.var("hellionq").set(Val::from(53))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Grout'he",
                                    args![
                                        "Oh yeah?",
                                        "Well listen, I dunno,",
                                        "but this looks to be one",
                                        "of those situations where",
                                        "two heads are better than one.",
                                        "I think we oughta team up~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("hellionq").get()?.number()? > 52 && ctx.var("hellionq").get()?.number()? < 55) {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Hey, how's it goin'",
                                            "looking for that one",
                                            "piece of the tablet?",
                                            "It's a lot of trouble,",
                                            "but I guess that's the",
                                            "case with all treasure."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 55 {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Oh, so all of those",
                                            "items came in handy?",
                                            "I don't believe it! So",
                                            "this puzzle actually",
                                            "makes sense?! So",
                                            "what'd the slab say?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "''Compassionate one?''",
                                            "Oh hey, there's a giant",
                                            "stone statue over in the",
                                            "Archer Village that fits",
                                            "that description perfectly!",
                                            "You should check it out."
                                        ],
                                    )?;
                                    ctx.var("hellionq").set(Val::from(56))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 56 {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Hey, you really ought",
                                            "to check out the huge",
                                            "stone statue over in the",
                                            "Archer Village. Now that",
                                            "I think about it, it's the",
                                            "perfect hiding place!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 57 {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Hey, this is great!",
                                            "You actually found the",
                                            "next piece of the tablet!",
                                            "You better take this back",
                                            "to Chilias'Tyus right now~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Oh. And um, thanks",
                                            "for being willing to pay",
                                            "me in cash for all those",
                                            "little bitty clues. I won't",
                                            "forget your help, pal~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if (ctx.var("hellionq").get()?.number()? > 57 && ctx.var("hellionq").get()?.number()? < 71) {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Hey, be careful if",
                                            "you manage to find that",
                                            "Hellion's gem. I dunno if",
                                            "it's true, but maybe old",
                                            "Tyus was right. Maybe it",
                                            "does hold a wicked power..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Grout'he",
                                        args![
                                            "Life sure is a lot",
                                            "less tense without",
                                            "having debt to worry",
                                            "about. I could get",
                                            "real used to this..."
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

pub fn grout_he_tuccok_hellion(ctx: &Ctx) -> Script {
    grout_he_tuccok_hellion_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Paypuzz1Step {
    Start,
    OnTouch,
}

fn paypuzz1_run(ctx: &Ctx, mut step: Paypuzz1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Paypuzz1Step::Start => {
                step = Paypuzz1Step::OnTouch;
                continue 'machine;
            }
            Paypuzz1Step::OnTouch => {
                if ctx.var("hellionq").get()? == 48 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Well, this is where",
                            "Grout'he told me to look.",
                            "Ooh, there's something",
                            "written on the ceiling. Um,",
                            "let's see... ''This garment",
                            "is one size fits all.''"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "That's it...?",
                            "Hopefully this will",
                            "make more sense once",
                            "I find more clues. I hope."
                        ],
                    )?;
                    ctx.var("hellionq").set(Val::from(49))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn paypuzz1(ctx: &Ctx) -> Script {
    paypuzz1_run(ctx, Paypuzz1Step::Start, Vec::new()).map(|_| ())
}

pub fn paypuzz1_ontouch(ctx: &Ctx) -> Script {
    paypuzz1_run(ctx, Paypuzz1Step::OnTouch, Vec::new()).map(|_| ())
}

fn pile_of_stone_paypuzz2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hellionq").get()? == 49 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well, this is the place",
                "that Grout'he told me about.",
                "Let's see, there's an engraving",
                "here that says, ''I used to pray for peaceful days here.'' Okay."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Man...",
                "This better not be",
                "one of those puzzles",
                "that looks easy once",
                "I get the answer. Well,",
                "if I ever get it, that is."
            ],
        )?;
        ctx.var("hellionq").set(Val::from(50))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pile_of_stone_paypuzz2(ctx: &Ctx) -> Script {
    pile_of_stone_paypuzz2_body(ctx, Vec::new()).map(|_| ())
}

fn dried_fish_paypuzz3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hellionq").get()? == 50 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "So... Dried fish all",
                "around me. This is getting",
                "to be nonsense. Let's see,",
                "this message says, ''Green",
                "Herbs are very useful for",
                "getting rid of fish smells.''"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I got it...!",
                "Whoever made up",
                "these clues must be",
                "some sort of crazy man!",
                "It's the only explanation!"
            ],
        )?;
        ctx.var("hellionq").set(Val::from(51))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn dried_fish_paypuzz3(ctx: &Ctx) -> Script {
    dried_fish_paypuzz3_body(ctx, Vec::new()).map(|_| ())
}

fn vat_paypuzz4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hellionq").get()? == 51 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Alright, the last",
                "place I have to check.",
                "It's a big old vat with a",
                "message here too. Now",
                "let me see, there should",
                "be a message here too. Ah--!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "This says, ''What will it be?",
                "Wine or Grape Juice? Oh,",
                "but I much prefer wine.'' Okay,",
                "what does this have to do with",
                "that Hellion's gem? Weird..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Am I missing something?",
                "I think I really need help in",
                "figuring this out. Hmm..."
            ],
        )?;
        ctx.var("hellionq").set(Val::from(52))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn vat_paypuzz4(ctx: &Ctx) -> Script {
    vat_paypuzz4_body(ctx, Vec::new()).map(|_| ())
}

fn wooden_floor_paypuzz5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hellionq").get()? == 53 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Hey, the floor around",
                "here doesn't seem very",
                "solid. Maybe there's",
                "something underneath?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Just check the floor.:Dig through the dust.")],
        )?) == 1
        {
            ctx.lines(args![
                "^3355FFYou examine the floor,",
                "but are unable to find",
                "anything that resembles",
                "a clue to the location",
                "of the Hellion's gem",
                "or a tablet piece.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh man, there's",
                "so much dust! What",
                "can I do about all of",
                "this nasty dirt in the air?"
            ],
        )?;
        ctx.next()?;
        if (((ctx.call(Function::CountItem, vec![Val::from(953)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(533)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(1049)])?.number()? > 0)
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Well, there are all these",
                    "clues Grout'he found. It sucks,",
                    "but I'll use this Grape Juice",
                    "to sort of keep this dust from",
                    "kicking up. And if I put my morals^FFFFFFa^000000 on hold, I'll cover my face..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "...With this Skirt of Virgin.",
                    "Oh, and I can use this Stone",
                    "Heart to prop any floor boards",
                    "I tilt while I look underneath",
                    "this floor. Oh hey! I found it!",
                    "It's... It's another clue!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Wait, wait...",
                    "I can't read the",
                    "engraving on this",
                    "stone slab, it's way",
                    "too old. Let's see..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou cover the surface",
                "of the slab and begin",
                "to rub a Green Herb over",
                "it. Fortunately, the juice from",
                "the Green Herb makes the",
                "engraving appear more clearly.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(953), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(511), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(533), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1049), Val::from(1)])?;
            ctx.var("hellionq").set(Val::from(54))?;
            ctx.next()?;
            ctx.lines_as(
                "Stone Engraving",
                args![
                    "^4D4DFF''This has been entrusted to",
                    "the care of the compassionate",
                    "one so that my friends may",
                    "find peace. To he who finds",
                    "this, remember that avarice",
                    "knows no bounds. --Tyus.''^000000"
                ],
            )?;
            ctx.var("hellionq").set(Val::from(55))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Where did I leave",
                "all that stuff I got from",
                "Grout'he? Let's see,",
                "there was a Green Herb,",
                "a Stone Heart, Grape Juice",
                "and one Skirt of Virgin..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hellionq").get()? == 54 {
        ctx.lines_as(
            "Stone Engraving",
            args![
                "^4D4DFF''This has been entrusted to",
                "the care of the compassionate",
                "one so that my friends may",
                "find peace. To he who finds",
                "this, remember that avarice",
                "knows no bounds. --Tyus.''^000000"
            ],
        )?;
        ctx.var("hellionq").set(Val::from(55))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hellionq").get()?.number()? > 54 {
        ctx.lines_as(
            "Stone Engraving",
            args![
                "^4D4DFF''This has been entrusted to",
                "the care of the compassionate",
                "one so that my friends may",
                "find peace. To he who finds",
                "this, remember that avarice",
                "knows no bounds. --Tyus.''^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn wooden_floor_paypuzz5(ctx: &Ctx) -> Script {
    wooden_floor_paypuzz5_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BuddhaStatuePaypuzz6Step {
    Start,
    OnTouch,
}

fn buddha_statue_paypuzz6_run(ctx: &Ctx, mut step: BuddhaStatuePaypuzz6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BuddhaStatuePaypuzz6Step::Start => {
                if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
                    || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
                {
                    ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("hellionq").get()? == 56 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Hey, here's a huge",
                            "stone statue. This",
                            "must be what Grout'he",
                            "was telling me about."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Echoing Voice", args!["^4d4dffYou must...", "You must be...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["What the--?", "There's a voice", "inside my head!", "And it's not even mine!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Echoing Voice",
                        args![
                            "You who have come",
                            "for the Tablet Piece,",
                            "I shall judge whether",
                            "or not you are worthy.",
                            "Answer this one question."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Echoing Voice",
                        args![
                            "You are in a situation",
                            "in which you will die,",
                            "but if you cause the death",
                            "of another person, you will",
                            "surely save yourself. So...",
                            "What do you do?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "I will kill to survive.:I have no choice, but to die.:I won't kill, but I'll find a way to live!:I'll ask that person if it's okay to kill him.",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Echoing Voice",
                                args!["There is a primal truth", "to your answer. However...", "You have chosen poorly."],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Echoing Voice",
                                args![
                                    "There is a kindness and",
                                    "compassion in your answer",
                                    "that is indeed rare. However,",
                                    "where is the respect for your",
                                    "own life? If you are that willing to throw it away, you are no hero."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Echoing Voice",
                                args![
                                    "Yes. Love for all life,",
                                    "including your own, is",
                                    "a trait that all true heroes",
                                    "must have. You have chosen...",
                                    "Wisely. Please, take this into",
                                    "your capable hands, adventurer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFA stone slab at the foot",
                                "of the statue slides open,",
                                "revealing a piece of the",
                                "tablet that will lead you",
                                "to the Hellion's gem."
                            ])?;
                            ctx.var("hellionq").set(Val::from(57))?;
                            ctx.call(Function::GetItem, vec![Val::from(7334), Val::from(1)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.lines_as(
                                "Echoing Voice",
                                args![
                                    "The veneer of honesty in",
                                    "your answer is surpassed by",
                                    "your own cowardice. You have",
                                    "chosen extremely poorly..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                step = BuddhaStatuePaypuzz6Step::OnTouch;
                continue 'machine;
            }
            BuddhaStatuePaypuzz6Step::OnTouch => {
                if (ctx.var("hellionq").get()? == 56 && ctx.call(Function::CountItem, vec![Val::from(7333)])?.number()? > 0) {
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                    ctx.lines(args![
                        "^3355FFThe piece of tablet",
                        "that you have is shining",
                        "with light as if in response",
                        "to something in the area.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn buddha_statue_paypuzz6(ctx: &Ctx) -> Script {
    buddha_statue_paypuzz6_run(ctx, BuddhaStatuePaypuzz6Step::Start, Vec::new()).map(|_| ())
}

pub fn buddha_statue_paypuzz6_ontouch(ctx: &Ctx) -> Script {
    buddha_statue_paypuzz6_run(ctx, BuddhaStatuePaypuzz6Step::OnTouch, Vec::new()).map(|_| ())
}

fn sage_welshyun_hellion_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("hellionq").get()?.number()? < 58 {
        ctx.lines_as(
            "Welshyun",
            args![
                "Hm? Oh, pardon me.",
                "I didn't notice you. I was",
                "far too occupied thinking",
                "deep important thoughts",
                "that your puny little mind",
                "couldn't possibly fathom."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hellionq").get()? == 58 {
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
            ctx.lines_as("Welshyun", args!["That...", "That eerie glow..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Welshyun",
                args![
                    "You! Identify yourself",
                    "and explain how you came",
                    "about to get those tablet",
                    "pieces! Now answer me!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Welshyun",
                args![
                    "Ah, forgive me. You're",
                    "helping Chilias, are you?",
                    "I see. So you must be here",
                    "to seek my help in finding",
                    "the final piece, am I correct?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Shut up, I know you have it, so give it!:Yes, please help me.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Welshyun",
                    args![
                        "Such insolence!",
                        "Huh. I suppose Chilias",
                        "may have misjudged you",
                        "when he entrusted you with",
                        "this important mission. Huh.",
                        "A reevaluation is in order..."
                    ],
                )?;
                ctx.var("hellionq").set(Val::from(60))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Welshyun",
                args![
                    "Of course, in the",
                    "interest of mankind,",
                    "I shall help you. But first,",
                    "I have a favor to ask of you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Welshyun",
                args![
                    "My student in the Geffen",
                    "Tower has not reported back to",
                    "me about his research project,",
                    "so I would like for you to find",
                    "him and retrieve the report",
                    "for me. Not a big deal, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Welshyun",
                args![
                    "The title of his report",
                    "should be, ''^4D4DFFMonster Life",
                    "in the Geffen Area^000000,'' so",
                    "please remember that.",
                    "Ah, and my student's",
                    "name is Enoz."
                ],
            )?;
            ctx.var("hellionq").set(Val::from(59))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hellionq").get()? == 59 {
                ctx.lines_as(
                    "Welshyun",
                    args![
                        "Please bring me the",
                        "report entitled, ''^4D4DFFMonster",
                        "Life in the Geffen Area^000000,''",
                        "from my student Enoz in",
                        "the Geffen Tower. Thank",
                        "you, I appreciate your help."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hellionq").get()? == 60 {
                    ctx.lines_as(
                        "Welshyun",
                        args![
                            "^333333*Ahem*^000000 Oh dear me...",
                            "I'm unfathomably thirsty...",
                            "There's no way I can do any",
                            "serious thinking with such",
                            "a dry, parched throat..."
                        ],
                    )?;
                    ctx.var("hellionq").set(Val::from(61))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hellionq").get()? == 61 {
                        if ctx.call(Function::CountItem, vec![Val::from(504)])?.number()? > 0 {
                            ctx.lines_as(
                                "Welshyun",
                                args![
                                    "Oh, a White Potion?",
                                    "Yes, may I please have",
                                    "one? Ahh, most refreshing~",
                                    "You may not be such a bad",
                                    "person after all! Now shall I help you find that tablet piece?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Welshyun",
                                args![
                                    "Of course, in the",
                                    "interest of mankind,",
                                    "I shall help you. But first,",
                                    "I have a favor to ask of you."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Welshyun",
                                args![
                                    "My student in the Geffen",
                                    "Tower has not reported back to",
                                    "me about his research project,",
                                    "so I would like for you to find",
                                    "him and retrieve the report",
                                    "for me. Not a big deal, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Welshyun",
                                args![
                                    "The title of his report",
                                    "should be, ''^4D4DFFMonster Life",
                                    "in the Geffen Area^000000,'' so",
                                    "please remember that.",
                                    "Ah, and my student's",
                                    "name is Enoz."
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(504), Val::from(1)])?;
                            ctx.var("hellionq").set(Val::from(62))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Welshyun",
                            args![
                                "Oh...! Still so...",
                                "Exasperatingly thirsty!",
                                "If I only had something",
                                "cool, refreshing and with",
                                "a high restorative value",
                                "to quench this thirst!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hellionq").get()? == 62 {
                            ctx.lines_as(
                                "Welshyun",
                                args![
                                    "Please bring me the",
                                    "report entitled, ''^4D4DFFMonster",
                                    "Life in the Geffen Area^000000,''",
                                    "from my student Enoz in",
                                    "the Geffen Tower. Thank",
                                    "you, I appreciate your help."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("hellionq").get()? == 63 {
                                ctx.lines_as(
                                    "Welshyun",
                                    args![
                                        "Hm? What's that strange",
                                        "expression on your face?",
                                        "Prank? Oh please. Such",
                                        "chicanery is beneath me...",
                                        "But even if I did play a joke, believe me. It builds character."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Welshyun",
                                    args![
                                        "First and foremost,",
                                        "I want to test you to see",
                                        "if you are worthy of this",
                                        "tablet piece. I want to see",
                                        "for myself if you are a person",
                                        "of honor. Someone of character."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Welshyun",
                                    args![
                                        "Oh yes, I accidentally",
                                        "took this Master Science",
                                        "Reference Book from Enoz,",
                                        "so would you please return it",
                                        "to him for me? Oh, and please",
                                        "bring back 1 Blue Gemstone."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFYou received the",
                                    "incredibly heavy tome,",
                                    "the Master Science",
                                    "Reference Book.^000000"
                                ])?;
                                ctx.var("hellionq").set(Val::from(64))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("hellionq").get()? == 64 {
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "Please deliver the Master",
                                            "Science Reference Book to",
                                            "Enoz, and come back to me",
                                            "with 1 Blue Gemstone."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 65 {
                                    if ctx.call(Function::CountItem, vec![Val::from(717)])?.number()? > 0 {
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Thank you. You've",
                                                "brought the book to Enoz",
                                                "and delivered a Gemstone",
                                                "to me as I've asked. Now,",
                                                "there is one final test..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Answer this question.",
                                                "Where is the abode of the",
                                                "departed souls with a shining",
                                                "silver roof that is mentioned",
                                                "in the third part of the ballad",
                                                "of Grimnir? Well, adventurer?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let (input, status) = runtime::input_text(ctx, None, None)?;
                                        l_input_s = input;
                                        if l_input_s.clone() == "Valaskjalf" {
                                            ctx.lines_as(
                                                "Welshyun",
                                                args![
                                                    "Ah, well met, well met.",
                                                    "You are as well learned as",
                                                    "you are brave. As promised,",
                                                    "you may have this piece of",
                                                    "the tablet, which I've already found. The clues were too simple..."
                                                ],
                                            )?;
                                            ctx.call(Function::DelItem, vec![Val::from(717), Val::from(1)])?;
                                            ctx.var("hellionq").set(Val::from(66))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7336), Val::from(1)])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Welshyun",
                                                args![
                                                    "Please send my regards",
                                                    "to my dear friend, Chilias,",
                                                    "who has dedicated his life",
                                                    "to sealing the evil within",
                                                    "the Hellion's gem."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Bwahahaah! Only",
                                                "a superior mind could",
                                                "know the answer to such",
                                                "a question! Go forth and",
                                                "learn the answer, else",
                                                "I cannot help you, adventurer~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "Ah, I've heard that",
                                            "Enoz received the Master",
                                            "Science Reference Book",
                                            "from you. But did you remember",
                                            "to bring me a Blue Gemstone?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 66 {
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "Hm. You should visit",
                                            "Chilias and determine",
                                            "your next course of action,",
                                            "now that you have all four",
                                            "of the pieces of the tablet."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("hellionq").get()? == 67 {
                                    if (((ctx.call(Function::CountItem, vec![Val::from(7333)])?.number()? > 0
                                        && ctx.call(Function::CountItem, vec![Val::from(7334)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(7335)])?.number()? > 0)
                                        && ctx.call(Function::CountItem, vec![Val::from(7336)])?.number()? > 0)
                                    {
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Hm? Did you need",
                                                "me to combine all four",
                                                "pieces of the tablet and",
                                                "make it whole again? Oh.",
                                                "All this time I thought you",
                                                "knew how to do it. Alright."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "As a matter of fact,",
                                                "I was planning on using",
                                                "the Blue Gemstone you gave",
                                                "me to do this. But since you",
                                                "never really asked... In any",
                                                "case, let me concentrate."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Shadows remembered by time.",
                                                "Help me retrieve the forgotten",
                                                "stories that have been scattered in the wind. Right here. Right now."
                                            ],
                                        )?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                                        ctx.next()?;
                                        ctx.lines_as("Welshyun", args!["...", "Okay...", "That was tough."])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPELLBREAKER")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args!["That was tough", "complete tablet.", "Please bring this", "safely back to Chilias."],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(7333), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7334), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7335), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7336), Val::from(1)])?;
                                        ctx.var("hellionq").set(Val::from(68))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7332), Val::from(1)])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Wait, take a look!",
                                                "There's a map on the back",
                                                "of the tablet. Although the gem",
                                                "is already embedded within the",
                                                "tablet, who knows where this map may lead? Perhaps Hellion Revenant?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "I don't know if Chilias",
                                                "told you, but the name of",
                                                "the monster that guards and",
                                                "follows this gem is Hellion",
                                                "Revenant. But why mark its",
                                                "location on this tablet?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Well, it will be dangerous, but",
                                                "I'm sure that you're strong enough to confront this monster. I believe",
                                                "it was the wish of this tablet's creator for someone to defeat",
                                                "the Hellion Revenant..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if Val::from(runtime::select_values(
                                            ctx,
                                            &[Val::from("Alright, I'll do it!:I better get this tablet to Chilias...")],
                                        )?) == 1
                                        {
                                            ctx.lines_as(
                                                "Welshyun",
                                                args![
                                                    "Well, I'm almost certain",
                                                    "this map will lead you to",
                                                    "Hellion Revenant. Let me",
                                                    "warp you to the location",
                                                    "marked on the tablet's map..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            ctx.call(Function::Warp, vec![Val::from("gef_fild09"), Val::from(368), Val::from(88)])?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Welshyun",
                                            args![
                                                "Yes, that's true.",
                                                "Chilias has been waiting",
                                                "his whole life to seal this",
                                                "gem. Plus, who knows what",
                                                "may happen while it is in",
                                                "your possession?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "So, were you able",
                                            "to deliver the tablet",
                                            "and the Hellion's gem to",
                                            "Chilias safely? I hope so..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if (ctx.var("hellionq").get()?.number()? > 67 && ctx.var("hellionq").get()?.number()? < 71) {
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "So, were you able",
                                            "to deliver the tablet",
                                            "and the Hellion's gem to",
                                            "Chilias safely? Ah, and",
                                            "how has my friend been?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Welshyun",
                                        args![
                                            "Heh heh~",
                                            "Enoz must be",
                                            "panicking right",
                                            "about now. Oh,",
                                            "students are always",
                                            "good for a laugh..."
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

pub fn sage_welshyun_hellion(ctx: &Ctx) -> Script {
    sage_welshyun_hellion_body(ctx, Vec::new()).map(|_| ())
}

fn sage_welshyun_hellion_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("hellionq").get()? == 58 && ctx.call(Function::CountItem, vec![Val::from(7335)])?.number()? > 0) {
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
        ctx.lines(args![
            "^3355FFThe piece of tablet",
            "that you have is shining",
            "with light as if in response",
            "to something in the area.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn sage_welshyun_hellion_ontouch(ctx: &Ctx) -> Script {
    sage_welshyun_hellion_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn enoz_hellion_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if (ctx.var("hellionq").get()? == 59 || ctx.var("hellionq").get()? == 62) {
        ctx.lines_as(
            "Enoz",
            args![
                "It's gone, it's gone!",
                "Where the hell did it go?!",
                "Oh man, who could have",
                "taken it? Wait, was it... You?!"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Actually, Welshyun sent me.:No way man, don't go nuts.")],
        )?) == 1
        {
            ctx.lines_as(
                "Enoz",
                args![
                    "My mentor, Welshyun?",
                    "Huh. Is there a report",
                    "I haven't submitted to him",
                    "yet? So what was it called?"
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() == "Monster Life in the Geffen Area" {
                ctx.lines_as(
                    "Enoz",
                    args![
                        "''Monster Life in",
                        "the Geffen Area?''",
                        "Oh no. My mentor took",
                        "that three days ago!",
                        "Is he still up to his",
                        "old tricks again?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Enoz",
                    args![
                        "Awww nuts!",
                        "Where the heck is",
                        "my Master Science",
                        "Reference Book? ",
                        "It was right here",
                        "three days ago..."
                    ],
                )?;
                ctx.var("hellionq").set(Val::from(63))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Enoz",
                args![
                    ((Val::from("") + l_input_s.clone()) + Val::from("?")),
                    "I haven't completed",
                    "any research related",
                    "to that subject. Or did I?",
                    "Anyway, I need the exact",
                    "name of the report you want!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Enoz",
            args![
                "Nuts?! Oh, you wanna",
                "see crazy, is that it?! You",
                "wanna see insaaaane?!",
                "Cuz I'll go freakin' medieval",
                "if you keep bothering me!!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hellionq").get()? == 63 {
        ctx.lines_as(
            "Enoz",
            args![
                "My mentor Welshyun's",
                "been playing jokes again,",
                "so you better go talk to him.",
                "Damn. Where is that Master",
                "Science Reference Book?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hellionq").get()? == 64 {
        ctx.lines_as(
            "Enoz",
            args![
                "Oh hey, I remember you~",
                "So did my mentor send you",
                "to me again for some reason?"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou cautiously hand Enoz the",
            "Master Science Reference Book.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Enoz",
            args![
                "Yes! Oh yes!",
                "This is the book",
                "I lost three days",
                "ago! Thanks so much!",
                "Now I can finally finish",
                "this research project that--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Enoz",
            args![
                "Wait, the page I really",
                "need is missing! And there's",
                "some sort of note... Umm...",
                "^333333''Enoz, I think this page",
                "is worth at least one Apple",
                "Juice. Right? --Welshyun.''^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Enoz",
            args![
                "NOooOOoOo!",
                "Not agaaaaain!",
                "Why is he always",
                "playing these pranks?!",
                "Apple Juice! I need",
                "some Apple Juice!"
            ],
        )?;
        ctx.var("hellionq").set(Val::from(65))?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt looks like it's time",
            "to get 1 Blue Gemstone,",
            "and then bring it back",
            "to Welshyun the Sage.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Enoz",
        args![
            "Gone! It's gone!",
            "Where did the book",
            "I really need go to?",
            "Oh no oh no oh no oh no!",
            "Waaaaaah, I'm gonna cry!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn enoz_hellion(ctx: &Ctx) -> Script {
    enoz_hellion_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HiddenCaveHellionStep {
    Start,
    OnTouch,
}

fn hidden_cave_hellion_run(ctx: &Ctx, mut step: HiddenCaveHellionStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HiddenCaveHellionStep::Start => {
                if ctx.var("hellionq").get()? == 68 {
                    ctx.lines(args![
                        "^3355FFAmongst the wild bushes",
                        "and overgrown grass, you",
                        "see a large rock slab that",
                        "resembles a door. As you",
                        "come closer, you can see a",
                        "groove on the rock's surface.^000000"
                    ])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Insert the Tablet.:Ignore.")])?) == 1 {
                        ctx.lines(args![
                            "^3355FFYou insert the tablet into",
                            "the rock's groove and the",
                            "Hellion's gem begins to hum",
                            "and glow. The rock slides open,",
                            "revealing an engraved message",
                            "on the ground for you to read.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^4D4DFF''To you who have found this",
                            "place, I reveal the truth about",
                            "the Hellion Revenant. It was",
                            "never a devil, like we believed, but a human being, slowly",
                            "corrupted by the darkness",
                            "of the Hellion's gem.''^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^4D4DFF''I learned this horrible",
                            "truth too late. Already,",
                            "I can feel the darkness",
                            "welling up within me.",
                            "It is all I can do to trap",
                            "myself in this chamber",
                            "before I lose all reason.''^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^4D4DFF''I failed and it's too",
                            "late for me, but I hope",
                            "someone in the future",
                            "can seal the Hellion gem's",
                            "dark power. And I hope that",
                            "someone will grant me the",
                            "sweet release I crave...''^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^4D4DFF''Brave hero, I beg of",
                            "you. Please kill me and",
                            "bring salvation to my soul.",
                            "Please release me so that",
                            "I can finally join my friends.",
                            "^FFFFFF_^000000",
                            "^FFFFFFOOOOOOOOOOO^4D4DFF-Tyus''^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("gef_dun03"), Val::from(140), Val::from(119)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args!["^3355FFYou do nothing.", "And nothing happens.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = HiddenCaveHellionStep::OnTouch;
                continue 'machine;
            }
            HiddenCaveHellionStep::OnTouch => {
                if (ctx.var("hellionq").get()? == 68 && ctx.call(Function::CountItem, vec![Val::from(7332)])?.number()? > 0) {
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HEAL2")?])?;
                    ctx.lines(args![
                        "^3355FFThe tablet in your",
                        "hands begins to shine",
                        "with light, responding",
                        "to something in the area.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn hidden_cave_hellion(ctx: &Ctx) -> Script {
    hidden_cave_hellion_run(ctx, HiddenCaveHellionStep::Start, Vec::new()).map(|_| ())
}

pub fn hidden_cave_hellion_ontouch(ctx: &Ctx) -> Script {
    hidden_cave_hellion_run(ctx, HiddenCaveHellionStep::OnTouch, Vec::new()).map(|_| ())
}
