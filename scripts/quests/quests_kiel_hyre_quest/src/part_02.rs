use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn elly_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
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
    if ctx.var("kielhyrequest").get()?.number()? < 30 {
        ctx.lines_as(
            "Elly",
            args![
                "Who the heck are you?",
                "Y-you're not supposed",
                "to be able to get inside!",
                "Get out of here right now!"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(-99), Val::from(0)])?;
        ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(100), Val::from(100)])?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()?.number()? < 32 {
        ctx.lines_as("Elly", args!["......", ".........", "............"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 32 {
        ctx.lines_as("Elly", args!["......", ".........", "............"])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFElly seems to have",
            "been cursed, and is",
            "completely still and",
            "lifeless. You've got to",
            "try something, but what?^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Wake her up by shaking.:Wake her up by yelling.:Ignore")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou grab Elly by the",
                    "shoulders, and try to get",
                    "her to respond by violently",
                    "shaking her entire body.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Elly? Elly! No!", "Come back to us!"],
                )?;
                ctx.next()?;
                ctx.mes("^3355FFElly's not responding...^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                if l_khinput_s.clone() == "Wake up, Elly!" {
                    ctx.lines_as("Elly", args!["............."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFAs soon as you said those",
                        "words, a ^000000Small Golden Key^3355FF",
                        "and a ^000000Button^3355FF drop from Elly's",
                        "hands. It looks like she woke",
                        "up, but only for an instant.^000000"
                    ])?;
                    ctx.call(Function::GetItem, vec![Val::from(7493), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(7494), Val::from(1)])?;
                    ctx.var("kielhyrequest").set(Val::from(34))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Elly", args!["......", ".........", "............"])?;
                    ctx.next()?;
                    ctx.mes("^3355FFElly's not responding...^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            3 => {
                ctx.lines(args![
                    "^3355FFAlright...",
                    "But sooner or later,",
                    "you should try to break",
                    "the curse placed on Elly.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("kielhyrequest").get()? == 34 {
        if ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? < 1 {
            ctx.lines(args![
                "^3355FFMaybe the golden key^000000",
                "unlocks something in the^000000",
                "Cottage. Let's take another look.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFIt looks like Elly's",
                "Golden Key might fit",
                "into the keyhole on the",
                "Grey Box you found inside",
                "Kiel Hyre's Cottage.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou successfully open the",
                "Grey Box with the Golden Key,",
                "and find a Blue Keycard, along",
                "with a folded note, inside the",
                "Grey Box. You quickly read",
                "the note's contents...^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "Dearest Elly,",
                "Kiehl finally broke the taboo,",
                "and tried to transform you guys",
                "into something horrible.",
                "^FFFFFF_^000000",
                "By the time you read this,",
                "my life is probably in danger.",
                "Whether I live depends on you.",
                "^FFFFFF_^000000",
                "You'll already learn if you",
                "meet Puppet, but I want to tell",
                "you myself: you're not human.",
                "You'll learn the details if you",
                "enter the factory by using the",
                "entrance near the grave next",
                "to the church. Then, I want",
                "you to find Allysia inside",
                "the factory's secret room.",
                "I've registered your name in",
                "her security system, so don't",
                "worry. Hopefully, Allysia will",
                "then come to save me...",
                "^FFFFFF_^000000",
                "Sorry about that,",
                "Grandpa"
            ])?;
            ctx.call(Function::GetItem, vec![Val::from(7495), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7491), Val::from(1)])?;
            ctx.var("kielhyrequest").set(Val::from(36))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("kielhyrequest").get()?.number()? >= 36 {
        ctx.lines(args![
            "^3355FFThis is where you",
            "discovered the note",
            "locked inside the Grey Box.",
            "The following message was",
            "written in the note by Elly's",
            "grandfather, Kiel Hyre.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "Dearest Elly,",
            "Kiehl finally broke the taboo,",
            "and tried to transform you guys",
            "into something horrible.",
            "^FFFFFF_^000000",
            "By the time you read this,",
            "my life is probably in danger.",
            "Whether I live depends on you.",
            "^FFFFFF_^000000",
            "You'll already learn if you",
            "meet Puppet, but I want to tell",
            "you myself: you're not human.",
            "You'll learn the details if you",
            "enter the factory by using the",
            "entrance near the grave next",
            "to the church. Then, I want",
            "you to find Allysia inside",
            "the factory's secret room.",
            "I've registered your name in",
            "her security system, so don't",
            "worry. Hopefully, Allysia will",
            "then come to save me...",
            "^FFFFFF_^000000",
            "Sorry about that,",
            "Grandpa"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn elly(ctx: &Ctx) -> Script {
    elly_body(ctx, Vec::new()).map(|_| ())
}

fn cookie_basket_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    if ctx.var("kielhyrequest").get()?.number()? < 30 {
        ctx.lines(args!["^3355FFIt's a cookie", "basket filled with", "delicious cookies.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 30 {
        ctx.lines(args![
            "^3355FFThis must be",
            "Elly's cookie basket.",
            "There appears to be",
            "a folded note wedged",
            "between the cookies.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Read Note:Ignore")])? {
            1 => {
                ctx.lines(args![
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", help!")),
                    "There's this guy dressed in",
                    "black who's walking around,",
                    "and casting this weird spell!",
                    "He's the one that's been making",
                    "people cold and lifeless as",
                    "puppets! I'm getting scared!",
                    "^FFFFFF_^000000",
                    "I hope you get this note...",
                    "He ran after me, but I locked",
                    "myself in my room. I'm going",
                    "to leave my window open so that",
                    "you can still find me. I hope he",
                    "doesn't cast his curse on me!"
                ])?;
                ctx.var("kielhyrequest").set(Val::from(32))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFThat note probably",
                    "wasn't written just",
                    "for you, anyway.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("kielhyrequest").get()?.number()? < 108 {
        ctx.lines(args![
            "^3355FFThese cookies aren't",
            "stale yet, but they're no",
            "longer warm. If they're",
            "not at room temperature,",
            "then they're a little cold.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 108 {
        ctx.lines(args![
            "^3355FFAs you stare at the",
            "cookie basket, the wind",
            "from the window jostles",
            "it, revealing a letter that",
            "was placed underneath.",
            "It was probably written",
            "by Elly for you to read.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            ((Val::from("^333333Dearest ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
            " ",
            "My teachers usually yell at me",
            "since I make so many mistakes,",
            "but today Mrs. Lecollane gave me",
            "praise for my yummy cookies!",
            "It's all thanks to you, my friend.",
            "I'm very happy we've met:",
            "you've taught me that there",
            "are good people in the world.",
            "And I know Grandpa will like",
            "you, though, I don't know",
            "where he could be...",
            " ",
            "Someday, I hope to become",
            "as nice a person as you are.",
            "Let's keep in touch and be",
            "really good friends, okay?",
            " ",
            "Yours, Elly^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt feels like you can",
            "still sense Elly's warmth",
            "and kindness from her",
            "cookie basket. You picked",
            "out a cookie, and put it",
            "in your mouth. It was",
            "deliciously bittersweet."
        ])?;
        ctx.var("kielhyrequest").set(Val::from(109))?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                ((Val::from("") + l_input_s.clone()) + Val::from("")),
                "It's time to go back. I can't",
                "stay here much longer."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(69), Val::from(183)])?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFAll the cookies",
            "in this basket are",
            "stale! Well, it shouldn't",
            "come as a surprise.",
            "It's been a long time",
            "since they were baked.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn cookie_basket_kh(ctx: &Ctx) -> Script {
    cookie_basket_kh_body(ctx, Vec::new()).map(|_| ())
}

fn window_kh2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("yuno_fild08"), Val::from(69), Val::from(183)])?;
    return Err(Stop::End);
}

pub fn window_kh2(ctx: &Ctx) -> Script {
    window_kh2_body(ctx, Vec::new()).map(|_| ())
}

fn grave_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khinput_s = Val::from("");
    if ctx.var("kielhyrequest").get()?.number()? < 36 {
        ctx.lines(args![
            "^3355FFIt's just a grave.",
            "It might be important",
            "to you later, but now",
            "it's not really all that",
            "helpful to you.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 36 {
        if ctx.call(Function::CountItem, vec![Val::from(7492)])?.number()? < 1 {
            ctx.lines(args![
                "^3355FFIt's just a grave.",
                "It might be important",
                "to you later, but now",
                "it's not really all that",
                "helpful to you.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFThere's a secret door",
                "near this grave. It looks",
                "like there's some kind of",
                "slot and a number pad",
                "installed on the door.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args!["^3355FFWhat should you", "insert into the slot?^000000"])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_khinput_s = input;
            if l_khinput_s.clone() == "Yellow Keycard" {
                ctx.lines(args![
                    "^3355FFAn electronic confirmation",
                    "chime sounds once you insert",
                    "the Yellow Keycard, followed by",
                    "an automated voice that asks:^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Security System", args!["Please enter the password."])?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khinput_s = input;
                ctx.next()?;
                if l_khinput_s.clone() == "4772961" {
                    ctx.lines_as("Security System", args!["Password confirmed.", "Welcome, Kiel Hyre."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou hear another ",
                        "pleasant beep, and",
                        "a secret path opens.^000000"
                    ])?;
                    ctx.var("kielhyrequest").set(Val::from(38))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Security System", args!["Incorrect password.", "Please try again."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines(args![
                    "^3355FFWhatever you're trying",
                    "to insert into the slot",
                    "isn't working at all...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if (ctx.var("kielhyrequest").get()?.number()? >= 38 && ctx.var("kielhyrequest").get()?.number()? < 106) {
            ctx.lines(args!["^3355FFThe door to the", "factory is wide open.^000000"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Enter:Cancel")])? {
                1 => {
                    ctx.call(Function::Warp, vec![Val::from("kh_dun01"), Val::from(3), Val::from(230)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines(args!["......", ".........", "............"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("kielhyrequest").get()?.number()? >= 106 {
            if ctx.call(Function::CountItem, vec![Val::from(7509)])?.number()? < 1 {
                ctx.lines(args!["^3355FFThe secret entrance", "has now been sealed.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "As you aproach the",
                    "grave, it begins to",
                    "emit flashes of light.^000000"
                ])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Use the ^0000FFLuxurious Keycard^000000:Ignore")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines(args!["Once you use the", "Luxurious Keycard,", "a secret path opens^000000"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Enter:cancel")])? {
                            1 => {
                                ctx.call(Function::Warp, vec![Val::from("kh_dun01"), Val::from(3), Val::from(230)])?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines(args!["......", ".........", "............"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines(args!["......", ".........", "............"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn grave_kh(ctx: &Ctx) -> Script {
    grave_kh_body(ctx, Vec::new()).map(|_| ())
}

fn cottage_keeper_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("kielhyrequest").get()?.number()? < 12 || ctx.var("kielhyrequest").get()?.number()? > 12) {
        ctx.lines_as(
            "Cottage Keeper",
            args![
                "This is private property,",
                "so please do not enter this",
                "area unless you're authorized."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 12 {
        ctx.lines_as(
            "Cottage Keeper",
            args![
                "This is private property,",
                "so please do not enter this",
                "area unless you're authorized."
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("I have an appointment with Kiel Hyre.:Alright.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Cottage Keeper",
                    args!["You have an", "appointment with", "Master Kiel Hyre?", "Um, are you sure?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                    1 => {
                        ctx.lines_as(
                            "Cottage Keeper",
                            args![
                                "There must be some sort",
                                "of mistake. Mister Hyre",
                                "wouldn't have left if he",
                                "was supposed to keep",
                                "an appointment..."
                            ],
                        )?;
                        ctx.var("kielhyrequest").set(Val::from(14))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Cottage Keeper",
                            args![
                                "Well unless you have",
                                "an appointment, I don't",
                                "think you'll be able to",
                                "meet with Mister Hyre."
                            ],
                        )?;
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
                ctx.lines_as("Cottage Keeper", args!["Goodbye."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn cottage_keeper_kh(ctx: &Ctx) -> Script {
    cottage_keeper_kh_body(ctx, Vec::new()).map(|_| ())
}

fn door_kh2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 16 {
        ctx.lines(args![
            "^3355FFThis door is locked.",
            "If someone inside won't",
            "open it for you, then you'll",
            "need the right key to unlock it."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()? == 16 {
        ctx.lines(args![
            "^3355FFThis door is locked.",
            "If someone inside won't",
            "open it for you, then you'll",
            "need the right key to unlock it."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Unlock:Cancel")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou unlock the door with the",
                    "key that Elly gave you, and",
                    "as you push it open, a folded",
                    "note dropped from top of the",
                    "door. The following words",
                    "are written on this crude note."
                ])?;
                ctx.next()?;
                ctx.lines(args!["6 Forward,", "3 Left,", "3 Forward,", "4 Left"])?;
                ctx.close_window()?;
                ctx.var("kielhyrequest").set(Val::from(18))?;
                ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(188), Val::from(18)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.mes("^3355FFThe door is open.^000000")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Enter:Cancel")])? {
            1 => {
                ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(188), Val::from(18)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn door_kh2(ctx: &Ctx) -> Script {
    door_kh2_body(ctx, Vec::new()).map(|_| ())
}

fn door_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()? != 16 {
        ctx.lines(args![
            "^3355FFThis door is locked.",
            "If someone inside won't",
            "open it for you, then you'll",
            "need the right key to unlock it."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()? == 16 {
        ctx.lines(args![
            "^3355FFThis door is locked.",
            "If someone inside won't",
            "open it for you, then you'll",
            "need the right key to unlock it."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Open:Cancel")])? {
            1 => {
                ctx.lines(args!["^3355FFYou don't have", "the key that can", "unlock this door.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn door_kh1(ctx: &Ctx) -> Script {
    door_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn wall_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 28 {
        ctx.lines(args![
            "^3355FFIt's just a wall.",
            "It's not particularly",
            "standing in your way.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("kielhyrequest").get()?.number()? >= 28 {
        ctx.lines(args!["^3355FFThe wall is now", "open, revealing", "a secret path.^000000"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Enter:Cancel")])? {
            1 => {
                ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(17), Val::from(177)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn wall_kh(ctx: &Ctx) -> Script {
    wall_kh_body(ctx, Vec::new()).map(|_| ())
}

fn book_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()? != 26 {
        ctx.lines(args![
            "^3355FFThis bookshelf is^000000",
            "^3355FFcrammed with many^000000",
            "^3355FFlarge, hardcover books^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 26 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 8 {
            ctx.lines(args![
                "^3355FFWithout thinking, you",
                "reach for a book from",
                "the bookshelf. As you",
                "pull it towards you, the",
                "wall slides opens open",
                "to reveal a secret path.^000000"
            ])?;
            ctx.var("kielhyrequest").set(Val::from(28))?;
            ctx.call(
                Function::NpcSpecialEffect,
                vec![ctx.constant("EF_READYPORTAL2")?, ctx.constant("AREA")?, Val::from("Wall#kh")],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFThis bookshelf is",
                "crammed with many",
                "large, hardcover books.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn book_kh1(ctx: &Ctx) -> Script {
    book_kh1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LetterKhStep {
    Start,
    OnTouch,
}

fn letter_kh_run(ctx: &Ctx, mut step: LetterKhStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LetterKhStep::Start => {
                step = LetterKhStep::OnTouch;
                continue 'machine;
            }
            LetterKhStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 18 {
                    if ctx.call(Function::CheckWeight, vec![Val::from(7490), Val::from(1)])? == 0 {
                        ctx.lines(args![
                            "^3355FFJust a minute...!",
                            "There's something on",
                            "the floor here, but you",
                            "can't pick it up since",
                            "you're carrying too",
                            "many items now.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines(args![
                        "^3355FFYou've found a letter",
                        "on the floor. Naturally,",
                        "you pick it up, despite",
                        "the fact that it's not",
                        "addressed to you.^000000"
                    ])?;
                    ctx.call(Function::GetItem, vec![Val::from(7490), Val::from(1)])?;
                    ctx.var("kielhyrequest").set(Val::from(20))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFYou entered the room.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn letter_kh(ctx: &Ctx) -> Script {
    letter_kh_run(ctx, LetterKhStep::Start, Vec::new()).map(|_| ())
}

pub fn letter_kh_ontouch(ctx: &Ctx) -> Script {
    letter_kh_run(ctx, LetterKhStep::OnTouch, Vec::new()).map(|_| ())
}

fn box_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^FFFFFF##^000000*Tasty-Nutricious-Delicious*",
        "*Cute Pet Doof Vending Machine*",
        "The best food for your Cute Pets",
        "that is superior to the Pet Food",
        "you can buy in the market!",
        "^FFFFFF_^000000",
        "Price: 1,100 zeny (cheap!)"
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Buy!:Cancel")])? {
        1 => {
            if ctx.var("Zeny").get()?.number()? < 1100 {
                ctx.lines(args![
                    "^3355FFUnfortunately, you",
                    "don't have enough",
                    "zeny to insert into",
                    "the vending machine.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "^3355FFOh no...!^000000",
                    "^3355FF1,100 zeny seems kind",
                    "of expensive for Pet Food,",
                    "but if it's better than the",
                    "normal stuff, it might",
                    "be worth a shot.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFOh no...!",
                    "There's nothing",
                    "special about it",
                    "at all! It's just",
                    "normal Pet Food!"
                ])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1100))?))?;
                ctx.call(Function::GetItem, vec![Val::from(537), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines(args![
                "^3355FFThis vending machine",
                "seems really shady and",
                "suspicious for some reason.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn box_khp1(ctx: &Ctx) -> Script {
    box_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn apple_box_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["^3355FFYou find a box filled with", "ripe, delicious apples.^000000"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Take the box:Leave it")])? {
        1 => {
            ctx.lines(args![
                "^3355FFNo...! Wait!",
                "Something's wrong!",
                "Whatever you do,",
                "don't take this box!^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFAnd so, your inner",
                "voice, your Jungian",
                "shadow if you will,",
                "prevented you from",
                "taking the box.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines(args!["^3355FFSomething is wrong", "with this box of apples.^000000"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn apple_box_khp1(ctx: &Ctx) -> Script {
    apple_box_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn map_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("khcottagepoem1").get()?.number()? < 2 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFA magnificent world map,",
            "detailing the Rune-Midgarts",
            "Kingdom, Schwarzwald Republic,",
            "as well as another country to the",
            "west whose name is unfamiliar",
            "to you, is pasted to this wall.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAs you examine the",
            "world map more closely,",
            "you find that something has",
            "been hidden underneath it.^000000"
        ])?;
        match runtime::select_values(ctx, &[Val::from("Ignore:Examine")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFWhatever might be",
                    "hidden beneath this",
                    "map probably isn't",
                    "important enough for",
                    "you to investigate.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFUnfortunately, you can't",
                    "see what's under the map",
                    "since it's pasted to the wall.",
                    "You'll need to find some",
                    "way to loosen the glue",
                    "without damaging the map...^000000"
                ])?;
                ctx.var("khcottagepoem1").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("khcottagepoem1").get()? == 2 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFYou bring the pot of",
            "steaming hot liquid",
            "close to the world map.",
            "As the paste on the wall",
            "moistens, the map slowly",
            "begins to peel back.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe peeling map reveals",
            "a folded piece of paper",
            "hidden beneath it. You",
            "take the paper, and smooth",
            "the map out to adhere it to the",
            "wall once again. A message is",
            "written on the piece of paper.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^0000FFShe will be in a place",
            "as cold as the poles.",
            "When the well is dried",
            "and the earth is cracked,",
            "the path to her heart, a",
            "heart as transparent as",
            "crystal, will be open.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFI'll have to fight four",
            "snakes with four swords",
            "to find her. The first sword",
            "is love. The second sword",
            "is despair. The third sword",
            "is rage. The fourth sword is",
            "hope. To find her, to rescue her.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FF...............................",
            "The deeper meaning",
            "of this poem, if it even",
            "exists, eludes you.^000000"
        ])?;
        ctx.var("khcottagepoem1").set(Val::from(3))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("khcottagepoem1").get()?.number()? > 2 || ctx.var("kielhyrequest").get()?.number()? >= 30) {
        ctx.lines(args![
            "^3355FFThis is were you found",
            "the paper on which the",
            "poem was written. Perhaps",
            "it would be a good idea to",
            "refresh your memory and",
            "read that poem again.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Read:Cancel")])? {
            1 => {
                ctx.lines(args![
                    "^0000FFShe will be in a place",
                    "as cold as the poles.",
                    "When the well is dried",
                    "and the earth is cracked,",
                    "the path to her heart, a",
                    "heart as transparent as",
                    "crystal, will be open.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFI'll have to fight four",
                    "snakes with four swords",
                    "to find her. The first sword",
                    "is love. The second sword",
                    "is despair. The third sword",
                    "is rage. The fourth sword is",
                    "hope. To find her, to rescue her.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FF...............................",
                    "The deeper meaning",
                    "of this poem, if it even",
                    "exists, eludes you.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFThere's no need for",
                    "you to reread this poem.",
                    "You're a freakin' genius!^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn map_khp1(ctx: &Ctx) -> Script {
    map_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn pot_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("khcottagepoem1").get()?.number()? < 1 {
        ctx.lines(args![
            "^3355FFYou've found a pot",
            "filled with boiling,",
            "steaming liquid.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("khcottagepoem1").get()? == 1 {
        ctx.lines(args![
            "^3355FFYou've found a pot",
            "filled with boiling,",
            "steaming liquid.",
            "Steam... That you could",
            "use to loosen the glue on",
            "the map... You're a genius!^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Try it.:That? That won't work!")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou picked up the",
                    "boiling pot, but",
                    "slightly burned your",
                    "hands by accident."
                ])?;
                ctx.var("khcottagepoem1").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args!["^3355FFNo, no...", "We'd better try", "something else.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("khcottagepoem1").get()?.number()? > 1 || ctx.var("kielhyrequest").get()?.number()? >= 30) {
        ctx.lines(args![
            "^3355FFThis is where you",
            "picked up the pot filled",
            "with steaming hot liquid.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pot_khp1(ctx: &Ctx) -> Script {
    pot_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn calabash_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(7329)])?.number()? < 1
        && ctx.call(Function::CountItem, vec![Val::from(7516)])?.number()? < 1)
        && ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? < 1)
        && ctx.var("kielhyrequest").get()?.number()? < 38)
    {
        if ctx.call(Function::CheckWeight, vec![Val::from(7329), Val::from(1)])? == 0 {
            ctx.lines(args![
                "^3355FFThat's a nice looking",
                "calabash. You might even",
                "get something from it...",
                "But first you better get",
                "rid of all your extra weight.",
                "And by weight, I mean items.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFIt's an expensive",
            "looking calabash--",
            "or in less fancy",
            "words, a ''gourd.''",
            "Would do you do?^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Break Open Calabash:Look Inside Calabash:Ignore")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou can't break",
                    "open that calabash...",
                    "You're a hero, not a vandal.",
                    "Now, if this act of vandalism",
                    "can be considered an act of",
                    "heroism, then it'd be okay.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFYou tenderly place",
                    "your hand into the",
                    "calabash, and gently",
                    "feel around with your",
                    "fingers until you retrieve",
                    "an Old Bronze Key.^000000"
                ])?;
                ctx.call(Function::GetItem, vec![Val::from(7329), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines(args!["^3355FFThere's probably", "nothing inside anyway.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is the expensive",
            "looking calabash from",
            "which you've obtained",
            "the Old Bronze Key.",
            "It's useless to you now.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn calabash_khp1(ctx: &Ctx) -> Script {
    calabash_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn pool_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("khcottagepoem1").get()?.number()? < 3 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args!["^3355FFYou find a fancy pool", "filled with fresh water.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("khcottagepoem1").get()? == 3 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFYou find a fancy pool",
            "filled with fresh water,",
            "along with a conscpicuous",
            "lion statue with two handles.^000000"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Pull Handles:Cancel")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args!["Which handle would", "you like to pull first?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Right Handle:Left Handle")])? {
                    1 => {
                        ctx.lines(args![
                            "^3355FFYou pull the right",
                            "handle, causing water",
                            "to gush out of the lion's",
                            "mouth. It looks pretty cool~^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args![
                            "^3355FFAs soon as you pull the",
                            "left handle, the water in",
                            "the pool drains away. The",
                            "pool empties, and you can",
                            "see a layer of green moss",
                            "covering the pool's bottom.^000000"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Ignore:Investigate")])? {
                            1 => {
                                ctx.lines(args![
                                    "^3355FFYou decide that",
                                    "a pool is little more",
                                    "than a hole without",
                                    "any water to fill it.^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines(args![
                                    "^3355FFAs you look through",
                                    "the wet moss at the",
                                    "bottom of the pool,",
                                    "you stumble upon a",
                                    "small, peculiar button.^000000"
                                ])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Press button:Don't Press Button")])? {
                                    1 => {
                                        ctx.lines(args![
                                            "^3355FFYou press the button,",
                                            "which seems to trigger",
                                            "a strange sound coming",
                                            "from the stairs at the hallway.^000000"
                                        ])?;
                                        ctx.var("khcottagepoem1").set(Val::from(4))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines(args![
                                            "^3355FFYou'd better not push",
                                            "this button. Your enemies",
                                            "must have hidden it carefully",
                                            "for you to find: it must be",
                                            "some sort of nefarious trap.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args!["^3355FFYou decided not", "to pull any handles.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (ctx.var("khcottagepoem1").get()?.number()? > 3 || ctx.var("kielhyrequest").get()?.number()? >= 30) {
        ctx.lines(args![
            "^3355FFThis is where you pressed",
            "the small button that caused",
            "some strange sound to come",
            "from the stairs near the hallway."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn pool_khp1(ctx: &Ctx) -> Script {
    pool_khp1_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ViciousDogKhp1Step {
    Start,
    OnTouch,
}

fn vicious_dog_khp1_run(ctx: &Ctx, mut step: ViciousDogKhp1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ViciousDogKhp1Step::Start => {
                step = ViciousDogKhp1Step::OnTouch;
                continue 'machine;
            }
            ViciousDogKhp1Step::OnTouch => {
                ctx.mes("*Grrr~*")?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFAn extremely vicious",
                    "looking dog is glaring",
                    "at you. Can you really",
                    "pass by this creature",
                    "without getting hurt?^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("March forward:Run away")])? {
                    1 => {
                        if ctx.call(Function::CountItem, vec![Val::from(537)])?.number()? >= 1 {
                            ctx.lines(args![
                                "^3355FFWait! Perhaps you can",
                                "use food to soothe the",
                                "savage beast. Why don't",
                                "you feed it some of your Pet",
                                "Food and see what happens?^000000"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Give Pet Food:It won't work!")])? {
                                1 => {
                                    ctx.lines(args![
                                        "^3355FFYou gingerly throw the",
                                        "Pet Food towards the dog.",
                                        "Its tail shakes violently as",
                                        "it devours the food. You'd",
                                        "better pass this dog now",
                                        "while you have the chance!^000000"
                                    ])?;
                                    ctx.call(Function::DelItem, vec![Val::from(537), Val::from(1)])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(173), Val::from(182)])?;
                                    return Err(Stop::End);
                                }
                                2 => {}
                                _ => {}
                            }
                        }
                        ctx.lines(args![
                            "^3355FFYou slowly approach",
                            "the dog, but it won't",
                            "stop snarling at you.",
                            "You try to run past the",
                            "dog, but it blocks all of",
                            "your moves. What to do?^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as("Vicious Dog", args!["BOW WOW!", "BOW WOW!", "BOW WOW WOW!"])?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThe dog chased you",
                            "downstairs like the",
                            "suckah chump you are.^000000"
                        ])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(126), Val::from(70)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("^3355FFLet's get out of here!^000000")?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("kh_vila"), Val::from(126), Val::from(70)])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn vicious_dog_khp1(ctx: &Ctx) -> Script {
    vicious_dog_khp1_run(ctx, ViciousDogKhp1Step::Start, Vec::new()).map(|_| ())
}

pub fn vicious_dog_khp1_ontouch(ctx: &Ctx) -> Script {
    vicious_dog_khp1_run(ctx, ViciousDogKhp1Step::OnTouch, Vec::new()).map(|_| ())
}

fn drawer_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7492), Val::from(1)])? == 0 {
        ctx.lines(args![
            "^3355FFThere's something inside",
            "this drawer, but you can't",
            "take it since you're carrying",
            "to many items with you. Maybe",
            "you should pay a visit to your",
            "trusty Kafra Storage first.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(7492)])?.number()? < 1 && ctx.var("kielhyrequest").get()?.number()? <= 36) {
        ctx.lines(args![
            "^3355FFYou find an old drawer",
            "^that has been closed.",
            "^If you open it, then you",
            "^might find something",
            "^inside. Or could it be",
            "^empty. Who knows?^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Open Drawer:Ignore")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou obtain a Yellow",
                    "Keycard from inside the",
                    "drawer. Opening that drawer",
                    "turned out to be worthwhile.^000000"
                ])?;
                ctx.call(Function::GetItem, vec![Val::from(7492), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFForget opening that",
                    "drawer. I mean, come on,",
                    "what would be the point?^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is where you",
            "found the Yellow Keycard.",
            "This drawer is now empty.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn drawer_khp1(ctx: &Ctx) -> Script {
    drawer_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn box_khp2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(7329)])?.number()? < 1
        && ctx.call(Function::CountItem, vec![Val::from(7516)])?.number()? < 1)
        && ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? < 1)
        && ctx.var("kielhyrequest").get()?.number()? < 38)
    {
        ctx.lines(args![
            "^3355FFA solid box is laid",
            "on the floor in which",
            "a smaller, locked box",
            "has been placed inside.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (((ctx.call(Function::CountItem, vec![Val::from(7329)])?.number()? >= 1
        && ctx.call(Function::CountItem, vec![Val::from(7516)])?.number()? < 1)
        && ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? < 1)
        && ctx.var("kielhyrequest").get()?.number()? < 38)
    {
        ctx.lines(args![
            "^3355FFA solid box is laid",
            "on the floor in which",
            "a smaller, locked box",
            "has been placed inside.",
            "Perhaps you can use your",
            "Old Bronze Key to unlock it.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Use Old Bronze Key:No, it'll never work.")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYour hunch paid off!",
                    "The Old Bronze Key really",
                    "did unlock that box! You",
                    "open the inner box and",
                    "obtain the Green Keycard",
                    "that was locked inside."
                ])?;
                ctx.call(Function::GetItem, vec![Val::from(7516), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7329), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines(args![
                    "^3355FFWhat...?",
                    "What? Using a key",
                    "to unlock a lock?",
                    "Come on, that's",
                    "freakin' crazy talk!^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThis is the box from",
            "which you've obtained",
            "the Green Keycard.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn box_khp2(ctx: &Ctx) -> Script {
    box_khp2_body(ctx, Vec::new()).map(|_| ())
}

fn bookshelf_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khfirstkeyhole = Val::from(0);
    let mut l_khsecondkeyhole = Val::from(0);
    if ((ctx.call(Function::CountItem, vec![Val::from(7491)])?.number()? < 1
        && ctx.call(Function::CountItem, vec![Val::from(7329)])?.number()? >= 1)
        || ctx.call(Function::CountItem, vec![Val::from(7516)])?.number()? >= 1)
    {
        ctx.lines(args![
            "^3355FFOne of the books on",
            "this crammed bookshelf",
            "is labeled with the note,",
            "''To Elly.'' It must have been",
            "left behind by her grandfather.^000000"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Examine Book:Ignore")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFYou skim through the",
                    "book's pages, and don't",
                    "find anything particularly",
                    "interesting. However, you",
                    "notice a steel surface behind the",
                    "shelf as you place the book back.^000000"
                ])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Examine the steel surface:Ignore")])?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines(args![
                            "^3355FFYou remove more of the",
                            "shelf's books to reveal",
                            "that the steel surface is",
                            "part of a safe hidden behind",
                            "the bookshelf. There are two",
                            "keyholes on the steel safe.^000000"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Try all your keys:Cancel")])? {
                            1 => {
                                ctx.lines(args!["^3355FFWhich key will", "you insert into", "the first keyhole?^000000"])?;
                                ctx.next()?;
                                if ctx.call(Function::CountItem, vec![Val::from(7329)])?.number()? >= 1 {
                                    let choice = runtime::select_values(ctx, &[Val::from("Old Bronze Key:Cottage Key")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.lines(args!["^3355FFWhich key will", "you insert into", "the second keyhole?^000000"])?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("Old Bronze Key:Cottage Key")])?;
                                    ctx.var("@menu").set(choice)?;
                                } else if ctx.call(Function::CountItem, vec![Val::from(7516)])?.number()? >= 1 {
                                    match runtime::select_values(ctx, &[Val::from("Green Keycard:Cottage Key")])? {
                                        1 => {
                                            l_khfirstkeyhole = Val::from(1);
                                        }
                                        2 => {
                                            l_khfirstkeyhole = Val::from(2);
                                        }
                                        _ => {}
                                    }
                                    ctx.lines(args!["^3355FFWhich key will", "you insert into", "the second keyhole?^000000"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Green Keycard:Cottage Key")])? {
                                        1 => {
                                            l_khsecondkeyhole = Val::from(1);
                                        }
                                        2 => {
                                            l_khsecondkeyhole = Val::from(2);
                                        }
                                        _ => {}
                                    }
                                    if (l_khfirstkeyhole.clone() == 2 && l_khsecondkeyhole.clone() == 1) {
                                        ctx.lines(args![
                                            "^3355FFThe safe opens with",
                                            "a click, and you see",
                                            "a Grey Box inside.",
                                            "You take the Grey Box",
                                            "with you, hoping that it",
                                            "will come in handy later.^000000"
                                        ])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7489), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7516), Val::from(1)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(7491), Val::from(1)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                ctx.mes("^3355FFThe safe won't open.^000000")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines(args![
                                    "^3355FFIt's probably a better",
                                    "idea to investigate the",
                                    "cottage for the keys that",
                                    "will open up this safe...^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines(args!["^3355FFYou place the books", "back on the bookshelf.^000000"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFYou can probably",
                    "find better clues",
                    "somewhere else",
                    "around here.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines(args!["^3355FFYou've found a", "bookshelf that's", "crammed with books.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn bookshelf_khp1(ctx: &Ctx) -> Script {
    bookshelf_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn desk_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("khcottagepoem2").get()?.number()? < 1 {
        ctx.lines(args![
            "^3355FFThere are piles of papers",
            "and books stacked on top",
            "of the desk covering topics",
            "like artificial power, Sage",
            "Varmundt's research, factory",
            "robotization, and magic scrolls...^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFWhile rummaging through",
            "the books and papers, you",
            "find a piece of paper with",
            "the Kiel Hyre Foundation's",
            "official seal. You decide that",
            "it might come in handy someday.^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou've obtained",
            "a blank piece of",
            "paper with the Kiel",
            "Hyre Foundation seal.^000000"
        ])?;
        ctx.var("khcottagepoem2").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("khcottagepoem2").get()?.number()? >= 1 {
        ctx.lines(args![
            "^3355FFThere are piles of papers",
            "and books, covering various",
            "scientific and magic topics,",
            "stacked on top of this desk.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn desk_khp1(ctx: &Ctx) -> Script {
    desk_khp1_body(ctx, Vec::new()).map(|_| ())
}

fn medicine_chest_khp1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khpotioncolor_s = Val::from("");
    if (ctx.var("khcottagepoem2").get()?.number()? < 1 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFThis medicine cabinet",
            "is filled with bottles of",
            "various colors. What kind",
            "of medicine can be found here?^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("khcottagepoem2").get()? == 1 && ctx.var("kielhyrequest").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFThis medicine cabinet",
            "is filled with bottles of",
            "various colors. What kind",
            "of medicine can be found here?^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFWait, you have a blank piece",
            "of paper with the Kiel Hyre",
            "Foundation seal! It's strange",
            "that the seal was put on a blank",
            "piece of paper. Maybe something",
            "is written on it with special ink?^000000"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFIt's a crazy hunch, but",
            "maybe, just maybe, you can",
            "use something inside this",
            "medicine cabinet that will",
            "reveal any invisible ink",
            "written on this document!^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("What? That's crazy!:Of course! Let's try it!")])? {
            1 => {
                ctx.lines(args!["^3355FFSorry.", "I thought it", "was a good idea...^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Try Red Liquid:Try Blue Liquid:Try Yellow Liquid:Try Green Liquid:Cancel",
                    )],
                )? {
                    1 => {
                        l_khpotioncolor_s = Val::from("red");
                    }
                    2 => {
                        ctx.lines(args![
                            "^3355FFIt works!",
                            "The blue liquid is",
                            "revealing small text",
                            "written on the paper.",
                            "It looks like some",
                            "kind of long poem...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poem",
                            args![
                                "The first snake is made of",
                                "steel, but I used my rage",
                                "to destroy it. The second",
                                "snake is made of magic,",
                                "but my love pierced its heart."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poem",
                            args![
                                "The third snake is flesh",
                                "and blood, but my hope",
                                "defeated it in the end.",
                                "However, the fourth and",
                                "final snake is formless, and",
                                "no one knows its appearance."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Poem",
                            args![
                                "I cast my despair to the air,",
                                "but nobody knows if it killed",
                                "the snake. I am merely ^0000FFa little",
                                "lost devil^000000 with four swords and",
                                "four snakes, searching for that",
                                "girl in the darkness."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFThis is a very",
                            "strange poem. What",
                            "could it possibly mean?^000000"
                        ])?;
                        ctx.var("khcottagepoem2").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        l_khpotioncolor_s = Val::from("yellow");
                    }
                    4 => {
                        l_khpotioncolor_s = Val::from("green");
                    }
                    5 => {
                        ctx.lines(args![
                            "^3355FFNever mind.",
                            "This idea sounds",
                            "too crazy to work...",
                            "like puttting a man",
                            "on the moon. Can you",
                            "believe that hogwash?^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.lines(args![
                    "^3355FFYou tried sprinkling",
                    ((Val::from("the ") + l_khpotioncolor_s.clone()) + Val::from(" liquid from the")),
                    "medicine cabinet onto",
                    "the blank paper with the",
                    "Kiel Hyre Foundation seal.",
                    "However, nothing happened...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.var("khcottagepoem2").get()?.number()? >= 2 || ctx.var("kielhyrequest").get()?.number()? >= 30) {
        ctx.lines(args![
            "^3355FFThis is where you poured",
            "some blue liquid to read",
            "a poem written in invisible",
            "ink on the blank piece of paper",
            "with the Kiel Hyre Foundation",
            "seal. Would you like read it?^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("No time!:Read")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFYou're running out of",
                    "time! For now, it would",
                    "be best for you to search",
                    "every inch of this cottage.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Poem",
                    args![
                        "The first snake is made of",
                        "steel, but I used my rage",
                        "to destroy it. The second",
                        "snake is made of magic,",
                        "but my love pierced its heart."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Poem",
                    args![
                        "The third snake is flesh",
                        "and blood, but my hope",
                        "defeated it in the end.",
                        "However, the fourth and",
                        "final snake is formless, and",
                        "no one knows it's appearance."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Poem",
                    args![
                        "I cast my despair to the air,",
                        "but nobody knows if it killed",
                        "the snake. I am merely a ^3355FFlittle^000000",
                        "^3355FFlost devil^000000 with four swords and",
                        "four snakes, searching for that",
                        "girl in the darkness."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn medicine_chest_khp1(ctx: &Ctx) -> Script {
    medicine_chest_khp1_body(ctx, Vec::new()).map(|_| ())
}
