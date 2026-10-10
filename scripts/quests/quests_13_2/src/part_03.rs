use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum ToIn013ep13mdwarp01Step {
    Start,
    OnTouch,
}

fn to_in013ep13mdwarp01_run(ctx: &Ctx, mut step: ToIn013ep13mdwarp01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ToIn013ep13mdwarp01Step::Start => {
                step = ToIn013ep13mdwarp01Step::OnTouch;
                continue 'machine;
            }
            ToIn013ep13mdwarp01Step::OnTouch => {
                if (ctx.var("ep13_mdrama").get()?.number()? > 17 && ctx.var("ep13_mdrama").get()?.number()? < 24) {
                    ctx.call(Function::Warp, vec![Val::from("man_in01"), Val::from(13), Val::from(125)])?;
                    return Err(Stop::End);
                } else if ctx.var("ep13_mdrama").get()?.number()? > 23 {
                    ctx.call(Function::Warp, vec![Val::from("man_in01"), Val::from(68), Val::from(125)])?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("The doors are locked... I can't get in.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn to_in013ep13mdwarp01(ctx: &Ctx) -> Script {
    to_in013ep13mdwarp01_run(ctx, ToIn013ep13mdwarp01Step::Start, Vec::new()).map(|_| ())
}

pub fn to_in013ep13mdwarp01_ontouch(ctx: &Ctx) -> Script {
    to_in013ep13mdwarp01_run(ctx, ToIn013ep13mdwarp01Step::OnTouch, Vec::new()).map(|_| ())
}

fn captured_laphine_ep13md_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()?.number()? < 19 {
            ctx.lines(args![
                "She's unconscious.",
                "I can see a bandage to stop the bleeding...",
                "Looks like Luik did it."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 19 {
            ctx.lines(args![
                "Luik and Snorren did everything to extract juice out of a Yggdrasilberry",
                "chopped and stuffed it into a mouth and paste them on the body..."
            ])?;
            ctx.next()?;
            ctx.lines_as("Wounded Laphine", args!["Hmm..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Luik",
                args![
                    "Finally she's coming to.",
                    ((Val::from("Ok, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", it's up to you now.")),
                    "And you better not fake any of the words she's saying."
                ],
            )?;
            ctx.next()?;
            ctx.mes("- I nod to Luik and began talking to the Laphine. -")?;
            ctx.next()?;
            ctx.mes("- After the Laphine gained her full consciousness, she gazed around and spoke in shaky voice. -")?;
            ctx.next()?;
            ctx.lines_as(
                "Wounded Laphine",
                args!["Where... am I?", "Oh My! That's right! I need Bradium!", "Get me out of here!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wounded Laphine",
                args!["If I don't go back to him quickly, he... he is going to die!!"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Who is HE?:Is your name Terra?")])? {
                1 => {
                    ctx.lines_as(
                        "Wounded Laphine",
                        args![
                            "A giant... a giant in a cave...",
                            "He...He got hurt trying to protect me...",
                            "He wouldn't move!",
                            "His body was getting cold."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wounded Laphine",
                        args![
                            "We were each other's enemy......",
                            "but he saved me...",
                            "I heard they need a bradium to live!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Wounded Laphine", args!["Give me some bradium. Hurry!", "I can't be late!"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("You are Terra. Right?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Terra", args!["How...How do you know my name?", "Who are you?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Arc asked me to find you. So I came here.",
                            "By the way.. You were saying about a giant, do you mean a Sapha?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["The Sapha in the cave...", "His body was already petrified when I got there."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args![
                            "Giants.. It is said that the Sapha people are cursed to be petrified.",
                            "And that the Bradium is what prevents them from becoming petrified."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args!["So I tried to bring a bradium to him... ...", "But he... Sob..."],
                    )?;
                    ctx.next()?;
                    ctx.mes("- Terra couldn't continue to talk as she was agonizing in the pain from her wounds. -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Luik",
                        args!["Why did she break down like that? What did she say?", "Tell me what she said!!!"],
                    )?;
                    ctx.var("ep13_mdrama").set(Val::from(20))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Terra", args!["How...How do you know my name?", "Who are you?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I came looking for you on behalf of Arc.", "Just how did you end up here?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args!["To find a... Bra.. dium.", "That's right. Bradium! Give me a bradium!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args![
                            "A giant... a giant in a cave...",
                            "He...He got hurt trying to protect me...",
                            "He wouldn't move!",
                            "His body was getting cold."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args![
                            "We were each other's enemy......",
                            "but he saved me...",
                            "I heard they need a bradium to live!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Terra", args!["Give me some bradium. Hurry!", "I can't be late!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["The Sapha in the cave...", "His body was already petrified when I got there."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args![
                            "Giants.. It is said that the Sapha people are cursed to be petrified.",
                            "And that the Bradium is what prevents them from becoming petrified."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Terra",
                        args!["So I tried to bring a bradium to him... ...", "But he... Sob..."],
                    )?;
                    ctx.next()?;
                    ctx.mes("- Terra couldn't continue to talk as she was agonizing in the pain from her wounds. -")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Luik",
                        args!["Why did she break down like that? What did she say?", "Tell me what she said!!!"],
                    )?;
                    ctx.var("ep13_mdrama").set(Val::from(20))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("ep13_mdrama").get()? == 20 {
            ctx.lines_as(
                "Luik",
                args!["Let that Fairy rest for a while, now tell me what you were talking about."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("ep13_mdrama").get()?.number()? > 20 && ctx.var("ep13_mdrama").get()?.number()? < 24) {
            ctx.lines_as("Luik", args!["She's just too exhausted and fell asleep...", "Leave her alone."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Luik",
                args![
                    "She can leave any time she likes.",
                    "It feels so dry in here.",
                    "Oh well, that's normal."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("ep13_mdrama").get()?.number()? < 19 {
            ctx.lines(args![
                "She's unconscious.",
                "I can see a bandage to stop the bleeding...",
                "Looks like Luik did it."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Luik", args!["Ier er ee ras d?", "Ye ada sd?", "Nffd..?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Wounded Laphine",
                args!["...Riveh...AshIman Or mah...", "..ah..Thor..ThorOsa Yee Lu..ung..."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn captured_laphine_ep13md(ctx: &Ctx) -> Script {
    captured_laphine_ep13md_body(ctx, Vec::new()).map(|_| ())
}

fn snorren_ep13md_15_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()? == 18 {
            ctx.lines_as(
                "Snorren",
                args![
                    "Here, Luik. He said this is a cure for Laphine.",
                    "Should we just let her eat it as it is?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Snorren", args!["Uh.. and your name was..?"])?;
            ctx.next()?;
            let choice = runtime::select_values(
                ctx,
                &[((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))],
            )?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Snorren",
                args![
                    "Yeah, Right.",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                        + Val::from(", you should try talking to that Laphine...")),
                    "Let's hear her story."
                ],
            )?;
            ctx.var("ep13_mdrama").set(Val::from(19))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(7064), Val::from(7065)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 19 {
            ctx.lines_as(
                "Snorren",
                args![
                    "Please help me translater.",
                    "We'll be watching as you speak to her.",
                    "Right, Luik?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Luik", args!["Maybe.", "Let's ask her what happened, before it is too late."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 20 {
            ctx.lines_as(
                "Snorren",
                args![
                    "Let's listen to her story.",
                    "Why don't we start from... Why does she want a bradium?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Snorren", args!["This part must be told to Luik in exact detail."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 21 {
            ctx.lines_as(
                "Snorren",
                args![
                    "What should I do if the Sapha in the Cave is really Ogen...?",
                    "Ogen... What should I do if anything happened to him...?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 22 {
            if (ctx.call(Function::CountItem, vec![Val::from(6085)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(6084)])?.number()? > 0)
            {
                ctx.lines_as(
                    "Luik",
                    args![
                        "...This Bradium has not been refined properly...",
                        "This would be no help...",
                        "and, this muffler..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["This is Ogen's! Ogen's muffler!", "Luik! Ogen! This is Ogen!", "Unrefined Bradium!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["Lu.. Luik. Ogen.. Ogen...?!", "I'm going to Ogen!", "Where is that cave?! Where!"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Calm down!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Luik",
                    args![
                        "Yeah. Listen to him, calm down. Snorren.",
                        "If it is Ogen... We could be able to save him if we make haste."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args!["Snorren. Go to Refinery and get the finest bradium...", "Save him."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...eh?...")),
                        "Please look after Snorren...and Ogen.",
                        "This Laphine, I mean Terra...",
                        "She truly wanted to save Ogen..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args!["I will talk to my superiors to try and settle this matter...", "Please save Ogen."],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(6085), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(6084), Val::from(1)])?;
                ctx.var("ep13_mdrama").set(Val::from(23))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7067), Val::from(7068)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Luik",
                    args![
                        "You should bring something that can prove it?",
                        "... We have been fighting them for so long, it is hard for us to believe you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["...Yes... regardless of anything, just to prove what you say is true."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("ep13_mdrama").get()? == 23 {
                ctx.lines_as("Snorren", args!["Hurry up and take the lead!", "Which way should we go now?"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Snorren", args!["How did you get in here?!", "Get out!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as("Snorren", args!["We rs...", "F as d dd ", "Tb ds dfw we!", "Nd fs asd as...!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn snorren_ep13md_15(ctx: &Ctx) -> Script {
    snorren_ep13md_15_body(ctx, Vec::new()).map(|_| ())
}

fn luik_ep13md16_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()?.number()? < 20 {
            ctx.lines_as(
                "Luik",
                args![
                    "...I really look forward to drawing something useful out of her.",
                    "Your interpretion is trustworthy, right?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 20 {
            ctx.lines_as(
                "Luik",
                args!["First of all, Why did that Laphine come here?", "Why does she need a bradium?"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("To give it to a Sapha in a cave...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Luik",
                args![
                    "Sapha in a cave? What cave?",
                    "Nevermind.... Snorren. You must know something about that cave, right?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Luik", args!["Wait...", "Why would a Laphine give a bradium to our ally?"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "They fought in the cave and your friend got hurt and began to petrify.",
                    "So she came here to get a bradium to try to save his life..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Luik",
                args![
                    "They fought?",
                    "Snorren! What's that look on your face?",
                    "Spit if out if you have anything to say."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Snorren",
                args![
                    "Frankly, Luik. I... told you... Ogen's missing.",
                    "Ogen's disappearance and that Laphine's arrival coincide..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Luik", args!["And?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Snorren",
                args![
                    "Maybe the Sapha that Laphine is talking about is Ogen.",
                    "Or do we have any other comrades who go to strange caves?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Luik",
                args![
                    "We don't, because there are no caves like that here.",
                    "First, we should find out about the cave they are talking about."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Luik",
                args![
                    "You outsiders are in contact with the Laphine right?",
                    "What if this is an evil plot by them?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("That's not true!")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Luik",
                args![
                    "And how would you prove it?",
                    "We've been fighting them for a long time.",
                    "We don't have any reason to trust what you or they are saying."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Luik", args!["And talking about some cave that doesn't even exist."])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "That cave really exists, I swear!",
                    "I.. yeah. I saw a petrified Sapha in that cave."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I will bring back something that will make you believe.",
                    "I'm not sure if that Sapha is this Ogen you are looking for...",
                    "But I'll come back..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Don't you dare...try to do any harm to Terra."],
            )?;
            ctx.var("ep13_mdrama").set(Val::from(21))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(7065), Val::from(7066)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 21 {
            ctx.lines_as(
                "Luik",
                args![
                    "...Anything will do. Try and make me believe you -Outsider- and that Laphine.",
                    "We Sapha are not stupid people."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Luik",
                args!["I promise you that we will continue to treat that Laphine as we did so far and will not do any harm to her."],
            )?;
            ctx.next()?;
            ctx.lines_as("Luik", args!["Hence, you just focus on finding a way to make us believe you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 22 {
            if (ctx.call(Function::CountItem, vec![Val::from(6085)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(6084)])?.number()? > 0)
            {
                ctx.lines_as(
                    "Luik",
                    args![
                        "...This Bradium has not been refined properly...",
                        "This would be no help...",
                        "and, this muffler..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["This is Ogen's! Ogen's muffler!", "Luik! Ogen! This is Ogen!", "Unrefined Bradium!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["Lu.. Luik. Ogen.. Ogen...?!", "I'm going to Ogen!", "Where is that cave?! Where!"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Calm down!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Luik",
                    args![
                        "Yeah. Listen to him, calm down. Snorren.",
                        "If it is Ogen... We could be able to save him if we make haste."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args!["Snorren. Go to Refinery and get the finest bradium...", "Save him."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args![
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...eh?...")),
                        "Please look after Snorren...and Ogen.",
                        "This Laphine, I mean Terra...",
                        "She truly wanted to save Ogen..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Luik",
                    args!["I will talk to my superiors to try and settle this matter...", "Please save Ogen."],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(6085), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(6084), Val::from(1)])?;
                ctx.var("ep13_mdrama").set(Val::from(23))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(7067), Val::from(7068)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Luik",
                    args![
                        "You should bring something that can prove it?",
                        "... We have been fighting them for so long, it is hard for us to believe you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Snorren",
                    args!["...Yes... regardless of anything, just to prove what you say is true."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("ep13_mdrama").get()? == 23 {
                ctx.lines_as("Luik", args!["Please... Help Ogen."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as("Luik", args!["How did you get in here?", "You shouldn't be here."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as("Luik", args!["Na w ewe w", "Aewrf sd fsd iyu. ", "Ou uur?"])?;
        ctx.next()?;
        ctx.mes("Looks like Luik is giving me a look of scorn.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn luik_ep13md16(ctx: &Ctx) -> Script {
    luik_ep13md16_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TerraGoneStep {
    Start,
    OnTouch,
}

fn terra_gone_run(ctx: &Ctx, mut step: TerraGoneStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TerraGoneStep::Start => {
                step = TerraGoneStep::OnTouch;
                continue 'machine;
            }
            TerraGoneStep::OnTouch => {
                ctx.lines(args![
                    "There's nothing else.",
                    "Only traces of the cage with something confined."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn terra_gone(ctx: &Ctx) -> Script {
    terra_gone_run(ctx, TerraGoneStep::Start, Vec::new()).map(|_| ())
}

pub fn terra_gone_ontouch(ctx: &Ctx) -> Script {
    terra_gone_run(ctx, TerraGoneStep::OnTouch, Vec::new()).map(|_| ())
}

fn arc_ep13md_l02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 {
        if ctx.var("ep13_mdrama").get()? == 26 {
            ctx.lines_as("Terra", args!["........."])?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "I see. That's what happened.",
                    "But... That Sapha...What has he become to you in that short period of time?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Arc?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Arc",
                args![
                    "Ah, finally. I've been speaking with Terra while we waited for you.",
                    "Terra. This is who saved you...",
                    (Val::from("You should thank ")
                        + (if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            Val::from("him.")
                        } else {
                            Val::from("her.")
                        }))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Terra", args!["Thank you...", "I'm sorry Arc... Sorry...", "......"])?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "So, this is the story.",
                    "Terra was wandering around your Camp and found a Sapha who was also wandering around your Camp at the time."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "They were both being cautious not to be seen by you humans but they caught sight of each other and started to fight."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args!["And then they eventually fell into that hole in the swamp and ended up in that cave."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Terra",
                args![
                    "Yeah... We were both unconscious for some time..",
                    "And by the time we were able to wake up and see... We were surrounded."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Terra",
                args![
                    "Our main concern was getting out of there...",
                    "Even though we couldn't understand each other's language... We made a temporary truce."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Terra", args!["And then.....", "........."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("So that's what happened.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["You were outnumbered and at the last moment Ogen sacrificed himself to protect you."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Terra",
                args!["So... I wanted to help him...", "I was just trying to repay him..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Terra", args!["Was I wrong...?", "Was I thinking wrong?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "Repaying one's dept is a good thing.",
                    "Especially for a proud Laphine, It sure is."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "But Terra...",
                    "You've made two big mistakes.",
                    "First is, You went away without permission...",
                    "And secondly..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "You didn't ask for help.",
                    "If something like that happened... of course I would help..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Terra",
                args!["I'm sorry Arc... I'm sorry...", "I won't act foolish ever again."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "Get some rest.",
                    "I will put in a good word to the superiors...",
                    ((Val::from("And ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                    "Thank you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arc",
                args![
                    "Here, I will give you these to show my appreciation.",
                    "It's not much, but you will be able to buy things in Splendide with these."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Arc", args!["I'm sorry this is all I can give you for now."])?;
            ctx.var("ep13_mdrama").set(Val::from(27))?;
            ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(25)])?;
            ctx.call(Function::GetExperience, vec![Val::from(1200000), Val::from(100000)])?;
            ctx.call(Function::CompleteQuest, vec![Val::from(7071)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()? == 27 {
            if ctx.call(Function::StrNpcInfo, vec![Val::from(1)])? == "Arc" {
                ctx.lines_as("Arc", args!["Terra. Get some rest...", "Rest easy...", "And..."])?;
            } else {
                ctx.lines_as("Terra", args!["Arc... I will get some rest...", "I'm sorry... And. You..."])?;
            }
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?,
                args![
                    "Originally... we Laphine were extremely reluctant to have others in our area.",
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", you will be a special exception."))
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                ((Val::from("[") + ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?) + Val::from("]"))
            ])?;
            if ctx.call(Function::StrNpcInfo, vec![Val::from(1)])? == "Arc" {
                ctx.mes("That's what Terra wants too.")?;
            }
            ctx.mes("It might be cramped, but you are always welcome to visit us.")?;
            ctx.var("ep13_mdrama").set(Val::from(28))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_mdrama").get()?.number()? > 27 {
            if ctx.call(Function::StrNpcInfo, vec![Val::from(1)])? == "Arc" {
                ctx.lines_as(
                    "Arc",
                    args![
                        "How are you adapting to Splendide?",
                        "Terra's still not fully recovered yet, so keep that in mind."
                    ],
                )?;
            } else {
                ctx.lines_as("Terra", args!["I'm sorry...", "My body is not fully recovered yet..."])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.call(Function::StrNpcInfo, vec![Val::from(1)])? == "Arc" {
                ctx.lines_as("Arc", args!["The back of the right ... ", "......"])?;
            } else {
                ctx.lines_as("Terra", args!["I'm sorry... I'm so sleepy...", "...I want to sleep..."])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.call(Function::StrNpcInfo, vec![Val::from(1)])? == "Arc" {
            ctx.lines_as(
                "Arc",
                args![
                    "HirWosWeh. Yee DiebVilFar U manTalVil.",
                    "LarsNeiser...??",
                    "VeldTiTal Ko SharDurYur Di ?"
                ],
            )?;
        } else {
            ctx.lines_as(
                "Terra",
                args![
                    "ModBurDana...? Mu AnduWehFus Yee OsaLoLars...",
                    "eoFusser....",
                    "maurNohser Ur...... ThorNuffLars So "
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn arc_ep13md_l02(ctx: &Ctx) -> Script {
    arc_ep13md_l02_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TerrashomeInStep {
    Start,
    OnTouch,
}

fn terrashome_in_run(ctx: &Ctx, mut step: TerrashomeInStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TerrashomeInStep::Start => {
                step = TerrashomeInStep::OnTouch;
                continue 'machine;
            }
            TerrashomeInStep::OnTouch => {
                if ctx.var("ep13_mdrama").get()?.number()? > 25 {
                    ctx.call(Function::Warp, vec![Val::from("spl_in02"), Val::from(237), Val::from(89)])?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("It's locked.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn terrashome_in(ctx: &Ctx) -> Script {
    terrashome_in_run(ctx, TerrashomeInStep::Start, Vec::new()).map(|_| ())
}

pub fn terrashome_in_ontouch(ctx: &Ctx) -> Script {
    terrashome_in_run(ctx, TerrashomeInStep::OnTouch, Vec::new()).map(|_| ())
}

fn ep13mdf01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_apple = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_mdrama").get()? == 14 {
        ctx.lines(args![
            "There's some kind of fruit lying on the ground.",
            "I see a small berry inside of a big outer shell...",
            "It is a Yggdrasil!"
        ])?;
        ctx.var("ep13_mdrama").set(Val::from(15))?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(522), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_mdrama").get()? == 15 {
        ctx.mes("I've already pulled out a Yggdradsil... but is there anyting else..?")?;
        ctx.next()?;
        l_apple = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
        if l_apple.clone().number()? < 50 {
            ctx.lines(args!["I've been bitten by an unknown insect.", "It hurts!"])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_apple.clone() == 50 {
            ctx.mes("I found an apple.")?;
            ctx.call(Function::GetItem, vec![Val::from(512), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("There's nothing else.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("ep13_mdrama").get()?.number()? > 15 {
        ctx.mes("There's only an empty shell left.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["A huge fruit is here.", "I don't know what kind of fruit it is."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13mdf01(ctx: &Ctx) -> Script {
    ep13mdf01_body(ctx, Vec::new()).map(|_| ())
}

fn ep13mdf02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_apple = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_mdrama").get()? == 15 {
        ctx.lines(args![
            "There's some kind of fruit lying on the ground.",
            "I see a small berry inside of a big outer shell...",
            "It is a Yggdrasil!"
        ])?;
        ctx.var("ep13_mdrama").set(Val::from(16))?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(522), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_mdrama").get()? == 16 {
        ctx.mes("I've already pulled out a Yggdradsil... but is there anyting else..?")?;
        ctx.next()?;
        l_apple = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
        if l_apple.clone().number()? < 50 {
            ctx.lines(args!["I've been bitten by an unknown insect.", "It hurts!"])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_apple.clone() == 50 {
            ctx.mes("I found an apple.")?;
            ctx.call(Function::GetItem, vec![Val::from(512), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("There's nothing else.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("ep13_mdrama").get()?.number()? > 16 {
        ctx.mes("There's only an empty shell left.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["A huge fruit is here.", "I don't know what kind of fruit it is."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13mdf02(ctx: &Ctx) -> Script {
    ep13mdf02_body(ctx, Vec::new()).map(|_| ())
}

fn ep13mdf03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_apple = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_mdrama").get()? == 16 {
        ctx.lines(args![
            "There's some kind of fruit lying on the ground.",
            "I see a small berry inside of a big outer shell...",
            "It is a Yggdrasil!"
        ])?;
        ctx.var("ep13_mdrama").set(Val::from(17))?;
        ctx.call(Function::GetItem, vec![Val::from(607), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(522), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(7063), Val::from(7064)])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Arc gave me three Yggdrasilberries and...",
                "I've found... three. Six of them should be enough.",
                "Let's go back to Snorren."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_mdrama").get()? == 17 {
        ctx.mes("I've already pulled out a Yggdradsil... but is there anyting else..?")?;
        ctx.next()?;
        l_apple = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
        if l_apple.clone().number()? < 50 {
            ctx.lines(args!["I've been bitten by an unknown insect.", "It hurts!"])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_apple.clone() == 50 {
            ctx.mes("I found an apple.")?;
            ctx.call(Function::GetItem, vec![Val::from(512), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.mes("There's nothing else.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("ep13_mdrama").get()?.number()? > 17 {
        ctx.mes("There's only an empty shell left.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args!["A huge fruit is here.", "I don't know what kind of fruit it is."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn ep13mdf03(ctx: &Ctx) -> Script {
    ep13mdf03_body(ctx, Vec::new()).map(|_| ())
}

fn manuk_galtun_ep13_2day_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_qst_cpl01 = Val::from(0);
    let mut l_qst_cpl02 = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()?.number()? > 99) {
        if ctx.var("ep13_2_days01").get()? == 0 {
            ctx.lines_as("Strom", args!["Hello.", "You're a human right?", "My name is Strom."])?;
            ctx.next()?;
            ctx.lines_as(
                "Strom",
                args!["But, you guys seem so weak.", "You don't have solid skin nor enough power."],
            )?;
            ctx.next()?;
            ctx.lines_as("Strom", args!["How did you guys get here?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Strom",
                args!["Your language seems gentle, but it seems like you are here to create more tension."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Keep the peace.:...")])? {
                1 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Yes, we humans are smaller than you... but...",
                            "we have been training ourselves to be strong."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Now let me show you that", "we can match your strength!"],
                    )?;
                    ctx.next()?;
                    if ctx.var("ep13_mdrama").get()?.number()? > 5 {
                        ctx.lines_as(
                            "Strom",
                            args![
                                "Anyway...",
                                "Some bad things happened to my one of my colleagues in that cave you found recently."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Strom",
                            args![
                                "There're monsters called ^4d4dffRata^000000 and ^4d4dffDueyrr^000000.",
                                "If you terminate them, I would regard you as a decent and strong person."
                            ],
                        )?;
                        ctx.var("ep13_2_days01").set(Val::from(1))?;
                        ctx.call(Function::SetQuest, vec![Val::from(7074)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(7075)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Strom",
                            args![
                                "...is that it?",
                                "Let me think.",
                                "Since you have so much confidence in their abilities...",
                                "I will find you a worthy challenge to prove it."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Yes, but human beings can be stronger when they all work together."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("ep13_2_days01").get()? == 1 {
            l_qst_cpl01 = ctx.call(Function::CheckQuest, vec![Val::from(7074), ctx.constant("HUNTING")?])?;
            l_qst_cpl02 = ctx.call(Function::CheckQuest, vec![Val::from(7075), ctx.constant("HUNTING")?])?;
            if (l_qst_cpl01.clone() == 2 && l_qst_cpl02.clone() == 2) {
                ctx.lines_as(
                    "Strom",
                    args![
                        "Sure enough... I, the Sapha Galtun, Strom, apologize to you. I should not have been so quick to despise you.",
                        "I admit that you are a brave soldier, please feel free to visit Manuk."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Strom",
                    args![
                        "Although I don't have much to offer, please accept these coins.",
                        "If you need to buy something, you can use them."
                    ],
                )?;
                ctx.call(Function::CompleteQuest, vec![Val::from(7074)])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(7075)])?;
                ctx.var("ep13_2_days01").set(Val::from(2))?;
                ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(300000)])?;
                ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(10)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Strom",
                    args![
                        "If you want to show you're so strong...",
                        "Please find and defeat the Rata and Duneyrr in the cave."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Strom",
                args![
                    "Here icy climate is not suitable for biological survival, but we do not feel cold,",
                    "so we're not very concerned about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Strom",
                args![
                    "But... you're different.",
                    "In such cold weather, you are best to don Hillslions fur..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as("Manuk Galtun", args!["Arpe? Yu osp sad?", "EW pisdn psa?", "We psis?"])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn manuk_galtun_ep13_2day(ctx: &Ctx) -> Script {
    manuk_galtun_ep13_2day_body(ctx, Vec::new()).map(|_| ())
}

fn manuk_engineer_ep13_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_alba = Val::from(0);
    let mut l_alba2 = Val::from(0);
    let mut l_time_chek = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()?.number()? > 99) {
        l_alba = ctx.call(Function::CheckQuest, vec![Val::from(7080)])?;
        if (l_alba.clone() == 0 || l_alba.clone() == 1) {
            l_time_chek = ctx.call(Function::CheckQuest, vec![Val::from(7080), ctx.constant("PLAYTIME")?])?;
            if l_time_chek.clone() != 2 {
                ctx.lines_as(
                    "Manuk Engineer",
                    args![
                        "Thank you for collecting the Enriched Bradium for me, it was very helpful.",
                        "We've got more than enough for now though."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Manuk Engineer",
                    args![
                        "Thank you for collecting the Enriched Bradium for me, it was very helpful.",
                        "I hope you can help me again."
                    ],
                )?;
                ctx.call(Function::EraseQuest, vec![Val::from(7080)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            l_alba2 = ctx.call(Function::CheckQuest, vec![Val::from(7079)])?;
            if (l_alba2.clone() == 0 || l_alba2.clone() == 1) {
                if ctx.call(Function::CountItem, vec![Val::from(6090)])?.number()? > 19 {
                    ctx.lines_as(
                        "Manuk Engineer",
                        args![
                            "Oh, that will do very well.",
                            "On behalf of the Sapha, I extend our thanks to you.",
                            "I hope you can help us again."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(6090), Val::from(20)])?;
                    ctx.call(Function::EraseQuest, vec![Val::from(7079)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(7080)])?;
                    ctx.call(Function::GetExperience, vec![Val::from(40000), Val::from(40000)])?;
                    ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(3)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Manuk Engineer", args!["What's up?", "You're not prepared yet."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as("Manuk Engineer", args!["Hello.", "This is where we refine stones."])?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("About collecting stones.:About the usage of stones.")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Manuk Engineer",
                            args![
                                "To prevent our bodies from becoming numb, we need Bradium.",
                                "From the Bradium we can extract a special element that we need to survive."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manuk Engineer",
                            args![
                                "The mining industry is primary here since our lives depend on it.",
                                "But the mine has become clogged."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manuk Engineer",
                            args![
                                "I've already heard that you guys are good soldiers.",
                                "Would you mind terminating the Bradium Golem, and bringing back some Bradium for me?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure, I don't mind.:Nope, I can't.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Manuk Engineer",
                                    args![
                                        "Then take this mission.",
                                        "Please hunt Bradium Golem and bring me 20 blended Bradium."
                                    ],
                                )?;
                                ctx.call(Function::SetQuest, vec![Val::from(7079)])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Manuk Engineer",
                                    args!["Oh, this is our fate.", "How could I be so naive to depend on others?"],
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
                        ctx.lines_as(
                            "Manuk Engineer",
                            args![
                                "We refine Bradium and extract special things from it.",
                                "Then form that into an injection.",
                                "When it is refined it is used to maintain our body cycles."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manuk Engineer",
                            args![
                                "That's why we are quite advanced in this industry.",
                                "Other industries are also well developed... but..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manuk Engineer",
                            args!["This is our top priority because it is crucial for our survival."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        }
    } else {
        ctx.lines_as("Manuk Engineer", args!["Aweo wpe?", "Sdd psiem!", "Awq ouwn ksudh bud ds."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn manuk_engineer_ep13_2(ctx: &Ctx) -> Script {
    manuk_engineer_ep13_2_body(ctx, Vec::new()).map(|_| ())
}

fn laphine_craftsman_ep13_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_alba = Val::from(0);
    let mut l_alba2 = Val::from(0);
    let mut l_time_chek = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()?.number()? > 99) {
        if ctx.var("ep13_mdrama").get()?.number()? > 5 {
            l_alba = ctx.call(Function::CheckQuest, vec![Val::from(7082)])?;
            if l_alba.clone() == 1 {
                l_time_chek = ctx.call(Function::CheckQuest, vec![Val::from(7082), ctx.constant("PLAYTIME")?])?;
                if (l_time_chek.clone() == 0 || l_time_chek.clone() == 1) {
                    ctx.lines_as(
                        "Laphine craftsman",
                        args![
                            "Thank you for collecting those items for me.",
                            "That should be sufficient for the time being."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Laphine craftsman",
                    args!["Thank you for collecting those items for me.", "Hopefully you can help us again."],
                )?;
                ctx.call(Function::EraseQuest, vec![Val::from(7082)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_alba2 = ctx.call(Function::CheckQuest, vec![Val::from(7081)])?;
            if (l_alba2.clone() == 0 || l_alba2.clone() == 1) {
                if (ctx.call(Function::CountItem, vec![Val::from(7326)])?.number()? > 14
                    && ctx.call(Function::CountItem, vec![Val::from(6075)])?.number()? > 14)
                {
                    ctx.lines_as(
                        "Laphine craftsman",
                        args![
                            "Oh, fantastic.",
                            "These are enough materials for today.",
                            "I'll let you know if I need more."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7326), Val::from(15)])?;
                    ctx.call(Function::DelItem, vec![Val::from(6075), Val::from(15)])?;
                    ctx.call(Function::EraseQuest, vec![Val::from(7081)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(7082)])?;
                    ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(30000)])?;
                    ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(3)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Laphine craftsman",
                        args!["What can I help you with?", "Not yet done with my request?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            ctx.lines_as(
                "Laphine craftsman",
                args!["What's up??", "I need to make more decorations for the Yai."],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("May I help you?:What's a Yai?")])?) == 1 {
                ctx.lines_as(
                    "Laphine craftsman",
                    args!["You want to help?", "This place is being used for battle but it is also our home."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine craftsman",
                    args![
                        "We prefer to be alone so we must each have our own Yai.",
                        "Though we are in a military base we should take care of each Yai."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine craftsman",
                    args![
                        "I am not a soldier but we hope all of us to be as confident as they are.",
                        "I would like to support all Laphine with my skills."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine craftsman",
                    args![
                        "So I want to help make decorations for everyone's Yai.",
                        "I need more materials to make them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine craftsman",
                    args![
                        "I heard that a water ghost has been appearing in the cave.",
                        "Can you bring me 15 Crystallized Teardrop and 15 Fluorescent Liquid?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:Sorry.")])?) == 1 {
                    ctx.lines_as(
                        "Laphine craftsman",
                        args![
                            "Please take my request.",
                            "Can you bring me 15 Crystallized Teardrop and 15 Fluorescent Liquid?"
                        ],
                    )?;
                    ctx.call(Function::SetQuest, vec![Val::from(7081)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Laphine craftsman",
                    args![
                        "Yes, I'm sure that you have your own business to attend to.",
                        "I feel embarrassed asking a favor from you, as a Laphine."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Laphine craftsman",
                    args!["But, why are you trying to be involved in our business yet you won't take up our request?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Laphine craftsman",
                args![
                    "Yai...",
                    "All grown Laphine have one.",
                    "They're essential for all Laphine to live in since we are very private people."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("It's a house...?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Laphine craftsman",
                args![
                    "Yes...",
                    "That's it. We sometimes invite our friends or guests to our Yai.",
                    "And decorations for Yai are quite important to us..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Laphine craftsman",
                args!["We Laphine care about ourselves and our Yai as well."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Laphine craftsman",
                args![
                    "We are far away from our hometown now...",
                    "If we don't have this kind of hobby, there's nothing to enjoy here."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Laphine craftsman",
            args!["Hm? Oh, can't you see I'm busy?", "I'm making ornaments though, if you must know."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Laphine craftsman",
        args![
            "Yur,Dur AnoVa?",
            "Wha? Dieb OsaDur .. ",
            "ah..RuffThus NeAsh. man nesAsh OdesAlah ?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn laphine_craftsman_ep13(ctx: &Ctx) -> Script {
    laphine_craftsman_ep13_body(ctx, Vec::new()).map(|_| ())
}

fn pet_breeder_ep13_eden01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_alba_check = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(2)])? == 0 {
        ctx.lines(args![
            "Wait a moment!!",
            "You have too many items.",
            "You can't receive this.",
            "Lighten your weight and",
            "try again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_mdrama").get()?.number()? < 6 {
        ctx.lines_as("Pinedel", args!["Why isn't it working, Taab?", "That should be working."])?;
        ctx.next()?;
        ctx.lines_as("Taab", args!["That's very dangerous. Do you want to die..?!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Pinedel",
            args!["Are you ignoring cute pets??", "We've been able to get Pickys and even Zealotus!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Taab",
            args!["No way, even if you ask me...", "Hillslion is very adorable...", "But you can't."],
        )?;
        ctx.next()?;
        ctx.lines_as("Pinedel", args!["Then let me know why you object to this."])?;
        ctx.next()?;
        ctx.lines_as(
            "Taab",
            args![
                "It's too dangerous. I've trained monsters before.",
                "Hillslions look cute and adorable, but it's not very realistic."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Taab",
            args![
                "Can't you see that?",
                "I've tried to train them... but I couldn't.",
                "How can we can train them as a cute pet...?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Taab",
            args![
                "It would definitely try to escape from us...",
                "Then he would become very awful.",
                "I don't agree with this."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pinedel",
            args!["Nah...", "Can't we control them with magical power?", "We've done it before......"],
        )?;
        ctx.next()?;
        ctx.mes("- They just continue to argue -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Pinedel", args!["Ah, Taab isn't flexible at all.", "Hey there! What's up??"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Notice for criminal report:Cute pet investigation.")],
    )?) == 1
    {
        if ctx.var("ep13_2_wanted").get()? == 1 {
            if ctx.call(Function::CheckQuest, vec![Val::from(7076), ctx.constant("HUNTING")?])? == 2 {
                ctx.lines_as(
                    "Pinedel",
                    args![
                        "Have you hunted the Runaway Dandelion?",
                        "Ok, I accept you.",
                        "Here's something Rin was storing here."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pinedel",
                    args!["This is Rin's cherished treasure box...", "This is enough payment, right?"],
                )?;
                ctx.call(Function::CompleteQuest, vec![Val::from(7076)])?;
                ctx.var("ep13_2_wanted").set(Val::from(2))?;
                ctx.call(Function::GetItem, vec![Val::from(7444), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Pinedel",
                args![
                    "I know you haven't terminated the Runaway Dandelion yet.",
                    "Don't take me for a fool."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_2_wanted").get()? == 2 {
            ctx.lines_as(
                "Pinedel",
                args![
                    "I will forward everything to Rin.",
                    "She'll love it.",
                    ((Val::from("Just tell her your name ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pinedel",
                args![
                    "I can remember that I've heard about you from Rin.",
                    "But Rin likes you, so that must mean that you're nice."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pinedel",
            args!["Seaching for a criminal?", "I'm not sure that you're involved in it."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_dayegg").get()? == 1 {
        if ctx.call(Function::CountItem, vec![Val::from(6093)])?.number()? > 9 {
            ctx.lines_as(
                "Pinedel",
                args!["Oh! Nice!", "I can go through this study deeper with your assistance."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pinedel",
                args![
                    "I will try to make these Dragon eggs into cute pets.",
                    "Don't expect much!",
                    "Come back after a day to check on the progress."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(6093), Val::from(10)])?;
            ctx.call(Function::EraseQuest, vec![Val::from(7077)])?;
            ctx.call(Function::SetQuest, vec![Val::from(7078)])?;
            ctx.var("ep13_2_dayegg").set(Val::from(2))?;
            ctx.call(Function::GetExperience, vec![Val::from(40000), Val::from(40000)])?;
            ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pinedel",
            args![
                "Draco was being kept in the nest just before hatching. Please collect 10 Dragon eggs for me.",
                "You know, those are hard to hatch in captivity."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_2_dayegg").get()? == 2 {
        ctx.lines_as(
            "Pinedel",
            args!["Those eggs are about to hatch now.", "I will investigate those back home."],
        )?;
        l_alba_check = ctx.call(Function::CheckQuest, vec![Val::from(7078), ctx.constant("PLAYTIME")?])?;
        if l_alba_check.clone() == -1 {
            ctx.close_window()?;
            ctx.call(Function::EraseQuest, vec![Val::from(7078)])?;
            ctx.var("ep13_2_dayegg").set(Val::from(3))?;
            return Err(Stop::End);
        } else if (l_alba_check.clone() == 0 || l_alba_check.clone() == 1) {
            ctx.mes("For now I still need more time.")?;
            ctx.next()?;
            ctx.lines_as("Pinedel", args!["Can you come back here tomorrow?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.close_window()?;
        ctx.call(Function::EraseQuest, vec![Val::from(7078)])?;
        ctx.var("ep13_2_dayegg").set(Val::from(3))?;
        return Err(Stop::End);
    } else if ctx.var("ep13_2_dayegg").get()? == 3 {
        ctx.lines_as(
            "Pinedel",
            args![
                "Do you want to help me gather more Dragon eggs today?",
                "I want to try many things with those eggs."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:Sorry, I can't.")])?) == 1 {
            ctx.lines_as("Pinedel", args!["Please collect 10 Draco Eggs."])?;
            ctx.var("ep13_2_dayegg").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(7077)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Pinedel", args!["Ok. You have your own business here.", "Go ahead."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Pinedel",
        args![
            "I am investigating cute pets.",
            "Ash-vacuum has several monsters that I think could become cute pets."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pinedel",
        args!["The objective is one Dragon egg!", "Can you bring me a Dragon Egg?"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:Nope.")])?) == 1 {
        ctx.lines_as(
            "Pinedel",
            args![
                "There would be Dragon nest in the cave you found out.",
                "There should be nice Dragon Eggs."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pinedel",
            args!["Please collect 10 Draco Eggs.", "Let's work together to make them as pets."],
        )?;
        ctx.var("ep13_2_dayegg").set(Val::from(1))?;
        ctx.call(Function::SetQuest, vec![Val::from(7077)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Pinedel",
        args![
            "I know it's not the easiest task.",
            "But I refuse to give up.",
            "I will make it into a cute pet someday."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Taab",
        args![
            "Would you just give up making exogamous beings into cute pets...",
            " ",
            "[Pinedel]",
            "Taab doesn't have any guts!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pet_breeder_ep13_eden01(ctx: &Ctx) -> Script {
    pet_breeder_ep13_eden01_body(ctx, Vec::new()).map(|_| ())
}
