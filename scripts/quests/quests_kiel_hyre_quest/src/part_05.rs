use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn old_lady_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 56 {
        ctx.lines_as(
            "Old Lady",
            args!["Oooh, my legs and back", "are so sore. These old", "bones ache all over..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 56 {
        if ctx.call(Function::CheckWeight, vec![Val::from(7498), Val::from(1)])? == 0 {
            ctx.lines(args![
                "^3355FFJust a second...",
                "You're carrying too",
                "many items with you",
                "right now, so you'll",
                "need to free up more",
                "Inventory space first...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Old Lady",
            args![
                "Goodness, I hate this",
                "weather! Reminds me",
                "of how old I've gotten!",
                "It chills my bones, it does!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("......:Do you know that grandma?")])? {
            1 => {
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "Damn it! If only I didn't",
                        "have all those adventures",
                        "in my youth! Then maybe",
                        "I wouldn't suffer so in",
                        "my advanced age!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "Oh... Yes. She was",
                        "the mother of my best",
                        "friend, ^3355FFAllysia^000000. Ever since",
                        "she commited suicide, things",
                        "haven't been the same. Her",
                        "mother lost her sanity..."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Was ^3355FFAllysia^000000...?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "Oh, Allysia was such",
                        "a beautiful girl. So many",
                        "men wanted her, especially",
                        "that dashing James Rosimier.",
                        "I remember hearing that they",
                        "were going to get married..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "I was so happy for her!",
                        "But then, all of a sudden,",
                        "she killed herself. Well,",
                        "that's what they all say.",
                        "Even today, I'm still not",
                        "sure what happened."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Wait, who's James Rosimier?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "Oh, James belonged to",
                        "one of the oldest and richest",
                        "families in Juno. Everything",
                        "was going great for them, but",
                        "some time after Allysia died,",
                        "the family went bankrupt."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "The city manages their",
                        "old residence now. For",
                        "some reason, they decided",
                        "to entrust me with the master",
                        "key to the Rosimier Mansion."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("May I borrow the Master Key?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "Well, I'm really not",
                        "supposed to give it to just",
                        "anyone, but I can tell that",
                        "you're working with Allysia's",
                        "best interests at heart."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Old Lady",
                    args![
                        "However, you've got to",
                        "make sure that you return",
                        "it to me before the people",
                        "from City Hall ask me for it.",
                        "Alright then, I hope you find",
                        "what you're looking for."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(7498), Val::from(1)])?;
                ctx.var("kielhyrequest").set(Val::from(58))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ((((ctx.var("kielhyrequest").get()? == 58 && ctx.call(Function::CountItem, vec![Val::from(7499)])?.number()? < 1)
        || ctx.call(Function::CountItem, vec![Val::from(7500)])?.number()? < 1)
        || ctx.call(Function::CountItem, vec![Val::from(7501)])?.number()? < 1)
        || ctx.call(Function::CountItem, vec![Val::from(7502)])?.number()? < 1)
    {
        ctx.lines_as(
            "Old Lady",
            args![
                "Please hurry and find",
                "whatever you're searching",
                "for in the Rosimier Mansion.",
                "I might get in trouble if",
                "the people from City Hall",
                "come and ask me for the key..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((((ctx.var("kielhyrequest").get()? == 58 && ctx.call(Function::CountItem, vec![Val::from(7499)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(7500)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(7501)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(7502)])? == 1)
    {
        ctx.lines_as(
            "Old Lady",
            args![
                "Oh, you're finished",
                "searching the mansion?",
                "Depressing, isn't it?",
                "The creditors basically",
                "ransacked everything",
                "a very long time ago."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Why did ^3355FFAllysia^000000...?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Old Lady",
            args![
                "Well, I know that James",
                "and Allysia were in love,",
                "and he promised to marry",
                "her. Now, supposedly his",
                "family already betrothed",
                "him to another woman."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old Lady",
            args![
                "Time went by, and he",
                "was forced to marry his",
                "fiancee. Allysia was pretty",
                "devastated. I think maybe",
                "that's what she... you know..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old Lady",
            args![
                "Listen, if you want to learn",
                "more about what happened,",
                "then I think you should talk",
                "to the ^3355FFfisherman that lives^000000",
                "^3355FFsouth of the Kiel Hyre Academy.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Old Lady",
            args![
                "He's the one that found",
                "Allysia's body in the river,",
                "so he might have a better",
                "idea of what had happened."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7498), Val::from(1)])?;
        ctx.var("kielhyrequest").set(Val::from(60))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? >= 60 {
        ctx.lines_as("Old Lady", args!["Yep, Rain's coming.", "Can feel it in my bones."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn old_lady_kh(ctx: &Ctx) -> Script {
    old_lady_kh_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RosimmirEntranceStep {
    Start,
    OnTouch,
}

fn rosimmir_entrance_run(ctx: &Ctx, mut step: RosimmirEntranceStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RosimmirEntranceStep::Start => {
                step = RosimmirEntranceStep::OnTouch;
                continue 'machine;
            }
            RosimmirEntranceStep::OnTouch => {
                if ctx.call(Function::CountItem, vec![Val::from(7498)])?.number()? < 1 {
                    ctx.lines(args![
                        "That mansion seems to have",
                        "been destroyed by the time.",
                        "However, the door looks like",
                        "it'd be still operational if",
                        "you had the right key."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::Warp, vec![Val::from("kh_rossi"), Val::from(20), Val::from(92)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn rosimmir_entrance(ctx: &Ctx) -> Script {
    rosimmir_entrance_run(ctx, RosimmirEntranceStep::Start, Vec::new()).map(|_| ())
}

pub fn rosimmir_entrance_ontouch(ctx: &Ctx) -> Script {
    rosimmir_entrance_run(ctx, RosimmirEntranceStep::OnTouch, Vec::new()).map(|_| ())
}

fn table_khr2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7499), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? < 58 {
        ctx.lines(args!["^3355FFThere's nothing here", "of importance to you.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 60 {
        if ctx.call(Function::CountItem, vec![Val::from(7499)])?.number()? < 1 {
            ctx.call(Function::Cutin, vec![Val::from("kh_family_port"), Val::from(1)])?;
            ctx.lines(args![
                "^3355FFYou examine the table,",
                "and find a framed portrait",
                "inside the open drawer.^000000"
            ])?;
            ctx.call(Function::GetItem, vec![Val::from(7499), Val::from(1)])?;
        } else {
            ctx.lines(args!["^3355FFThe open drawer of", "this desk is now empty.^000000"])?;
        }
    } else {
        ctx.lines(args![
            "^3355FFThis was the desk in",
            "which you obtained the",
            "Rosimier family portrait.",
            "Its drawers are empty now.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn table_khr2(ctx: &Ctx) -> Script {
    table_khr2_body(ctx, Vec::new()).map(|_| ())
}

fn shelf_khr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7500), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? < 58 {
        ctx.lines(args!["^3355FFThere's nothing here", "of importance to you.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 60 {
        if ctx.call(Function::CountItem, vec![Val::from(7500)])?.number()? < 1 {
            ctx.lines(args![
                "^3355FFThere are locked",
                "boxes on these bookshelves.",
                "Perhaps if you used this",
                "mansions's Master Key, you",
                "might be able to open them.^000000"
            ])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Use Key:Pass")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines(args!["^3355FFWhich box do you", "want to try to open?^000000"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("First Box:Second Box")])? {
                        1 => {
                            ctx.lines(args![
                                "^3355FFYou use the Master Key to",
                                "unlock the box, and obtain",
                                "a portait of a woman that",
                                "looks just like Kiel Hyre's",
                                "assistant, Allysia.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.call(Function::Cutin, vec![Val::from("kh_ellisia_port"), Val::from(2)])?;
                            ctx.lines(args![
                                "^3355FFThe message, ''To my love,",
                                "Allysia. From James.'' is",
                                "written on the back.^000000"
                            ])?;
                            ctx.call(Function::GetItem, vec![Val::from(7500), Val::from(1)])?;
                            ctx.close_window()?;
                            ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("^3355FFThis box is empty.^000000")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.mes("^3355FFThis box is empty.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            ctx.lines(args![
                "^3355FFYou've found a woman's",
                "portrait in one of the",
                "boxes on this shelf.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args![
            "^3355FFYou've found a woman's",
            "portrait in one of the",
            "boxes on this shelf.",
            "This box is now empty.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn shelf_khr(ctx: &Ctx) -> Script {
    shelf_khr_body(ctx, Vec::new()).map(|_| ())
}

fn desk_khr3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7501), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? < 58 {
        ctx.mes("^3355FFIt's just a desk.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 60 {
        ctx.lines(args!["^3355FFThe desk has", "three drawers.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("First Drawer:Second Drawer:Third Drawer:Cancel")])? {
            1 => {
                ctx.mes("^3355FFThe first drawer is locked.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                if ctx.call(Function::CountItem, vec![Val::from(7501)])?.number()? < 1 {
                    ctx.lines(args![
                        "^3355FFThere is a letter inside",
                        "this second drawer. It",
                        "was sent by a person",
                        "with the initials, K.H.,",
                        "and addressed to Allysia.^000000"
                    ])?;
                    ctx.call(Function::GetItem, vec![Val::from(7501), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThis drawer is now empty.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.lines(args![
                    "^3355FFThere is a small note",
                    "inside this third drawer.",
                    "It's written by James, and",
                    "mentions that he wants to",
                    "marry Allysia, and that she",
                    "received an engagement ring.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            4 => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is the desk where",
            "you found a letter written",
            "by K.H., and a note scribbled",
            "by James Rosimier. Both of",
            "these are addressed to",
            "the same woman, Allysia.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn desk_khr3(ctx: &Ctx) -> Script {
    desk_khr3_body(ctx, Vec::new()).map(|_| ())
}

fn bookshelf_khr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7502), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? < 58 {
        ctx.lines(args![
            "^3355FFYou encounter a dusty",
            "bookshelf filled with",
            "numerous books.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 60 {
        if ctx.call(Function::CountItem, vec![Val::from(7502)])?.number()? < 1 {
            ctx.lines(args![
                "^3355FFYou encounter a dusty",
                "bookshelf filled with",
                "numerous books. You",
                "find a folded note between",
                "the books as you examine them.^000000"
            ])?;
            ctx.call(Function::GetItem, vec![Val::from(7502), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFYou encounter a dusty",
                "bookshelf filled with",
                "numerous books.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args![
            "^3355FFYou encounter a dusty",
            "bookshelf filled with",
            "numerous books.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn bookshelf_khr(ctx: &Ctx) -> Script {
    bookshelf_khr_body(ctx, Vec::new()).map(|_| ())
}

fn bed_khr_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 58 {
        ctx.lines(args![
            "^3355FFYou found a well made",
            "bed that has collected",
            "a thick layer of dust",
            "after years of disuse.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 60 {
        ctx.lines(args![
            "^3355FFYou found a well made",
            "bed that has collected",
            "a thick layer of dust",
            "after years of disuse.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Check Bedsheets:Check Under Bed")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou brush the bed's",
                    "surface with your hand,",
                    "causing a cloud of nasty",
                    "dust to irritate your nose",
                    "and throat. Eww, yucky!^000000"
                ])?;
                ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFYou search underneath",
                    "the bed, and find an empty",
                    "engagement ring box.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.mes("^3355FFThis is a dirty bed.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn bed_khr(ctx: &Ctx) -> Script {
    bed_khr_body(ctx, Vec::new()).map(|_| ())
}

fn old_fisherman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 60 {
        ctx.lines_as(
            "Fisherman",
            args![
                "These days, it's much",
                "harder to catch and fish.",
                "Ever since they built",
                "this factory, the fish",
                "have started to change,",
                "and they look different too..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 60 {
        ctx.lines_as(
            "Fisherman",
            args![
                "Eh? You want something?",
                "Heh, youngsters! I know",
                "how much you love handouts,",
                "but you're not getting any.",
                "Now, if you bring me some",
                "Raw Fish, I'd be more friendly~"
            ],
        )?;
        if ctx.call(Function::CountItem, vec![Val::from(544)])?.number()? >= 10 {
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "Oh, is all this fish",
                    "for me? Heh, how very",
                    "generous of you. If you're",
                    "going to be so kind, then",
                    "I suppose I have to repay",
                    "the favor. Ask me anything~"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("30 years ago, a woman killed herself...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "Oh? Ohh. Oh yeah.",
                    "I remember that. Yeah.",
                    "it was August 20th, my",
                    "wife's birthday. That day,",
                    "instead of catching fish,",
                    "I caught a dead woman."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "Of course, I reported it",
                    "to the Juno Police!  They told",
                    "me she killed herself since",
                    "she was betrayed by her lover,",
                    "who also happened to be her",
                    "employer. Really tragic stuff."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "Anyway, when they were",
                    "moving her body out of the",
                    "river, her hand dropped",
                    "some ring. I picked it up,",
                    "hoping to sell it later for",
                    "some zeny. I know, I know..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "I was pretty lucky the",
                    "police didn't see me take",
                    "it. Later that day, some guy",
                    "came up to me and offered",
                    "me a lot of money for it.",
                    "I guess it was my lucky day!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "I found out later that he was",
                    "some mechanical repairman--",
                    "something. He sold everything",
                    "to buy that ring, so I guess",
                    "he wanted it desperately.",
                    "Then he just dissapeared."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Do you remember his name?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "His name...?",
                    "It was something like...",
                    "Heil? Hyre? Anyway, it",
                    "was a long time ago. Oh,",
                    "his old house is still around."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fisherman",
                args![
                    "If you're going to be",
                    "that curious, you might",
                    "as well check it out.",
                    "Let's see, he lived in",
                    "a hut near the northeast",
                    "forest guard camp."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(544), Val::from(10)])?;
            ctx.var("kielhyrequest").set(Val::from(62))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("kielhyrequest").get()?.number()? >= 62 {
        ctx.lines_as(
            "Fisherman",
            args![
                "Don't you remember",
                "what I told you? That",
                "guy lived in a hut near",
                "the northeast forest",
                "guard camp! Why don't",
                "you check that place out?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn old_fisherman(ctx: &Ctx) -> Script {
    old_fisherman_body(ctx, Vec::new()).map(|_| ())
}

fn wooden_board_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7503), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("kielhyrequest").get()?.number()? < 62 || ctx.var("kielhyrequest").get()?.number()? >= 64) {
        ctx.lines(args!["^3355FFIt's a useless", "wooden board", "in the bushes.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 62 {
        ctx.lines(args![
            "^3355FFYou found a long",
            "wooden board carved",
            "with the initials, ''K.H.''^000000"
        ])?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from("kh_kyel_port"), Val::from(2)])?;
        ctx.lines(args![
            "^3355FFYou find a portrait of",
            "a young man, that looks",
            "like a younger version",
            "of Kiel Hyre, in a broken",
            "picture frame underneath",
            "the old wooden board.^000000"
        ])?;
        ctx.call(Function::GetItem, vec![Val::from(7503), Val::from(1)])?;
        ctx.var("kielhyrequest").set(Val::from(64))?;
        ctx.next()?;
        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
        ctx.lines(args![
            "^3355FFYou have enough",
            "information by now,",
            "so you should report",
            "back to Mitchell.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn wooden_board_kh(ctx: &Ctx) -> Script {
    wooden_board_kh_body(ctx, Vec::new()).map(|_| ())
}

fn receiver_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(
            Function::MobCount,
            vec![Val::from("kh_kiehl01"), Val::from("Receiver#kh::OnMyMobDead")],
        )?
        .number()?
        < 1
    {
        ctx.mes("^333333*BBBZZZ*^000000")?;
        if ctx.var("kielhyrequest").get()? == 74 {
            ctx.next()?;
            ctx.lines_as(
                "????",
                args![
                    "^333333*Bzzzz...*",
                    "I've never seen you",
                    "before. Did Father send",
                    "you to kill me? We'll just",
                    "see about that! Go ahead,",
                    "try to find me, adventurer.^000000."
                ],
            )?;
            ctx.var("kielhyrequest").set(Val::from(76))?;
        }
        ctx.close_window()?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(16),
                Val::from(32),
                Val::from("Alicel"),
                Val::from(1739),
                Val::from(1),
                Val::from("Receiver#kh::OnMyMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(18),
                Val::from(31),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("Receiver#kh::OnMyMobDead"),
            ],
        )?;
        return Err(Stop::End);
    } else {
        return Err(Stop::End);
    }
}

pub fn receiver_kh(ctx: &Ctx) -> Script {
    receiver_kh_body(ctx, Vec::new()).map(|_| ())
}

fn receiver_kh_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(
            Function::MobCount,
            vec![Val::from("kh_kiehl01"), Val::from("Receiver#kh::OnMyMobDead")],
        )?
        .number()?
        < 1
    {
        ctx.call(
            Function::MakeItem,
            vec![Val::from(7506), Val::from(1), Val::from("this"), Val::from(19), Val::from(36)],
        )?;
    }
    return Err(Stop::End);
}

pub fn receiver_kh_onmymobdead(ctx: &Ctx) -> Script {
    receiver_kh_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn flower_vase_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 74 {
        ctx.lines(args!["^3355FFYou found", "a flower vase.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? >= 74 {
        ctx.lines(args!["^3355FFYou found", "a flower vase.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Pick Up Vase:Break Vase:Turn Vase Upside-down")])? {
            1 => {
                ctx.mes("^3355FFThe vase is empty.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFYou can't destroy",
                    "this vase, even by",
                    "striking it with all your",
                    "might. It must have been",
                    "specially manufactured by",
                    "the Rekenber Corporation.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines(args![
                    "^3355FFThe following words",
                    "are written at the",
                    "bottom of the vase.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vase Message",
                    args!["''The rabbit often", "observes the door", "The night eats the", "pickled orange.''"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThe following words",
            "are written at the",
            "bottom of the vase.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Vase Message",
            args![
                "''The rabbit often",
                "observes the door.",
                "The night eats the",
                "pickled orange.''"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn flower_vase_kh1(ctx: &Ctx) -> Script {
    flower_vase_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn box_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(7505), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFJust a second...",
            "You're carrying too",
            "many items with you",
            "right now, so you'll",
            "need to free up more",
            "Inventory space first...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? < 74 {
        ctx.lines(args![
            "^3355FFYou found a box with",
            "a button for each letter",
            "of the alphabet on top of it.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        ctx.mes("^3355FFNothing happened.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? >= 74 {
        if ctx.call(Function::CountItem, vec![Val::from(7505)])?.number()? < 1 {
            ctx.lines(args![
                "^3355FFYou found a box with",
                "a button for each letter",
                "of the alphabet on top of it.^000000"
            ])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_khinput_s = input;
            if l_khinput_s.clone() == "open the door" {
                ctx.lines(args![
                    "^3355FFAs soon as you enter the",
                    "password, the nearby door",
                    "emits a pleasant chiming",
                    "sound, and the box pops",
                    "open to reveal a small key.^000000"
                ])?;
                ctx.call(Function::GetItem, vec![Val::from(7505), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.mes("^3355FFNothing happened.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines(args!["^3355FFThis is where you", "found the Toy key^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args!["^3355FFThe box is wide", "open, and there", "is nothing in it.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn box_kh1(ctx: &Ctx) -> Script {
    box_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_bigdoorkhq1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var(".khdoor1opened").get()? == 0 {
        ctx.lines(args![
            "^3355FFThe door is locked,",
            "but there is a narrow",
            "slot next to the doorknob.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Black Keycard" && ctx.call(Function::CountItem, vec![Val::from(7506)])?.number()? >= 1 {
            ctx.lines(args![
                "^3355FFYou insert the",
                "Black Keycard into the",
                "slot, and successfully",
                "unlock and open the door.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7506), Val::from(1)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Big_Door_1_Warp::OnEnable")])?;
            ctx.call(Function::EnableNpc, vec![Val::from("Big_Door_1_Warp")])?;
            ctx.var(".khdoor1opened").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou try to push the",
            "door open with all",
            "your might, but fail",
            "to make it budge.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("^3355FFThe door is open.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn big_door_bigdoorkhq1(ctx: &Ctx) -> Script {
    big_door_bigdoorkhq1_body(ctx, Vec::new()).map(|_| ())
}

pub fn big_door_1_warp(ctx: &Ctx) -> Script {
    big_door_1_warp_run(ctx, BigDoor1WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn big_door_1_warp_onenable(ctx: &Ctx) -> Script {
    big_door_1_warp_run(ctx, BigDoor1WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn big_door_1_warp_oninit(ctx: &Ctx) -> Script {
    big_door_1_warp_run(ctx, BigDoor1WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn big_door_1_warp_ontimer30000(ctx: &Ctx) -> Script {
    big_door_1_warp_run(ctx, BigDoor1WarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn big_door_1_warp_ontouch(ctx: &Ctx) -> Script {
    big_door_1_warp_run(ctx, BigDoor1WarpStep::OnTouch, Vec::new()).map(|_| ())
}

fn big_door_bigdoorkhq2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var(".khdoor2opened").get()? == 0 {
        ctx.lines(args![
            "^3355FFThe door is locked,",
            "but there is a small",
            "keyhole next to the knob.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Toy Key" && ctx.call(Function::CountItem, vec![Val::from(7505)])?.number()? >= 1 {
            ctx.lines(args![
                "^3355FFYou insert the key into",
                "the keyhole, and the door",
                "unlocks with a click as",
                "you turn the key.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7505), Val::from(1)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Big_Door_2_Warp::OnEnable")])?;
            ctx.call(Function::EnableNpc, vec![Val::from("Big_Door_2_Warp")])?;
            ctx.var(".khdoor2opened").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou try to push the",
            "door open with all",
            "your might, but fail",
            "to make it budge.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("^3355FFThe door is open.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn big_door_bigdoorkhq2(ctx: &Ctx) -> Script {
    big_door_bigdoorkhq2_body(ctx, Vec::new()).map(|_| ())
}

pub fn big_door_2_warp(ctx: &Ctx) -> Script {
    big_door_2_warp_run(ctx, BigDoor2WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn big_door_2_warp_onenable(ctx: &Ctx) -> Script {
    big_door_2_warp_run(ctx, BigDoor2WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn big_door_2_warp_oninit(ctx: &Ctx) -> Script {
    big_door_2_warp_run(ctx, BigDoor2WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn big_door_2_warp_ontimer30000(ctx: &Ctx) -> Script {
    big_door_2_warp_run(ctx, BigDoor2WarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn big_door_2_warp_ontouch(ctx: &Ctx) -> Script {
    big_door_2_warp_run(ctx, BigDoor2WarpStep::OnTouch, Vec::new()).map(|_| ())
}

fn big_door_bigdoorkhq3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var(".khdoor3opened").get()? == 0 {
        ctx.lines(args![
            "^3355FFThe door is locked,",
            "but there is a narrow",
            "slot next to the doorknob.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Black Keycard" && ctx.call(Function::CountItem, vec![Val::from(7506)])?.number()? >= 1 {
            ctx.lines(args![
                "^3355FFYou insert the",
                "Black Keycard into the",
                "slot, and successfully",
                "unlock and open the door.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7506), Val::from(1)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Big_Door_3_Warp::OnEnable")])?;
            ctx.call(Function::EnableNpc, vec![Val::from("Big_Door_3_Warp")])?;
            ctx.var(".khdoor3opened").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou try to push the",
            "door open with all",
            "your might, but fail",
            "to make it budge.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("^3355FFThe door is open.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn big_door_bigdoorkhq3(ctx: &Ctx) -> Script {
    big_door_bigdoorkhq3_body(ctx, Vec::new()).map(|_| ())
}

pub fn big_door_3_warp(ctx: &Ctx) -> Script {
    big_door_3_warp_run(ctx, BigDoor3WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn big_door_3_warp_onenable(ctx: &Ctx) -> Script {
    big_door_3_warp_run(ctx, BigDoor3WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn big_door_3_warp_oninit(ctx: &Ctx) -> Script {
    big_door_3_warp_run(ctx, BigDoor3WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn big_door_3_warp_ontimer30000(ctx: &Ctx) -> Script {
    big_door_3_warp_run(ctx, BigDoor3WarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn big_door_3_warp_ontouch(ctx: &Ctx) -> Script {
    big_door_3_warp_run(ctx, BigDoor3WarpStep::OnTouch, Vec::new()).map(|_| ())
}

fn big_door_bigdoorkhq4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var(".khdoor4opened").get()? == 0 {
        ctx.lines(args![
            "^3355FFThe door is locked,",
            "but there is a narrow",
            "slot next to the doorknob.^000000"
        ])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_khinput_s = input;
        if l_khinput_s.clone() == "Black Keycard" && ctx.call(Function::CountItem, vec![Val::from(7506)])?.number()? >= 2 {
            ctx.lines(args![
                "^3355FFYou insert the",
                "Black Keycard into the",
                "slot, and successfully",
                "unlock and open the door.^000000"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7506), Val::from(2)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Big_Door_4_Warp::OnEnable")])?;
            ctx.call(Function::EnableNpc, vec![Val::from("Big_Door_4_Warp")])?;
            ctx.var(".khdoor4opened").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou try to push the",
            "door open with all",
            "your might, but fail",
            "to make it budge.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("^3355FFThe door is open.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn big_door_bigdoorkhq4(ctx: &Ctx) -> Script {
    big_door_bigdoorkhq4_body(ctx, Vec::new()).map(|_| ())
}

pub fn big_door_4_warp(ctx: &Ctx) -> Script {
    big_door_4_warp_run(ctx, BigDoor4WarpStep::Start, Vec::new()).map(|_| ())
}

pub fn big_door_4_warp_onenable(ctx: &Ctx) -> Script {
    big_door_4_warp_run(ctx, BigDoor4WarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn big_door_4_warp_oninit(ctx: &Ctx) -> Script {
    big_door_4_warp_run(ctx, BigDoor4WarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn big_door_4_warp_ontimer30000(ctx: &Ctx) -> Script {
    big_door_4_warp_run(ctx, BigDoor4WarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn big_door_4_warp_ontouch(ctx: &Ctx) -> Script {
    big_door_4_warp_run(ctx, BigDoor4WarpStep::OnTouch, Vec::new()).map(|_| ())
}

fn robots_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(
            Function::MobCount,
            vec![Val::from("kh_kiehl01"), Val::from("Robots#kh1::OnMyMobDead")],
        )?
        .number()?
        < 1
    {
        ctx.lines(args![
            "^3355FFAs soon as you",
            "touch the test tube,",
            "a bunch of robots",
            "suddenly appeared.^000000."
        ])?;
        ctx.close_window()?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(18),
                Val::from(181),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("Robots#kh1::OnMyMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(18),
                Val::from(180),
                Val::from("Alicel"),
                Val::from(1739),
                Val::from(1),
                Val::from("Robots#kh1::OnMyMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(18),
                Val::from(179),
                Val::from("Aliot"),
                Val::from(1740),
                Val::from(1),
                Val::from("Robots#kh1::OnMyMobDead"),
            ],
        )?;
        ctx.call(
            Function::Monster,
            vec![
                Val::from("kh_kiehl01"),
                Val::from(18),
                Val::from(178),
                Val::from("Alicel"),
                Val::from(1739),
                Val::from(1),
                Val::from("Robots#kh1::OnMyMobDead"),
            ],
        )?;
        return Err(Stop::End);
    } else {
        return Err(Stop::End);
    }
}

pub fn robots_kh1(ctx: &Ctx) -> Script {
    robots_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn robots_kh1_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx
        .call(
            Function::MobCount,
            vec![Val::from("kh_kiehl01"), Val::from("Robots#kh1::OnMyMobDead")],
        )?
        .number()?
        < 1
    {
        ctx.call(
            Function::MakeItem,
            vec![Val::from(7506), Val::from(1), Val::from("this"), Val::from(18), Val::from(180)],
        )?;
    }
    return Err(Stop::End);
}

pub fn robots_kh1_onmymobdead(ctx: &Ctx) -> Script {
    robots_kh1_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn robots_kh2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn robots_kh2(ctx: &Ctx) -> Script {
    robots_kh2_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_bigdoorkhq5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khpryingitem_s = Val::from("");
    if ((ctx.var("$@khquestbusy").get()? == 0 && ctx.var("kielhyrequest").get()?.number()? >= 74)
        && ctx.var("kielhyrequest").get()?.number()? <= 106)
    {
        if ctx.var("kielhyrequest").get()?.number()? < 86 {
            ctx.lines(args![
                "^3355FFThis large door..",
                "is closed shut.",
                "If you listen carefully,",
                "you can hear the door",
                "hinges slightly squeak.^000000"
            ])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Push Door:Kick Door:Shake Door:Pull Door:Lift Door")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines(args![
                "^3355FFA group of monsters",
                "suddenly appeared as",
                "soon as you applied",
                "pressure to the door.",
                "This must be some",
                "kind of security device.^000000"
            ])?;
            ctx.close_window()?;
            ctx.var("@khdoorpushattempt")
                .set((ctx.var("@khdoorpushattempt").get()? + Val::from(1)))?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("kh_kiehl01"),
                    Val::from(163),
                    Val::from(183),
                    Val::from("Alicel"),
                    Val::from(1739),
                    Val::from(1),
                ],
            )?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("kh_kiehl01"),
                    Val::from(163),
                    Val::from(179),
                    Val::from("Aliot"),
                    Val::from(1740),
                    Val::from(1),
                ],
            )?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("kh_kiehl01"),
                    Val::from(169),
                    Val::from(183),
                    Val::from("Alicel"),
                    Val::from(1739),
                    Val::from(1),
                ],
            )?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("kh_kiehl01"),
                    Val::from(169),
                    Val::from(179),
                    Val::from("Aliot"),
                    Val::from(1740),
                    Val::from(1),
                ],
            )?;
            if ctx.var("@khdoorpushattempt").get()?.number()? >= 3 {
                ctx.var("kielhyrequest").set(Val::from(86))?;
            }
            return Err(Stop::End);
        } else if (ctx.var("kielhyrequest").get()?.number()? > 84 && ctx.var("kielhyrequest").get()?.number()? < 94) {
            ctx.lines(args![
                "^3355FFYou apply some",
                "pressure to the door,",
                "and find that you can",
                "budge it slightly, but",
                "you can't fully open it.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFIf you wedged something",
                "into the gap between the",
                "door and its frame, and",
                "fully leveraged it, then you",
                "should be able to open it.^000000"
            ])?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Steel:Rusty Iron Piece:Solid Iron Piece:Iron Piece:Screw:Cancel")],
                )?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1))
                    && !subject1.loosely_equals(&Val::from(2))
                    && !subject1.loosely_equals(&Val::from(3))
                    && !subject1.loosely_equals(&Val::from(4))
                    && !subject1.loosely_equals(&Val::from(5))
                    && !subject1.loosely_equals(&Val::from(6));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    l_khpryingitem_s = Val::from("Steel");
                    break 'b1;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    l_khpryingitem_s = Val::from("Rusty Iron Piece");
                    break 'b1;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    if ctx.call(Function::CountItem, vec![Val::from(7507)])?.number()? >= 1 {
                        if ctx.var("kielhyrequest").get()?.number()? < 92 {
                            ctx.lines(args![
                                "^3355FFYou insert one end of",
                                "a Solid Iron Piece into the",
                                "door's gap in a strenuous",
                                "effort to pry the door open",
                                "The gap widens a little bit,",
                                "but you break one of your",
                                "Solid Iron Pieces.^000000"
                            ])?;
                            ctx.call(Function::DelItem, vec![Val::from(7507), Val::from(1)])?;
                            ctx.var("kielhyrequest").set((ctx.var("kielhyrequest").get()? + Val::from(2)))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("kielhyrequest").get()? == 92 {
                            ctx.lines(args![
                                "^3355FFWith a mighty heave,",
                                "you pry a Solid Iron",
                                "Piece into the door jamb,",
                                "and fling the door wide open",
                                "Unable the withstand the",
                                "awesome force, this Solid",
                                "Iron Piece shatters into dust.^000000"
                            ])?;
                            ctx.call(Function::DelItem, vec![Val::from(7507), Val::from(1)])?;
                            ctx.var("kielhyrequest").set(Val::from(94))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.lines(args![
                            "^3355FFIf only you had a Solid",
                            "Iron Piece you could use to",
                            "pry open this door.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                    matched1 = true;
                }
                if matched1 {
                    l_khpryingitem_s = Val::from("Iron Piece");
                    break 'b1;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                    matched1 = true;
                }
                if matched1 {
                    l_khpryingitem_s = Val::from("Screw");
                    break 'b1;
                }
                if !matched1 && subject1.loosely_equals(&Val::from(6)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines(args![
                        "^3355FFLet's look for something",
                        "heavy we can use to pry",
                        "open this door.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines(args![
                ((Val::from("^3355FFThis ") + l_khpryingitem_s.clone()) + Val::from(" is far")),
                "to weak for what you're using it for",
                "and breaks.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("kielhyrequest").get()?.number()? >= 94 && ctx.var("kielhyrequest").get()?.number()? <= 104) {
            if ctx.var(".khdoor5opened").get()? == 0 {
                ctx.lines(args![
                    "^3355FFThe large door",
                    "is wide open, and.",
                    "you may now enter.^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                    1 => {
                        ctx.close_window()?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Big_Door_5_Warp::OnEnable")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("Big_Door_5_Warp")])?;
                        ctx.var(".khdoor5opened").set(Val::from(1))?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args![
                            "^3355FFWho knows what is on the",
                            "other side of this door. Let's",
                            "think about it before barging in..^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.mes("^3355FFThe door is open.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines(args![
                "^3355FFThis large door..",
                "is closed shut.",
                "If you listen carefully,",
                "you can hear the door",
                "hinges slightly squeak.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args![
            "^3355FFThis large door..",
            "is closed shut.",
            "If you listen carefully,",
            "you can hear the door",
            "hinges slightly squeak.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn big_door_bigdoorkhq5(ctx: &Ctx) -> Script {
    big_door_bigdoorkhq5_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_5_warp_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn big_door_5_warp(ctx: &Ctx) -> Script {
    big_door_5_warp_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_5_warp_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
    return Err(Stop::End);
}

pub fn big_door_5_warp_onenable(ctx: &Ctx) -> Script {
    big_door_5_warp_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_5_warp_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_5_Warp")])?;
    ctx.call(
        Function::SetVariableOfNpc,
        vec![
            Val::from(".khdoor5opened"),
            Val::from("Big Door#BigDoorKHQ5"),
            Val::from(0),
            Val::from(0),
        ],
    )?;
    return Err(Stop::End);
}

pub fn big_door_5_warp_ontimer30000(ctx: &Ctx) -> Script {
    big_door_5_warp_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_5_warp_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_5_Warp")])?;
    return Err(Stop::End);
}

pub fn big_door_5_warp_oninit(ctx: &Ctx) -> Script {
    big_door_5_warp_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn big_door_5_warp_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? >= 46 {
        ctx.call(Function::Warp, vec![Val::from("kh_kiehl02"), Val::from(50), Val::from(7)])?;
    } else {
        ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(166), Val::from(183)])?;
    }
    return Err(Stop::End);
}

pub fn big_door_5_warp_ontouch(ctx: &Ctx) -> Script {
    big_door_5_warp_ontouch_body(ctx, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::Start, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_ontouch(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_ontimer300000(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_ontimer600000(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_ontimer900000(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnTimer900000, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_ontimer1200000(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnTimer1200000, Vec::new()).map(|_| ())
}

pub fn kiehl_room_trap_onglobaltimeroff(ctx: &Ctx) -> Script {
    kiehl_room_trap_run(ctx, KiehlRoomTrapStep::OnGlobalTimerOff, Vec::new()).map(|_| ())
}
