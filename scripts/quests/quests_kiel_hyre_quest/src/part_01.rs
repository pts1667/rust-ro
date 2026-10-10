use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PubMasterKhStep {
    Start,
    OnTouch,
}

fn pub_master_kh_run(ctx: &Ctx, mut step: PubMasterKhStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PubMasterKhStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7487), Val::from(1)])? == 0 {
                    ctx.lines_as(
                        "Vandt",
                        args![
                            "Just a second! You're",
                            "carrying too many items",
                            "right now. You'd better",
                            "put your stuff in Kafra",
                            "Storage or you won't be",
                            "able to pick up anything new..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("kielhyrequest").get()? == 0 {
                    ctx.lines_as(
                        "Vandt",
                        args!["Hi there, welcome", "to my pub. So what", "would you like to have?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[
                            Val::from("Beer, please."),
                            Val::from("A cocktail, please."),
                            Val::from("Soju, please."),
                            Val::from("Maybe later."),
                        ],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "Alright, let me get",
                                    "you a glass of beer on",
                                    "tap. There you are, this",
                                    "is out special Schwaltz Beer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("^3355FF*Gulp gulp gulp*^000000")?;
                            ctx.call(Function::PercentHeal, vec![Val::from(5), Val::from(-5)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Ahhh, it's really", "good! That really", "hits the spot!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "I'm sorry, but I have to",
                                    "deliver all of our cocktail",
                                    "ingredients to other customers.",
                                    "Maybe I'll have enough to make",
                                    "you something next time, okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "S-soju? I'm sorry,",
                                    "but we don't serve that",
                                    "here. It's too much of",
                                    "a tough guy drink for me..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        4 => {
                            ctx.lines_as(
                                "Vandt",
                                args!["Sure, just take", "your time, relax,", "and order something", "when you're ready."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if ctx.var("kielhyrequest").get()? == 1 {
                    ctx.lines_as(
                        "Vandt",
                        args!["Hi there, welcome", "to my pub. So what", "would you like to have?"],
                    )?;
                    ctx.next()?;
                    'b2: {
                        let subject2 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Beer, please:A cocktail, please:Soju, please.:You look worried, what's up?:Cancel",
                            )],
                        )?);
                        let mut matched2 = false;
                        let no_case2 = !subject2.loosely_equals(&Val::from(1))
                            && !subject2.loosely_equals(&Val::from(2))
                            && !subject2.loosely_equals(&Val::from(3))
                            && !subject2.loosely_equals(&Val::from(4))
                            && !subject2.loosely_equals(&Val::from(5));
                        if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "Alright, let me get",
                                    "you a glass of beer on",
                                    "tap. There you are, this",
                                    "is out special Schwaltz Beer."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.mes("^3355FF*Gulp gulp gulp*^000000")?;
                            ctx.call(Function::PercentHeal, vec![Val::from(5), Val::from(-5)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Ahhh, it's really", "good! That really", "hits the spot!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "I'm sorry, but I have to",
                                    "deliver all of our cocktail",
                                    "ingredients to other customers.",
                                    "Maybe I'll have enough to make",
                                    "you something next time, okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "S-soju? I'm sorry,",
                                    "but we don't serve that",
                                    "here. It's too much of",
                                    "a tough guy drink for me..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "Oh, did you overhear?",
                                    "I'm sorry, it's just that one",
                                    "of my employees had an",
                                    "accident and was pretty hurt,",
                                    "so I have nodoby that can",
                                    "deliver this merchandise."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vandt",
                                args![
                                    "I have an urgent order",
                                    "that I need to send to",
                                    "the Kiel Hyre Academy,",
                                    "but I can't find anyone",
                                    "that's available for this",
                                    "kind of temporary job."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("I'm sorry to hear that:Do you want me to help you?")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Vandt",
                                        args![
                                            "Well, I'm sure that I'll",
                                            "figure something out.",
                                            "Do you know anyone",
                                            "that'd be interested in",
                                            "some part time work?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Vandt",
                                        args![
                                            "Really? That's great!",
                                            "But first, I think it's fair to",
                                            "tell you that this job may not",
                                            "be as simple as you'd think.",
                                            "I expect you to complete the",
                                            "delivery, no matter what."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Sure I'll do it:Wait, let me think about it...")])? {
                                        1 => {
                                            ctx.lines_as(
                                                "Vandt",
                                                args![
                                                    "I'm glad to hear that.",
                                                    "Well then, please take this",
                                                    "bottle of Culinary Wine to",
                                                    "Mrs. ^ff0000Lecollane^000000 in the Kiel",
                                                    "Hyre Academy. I'll pay you",
                                                    "once you finish the job, okay?"
                                                ],
                                            )?;
                                            ctx.call(Function::GetItem, vec![Val::from(7487), Val::from(1)])?;
                                            ctx.var("kielhyrequest").set(Val::from(2))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Vandt",
                                                args![
                                                    "Sure thing. I really need",
                                                    "to get this done, so if you",
                                                    "can't do it, but know any",
                                                    "capable, responsible",
                                                    "people that can, then please",
                                                    "tell them about my situation."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                            matched2 = true;
                        }
                        if matched2 {
                            ctx.lines_as(
                                "Vandt",
                                args!["Sure, just take", "your time, relax,", "and order something", "when you're ready."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else if (ctx.var("kielhyrequest").get()?.number()? >= 2 && ctx.var("kielhyrequest").get()?.number()? < 6) {
                    ctx.lines_as(
                        "Vandt",
                        args![
                            "Please deliver that bottle",
                            "of Wine I gave you to Mrs.",
                            "Mrs. ^ff0000Lecollane^000000, who should be",
                            "inside the Kiel Hyre Academy.",
                            "Hurry and get it to her before she",
                            "can complain about the delivery."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("kielhyrequest").get()? == 6 {
                    if ctx.call(Function::CountItem, vec![Val::from(7487)])? == 0 {
                        ctx.lines_as(
                            "Vandt",
                            args![
                                "Oh, you're back.",
                                "Thanks for making that",
                                "delivery. Just give me",
                                "a moment, and then I can",
                                "pay you in zeny, okay?"
                            ],
                        )?;
                        ctx.next()?;
                        let choice = runtime::select_values(ctx, &[Val::from("I need more wine...")])?;
                        ctx.var("@menu").set(choice)?;
                        ctx.lines_as(
                            "Vandt",
                            args![
                                "Oh, you need to deliver",
                                "another bottle? Alright,",
                                "let me look around, and",
                                "I'll give you the wine",
                                "and your payment."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FFRummage Rummage^000000", "^3355FFRummage Rummage^000000"])?;
                        ctx.next()?;
                        ctx.lines_as("Vandt", args!["There you go!", "Thank you so much", "for helping me out~"])?;
                        ctx.call(Function::GetItem, vec![Val::from(7487), Val::from(1)])?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()? + Val::from(1000)))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Vandt",
                            args![
                                "Hey, thanks a lot",
                                "for helping me out that",
                                "last time. I knew I asked",
                                "you out of the blue, but",
                                "you ended up being a life",
                                "saver! I really appreciate it!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Vandt",
                        args![
                            "You're a really good",
                            "worker, you know that?",
                            "Dependable, responsible,",
                            "willing to help others, and",
                            "proactive too! I think you'll",
                            "go far in life, kid, I really do~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = PubMasterKhStep::OnTouch;
                continue 'machine;
            }
            PubMasterKhStep::OnTouch => {
                if ctx.var("kielhyrequest").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Vandt",
                        args![
                            "Arrrggghhh...",
                            "This can't be good...",
                            "This isn't good at all!",
                            "What am I suposed to do?"
                        ],
                    )?;
                    ctx.var("kielhyrequest").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn pub_master_kh(ctx: &Ctx) -> Script {
    pub_master_kh_run(ctx, PubMasterKhStep::Start, Vec::new()).map(|_| ())
}

pub fn pub_master_kh_ontouch(ctx: &Ctx) -> Script {
    pub_master_kh_run(ctx, PubMasterKhStep::OnTouch, Vec::new()).map(|_| ())
}

fn little_kid_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Cezu]")?;
    if ctx.var("kielhyrequest").get()?.number()? < 6 {
        ctx.lines(args![
            "Fresh, crunchy toast!",
            "If you want some, come",
            "and get some tooooast~"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 6 {
        if ctx.var("khtoastgirlend").get()?.number()? < 1 {
            ctx.lines(args![
                "Fresh, crunchy toast!",
                "If you want some, come",
                "and get some tooooast~",
                "Oh! Hi hi~ Did you want",
                "to buy some yummy toast?"
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I'm here for Elly:No, thanks")])? {
                1 => {
                    ctx.lines_as(
                        "Cezu",
                        args![
                            "Oh, I see. Elly must have",
                            "wasted another batch of",
                            "ingredients again. Well,",
                            "she's a regular customer,",
                            "so I really want to help, but",
                            "I can't really do anything."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cezu",
                        args![
                            "You see, I just ran",
                            "out of ingredients too!",
                            "But I can't really leave",
                            "to get some more. What if",
                            "people need to buy toast?",
                            "Listen, can you help me out?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cezu",
                        args![
                            "Would you please go get",
                            "some flour and eggs for me",
                            "from the ^3355FFLighthalzen Windmill^000000",
                            "Then, when you come back, I can",
                            "divide the ingredients, and you",
                            "can deliver some to Elly."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cezu",
                        args![
                            "I know that I'm basically",
                            "making you do everything",
                            "on your own, but please try",
                            "to understand that my hands",
                            "are tied. D-don't ask me",
                            "why, they just are!"
                        ],
                    )?;
                    ctx.var("khtoastgirlend").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Cezu", args!["Okay okay~", "Please come again!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("khtoastgirlend").get()? == 1 {
            ctx.lines(args![
                "Would you please go to",
                "the Lighthalzen Windmill",
                "and tell them that Cezu needs",
                "lots of flour and lots of eggs!",
                "Then, bring all the stuff over",
                "to me as soon as you can~"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("khtoastgirlend").get()? == 2 {
            ctx.lines(args![
                "Hey, you're back with the",
                "ingredients! Thank you so",
                "much, I really needed these!",
                "Now please give this flour",
                "and these eggs to Elly, and",
                "send her my regards. See you~"
            ])?;
            ctx.call(Function::DelItem, vec![Val::from(7488), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(7488), Val::from(1)])?;
            ctx.var("khtoastgirlend").set(Val::from(3))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("kielhyrequest").get()?.number()? > 6 {
        ctx.var("khtoastgirlend").set(Val::from(0))?;
    }
    ctx.lines(args!["Hot, fresh and", "cruuuunchy toast!", "Come and get some!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn little_kid_kh(ctx: &Ctx) -> Script {
    little_kid_kh_body(ctx, Vec::new()).map(|_| ())
}

fn windmill_owner_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7488), Val::from(1)])? == 0 {
        ctx.lines_as(
            "Mills",
            args![
                "Hey, you've got too much",
                "stuff on you right now. ",
                "Put your junk in Kafra Storage",
                "if you expect me to give you",
                "anything. That's why you came",
                "here to the miss, didn't you?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("khtoastgirlend").get()?.number()? < 1 {
        ctx.lines_as(
            "Mills",
            args![
                "Hey, whaddya want?",
                "I'm pretty busy right",
                "now, so you mind coming",
                "back later? Then we'll talk."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("khtoastgirlend").get()? == 1 {
        ctx.lines_as(
            "Mills",
            args![
                "Hey, whaddya want?",
                "You just happened to",
                "catch me at a good time,",
                "but if you need anything,",
                "you'd better spit it out quick",
                "before things get hectic again."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'm here for Cezu:......")])? {
            1 => {
                ctx.lines_as(
                    "Mills",
                    args![
                        "Oh, Cezu from the",
                        "toast stand? Okay,",
                        "I've got everything",
                        "that kid needs right",
                        "here. There's eggs inside,",
                        "so be really careful with it.",
                        "It's ready for you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mills",
                    args![
                        "Kid's one of my regular",
                        "customers, so you don't",
                        "have to pay me, or run any",
                        "extra errands on my end. Yeah,",
                        "I know how other people treat",
                        "you adventurers. Well, see ya."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(7488), Val::from(1)])?;
                ctx.var("khtoastgirlend").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Mills",
                    args![
                        "......",
                        "Um, okay, well, if",
                        "you need something,",
                        "just hollar, I guess.",
                        "Cuts, Cutz where are you?",
                        "You'd better not be goofing off!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("khtoastgirlend").get()?.number()? >= 2 {
        ctx.lines_as(
            "Mills",
            args![
                "That Cutz is such",
                "a lazy rascal. The guy",
                "thinks he can fool around",
                "when he's on the clock...!",
                "Ah well, he knows I can't find",
                "a better assistant. He'll learn..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn windmill_owner_kh(ctx: &Ctx) -> Script {
    windmill_owner_kh_body(ctx, Vec::new()).map(|_| ())
}

fn windmill_owner_s_helper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("khtoastgirlend").get()?.number()? < 1 {
        ctx.lines_as(
            "Cutz",
            args![
                "Man, why are we",
                "always so busy?",
                "Why does Mills have",
                "to work me to the bone?",
                "Eh, it's a living, I suppose..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("khtoastgirlend").get()?.number()? >= 2 {
        ctx.lines_as(
            "Cutz",
            args![
                "Wh-what makes you",
                "think I'm goofing off,",
                "Mills? L-look, look,",
                "my hands are moving,",
                "I'm busy, I'm working!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Cutz",
            args![
                "Hey, you're from",
                "Cezu's toast stand,",
                "right? How is cute",
                "little Cezu doing?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn windmill_owner_s_helper(ctx: &Ctx) -> Script {
    windmill_owner_s_helper_body(ctx, Vec::new()).map(|_| ())
}

fn hanie_kh1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hanie",
        args![
            "Oh, I wish I could study",
            "at the Kiel Hyre Academy...",
            "All of the graduates get",
            "really nice jobs! I'm almost",
            "jealous of the students!"
        ],
    )?;
    ctx.next()?;
    let choice = runtime::select_values(ctx, &[Val::from("Kiel Hyre Academy?")])?;
    ctx.var("@menu").set(choice)?;
    ctx.lines_as(
        "Hanie",
        args![
            "Oh, the Kiel Hyre Academy",
            "is a school founded by Kiel",
            "Hyre, an inventor that became",
            "rich from his strange machines.",
            "He decided to give back to society",
            "by building this private academy."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hanie",
        args![
            "However, the school will",
            "only accept orphans that are",
            "too poor to enroll in any other",
            "schools. Sometimes, it makes",
            "me wish that I was an orphan too!",
            "Well, not really, but you know..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hanie_kh1(ctx: &Ctx) -> Script {
    hanie_kh1_body(ctx, Vec::new()).map(|_| ())
}

fn security_guard_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_khdelivery_s = Val::from("");
    if (ctx.var("kielhyrequest").get()?.number()? < 2 || ctx.var("kielhyrequest").get()?.number()? > 31) {
        ctx.lines_as(
            "Security Guard",
            args![
                "I'm sorry, but if you aren't",
                "associated with this institution,",
                "then you're not authorized to",
                "enter the ^FF0000Kiel Hyre Academy^000000.",
                "Please leave if you don't have",
                "an appointment with the staff."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 2 {
        ctx.lines_as(
            "Security Guard",
            args![
                "I'm sorry, but if you aren't",
                "associated with this institution,",
                "then you're not authorized to",
                "enter the ^FF0000Kiel Hyre Academy^000000.",
                "Please leave if you don't have",
                "an appointment with the staff."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'm here for a delivery.:......")])? {
            1 => {
                ctx.lines_as(
                    "Security Guard",
                    args![
                        "You're here to deliver",
                        "something? Okay, just give",
                        "me the recipient's ^FF0000name^000000,",
                        "followed by the ^FF0000item^000000 being",
                        "delivered, and I'll verify it",
                        "before letting you inside."
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khdelivery_s = input;
                if l_khdelivery_s.clone() != "Lecollane" {
                    ctx.lines_as(
                        "Security Guard",
                        args![
                            "You're here to deliver some",
                            "Wine to...to who? What was",
                            "the name? I...I don't think",
                            "we have anybody in the",
                            ((Val::from("academy named ") + l_khdelivery_s.clone()) + Val::from(""))
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_khdelivery_s = input;
                if l_khdelivery_s.clone() != "Culinary Wine" {
                    ctx.lines_as(
                        "Security Guard",
                        args![
                            "So you're here to make",
                            "a delivery to Mrs. Lecollane?",
                            "What is it you've brought for",
                            ((Val::from("her? Some ^3355FF") + l_khdelivery_s.clone()) + Val::from("^000000?"))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Security Guard",
                        args![
                            "Let me buzz her first,",
                            "and check to make sure",
                            "that she's been expecting",
                            "you. Let's see now..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["......", ".........", "............"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Security Guard",
                        args![
                            "Huh. Mrs Lecollane",
                            "is expecting a delivery,",
                            "but not the item that you",
                            "say that you've brought for",
                            "her. You might want to check",
                            "to see if there's been a mixup..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Security Guard",
                    args![
                        "Alright....",
                        "So you're here to",
                        "deliver a bottle of",
                        "Wine to Mrs. Lecollane?",
                        "Let me buzz her, and get",
                        "this confirmed real quickly."
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args!["......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Security Guard",
                    args![
                        "Okay, everything looks",
                        "good. Mrs. Lecollane",
                        "has been expecting you.",
                        "I guess you can enter."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(71), Val::from(155)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Security Guard",
                    args![
                        "If you don't have",
                        "an appoointment, then",
                        "don't loiter around in",
                        "front of the academy!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("kielhyrequest").get()?.number()? < 32 {
        ctx.lines_as(
            "Security Guard",
            args![
                "Oh, did you have",
                "other business inside",
                "the academy? I remember",
                "you from before, so there",
                "shouldn't be any problems",
                "letting you back inside..."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(71), Val::from(155)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn security_guard_1(ctx: &Ctx) -> Script {
    security_guard_1_body(ctx, Vec::new()).map(|_| ())
}

fn student_kha_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Laci",
        args![
            "Oh, wow! We usually",
            "don't get visitors on",
            "campus! Um, you're not",
            "a new faculty member, are you?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn student_kha(ctx: &Ctx) -> Script {
    student_kha_body(ctx, Vec::new()).map(|_| ())
}

fn student_kha_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Laci",
        args![
            "Hey, Nesha...!",
            "have you heard why",
            "Aaci hasn't been coming",
            "to class for awhile?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Gui Nesha",
        args!["That's right, I haven't", "seen Aaci in awhile.", "Did something happen?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Laci",
        args![
            "It's because....",
            "Aaci saw a freakin' ghost!",
            "It's one hundred percent",
            "true! I heard about it from,",
            "well, you know, my sources."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Gui Nesha", args!["...Wha...?", "Oh, come on,", "get outta town."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn student_kha_ontouch(ctx: &Ctx) -> Script {
    student_kha_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn student_khb_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Gui Nesha",
        args![
            "Aren't we a little",
            "old to be talking about",
            "this kind of stuff? You",
            "know, rumors and ghost",
            "stories that make no sense?",
            "C'mon, Laci, knock it off!"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn student_khb(ctx: &Ctx) -> Script {
    student_khb_body(ctx, Vec::new()).map(|_| ())
}

fn lady_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 2 {
        ctx.lines_as(
            "Mrs. Lecollane",
            args!["Is there a problem?", "Outsiders are not allowed to come", "in here, please leave."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 2 {
        ctx.lines_as(
            "Mrs. Lecollane",
            args![
                "Oh, hello. Ah!",
                "have you come to",
                "deliver my wine? The",
                "security guard called and",
                "mentioned you were coming."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes, that's right!:Er, s-sorry!")])? {
            1 => {
                ctx.lines_as(
                    "Mrs. Lecollane",
                    args![
                        "Well, you've come a little",
                        "later than I thought, but",
                        "I suppose it can't be helped.",
                        "I guess the waiting has just",
                        "heightened my anticipation",
                        "for this bottle of wi--"
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^3355FF*Cling! Crrrack!*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Mrs. Lecollane",
                    args![
                        "Elly? Elly did you",
                        "break something again?!",
                        "You've got to be more careful!",
                        "If you don't finish baking those",
                        "cookies by the end of today, your",
                        "semester grades will suffer!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elly", args!["B-but I just..."])?;
                ctx.next()?;
                ctx.lines_as("Mrs. Lecollane", args!["^FF0000Elly^000000!!"])?;
                ctx.next()?;
                ctx.lines_as("Elly", args!["......", "Yes, Mrs. Lecollane."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mrs. Lecollane",
                    args![
                        "*Ahem* Excuse me.",
                        "Would you please leave",
                        "the wine over there? You",
                        "may go now, and please",
                        "don't wander needlessly",
                        "around the academy."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7487), Val::from(1)])?;
                ctx.var("kielhyrequest").set(Val::from(4))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Mrs. Lecollane",
                    args![
                        "Hm...?",
                        "I guess I must",
                        "be mistaken. I'm",
                        "sorry, I thought you",
                        "were somebody else."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("kielhyrequest").get()? == 4 {
        ctx.lines_as(
            "Mrs Lecollane",
            args![
                "Well, our business is",
                "completed, so would you",
                "please leave the campus",
                "as soon as you can? *Sigh*",
                "I can't believe our future",
                "is in these girls' hands..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn lady_kh(ctx: &Ctx) -> Script {
    lady_kh_body(ctx, Vec::new()).map(|_| ())
}

fn cute_student_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Elly",
            args![
                "I'm sorry, but would you",
                "put some of your things in",
                "your Kafra Storage first or",
                "something? I can't really",
                "help you when you're ",
                "carrying so much stuff."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("kh_elly01"), Val::from(2)])?;
    if ctx.var("kielhyrequest").get()?.number()? < 4 {
        ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
        ctx.lines_as(
            "Elly",
            args![
                "W-wah! Oh...!",
                "^333333*Phew*^000000 That was close,",
                "I almost dropped them ",
                "again! Why do I have so much",
                "trouble handling ingredients?"
            ],
        )?;
    } else {
        if ctx.var("kielhyrequest").get()? == 4 {
            ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
            ctx.lines_as(
                "Elly",
                args![
                    "Oh no, what should",
                    "I do? ^333333*Sob*^000000 Wh-what",
                    "am I going to do? ^333333*Sniff*^000000"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("What happened?:......")])? {
                1 => {
                    ctx.lines_as(
                        "Elly",
                        args![
                            "I... I have to finish baking",
                            "this batch of cookies by the",
                            "end of today, but then I spilled",
                            "all of the ingredients on the",
                            "floor. I don't know how I can",
                            "bake those cookies now..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("What can I do to help you?:Oh, I'm so sorry.")])? {
                        1 => {
                            ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                            ctx.lines_as(
                                "Elly",
                                args![
                                    "What was that...?",
                                    "You'll really help",
                                    "me? That's wonderful!",
                                    "Thank you! Thanks so much!"
                                ],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Elly",
                                args![
                                    "Wait, you don't go",
                                    "to this school, don't",
                                    "you? Y-you're one of",
                                    "those adventurers, right?"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Yeah, that's right.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines(args![
                                "Listen, I know you",
                                "probably have your own",
                                "plans, but do  you think",
                                "you can help me with this",
                                "huge problem that I have?"
                            ])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Accept:Decline")])? {
                                1 => {
                                    ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                                    ctx.lines(args![
                                        "How Wonderful!",
                                        "Thank you! Thank you",
                                        "so much! I'm supposed to",
                                        "finish baking this batch of",
                                        "cookies soon, but I spilled",
                                        "all of the ingredients..."
                                    ])?;
                                    ctx.next()?;
                                }
                                2 => {
                                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["I'm outta here."])?;
                                    ctx.next()?;
                                    ctx.call(Function::Cutin, vec![Val::from("kh_elly04"), Val::from(2)])?;
                                    ctx.lines_as(
                                        "Elly",
                                        args!["W-wait...!", "Come back, you", "d-don't...! I really", "need some help!"],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                    ctx.close_window()?;
                                    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Well, I guess the only way",
                            "I can bake these cookies is",
                            "to get some new ingredients.",
                            "I'm sorry to be such a burden,",
                            "but if you didn't offer to help",
                            "me, then I'd have no one to ask!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly04"), Val::from(2)])?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "You don't understand how",
                            "important it is that I bake",
                            "these cookies... If I don't",
                            "finish this assignment, then",
                            "Mrs. Crank will fail me for",
                            "the entire semester!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Elly]")?;
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly01"), Val::from(2)])?;
                    ctx.lines(args![
                        "Okay, I need to calm",
                        "down. I think I can do",
                        "this with your help.",
                        "Would you please do me",
                        "this huge favor and bring",
                        "all of these ingredients?"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "^3355FF1 Wine^000000,",
                            "^3355FF7 Milks^000000,",
                            "^3355FF5 Cacaos^000000,",
                            "^3355FF2 Cheeses^000000,",
                            "^3355FF1 Egg^000000, and",
                            "^3355FF1 Bag of Flour^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Let's see... You can",
                            "get Wine from a pub in",
                            "Juno, Cacaos from hunting",
                            "Yoyos, and you can get flour",
                            "and Eggs from the girl that",
                            "runs the Toast Stand in Juno."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Good luck, getting",
                            "everything, and I hope",
                            "you hurry back here with",
                            "those cookie ingredients",
                            "as quickly as you can, okay?"
                        ],
                    )?;
                    ctx.var("kielhyrequest").set(Val::from(6))?;
                }
                2 => {
                    ctx.lines_as("Elly", args!["*Sob*..."])?;
                }
                _ => {}
            }
        } else {
            if ctx.var("kielhyrequest").get()? == 6 {
                if ((((ctx.call(Function::CountItem, vec![Val::from(519)])?.number()? < 7
                    || ctx.call(Function::CountItem, vec![Val::from(548)])?.number()? < 2)
                    || ctx.call(Function::CountItem, vec![Val::from(7182)])?.number()? < 5)
                    || ctx.call(Function::CountItem, vec![Val::from(7487)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(7488)])?.number()? < 1)
                {
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Let's see, would you",
                            "like me to remind you",
                            "which ingredients I need?",
                            "Please bring these things as",
                            "soon as you can so that I can",
                            "quickly bake some cookies~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "^3355FF1 Wine^000000,",
                            "^3355FF7 Milks^000000,",
                            "^3355FF5 Cacaos^000000,",
                            "^3355FF2 Cheeses^000000,",
                            "^3355FF1 Egg^000000, and",
                            "^3355FF1 Bag of Flour^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Let's see... You can",
                            "get Wine from a pub in",
                            "Juno, Cacaos from hunting",
                            "Yoyos, and you can get flour",
                            "and Eggs from the girl that",
                            "runs the Toast Stand in Juno."
                        ],
                    )?;
                } else {
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Hooray! Finally, I have",
                            "everything I need! This is",
                            "great! Oh, would you please",
                            "give me a moment while I bake",
                            "these cookies?  It shouldn't take",
                            "long, so hold on just a bit."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(519), Val::from(7)])?;
                    ctx.call(Function::DelItem, vec![Val::from(548), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7182), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7487), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7488), Val::from(1)])?;
                    ctx.var("kielhyrequest").set(Val::from(8))?;
                    ctx.var("khpubmasterend").set(Val::from(0))?;
                    ctx.var("khtoastgirlend").set(Val::from(0))?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                }
            } else {
                if ctx.var("kielhyrequest").get()? == 8 {
                    ctx.lines_as(
                        "Elly",
                        args![
                            "Tadah! I did it!",
                            "They're finally done!",
                            "Elly's special cookies.",
                            "I'd like you to have some",
                            "as thanks for helping me out."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            "I'm sorry, but I was so",
                            "preoccupied with baking",
                            "these cookies that I didn't",
                            "even ask for your name. I'm",
                            "^FF0000Ellyja^000000, but everybody calls me,",
                            "''Elly.'' What's your name?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            ((Val::from("^3355FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000.")),
                            ((Val::from("It's ^3355FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000."))
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                    ctx.lines_as(
                        "Elly",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                            "...that name! It's wonderful!"
                        ],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(538), Val::from(5)])?;
                    ctx.var("kielhyrequest").set(Val::from(10))?;
                } else {
                    if ctx.var("kielhyrequest").get()? == 10 {
                        ctx.lines_as(
                            "Elly",
                            args![
                                "I'm sorry, but I have",
                                "another favor to ask you",
                                "if you don't mind helping",
                                "me out again. Don't worry,",
                                "you won't have to hurry as",
                                "quickly as you did last time."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("What do you need?:Sorry, but I'm pretty busy...")])? {
                            1 => {
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "You know how you helped",
                                        "me bake those cookies?",
                                        "It's the first time I was able",
                                        "to do it without burning them!",
                                        "I just know Grandfather'd",
                                        "be so proud of me!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "I really want Grandfather",
                                        "to taste the cookies I baked,",
                                        "but the cookies will be stale",
                                        "by the time I'm able to leave",
                                        "campus. Would you deliver these",
                                        "cookies to my grandfather for me?"
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Sure:I'm busy.")])? {
                                    1 => {
                                        ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("!")),
                                                "Thank you so much, I knew",
                                                "you'd understand! Would you",
                                                "please bring the cookies to",
                                                "him at ^FF0000Kiel Hyre's cottage^000000?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Oh, right! They're really",
                                                "careful about visitors and",
                                                "keeping strangers off the",
                                                "property, but if you mention",
                                                "my name, they'll let you in.",
                                                "Alright then, see you~"
                                            ],
                                        )?;
                                        ctx.var("kielhyrequest").set(Val::from(12))?;
                                    }
                                    2 => {
                                        ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
                                        ctx.lines_as("Elly", args!["Oh, um...", "Okay, I'm sorry to", "have bothered you..."])?;
                                    }
                                    _ => {}
                                }
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
                                ctx.lines_as("Elly", args!["Oh, um...", "Okay, I'm sorry to", "have bothered you..."])?;
                            }
                            _ => {}
                        }
                    } else {
                        if ctx.var("kielhyrequest").get()? == 12 {
                            ctx.lines_as(
                                "Elly",
                                args![
                                    "Oh, you don't know where",
                                    "to find Kiel Hyre's cottage?",
                                    "It's just north from this",
                                    "academy. Please deliver my",
                                    "cookies to Grandfather, and",
                                    "let him know I really miss him."
                                ],
                            )?;
                        } else {
                            if ctx.var("kielhyrequest").get()? == 14 {
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Hm? Grandfather's not",
                                        "home? That's strange, he",
                                        "didn't mention anything about",
                                        "any business trips. I thought",
                                        "he'd be at home all day..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly04"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Would you go back to his",
                                        "cottage one more time? Here,",
                                        "you can use this Cottage Key.",
                                        "That way, you can just go",
                                        "inside and talk to him~"
                                    ],
                                )?;
                                ctx.call(Function::GetItem, vec![Val::from(7489), Val::from(1)])?;
                                ctx.var("kielhyrequest").set(Val::from(16))?;
                            } else if (ctx.var("kielhyrequest").get()?.number()? >= 14 && ctx.var("kielhyrequest").get()?.number()? < 20) {
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "That's so weird...",
                                        "I thought Grandfather",
                                        "said that he'd be at",
                                        "home all day today..."
                                    ],
                                )?;
                            } else if ctx.var("kielhyrequest").get()? == 20 {
                                ctx.lines_as("Elly", args!["Hmm...?", "What's this,", "a letter for me?"])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^3355FFElly opened the envelope",
                                    "and started reading the letter.^000000"
                                ])?;
                                ctx.next()?;
                                ctx.lines(args![
                                    "^333333Dearest Elly,",
                                    " ",
                                    "I have something to discuss",
                                    "with my son Kiehl, so I am",
                                    "leaving to meet with him.",
                                    "If you don't hear from me",
                                    "after 7 days after I've written",
                                    "this letter, then you must",
                                    "escape the academy as soon",
                                    "as possible, and retrieve",
                                    "something inside our ",
                                    "cottage's study.",
                                    " ",
                                    "If you have a friend you",
                                    "can trust, please ask him",
                                    "to follow my traces in the",
                                    "cottage. I might be in danger,",
                                    "and in dire need of rescue.",
                                    " ",
                                    "Elly, don't trust anyone",
                                    "in the academy, even your",
                                    "classmates, since they may",
                                    "be influenced by Kiehl.",
                                    " ",
                                    "Be careful, and I love you.",
                                    " ",
                                    "--Grandpa^000000"
                                ])?;
                                ctx.next()?;
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "What? Oh no, it's been",
                                        "ten days since he wrote",
                                        "this letter! Ah, does this",
                                        "mean that he's in danger?!",
                                        "Oh no, what should I do?"
                                    ],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(7490), Val::from(1)])?;
                                ctx.var("kielhyrequest").set(Val::from(22))?;
                            } else if ctx.var("kielhyrequest").get()? == 22 {
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly03"), Val::from(2)])?;
                                ctx.lines_as("Elly", args!["......", ".........", "............"])?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("About your grandpa:Tell me about Kiehl:What's with this academy?")],
                                )? {
                                    1 => {
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Oh! My grandfather is",
                                                "Kiel Hyre, founder and",
                                                "CEO of the Kiel Hyre",
                                                "foundation. He looks",
                                                "strict and cold hearted,",
                                                "but he's actually very nice!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "We're not related by",
                                                "blood, but he's taken",
                                                "care of me ever since",
                                                "I lost my parents. Oh,",
                                                "Grandapa, where are you?",
                                                "I'm getting so worried!"
                                            ],
                                        )?;
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Kiehl? That's my",
                                                "grandfather's son...",
                                                "I don't know him that",
                                                "well, and only saw him",
                                                "once at an academy event."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Mmm, he's a good looking",
                                                "guy with pale skin, silver",
                                                "hair, and this cold, fierce",
                                                "stare. A lot of my classmates",
                                                "worship Kiehl because he's",
                                                "also a business genius~"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "It's weird though...",
                                                "I have no idea why my",
                                                "grandpa and Kiehl don't",
                                                "get along. They're both",
                                                "really good at what they do..."
                                            ],
                                        )?;
                                    }
                                    3 => {
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Well, I know the security",
                                                "here is really tight, but",
                                                "that's because everything",
                                                "here is so luxurious and",
                                                "expensive, you know~"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "We also have a very",
                                                "special curriculum where",
                                                "you learn more of what you",
                                                "want. Personally, I want to",
                                                "become a great career woman",
                                                "like ^0000FFMs. Allysia^000000. Heh heh~"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "This place certainly",
                                                "isn't like other schools.",
                                                "Yeah, everything is made to",
                                                "fit each of the student's needs."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "Recently, though?",
                                                "Some really weird stuff",
                                                "has been happening. All of",
                                                "my classmates are afraid of",
                                                "going out alone by themselves."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                                        ctx.lines_as(
                                            "Elly",
                                            args![
                                                "But I'll be okay!",
                                                "You'll be there to",
                                                "rescue me from danger,",
                                                ((Val::from("right, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("?"))
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Who is Ms. Allysia?:Strange incidents?")])? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Elly",
                                                    args![
                                                        "Oh, Ms. Allysia is",
                                                        "Grandfather's secretary~",
                                                        "She's so beautiful, and",
                                                        "my grandfather really",
                                                        "trusts her with everything!"
                                                    ],
                                                )?;
                                            }
                                            2 => {
                                                ctx.call(Function::Cutin, vec![Val::from("kh_elly04"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Elly",
                                                    args![
                                                        ((Val::from("Well, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                            + Val::from("...")),
                                                        "I'm not supposed to tell",
                                                        "anyone outside of the school,",
                                                        "but I can trust you! You see...",
                                                        "We're haunted by a ghost!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Elly",
                                                    args![
                                                        "I know it sounds crazy,",
                                                        "but this ghost wanders the",
                                                        "campus, and curses its victims,",
                                                        "making them so cold and lifeless.",
                                                        "It happened to my roommate,",
                                                        "Mayo. It's like she's a statue..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Elly",
                                                    args![
                                                        "I snuck into the medical",
                                                        "office to see her, and she...",
                                                        "She couldn't do anything!",
                                                        "What would happen if the",
                                                        "ghost decided to curse me?!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.call(Function::Cutin, vec![Val::from("kh_elly02"), Val::from(2)])?;
                                                ctx.lines_as(
                                                    "Elly",
                                                    args![
                                                        "You know what...?",
                                                        "If I ever got cursed",
                                                        "by that ghost, just yell",
                                                        "''^FF0000Wake up, Elly!^000000''",
                                                        "That'll wake me up for sure!"
                                                    ],
                                                )?;
                                                ctx.var("kielhyrequest").set(Val::from(24))?;
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            } else if (ctx.var("kielhyrequest").get()?.number()? >= 24 && ctx.var("kielhyrequest").get()?.number()? <= 26) {
                                ctx.call(Function::Cutin, vec![Val::from("kh_elly04"), Val::from(2)])?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Argh, I'm in a fix!",
                                        "I have to finish my",
                                        "homework before it's due!",
                                        "Yeah, I've got to go see",
                                        "Mrs. Lecollane now."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        ((Val::from("Say, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                        "if it's okay, would you",
                                        "please go find what my",
                                        "grandpa left for me in",
                                        "the ^FF0000cottage study^000000? You're",
                                        "the only one I can trust!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "Anyway, I'll see you later",
                                        "in the evening! Please come",
                                        "by my ^FF0000dorm room^000000. Um, the",
                                        "dorms are in the church just",
                                        "behind the academy, okay?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elly",
                                    args![
                                        "My room is in the back,",
                                        "and it's on the ^FF0000second floor^000000",
                                        "^FF0000on the left side^000000. Please use",
                                        "a ladder to come up, okay?",
                                        "I'll leave my window open",
                                        "for you, so just come, okay?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elly", args!["Hurry, hurry, the", "teacher's coming!"])?;
                                ctx.var("kielhyrequest").set(Val::from(26))?;
                            } else {
                                ctx.lines_as("Elly", args!["Hurry, hurry, the", "teacher's coming!"])?;
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.close_window()?;
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn cute_student_kh(ctx: &Ctx) -> Script {
    cute_student_kh_body(ctx, Vec::new()).map(|_| ())
}

fn window_kh_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("kielhyrequest").get()?.number()? < 29 {
        ctx.lines(args![
            "^3355FFYou can see a curtain",
            "decorated with a cute",
            "design through the",
            "window of this room,",
            "which is probably",
            "used by a young girl.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("kielhyrequest").get()? == 29 {
        ctx.lines(args![
            "^3355FFThis must be Elly's room.",
            "It doesn't sound like anyone",
            "is inside, so she probably",
            "isn't back yet. For now, you",
            "should try to find what her",
            "grandfather left for her",
            "in their cottage's Study.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("kielhyrequest").get()?.number()? >= 30 && ctx.var("kielhyrequest").get()?.number()? <= 45) {
        ctx.lines(args![
            "^3355FFThis slightly open window",
            "must lead into Elly's room.",
            "Although she asked you to",
            "find a ladder to enter her",
            "window, you probably won't",
            "find one. You might be able",
            "to climb up that water pipe...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Climb Water Pipe:Find Another Way")])? {
            1 => {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                    ctx.lines(args![
                        "^3355FFYou climbed up the",
                        "water pipe, and sneaked",
                        "into Elly's room successfully.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("kh_school"), Val::from(185), Val::from(185)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "^3355FFYou tried to climb",
                        "the water pipe, but",
                        "you ended up falling",
                        "and bumping your head.^000000"
                    ])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines(args![
                    "^3355FFThere must be some",
                    "other way to get into",
                    "Elly's room, aside from",
                    "climbing up this water pipe...^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args!["^3355FFYou can't go up into", "that open window.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn window_kh(ctx: &Ctx) -> Script {
    window_kh_body(ctx, Vec::new()).map(|_| ())
}
