use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn employee_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        ctx.lines_as(
            "Chang Pai",
            args![
                "Welcome, welcome!",
                "We are ready to serve you~!",
                "Now, go ahead and go upstairs~!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Chang Pai", args!["^666666*Yawn...*^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn employee_1(ctx: &Ctx) -> Script {
    employee_1_body(ctx, Vec::new()).map(|_| ())
}

fn employee_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()? == 2 || ctx.var("ch_tre").get()? == 3) {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                ctx.lines_as(
                    "Chang Pai",
                    args!["Wait, who are you?!", "Put that down and get", "of out here right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 10 {
                ctx.lines_as(
                    "Chang Pai",
                    args!["Wait, who are you?!", "Put that down and get", "of out here right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 21)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
                ctx.lines_as(
                    "Chang Pai",
                    args!["Wait, who are you?!", "Put that down and get", "of out here right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 5 {
            ctx.lines_as("Chang Pai", args!["^666666*Yawn...*^000000", "So...", "Sleepy..."])?;
            ctx.next()?;
            ctx.lines_as("Chang Pai", args!["Huh...?", "Who are you!?", "Hey, I got a thief here!"])?;
            ctx.next()?;
            ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
            ctx.var("ch_tre").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn employee_1_ontouch(ctx: &Ctx) -> Script {
    employee_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn employee_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        ctx.lines_as(
            "Huang Jia Xian",
            args!["Welcome~", "Sorry for making you wait. If you wish to rest, please go upstairs."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Jia Xian",
            args![
                "Recently, many tourists are visiting Luoyang and although business is great, we're now",
                "busier than ever."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Huang Jia Xian",
            args!["^666666*Sigh...*^000000", "I don't even", "have time to eat.", "I'm starving...!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Huang Jia Xian",
        args!["Ehhhh...", "Forgive me...", "....................", "...Zzzzz...Zzzz..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn employee_2(ctx: &Ctx) -> Script {
    employee_2_body(ctx, Vec::new()).map(|_| ())
}

fn employee_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()? == 2 || ctx.var("ch_tre").get()? == 3) {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                ctx.lines_as(
                    "Huang Jia Xian",
                    args!["What the...?", "Hey, what", "are you doing?", "Get out of here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 10 {
                ctx.lines_as(
                    "Huang Jia Xian",
                    args!["What the...?", "Hey, what", "are you doing?", "Get out of here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
                ctx.lines_as(
                    "Huang Jia Xian",
                    args!["What the...?", "Hey, what", "are you doing?", "Get out of here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 5 {
            ctx.lines_as("Huang Jia Xian", args!["*Yawn...*", "So very tired..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Huang Jia Xian",
                args![
                    "Hey, what are you",
                    "doing here? Are you",
                    "a Thief?! Somebody help!",
                    "There's a Thief!"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
            ctx.var("ch_tre").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn employee_2_ontouch(ctx: &Ctx) -> Script {
    employee_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn employee_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        if ctx.var("ch_tre").get()? == 5 {
            ctx.lines_as(
                "Ya Hua",
                args![
                    "Welcome, welcome!",
                    "We have many rooms",
                    "available! Why don't",
                    "you go upstairs?",
                    "Ha ha ha!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
            ctx.lines_as(
                "Ya Hua",
                args!["Oh, if by any chance you came to try the Dragon Soup, I'm sorry, but it's no longer availalbe."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ya Hua",
                args!["But don't be too disappointed, we serve many other delicious foods that you can choose from!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Ya Hua",
            args![
                "Welcome, welcome!",
                "We have many rooms",
                "available! Why don't",
                "you go upstairs?",
                "Ha ha ha!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Ya Hua", args!["^666666*Yawn...*^000000"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn employee_3(ctx: &Ctx) -> Script {
    employee_3_body(ctx, Vec::new()).map(|_| ())
}

fn employee_3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()? == 2 || ctx.var("ch_tre").get()? == 3) {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                ctx.lines_as(
                    "Ya Hua",
                    args!["What do you think", "you're doing here?!", "Put that down and", "leave right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 10 {
                ctx.lines_as(
                    "Ya Hua",
                    args!["What do you think", "you're doing here?!", "Put that down and", "leave right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
                ctx.lines_as(
                    "Ya Hua",
                    args!["What do you think", "you're doing here?!", "Put that down and", "leave right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 5 {
            ctx.lines_as(
                "Ya Hua",
                args!["^666666*Yawn...*^000000", "Eyelids...", "Getting...", "Heavier..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Ya Hua", args!["Wait a sec...", "Are you a Thief?!", "Get out of here!!"])?;
            ctx.next()?;
            ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
            ctx.var("ch_tre").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn employee_3_ontouch(ctx: &Ctx) -> Script {
    employee_3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn chef_1_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()?.number()? > 0 && ctx.var("ch_tre").get()?.number()? < 4) {
        ctx.lines_as(
            "Wang Shi Long",
            args!["Hm? Aren't you a customer? I am Wang Shi Long, the chef of this restaurant."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args![
                "My family has served food to Lord Bai Long for a long time. This restaurant is own by my family and I am the successor."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["Every dish my family cooks is counted among the most delicious food in Luoyang."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["Our Dragon Soup won especially high praise from our Lord Bai Long."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["I'm also proud to tell you that my family only uses the freshest, highest quality ingredients for our dishes."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["We have been popular in Luoyang for hundreds and hundreds of years because of the quality of our gourmet food."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["Recently, though, I've had a bad feeling that someone is trying to take over our restaurant..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args!["Oh, well. Maybe I'm in a different mood because of some other reason. It's probably nothing."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ch_tre").get()?.number()? > 3 && ctx.var("ch_tre").get()?.number()? < 6) {
        ctx.lines_as(
            "Wang Shi Long",
            args![
                "^666666*Moans and Cries*^000000",
                "I guess this is it...!",
                "The end of my family's glory.",
                "Someone stole the base broth",
                "of my Dragon Soup!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wang Shi Long",
            args![
                "What should I do, now?",
                "Without Dragon Soup, my family's restaurant will now just be like all the others..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Wang Shi Long",
        args![
            "Hello, are you",
            "one of our customers?",
            "I am Wang Shi Long,",
            "the chef of this",
            "restaurant."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wang Shi Long",
        args![
            "My family has served food",
            "to Lord Bai Long for a long time. This restaurant has been handed down the family line, and I am the successor~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wang Shi Long",
        args![
            "Our specialty dish, Dragon Soup, won especially high praise from Lord Bai Long, who is known for",
            "his extremely discerning sense of taste."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wang Shi Long",
        args!["I'm also proud to say that we cook with only the freshest and highest quality ingredients."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wang Shi Long",
        args!["We've always been popular in Luoyang for hundreds and hundreds of years because of our high quality gourmet cuisine."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wang Shi Long",
        args![
            "In this dry and hot weather,",
            "Dragon Soup is the best food for any appetite. I suggest that you try a bowl. You'll be quite pleased!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chef_1_2(ctx: &Ctx) -> Script {
    chef_1_2_body(ctx, Vec::new()).map(|_| ())
}

fn chef_1_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()? == 2 || ctx.var("ch_tre").get()? == 3) {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                ctx.lines_as(
                    "Wang Shi Long",
                    args!["Hey, what do you", "think you're doing?!", "Let go of that, and", "get outta here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(280), Val::from(161)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 10 {
                ctx.lines_as(
                    "Wang Shi Long",
                    args!["Hey, what do you", "think you're doing?!", "Let go of that, and", "get outta here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(280), Val::from(161)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6 {
                ctx.lines_as(
                    "Wang Shi Long",
                    args!["Hey, what do you", "think you're doing?!", "Let go of that, and", "get outta here!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(280), Val::from(161)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 5 {
            ctx.lines_as("Wang Shi Long", args!["^666666*Yawn...*^000000", "Hm...?", "Who's that?"])?;
            ctx.next()?;
            ctx.lines_as("Wang Shi Long", args!["Wait...!", "What are you doing?!", "G-Get out of here!"])?;
            ctx.next()?;
            ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
            ctx.var("ch_tre").set(Val::from(1))?;
            ctx.call(Function::Warp, vec![Val::from("louyang"), Val::from(280), Val::from(161)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn chef_1_2_ontouch(ctx: &Ctx) -> Script {
    chef_1_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn jiu_lian_bu_1_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ql_acceptsoup").get()?.is_true() {
        ctx.var("ch_tre").set(Val::from(1))?;
    }
    if ctx.var("ql_gotdragonsoup").get()?.is_true() {
        ctx.var("ch_tre").set(Val::from(2))?;
    }
    if ctx.var("ql_gotfakesoup").get()?.is_true() {
        ctx.var("ch_tre").set(Val::from(3))?;
    }
    if ctx.var("ql_soupquest").get()?.is_true() {
        ctx.var("ch_tre").set(Val::from(4))?;
    }
    if ctx.var("ql_soup2").get()?.is_true() {
        ctx.var("ch_tre").set(Val::from(5))?;
    }
    ctx.var("ql_acceptsoup").set(Val::from(0))?;
    ctx.var("ql_gotdragonsoup").set(Val::from(0))?;
    ctx.var("ql_gotfakesoup").set(Val::from(0))?;
    ctx.var("ql_soupquest").set(Val::from(0))?;
    ctx.var("ql_soup2").set(Val::from(0))?;
    if ctx.var("ch_tre").get()? == 0 {
        ctx.lines_as("Jiu Lian Bu", args!["Hey~", "What's up?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["I don't like hanging around too many people, so I came here. Listening to this stream really puts my mind at ease."],
        )?;
        ctx.next()?;
        ctx.lines_as("Jiu Lian Bu", args!["No offense, but I'm upset", "at the sheer number of tourists coming over to Luoyang. Sure, I can understand that our town is attractive and has beautiful sights."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "But I just can't stand crowds of people. People gather like sheep",
                "to any place that they hear is popular, and that really bugs me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Speaking of which, there's even",
                "a place in Luoyang that's just like that. Man, I hate that restaurant!"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("What restaurant?:Ignore him.")])?) == 1 {
            ctx.lines_as(
                "Jiu Lian Bu",
                args![
                    "West of Luoyang, there's",
                    "a restaurant built on a pond. It's been around for a long time, selling food for ridiculous prices!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args![
                    "Oh sure, the food and flavors",
                    "there have a long history, but I don't think that justifies how they charge their patrons!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args![
                    "As a young man",
                    "who loves his town,",
                    "I can't let them manipulate my people like that!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jiu Lian Bu", args!["But everyone here already knows who I am, so I can't do anything! I've already had one too many, shall we say, 'incidents' with the people living here already."])?;
            ctx.next()?;
            ctx.lines_as("Jiu Lian Bu", args!["I've been caught for tagging walls, shoplifting, scamming, stealing a few girlfriends... So yeah, I'm not exactly known as a sterling citizen."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args!["Say, wait a minute. The local people around here aren't too familiar with you. Hmmm..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args![
                    "Would you sneak into that restaurant and steal the",
                    "^3131FFDragon Soup Broth^000000 for me?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args!["The Dragon Soup Broth is the backbone for the restaurant", "owner's secret recipe."],
            )?;
            ctx.next()?;
            ctx.lines_as("Jiu Lian Bu", args!["But they've been cheating their customers by watering that broth down and selling it for a ridiculous price! Serves them right if their secrets were stolen!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jiu Lian Bu",
                args!["If you can steal some of the broth, I'll pay you back. So whaddya say?"],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("I'll do it!:No, stealing is wrong.")])?) == 1 {
                ctx.lines_as("Jiu Lian Bu", args!["Grrrrreat!", "I knew you'd", "see things my way!"])?;
                ctx.next()?;
                ctx.lines_as("Jiu Lian Bu", args!["Okay, the restaurant is at the West side of Luoyang. But you gotta be careful. The workers there watch over that broth like freakin' hawks!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jiu Lian Bu",
                    args!["When you finally steal the broth, make sure you bring it without spilling any. Okay? Good luck~"],
                )?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Jiu Lian Bu", args!["Stealing?", "You may have", "a point there."])?;
            ctx.next()?;
            ctx.lines_as("Jiu Lian Bu", args!["But then, that restaurant has been doing that to their customers for years! So, technically, we'd just be stealing back."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Jiu Lian Bu", args!["That joint isn't even that great. I mean, it's so obvious that they rip off their customers! Dragon Soup?! More like... Dragon Crap Soup!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_tre").get()? == 1 {
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Huh...?",
                "Whoa, I thought you were on your way to the restaurant. You better get a move on."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["Alright then, pal. Make sure the guys who work there don't catch you. Good luck~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_tre").get()? == 2 {
        ctx.lines_as("Jiu Lian Bu", args!["Wow! You made it!", "Let me see..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFJiu Lian Bu takes",
            "a hearty sip of the",
            "broth you've managed",
            "to steal for him.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Ohhhh man....",
                "This is soooo not Dragon Soup Broth. Sorry, but would you go",
                "and try to get it again?"
            ],
        )?;
        ctx.var("ch_tre").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_tre").get()? == 3 {
        ctx.lines_as("Jiu Lian Bu", args!["Wow!", "You made it!", "Let me see..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFJiu Lian Bu takes",
            "a hearty sip of the",
            "broth you've managed",
            "to steal for him.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Ooooh. Ooh yeah.",
                "This is the stuff.",
                "Muhahahahahaha~!",
                "This'll put the chef",
                "in agony for a while!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["Good job, chum! Heh heh heh, because you risked your neck for me, I'm gonna show you an awesome place! Just follow me~"],
        )?;
        ctx.var("ch_tre").set(Val::from(4))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lou_fild01"), Val::from(180), Val::from(170)])?;
        return Err(Stop::End);
    } else if (ctx.var("ch_tre").get()?.number()? > 3 && ctx.var("ch_tre").get()?.number()? < 6) {
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["Hey~", "So how ya been, ya smooth criminal? You wanna visit that place again?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sure, let's go~:Nah, maybe next time.")],
        )?) == 1
        {
            ctx.lines_as("Jiu Lian Bu", args!["Alright~", "Let's get", "a groove on."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("lou_fild01"), Val::from(180), Val::from(170)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Not in the mood, eh?",
                "No prob. But feel free",
                "to come see me whenever",
                "you want."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Jiu Lian Bu",
        args![
            "Wha...?",
            "Hey, who are you? If you don't got anything to say to me, then get lost!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jiu_lian_bu_1_1(ctx: &Ctx) -> Script {
    jiu_lian_bu_1_1_body(ctx, Vec::new()).map(|_| ())
}

fn jiu_lian_bu_1_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    if ctx.var("ch_tre").get()? == 4 {
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["So...", "Whaddya think?", "Prettiest place", "in Luoyang, isn't it?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Whenever I'm depressed",
                "or need to relax, I just sit here in enjoy the breeze. It helps me forget all my worries."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "Heh heh, the best part is, this place is far away from my older sister. Man, that woman can nag, nag, nag, all day long."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["Why don't you sit down and close your eyes, and feel that soothing wind. It's pretty refreshing..."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SCRATCH")?])?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["Also...", "I'm a little", "embarrassed to", "say this but..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Jiu Lian Bu", args!["I'm the kind of guy who speaks his mind. If there's something you just wanna say, but can't, it's kind of like poison in your mind."])?;
        ctx.next()?;
        ctx.lines_as("Jiu Lian Bu", args!["You know what I'm talking about, right? If you bottle something up inside of you, it just causes you anxiety you don't need."])?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["So...", "This is what you do.", "Clench your fist, take", "a deep breath."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "And just yell",
                "whatever you want!",
                "If you don't know",
                "the words, just",
                "screaming will do.",
                "Let everything out!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jiu Lian Bu", args!["Ready~!", "Say it out loud!"])?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("lou_fild01"),
                (((Val::from("'") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("' shouts : ")) + l_input_s.clone()),
                ctx.constant("BC_MAP")?,
                Val::from("0x9CFF00"),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args!["So, how do you feel?", "Don't you feel better now? Hahaha~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Lian Bu",
            args![
                "So from now on, whenever you",
                "wanna relieve yourself of stress, come see me and we'll come back to this place. Call it my way of saying thanks."
            ],
        )?;
        ctx.var("ch_tre").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_tre").get()? == 5 {
        ctx.lines_as("Jiu Lian Bu", args!["So...", "Whaddya wanna do?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Shout.:Leave.")])?) == 1 {
            ctx.lines_as("Jiu Lian Bu", args!["Alright~!", "Say it out loud!"])?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args![l_input_s.clone()])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    Val::from("lou_fild01"),
                    (((Val::from("'") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("' shouts : "))
                        + l_input_s.clone()),
                    ctx.constant("BC_MAP")?,
                    Val::from("0x9CFF00"),
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jiu Lian Bu", args!["So, how do you feel? Don't you feel better now?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Jiu Lian Bu", args!["Ahhh...", "Alright, let's", "get a groove on."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("lou_fild01"), Val::from(200), Val::from(174)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Jiu Lian Bu",
        args!["Hey, pal~! This is my turf! Whaddya think you're doin' here?!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Jiu Lian Bu", args!["Now...", "Get the hell", "outta here."])?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("lou_fild01"), Val::from(200), Val::from(174)])?;
    return Err(Stop::End);
}

pub fn jiu_lian_bu_1_2(ctx: &Ctx) -> Script {
    jiu_lian_bu_1_2_body(ctx, Vec::new()).map(|_| ())
}

fn pot_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_tre").get()? == 1 {
        ctx.lines(args![
            "^3131FFBeneath the shadows,",
            "you find a large pot filled with dark, red liquid. What do you",
            "want to do?^000000 "
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Take the pot.:Look for another pot.")],
        )?) == 1
        {
            ctx.lines(args![
                "^3131FFYou take a careful look around.",
                "It wouldn't be wise to steal this now if anyone is watching.^000000"
            ])?;
            ctx.next()?;
            if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
            {
                ctx.mes("^3131FFThe restaurant doesn't seem busy right now, so there's only a few employees and customers.^000000")?;
            } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
            {
                ctx.lines(args![
                    "^3131FFOnly the restaurant",
                    "employees are around,",
                    "and they busy chatting",
                    "amongst each other.^000000"
                ])?;
            } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
            {
                ctx.lines(args![
                    "^3131FFThe restaurant is filled",
                    "with customers, and the",
                    "hustle and bustle of the",
                    "restaurant employees.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3131FFEveryone seems so busy",
                    "and preoccupied that they don't",
                    "notice you approach the pot.^000000"
                ])?;
            } else {
                ctx.lines(args![
                    "^3131FFSince the restaurant",
                    "is closed, the whole place",
                    "is completely quiet. The",
                    "employees are all asleep.^000000"
                ])?;
            }
            ctx.next()?;
            ctx.lines(args![
                "^3131FFYou carefully lift the pot, and although it's heavy, you think",
                "you can carry it.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3131FFAll you have",
                "to do now is get",
                "away from this restaurant",
                "without getting caught...^000000"
            ])?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?.number()? < 4 {
                ctx.var("ch_tre").set(Val::from(2))?;
            } else {
                ctx.var("ch_tre").set(Val::from(3))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["^3131FFYou decide to^000000", "^3131FFlook for another pot.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ch_tre").get()?.number()? > 3 && ctx.var("ch_tre").get()?.number()? < 6) {
        ctx.lines(args!["^3131FFYou found a pot.^000000", "^3131FFHowever, it's empty.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        ctx.lines_as("Chef", args!["Ah...!", "Please, do not", "touch the pots!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3131FFBeneath the shadows,",
        "you find a large pot filled with dark, red liquid.^000000 "
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pot_1(ctx: &Ctx) -> Script {
    pot_1_body(ctx, Vec::new()).map(|_| ())
}

fn pot_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_tre").get()? == 1 {
        ctx.lines(args![
            "^3131FFBeneath the shadows,",
            "you find a large pot filled with dark, red liquid. What do you",
            "want to do?^000000"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Take the pot.:Look for another pot.")],
        )?) == 1
        {
            ctx.lines(args![
                "^3131FFYou take a careful look around.",
                "It wouldn't be wise to steal this now if anyone is watching.^000000"
            ])?;
            ctx.next()?;
            if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
            {
                ctx.mes("^3131FFThe restaurant doesn't seem busy right now, so there's only a few employees and customers.^000000")?;
            } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
            {
                ctx.lines(args![
                    "^3131FFOnly the restaurant",
                    "employees are around,",
                    "and they busy chatting",
                    "amongst each other.^000000"
                ])?;
            } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
                && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
            {
                ctx.lines(args![
                    "^3131FFThe restaurant is filled",
                    "with customers, and the",
                    "hustle and bustle of the",
                    "restaurant employees.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3131FFEveryone seems so busy",
                    "and preoccupied that they don't",
                    "notice you approach the pot.^000000"
                ])?;
            } else {
                ctx.lines(args![
                    "^3131FFSince the restaurant",
                    "is closed, the whole place",
                    "is completely quiet. The",
                    "employees are all asleep.^000000"
                ])?;
            }
            ctx.next()?;
            ctx.lines(args![
                "^3131FFYou carefully lift the pot, and although it's heavy, you think",
                "you can carry it.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3131FFAll you have",
                "to do now is get",
                "away from this restaurant",
                "without getting caught...^000000"
            ])?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?.number()? < 4 {
                ctx.var("ch_tre").set(Val::from(2))?;
            } else {
                ctx.var("ch_tre").set(Val::from(3))?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args!["^3131FFYou decided to^000000", "^3131FFlook for another pot.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("ch_tre").get()?.number()? > 3 && ctx.var("ch_tre").get()?.number()? < 6) {
        ctx.lines(args!["^3131FFYou found", "an empty pot.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        ctx.lines_as("Chef", args!["Ah...!", "Please, do not", "touch the pots!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3131FFBeneath the shadows,",
        "you find a large pot filled with dark, red liquid.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pot_2(ctx: &Ctx) -> Script {
    pot_2_body(ctx, Vec::new()).map(|_| ())
}

fn chef_assistant_lou1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Jin Wei Ling", args!["I used to be", "an enthusiastic", "martial artist."])?;
    ctx.next()?;
    ctx.lines_as(
        "Jin Wei Ling",
        args![
            "Although I became an",
            "assistant chef for a living, I always think of myself as a martial artist first."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jin Wei Ling",
        args![
            "So, I decided to reflect the spirit of the martial arts into my cooking. We are often very busy when there are many customers."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jin Wei Ling",
        args!["When we're busy, I can use my martial arts to cook cuisine much more quickly! Hahaha~ Martial arts can be very practical!"],
    )?;
    ctx.next()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
    ctx.lines_as("Jin Wei Ling", args!["Waaa-!!!!"])?;
    ctx.next()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ENDURE")?])?;
    ctx.lines_as("Jin Wei Ling", args!["Waaa Taaah-!!!!!"])?;
    ctx.next()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SONICBLOW")?])?;
    ctx.lines_as("Jin Wei Ling", args!["Waaa...", "Waaa Taaah-!!!!!"])?;
    ctx.next()?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SONICBLOWHIT")?])?;
    ctx.mes("^3355FF* Chop chop chop chop chop *^000000")?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
    ctx.lines_as(
        "Jin Wei Ling",
        args![
            "Hahahaha! Look these perfect vergetable slices! Muhahahaha!!",
            "I will continue to hone my martial arts through cooking!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chef_assistant_lou1(ctx: &Ctx) -> Script {
    chef_assistant_lou1_body(ctx, Vec::new()).map(|_| ())
}

fn chef_assistant_lou1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("ch_tre").get()? == 2 || ctx.var("ch_tre").get()? == 3) {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 10
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 14)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                ctx.lines_as(
                    "Jin Wei Ling",
                    args!["Wait! Who are you!", "Put that pot down", "and get out of", "here right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 14
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 10 {
                ctx.lines_as(
                    "Jin Wei Ling",
                    args!["Wait! Who are you!", "Put that pot down", "and get out of", "here right now!"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
                ctx.var("ch_tre").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 17
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
            && ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 6
        {
            ctx.lines_as(
                "Jin Wei Ling",
                args!["Wait! Who are you!", "Put that pot down", "and get out of", "here right now!"],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3131FFYou have failed^000000", "^3131FFto steal the pot.^000000"])?;
            ctx.var("ch_tre").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn chef_assistant_lou1_ontouch(ctx: &Ctx) -> Script {
    chef_assistant_lou1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn li_min_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_tre").get()? == 5 {
        ctx.lines_as("Li Min", args!["^666666*Sigh*^000000 I am so disappointed. I came all the way down here to taste the food! I can't believe they don't sell it anymore!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Li Min",
            args!["The worst part is that I'm already addicted to the taste! ^666666*Sob...*^000000"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Li Min",
        args![
            "Well, I don't really",
            "live here in Luoyang.",
            "Still, I come here often",
            "enough to visit."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Li Min",
        args![
            "I just returned",
            "because I've got",
            "a huge craving for the",
            "food I tasted here",
            "a while ago."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Li Min",
        args!["For some reason, I can't forget it. I can't get it out of my mind!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Li Min",
        args![
            "The taste, the texture.",
            "The sweetness, melting down into my mouth, and its tempting scent lingering on my lips..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Li Min", args!["Ummmmm...", "Royal Jelly!", "It makes my", "mouth water~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn li_min_lou(ctx: &Ctx) -> Script {
    li_min_lou_body(ctx, Vec::new()).map(|_| ())
}

fn liu_jia_lim_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_tre").get()? == 5 {
        ctx.lines_as(
            "Liu Jia Lim",
            args!["Do you know what was this restaurant's best dish throughout all of its history? Dragon Soup!"],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
        ctx.lines_as(
            "Liu Jia Lim",
            args![
                "Its delicate taste comes from",
                "a broth extracted from pure meat that does not contain any fat. So it's also a very popular diet food for the ladies."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Liu Jia Lim",
            args![
                "I'm not sure what happened,",
                "but people say this restaurant no longer sells Dragon Soup.",
                "Was it because of the price...?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Liu Jia Lim", args!["Do you know what's the best dish at this restaurant? It's Dragon Soup! They've been selling it here for as long as this restaurant has been around."])?;
    ctx.next()?;
    ctx.lines_as(
        "Liu Jia Lim",
        args![
            "Its delicate taste comes from",
            "a broth extracted from pure meat that does not contain any fat. So it's also a very popular diet food for the ladies."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Liu Jia Lim", args!["It's tasty and really good for your health. Why don't you order a bowl? I've never known anyone to taste Dragon Soup and not love it!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn liu_jia_lim_lou(ctx: &Ctx) -> Script {
    liu_jia_lim_lou_body(ctx, Vec::new()).map(|_| ())
}

fn jiang_rong_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jiang Rong",
        args!["Dragon Soup is known for its spicy, yet sweet and refreshing taste."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiang Rong",
        args![
            "It's made with all sorts of medicinal herbs, so it's good",
            "for your health as well."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiang Rong",
        args![
            "Dragon Soup draws out the unnecessary heat created inside",
            "the body and circulates the blood. So it helps optimize the body's functions and promotes longevity."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiang Rong",
        args!["I've eaten Dragon Soup regularly ever since I was young. Look at me, don't you think I look so healthy considering my age?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jiang_rong_lou(ctx: &Ctx) -> Script {
    jiang_rong_lou_body(ctx, Vec::new()).map(|_| ())
}

fn chi_wu_ping_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Chi Wu Ping", args!["I don't feel good...", "So... Totally out of it..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Chi Wu Ping",
        args![
            "Oh, my aching body!",
            "All my muscles are sore...",
            "There's only one thing that could cure all of this agonizing... pain..."
        ],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.lines_as(
        "Chi Wu Ping",
        args![
            "Hey kid~!",
            "You don't look like a local!",
            "Why don't you follow the road ahead and check out the big restaurant?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chi Wu Ping",
        args!["The soup that they sell there is probably the healthiest food you can ever find."],
    )?;
    ctx.next()?;
    ctx.mes("[Chi Wu Ping]")?;
    if ctx.var("BaseLevel").get()?.number()? < 80 {
        ctx.lines(args!["I guess you could eat", "some of that soup", "for your health."])?;
    } else {
        ctx.lines(args![
            "You seem to need",
            "that soup to ease",
            "the fatigue of your",
            "body and mind."
        ])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chi_wu_ping_lou(ctx: &Ctx) -> Script {
    chi_wu_ping_lou_body(ctx, Vec::new()).map(|_| ())
}

fn jiu_chi_ling_lou_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_tre").get()? == 5 {
        ctx.lines_as(
            "Jiu Chi Ling",
            args!["There's a strange rumor going around that the restaurant is no longer selling Dragon Soup..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jiu Chi Ling",
            args![
                "Do you think",
                "my brother did",
                "something bad again!?",
                "I hope not! If he did...",
                "What am I supposed to do?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Jiu Chi Ling",
        args![
            "I'm worried about my brother.",
            "He's young, rebellious and doesn't listen to anybody..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiu Chi Ling",
        args!["He just left the", "house while he was", "complaining about", "that restaurant..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jiu Chi Ling",
        args!["^666666*Sigh~~*^000000", "I'm not gonna let him", "get away this time!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn jiu_chi_ling_lou(ctx: &Ctx) -> Script {
    jiu_chi_ling_lou_body(ctx, Vec::new()).map(|_| ())
}

fn doctor_lyang_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.mes("^3355FFWait a minute! Right now, you're over weight, so you cannot receive more items. Please store some of your things in Kafra Storage and try again.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ch_par").get()? == 0 {
        ctx.lines_as("Hua Tuo", args!["There are many pressure points on the human body. Ever since ancient times, it has been believed that each pressure point was limited", "to its role and functions."])?;
        ctx.next()?;
        ctx.lines_as("Hua Tuo", args!["However, as I studied and experimented with every pressure point, I came to the conclusion that the use of pressure points, depending on the circumstances,", "can produce different results."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.next()?;
        ctx.lines_as("Hua Tuo", args!["Few pressure points tend to show the same symptoms, regardless of the problem. Most of the time, the effects of pressure points will differ depending on the body's health or the time of day."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hua Tuo",
            args!["For instance, the pressure point located on the upper side of navel is the most vulnerable point."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hua Tuo",
            args!["If pressed the wrong way, it can cause death. But between 5:15 am and 7:15 am, it's just a weak point."],
        )?;
        ctx.var("ch_par").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("ch_par").get()? == 1 {
            if ctx.var("BaseLevel").get()?.number()? < 40 {
                ctx.lines_as(
                    "Hua Tuo",
                    args!["Being strong as a person is not defined as mere physical strength."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "Factors such as intelligence, experience and knowledge are",
                        "also considered when judging one's strength."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args!["Let's say you're very strong and given the most powerful weapon."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args!["If you don't know how to use the weapon's power, you will not be strong... You will be weak."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "When the tools or weapons",
                        "overwhelm your capabilities,",
                        "the worst situations result."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Hua Tuo", args!["Hmmm...", "I'm in trouble..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hua Tuo",
                args![
                    "I cannot do anything without my medicine. But one my patients",
                    "needs immediate treatment and",
                    "I can't leave the office..."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("I can help you.:What a shame!")])?) == 1 {
                ctx.lines_as("Hua Tuo", args!["Huh...?", "Are...", "Are you serious?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args!["This won't take much effort, but it may be too much to ask this of you, especially since we have just met."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("If you feel burdened...:I don't mind, I'd be glad to help.")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Hua Tuo",
                        args![
                            "Thank you so",
                            "much for saying that.",
                            "I feel very uncomfortable asking a favor of someone I have only just met."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hua Tuo",
                        args!["However, I will ask you", "if we meet another time.", "Now, if you'll excuse me..."],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Hua Tuo", args!["Hmm, I see.", "Well then, let me", "ask a favor of you."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "As you heard earlier, I need",
                        "a special medicine to treat this patient. However, I'm running out of the medicine I need."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args!["I will need you to get it for me since I cannot leave the patients that are waiting for me right now."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "^666666*Sigh*^000000",
                        "Misfortunes always seem",
                        "to occur one after another, don't they? My staff is currently too busy doing other errands."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "Please visit the",
                        "Tool Shop in town and bring me",
                        "the medicine that I need. Master will understand if you tell him you've been sent by me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "I am sorry for causing you",
                        "so much trouble, but if you'll excuse me, I have other patients waiting. Please hurry back with",
                        "the medicine!"
                    ],
                )?;
                ctx.var("ch_par").set(Val::from(2))?;
                ctx.call(Function::SetQuest, vec![Val::from(11044)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Hua Tuo", args!["^666666*Sigh*^000000 For some reason, I never seem to have enough medicine in stock. Is there no one I can ask to help me?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("ch_par").get()?.number()? < 4 {
                ctx.lines_as(
                    "Hua Tuo",
                    args![
                        "You haven't gotten",
                        "the medicine yet...?",
                        "I hope you can get it",
                        "as soon as you can..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("ch_par").get()? == 4 {
                    ctx.lines_as(
                        "Hua Tuo",
                        args![
                            "In order to prescribe medicine",
                            "and apply acupuncture suited to",
                            "a patient, I must first consider many different factors related to health."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hua Tuo",
                        args!["I must be especially careful if the patient is in critical condition or is exhibiting unusual symptoms."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("ch_par").get()?.number()? < 9 {
                        ctx.lines_as(
                            "Hua Tuo",
                            args![
                                "Umm.....",
                                "Is that so...?",
                                "I can understand if you",
                                "take your time to bring the item. There's no need to rush."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hua Tuo",
                            args![
                                "Everybody has their own",
                                "worries, and I understand if your own problems must take priority."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hua Tuo",
                            args!["Still, it's good", "to put yourself in someone", "else's shoes sometimes."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ch_par").get()? == 9 {
                            if ctx.call(Function::CountItem, vec![Val::from(7252)])?.number()? < 1 {
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Umm.....",
                                        "Is that so...?",
                                        "I can understand if you",
                                        "take your time in bringing what I need. There's no rush."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Everybody has their own",
                                        "worries, and I understand if your own problems must take priority."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args!["Still, it's good", "to put yourself in someone", "else's shoes sometimes."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Hua Tuo",
                                args![
                                    "Ah...",
                                    "You've finally",
                                    "brought it to me!",
                                    "Thank you so much,",
                                    "I feel much more relieved..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hua Tuo",
                                args!["I apologize", "in advance,", "but may I ask", "another favor of you?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hua Tuo",
                                args![
                                    "I am asking you once more",
                                    "as now I see that you are trustworthy. Of course, I will compensate you for your trouble."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("I'm sorry, I can't help you.:No problem.")],
                            )?) == 1
                            {
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Alright....",
                                        "I understand.",
                                        "But let me thank you",
                                        "for helping me out.",
                                        "Please take this..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Hua Tuo", args!["This medicine is not that", "great, but it's an old family secret. I hope it will be useful to you if you find yourself in great danger."])?;
                                ctx.call(Function::DelItem, vec![Val::from(7252), Val::from(1)])?;
                                ctx.var("ch_par").set(Val::from(10))?;
                                ctx.call(Function::CompleteQuest, vec![Val::from(11056)])?;
                                ctx.call(Function::GetItem, vec![Val::from(679), Val::from(2)])?;
                                ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args!["Well then,", "I will see", "you around.", "Once again, thank", "you for your help."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Hua Tuo",
                                args![
                                    "Thank you,",
                                    "thank you so much!",
                                    "I just ran out of some other medicines again, and I hope that you can assist me once more."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hua Tuo",
                                args![
                                    "I hope that I am not causing",
                                    "you too much trouble. Um, so the medicines I'll need are..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Hua Tuo", args!["^0000ff2 Leopard Claw^000000 which strengthen bones, ^0000ff10 Solid Peach^000000 which strengthens muscle, ^0000ff5 Poisonous Toad Skin^000000 which replenishes the skin..."])?;
                            ctx.next()?;
                            ctx.lines_as("Hua Tuo", args!["^0000ff20 Brown Root^000000 which regulates the heart, ^0000ff10 Sprout^000000 which eases the abdomen and ^0000ff5 Honey Pot^000000 which provides nutrition."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hua Tuo",
                                args!["I hope you were", "able to memorize all", "of that. Once again, that's..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hua Tuo",
                                args![
                                    "^3355FF2 Leopard Claw^000000,",
                                    "^3355FF10 Solid Peach^000000,",
                                    "^3355FF5 Poisonous Toad Skin^000000,",
                                    "^3355FF20 Brown Root^000000,",
                                    "^3355FF10 Sprout^000000 and",
                                    "^3355FF5 Honey Pot^000000."
                                ],
                            )?;
                            ctx.var("ch_par").set(Val::from(17))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(11056), Val::from(11057)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("ch_par").get()? == 10 {
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args!["Being strong as a person is not defined as mere physical strength."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Factors such as intelligence, experience and knowledge are",
                                        "also considered when judging one's strength."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args!["Let's say you're very strong and given the most powerful weapon."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args!["If you don't know how to use the weapon's power, you will not be strong... You will be weak."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "When the tools or weapons",
                                        "overwhelm your capabilities,",
                                        "the worst situations result."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ch_par").get()?.number()? < 15 {
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Ah...",
                                        "Did you just say you're going to gather all the items soon? Oh, thank you for your kindness.",
                                        "I appreciate your effort",
                                        "on my behalf."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ch_par").get()? == 15 {
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Ah....",
                                        "Did you just say you're going to gather all the items soon? Oh, thank you for your kindness.",
                                        "I appreciate your effort",
                                        "on my behalf."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ch_par").get()? == 16 {
                                if ctx.call(Function::CountItem, vec![Val::from(7252)])?.number()? < 1 {
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args![
                                            "Umm.....",
                                            "Is that so...?",
                                            "I understand even",
                                            "if you can't bring",
                                            "the medicine right away."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args![
                                            "It's okay for now,",
                                            "so don't rush yourself.",
                                            "Everybody has their own problems, so I can understand if your own troubles must take priority."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args!["Still, it's good to see that you're understanding of the troubles other people are having."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Hua Tuo", args!["Ah, finally...", "You've brought what I need. Thank you so much, it's such a relief to have this medicine onhand again."])?;
                                ctx.next()?;
                                ctx.lines_as("Hua Tuo", args!["I apologize in advance,", "but may I ask you another favor?", "I am asking for your help once more, since I know I can depend on you. Of course, I will compensate you for your trouble."])?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(ctx, &[Val::from("I'm sorry...:No problem.")])?) == 1 {
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args![
                                            "Alright....",
                                            "I understand.",
                                            "But thank you for",
                                            "helping me out.",
                                            "Please take this..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args![
                                            "It's not much, but this medicine",
                                            "is an old family secret. I hope that it will be of use to you in dangerous situations."
                                        ],
                                    )?;
                                    ctx.call(Function::DelItem, vec![Val::from(7252), Val::from(1)])?;
                                    ctx.var("ch_par").set(Val::from(10))?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(11056)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(679), Val::from(2)])?;
                                    ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args!["Well then, I will see you around. Once again, I'd like to thank you for your help."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Hua Tuo", args!["Thank you, thank you so much!", "I've just run out of other medicines that my patients will be needing. I don't need too much, but you would be doing me a great favor."])?;
                                ctx.next()?;
                                ctx.lines_as("Hua Tuo", args!["The medicines I need are ^0000ff2 Leopard Claw^000000 which supports the bones, ^0000ff10 Solid Peach^000000 which strengthens the muscle, ^0000ff5 Poisonous Toad Skin^000000 which replenishes the skin..."])?;
                                ctx.next()?;
                                ctx.lines_as("Hua Tuo", args!["^0000ff20 Brown Root^000000 which regulates the heart, ^0000ff10 Sprout^000000 which eases the abdomen and ^0000ff5 Honey Pot^000000 which provides nutrition."])?;
                                ctx.next()?;
                                ctx.lines_as("Hua Tuo", args!["I hope you've", "memorized it all.", "Once again, that's..."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "^3355FF2 Leopard Claw^000000,",
                                        "^3355FF10 Solid Peach^000000,",
                                        "^3355FF5 Poisonous Toad Skin^000000,",
                                        "^3355FF20 Brown Root^000000,",
                                        "^3355FF10 Sprout^000000 and",
                                        "^3355FF5 Honey Pot^000000."
                                    ],
                                )?;
                                ctx.var("ch_par").set(Val::from(17))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("ch_par").get()? == 17 {
                                if (((((ctx.call(Function::CountItem, vec![Val::from(7172)])?.number()? > 1
                                    && ctx.call(Function::CountItem, vec![Val::from(7164)])?.number()? > 9)
                                    && ctx.call(Function::CountItem, vec![Val::from(7155)])?.number()? > 4)
                                    && ctx.call(Function::CountItem, vec![Val::from(7188)])?.number()? > 19)
                                    && ctx.call(Function::CountItem, vec![Val::from(7193)])?.number()? > 9)
                                    && ctx.call(Function::CountItem, vec![Val::from(7121)])?.number()? > 4)
                                {
                                    ctx.lines_as("Hua Tuo", args!["Hm? Ah, you have returned.", "Sorry, I was taking care of my other patients. Have you already gathered the medicines I'll need?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Hua Tuo",
                                        args![
                                            "Let's see...",
                                            "Oh, you've brought them all.",
                                            "Thank you so much for your",
                                            "generous help. It's such a relief to have these medicines available again..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Hua Tuo", args!["Please take this as a token of my appreciation. It's not much, but this medicine is an old family secret. I hope it will be helpful to you in dangerous situations."])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7172), Val::from(2)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7164), Val::from(10)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7155), Val::from(5)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7188), Val::from(20)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7193), Val::from(10)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7121), Val::from(5)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7252), Val::from(1)])?;
                                    ctx.var("ch_par").set(Val::from(18))?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(11057)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(679), Val::from(5)])?;
                                    ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                                    ctx.next()?;
                                    ctx.lines_as("Hua Tuo", args!["However, please remember not to take more than the recommended dosage for the pills I have given you. Even medicine, in the wrong doses, can become poison to the body."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "Hmm...",
                                        "Unfortunately, you haven't collected everything that I need yet. Once again, please gather..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Hua Tuo",
                                    args![
                                        "^3355FF2 Leopard Claw^000000,",
                                        "^3355FF10 Solid Peach^000000,",
                                        "^3355FF5 Poisonous Toad Skin^000000,",
                                        "^3355FF20 Brown Root^000000,",
                                        "^3355FF10 Sprout^000000 and",
                                        "^3355FF5 Honey Pot^000000."
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
    ctx.lines_as(
        "Hua Tuo",
        args![
            "Your health is affected by",
            "many factors. Nutricious food and medicine alone cannot guarantee",
            "a healthy lifestyle."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Hua Tuo", args!["Seemingly little things like your everyday mood and behavior, your thoughts and words also contribute to states of illness or well-being."])?;
    ctx.next()?;
    ctx.lines_as(
        "Hua Tuo",
        args![
            "Everyone needs time to quiet",
            "their thoughts and relax. If you can maintain a calm mood throughout your daily life, your body will greatly benefit."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hua Tuo",
        args!["Hahaha...", "I hope you enjoy", "your visit here", "in Luoyang."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn doctor_lyang(ctx: &Ctx) -> Script {
    doctor_lyang_body(ctx, Vec::new()).map(|_| ())
}

fn familiar_looking_patient_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_par").get()? == 0 {
        ctx.lines_as("??????", args!["Awwww.....", "Ummm....", "^666666*Cough...cough...*^000000"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.var("ch_par").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ch_par").get()?.number()? < 18 {
        ctx.lines_as(
            "??????",
            args!["^666666*Cough cough...*^000000", "Aww......www...", "Aww...wwww.."],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("??????", args!["Zzzzzzz", "Zzzzz.....", "Zzzzz....."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn familiar_looking_patient(ctx: &Ctx) -> Script {
    familiar_looking_patient_body(ctx, Vec::new()).map(|_| ())
}
