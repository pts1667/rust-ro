use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Guildsman1Step {
    Start,
    OnTouch,
}

fn guildsman_1_run(ctx: &Ctx, mut step: Guildsman1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guildsman1Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
                    ctx.lines(args![
                        "^3355FFWait a second!",
                        "Right now, you're carrying",
                        "too many things with you.",
                        "Please come back after",
                        "using the Kafra Service",
                        "to store some of your items.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if (!(ctx.var("BaseLevel").get()?.number()? > 59)
                    || !((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                        || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)))
                {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "There are rumors",
                            "going around about",
                            "the disappearances of",
                            "children in the Morocc",
                            "area. Some guilds believe",
                            "these are actually kidnappings."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Those poor kids...",
                            "If I could do something",
                            "to help them, by George,",
                            "I would do it in a flash."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("mao_request").get()?.is_true()) {
                    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Excuse me? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "Hi there, I've been waiting for",
                                "you to wander past me for such",
                                "a long time, you know that?",
                                "Novices and Super Novices",
                                "are so hard to track down..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, I don't know if you",
                                "know this, but a lot of kids are missing from Morocc recently.",
                                "We're not sure, but we think it's related to the latest assignment",
                                "for our Assassin Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, the client for this",
                                "assignment needs all the help",
                                "he can get. Now, I've heard",
                                "about you, and I think that",
                                "you could be really helpful to",
                                "us in this specific situation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Look, just do me a favor",
                                "and do the right thing like",
                                "you always do, okay? Here's",
                                "a letter of recommendation",
                                "to get you started on this",
                                "mission, alright?"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN_HIGH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "Great, I knew you'd",
                                "show up sooner or later.",
                                "Listen, I'm working here",
                                "as a representative of the",
                                "Swordman Assocation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Listen, there's something",
                                "big going on, and I think it's",
                                "the most important thing the",
                                "Swordman Association has ever",
                                "been involved in. Pack your bags and head to Morocc right now!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["All the way to Morocc...?", "Why, what's going on?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I don't know all the details,",
                                "but the Assassin Guild is",
                                "working on some missing",
                                "children's case, and they've",
                                "requested help from us and",
                                "all the other guilds..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Rogues, Bards, Novices,",
                                "Super Novices, Priests...",
                                "You name it. This is gonna be",
                                "huge. Listen, if you're going",
                                "to help, then let me give you this letter of recommendation..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_LORD_KNIGHT")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "I knew you'd show up if",
                                "Listen, I have a notice for",
                                "you from the Prontera Chivalry."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["For me? That sounds", "strange, but would you", "please read it to me?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("''RE: ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                                "As leader of the Prontera",
                                "Chivalry, I formally request",
                                "you to represent the Knights of",
                                "Rune-Midgarts in cooperation",
                                "with the Assassin Guild.''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Please assist the Assassin",
                                "Guild in any way befitting of",
                                "the Knighthood in a special",
                                "mission to rescue children",
                                "missing from Morocc.",
                                "-- Captain Herman''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "There, that's all it says.",
                                "Here, I think you'll need",
                                "this letter of recommendation",
                                "if you plan to follow these",
                                "orders. I hope you take that",
                                "mission for those kids' sake..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_PALADIN")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Excuse me...?",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "I'm sorry to bother you, but",
                                "I've got an urgent communique",
                                "for you from Sir Michael Halig",
                                "of the Crusaders..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "They usually don't",
                                "send messages. I guess",
                                "whatever he has to say",
                                "must be really important.",
                                "What does it say?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Let's see here... Something",
                                "about a missing children's case",
                                "the Assassin Guild is working on... Ah! The Assassin Guild has",
                                "requested help from the Crusaders. So I guess you were recommended."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Look, here's the letter of",
                                "recommendation that I'm",
                                "supposed to give you if you",
                                "plan on taking the mission.",
                                "For the sake of those missing kids, I really hope that you do."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_request").get()? == 1 {
                        if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Ah, I forgot to tell you",
                                    "exactly where you need to",
                                    "go for the mission details!",
                                    "Let's see... You're supposed",
                                    "to... Ah, now I remember~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "If you check out the",
                                    "west side of the Oasis",
                                    "inside Morocc, you'll find",
                                    "a very suspicious looking",
                                    "hut. Your contact from the",
                                    "Assassin Guild is near there."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "I'm sure you'll find it if",
                                    "you keep your eyes open.",
                                    "Anyway, that's all I know.",
                                    "Why don't you go check it out?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN_HIGH")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Right, for this mission,",
                                    "you'll need to meet with your",
                                    "contact from the Assassin",
                                    "Guild near a hut at the west",
                                    "side of the Oasis inside Morocc. So keep an eye out for him."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "I doubt you'll have",
                                    "trouble finding the guy,",
                                    "even if all Thieves and",
                                    "Assassins are starting",
                                    "to look the same. You",
                                    "know what I mean, right?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_LORD_KNIGHT")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_PALADIN")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Are you thinking of taking",
                                    "the mission? That's great!",
                                    "Now, you need to meet your",
                                    "contact from the Assassin",
                                    "Guild outside a hut on the west side of the Oasis inside Morocc."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "I know those directions",
                                    "aren't very clear, but this",
                                    "is supposedly a top secret",
                                    "location that's usually only",
                                    "known to the Assassins..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
                            || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "It looks like you're",
                                    "working well with the",
                                    "Assassin Guild. Still,",
                                    "be careful. There might",
                                    "be more to this mission",
                                    "than meets the eye, you know?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "It's a nice day, isn't it?",
                                    "Though, I hope something",
                                    "exciting happens soon. Peace",
                                    "is great and all, but I prefer",
                                    "to have my life shook up",
                                    "every now and then."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                step = Guildsman1Step::OnTouch;
                continue 'machine;
            }
            Guildsman1Step::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
                    ctx.lines(args![
                        "^3355FFWait a second!",
                        "Right now, you're carrying",
                        "too many things with you.",
                        "Please come back after",
                        "using the Kafra Service",
                        "to store some of your items.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseLevel").get()?.number()? > 59 && !(ctx.var("mao_request").get()?.is_true()) {
                    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Excuse me? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "Hi there, I've been waiting for",
                                "you to wander past me for such",
                                "a long time, you know that?",
                                "Novices and Super Novices",
                                "are so hard to track down..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, I don't know if you",
                                "know this, but a lot of kids are missing from Morocc recently.",
                                "We're not sure, but we think it's related to the latest assignment",
                                "for our Assassin Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Anyway, the client for this",
                                "assignment needs all the help",
                                "he can get. Now, I've heard",
                                "about you, and I think that",
                                "you could be really helpful to",
                                "us in this specific situation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Look, just do me a favor",
                                "and do the right thing like",
                                "you always do, okay? Here's",
                                "a letter of recommendation",
                                "to get you started on this",
                                "mission, alright?"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SWORDMAN_HIGH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                                "Great, I knew you'd",
                                "show up sooner or later.",
                                "Listen, I'm working here",
                                "as a representative of the",
                                "Swordman Assocation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Listen, there's something",
                                "big going on, and I think it's",
                                "the most important thing the",
                                "Swordman Association has ever",
                                "been involved in. Pack your bags and head to Morocc right now!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["All the way to Morocc...?", "Why, what's going on?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I don't know all the details,",
                                "but the Assassin Guild is",
                                "working on some missing",
                                "children's case, and they've",
                                "requested help from us and",
                                "all the other guilds..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Rogues, Bards, Novices,",
                                "Super Novices, Priests...",
                                "You name it. This is gonna be",
                                "huge. Listen, if you're going",
                                "to help, then let me give you this letter of recommendation..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_LORD_KNIGHT")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "I knew you'd show up if",
                                "Listen, I have a notice for",
                                "you from the Prontera Chivalry."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["For me? That sounds", "strange, but would you", "please read it to me?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("''RE: ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                                "As leader of the Prontera",
                                "Chivalry, I formally request",
                                "you to represent the Knights of",
                                "Rune-Midgarts in cooperation",
                                "with the Assassin Guild.''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Please assist the Assassin",
                                "Guild in any way befitting of",
                                "the Knighthood in a special",
                                "mission to rescue children",
                                "missing from Morocc.",
                                "-- Captain Herman''"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "There, that's all it says.",
                                "Here, I think you'll need",
                                "this letter of recommendation",
                                "if you plan to follow these",
                                "orders. I hope you take that",
                                "mission for those kids' sake..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_PALADIN")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Excuse me...?",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...?")),
                                "I'm sorry to bother you, but",
                                "I've got an urgent communique",
                                "for you from Sir Michael Halig",
                                "of the Crusaders..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "They usually don't",
                                "send messages. I guess",
                                "whatever he has to say",
                                "must be really important.",
                                "What does it say?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Let's see here... Something",
                                "about a missing children's case",
                                "the Assassin Guild is working on... Ah! The Assassin Guild has",
                                "requested help from the Crusaders. So I guess you were recommended."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Look, here's the letter of",
                                "recommendation that I'm",
                                "supposed to give you if you",
                                "plan on taking the mission.",
                                "For the sake of those missing kids, I really hope that you do."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guildsman_1(ctx: &Ctx) -> Script {
    guildsman_1_run(ctx, Guildsman1Step::Start, Vec::new()).map(|_| ())
}

pub fn guildsman_1_ontouch(ctx: &Ctx) -> Script {
    guildsman_1_run(ctx, Guildsman1Step::OnTouch, Vec::new()).map(|_| ())
}

fn nun_moc_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.var("BaseLevel").get()?.number()? > 59) {
        ctx.lines_as(
            "Nun",
            args![
                "Oh, hello there~",
                "Peace be with you,",
                "adventurer. Remember",
                "that no matter how different",
                "people may be, all of us are",
                "united in our humanity."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)) {
        ctx.lines_as(
            "Nun",
            args![
                "Ah, hello. Would you ask",
                "anyone that you know",
                "to be affiliated with the",
                "Prontera Church to talk to",
                "me? Acolytes, Priests, Monks...",
                "Any of those would be fine."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "I have something very",
                "important to tell them,",
                "but it's almost impossible",
                "to gather all the clergy",
                "in an emergency situation..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.var("mao_request").get()?.is_true()) {
        ctx.lines_as(
            "Nun",
            args![
                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                "It must be an act of",
                "divine providence that",
                "we finally meet. Actually,",
                "I have something very",
                "important to ask you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "You may have heard that",
                "there's been a recent influx",
                "of missing child reports from",
                "Morocc. Apparently, someone",
                "has hired the Assassin Guild",
                "to investigate these cases."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Since this is such a huge",
                "issue, the Assassin Guild has",
                "even gone so far as to request",
                "the Prontera Church for help, so I'd like you to aid the Assassin",
                "Guild in this mission."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "However, I believe that",
                "the Assassin Guild's client",
                "may have other intentions,",
                "so I also want you to keep",
                "an eye out and see if their",
                "client can be trusted."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Please visit Morocc",
                "as soon as you can.",
                "When you arrive, you",
                "will need the Bishop's",
                "letter of recommendation,",
                "so please take it now."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(1))?;
        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 1 {
        ctx.lines_as(
            "Nun",
            args![
                "Now, for this mission, you",
                "will need to meet your contact",
                "from the Assassin Guild near",
                "a hut on the west side of the",
                "Oasis inside Morocc. I wonder",
                "why they chose that location?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Normally, a representative",
                "of the Prontera Church would",
                "simply visit the Assassin Guild itself. Be careful. The Assassins",
                "must be being extra secret because of extraordinary circumstances..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
        || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
    {
        ctx.lines_as(
            "Nun",
            args![
                "It pleases me to see that",
                "you're working well with the",
                "Assassin Guild. They operate",
                "on a different methodology",
                "than the Prontera Church, but",
                "I still greatly respect them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Remember that you're",
                "representing the Prontera",
                "Church in this effort, so be",
                "sure to demonstrate your",
                "best for the Assassins, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Nun",
            args![
                "Although it is a time",
                "of peace, I can't help",
                "but feel this lingering",
                "anxiety. It's almost as if",
                "some monumental event",
                "is just over the horizon..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn nun_moc(ctx: &Ctx) -> Script {
    nun_moc_body(ctx, Vec::new()).map(|_| ())
}

fn nun_moc_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? > 59
        && !(ctx.var("mao_request").get()?.is_true())
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
    {
        ctx.lines_as(
            "Nun",
            args![
                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                "It must be an act of",
                "divine providence that",
                "we finally meet. Actually,",
                "I have something very",
                "important to ask you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "You may have heard that",
                "there's been a recent influx",
                "of missing child reports from",
                "Morocc. Apparently, someone",
                "has hired the Assassin Guild",
                "to investigate these cases."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Since this is such a huge",
                "issue, the Assassin Guild has",
                "even gone so far as to request",
                "the Prontera Church for help, so I'd like you to aid the Assassin",
                "Guild in this mission."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "However, I believe that",
                "the Assassin Guild's client",
                "may have other intentions,",
                "so I also want you to keep",
                "an eye out and see if their",
                "client can be trusted."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nun",
            args![
                "Please visit Morocc",
                "as soon as you can.",
                "When you arrive, you",
                "will need the Bishop's",
                "letter of recommendation,",
                "so please take it now."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(1))?;
        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn nun_moc_ontouch(ctx: &Ctx) -> Script {
    nun_moc_ontouch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guildsman2Step {
    Start,
    OnTouch,
}

fn guildsman_2_run(ctx: &Ctx, mut step: Guildsman2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guildsman2Step::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
                    ctx.lines(args![
                        "^3355FFWait a second!",
                        "Right now, you're carrying",
                        "too many things with you.",
                        "Please come back after",
                        "using the Kafra Service",
                        "to store some of your items.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("BaseLevel").get()?.number()? > 59) {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Say, have you heard",
                            "what's happening in",
                            "Morocc recently? There's",
                            "something major going on..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?)
                    || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?))
                {
                    ctx.lines_as(
                        "Guildsman",
                        args![
                            "Goodness, there's so",
                            "many Merchants here.",
                            "Wow... Hopefully I can",
                            "find who I'm looking for..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?) {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DHold it right there!",
                                "I've got a proposition",
                                "for you, so listen up!^000000",
                                " ",
                                "...N-no, you're doing it again!",
                                "Damn, g-get out of my head!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DShut up, this is important!^000000",
                                " ",
                                "No, you shut up, this is",
                                "my freaking body! Look,",
                                "just gimme a little control!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I... I can sense two",
                                "different souls within",
                                "your body. I don't mind",
                                "listening to what you have",
                                "to say. But if you both keep",
                                "talking, I'll get confused..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Fine, whatever.",
                                " ",
                                "^71637DAlright, so the Assassin",
                                "Guild is looking for outside",
                                "help in one of their missions.",
                                "Preeeeetty important stuff.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildman",
                            args![
                                "^71637DThey've been hired by some",
                                "client to... to... um, you know...^000000",
                                " ",
                                "...Investigate missing children? ",
                                "^71637DYeah, yeah. That.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildman",
                            args![
                                "^71637DAnyway, this is a pretty",
                                "big mission, so the Assassin",
                                "Guild has been soliciting for",
                                "help. I think you'd be perfect",
                                "to represent the Soul Linkers,",
                                "so I've decided to choose you.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DSo, go and help the Assassins!^000000",
                                " ",
                                "Wait, don't forget the",
                                "letter of recommendation!",
                                " ",
                                "^71637DOh, right! Here, take this!^000000"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Alright, I did what you wanted.",
                                "You can leave my body now...",
                                " ",
                                "^71637DEh, I'll think about it.^000000",
                                " ",
                                "NOOOOOOO! PLEEEEEASE~!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT_HIGH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Well, if it isn't",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". Hey,")),
                                "would you wait a minute?",
                                "I've got a message for you",
                                "from the Merchant Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I'm not sure why, but",
                                "the Assassin Guild has",
                                "been requesting help from",
                                "someone in the Merchant",
                                "Guild. Would you go help",
                                "them and represent us?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I think they're working for",
                                "a client, investigating these",
                                "children that are missing",
                                "from Morocc. If you want to",
                                "help them, you'll need this",
                                "letter of recommendation, okay?"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...!")),
                                "I'm so glad that I finally",
                                "ran into you. I know this is",
                                "sudden, but the Blacksmith",
                                "Guild has an assignment",
                                "for you over in Morocc."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "You see, the Assassin Guild",
                                "has formally requested for our",
                                "help in a mission regarding",
                                "children that have been missing",
                                "from Morocc recently. Here, take this letter of recommendation..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "If the Assassin Guild",
                                "is asking for help, I have",
                                "no doubt that this will be",
                                "a very difficult mission.",
                                "You should prepare yourself",
                                "if you plan to get involved..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CREATOR")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "I've got a message for",
                                "you from the Alchemist",
                                "Guild. Well, they're more",
                                "like orders than a message.",
                                "I'm so lucky to have found you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "The Assassin Guild has",
                                "officially asked our guild",
                                "for help in a mission regarding",
                                "children that have been missing",
                                "from Morocc, and they want us",
                                "to send somebody... you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Or... At least, you're",
                                "one of the people that the",
                                "Alchemist Guild wants to send",
                                "to represent us. So why don't",
                                "you go? You know, do it for the",
                                "children. Just think about it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Here, take this letter of",
                                "recommendation with you and",
                                "head over to Morocc as soon",
                                "as you can. If even the Assassin Guild needs help, I'm sure that",
                                "spells really big trouble..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("mao_request").get()? == 1 {
                        if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?) {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "^71637DGood, you came back! This",
                                    "guy kept distracting me from",
                                    "telling you something important. ",
                                    " ",
                                    "^000000What are you talking about?",
                                    "That's your own damn fault!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "^71637DYou know the oasis inside",
                                    "Morocc? To the west of that,",
                                    "you'll find your contact from",
                                    "the Assassin Guild.^000000",
                                    " ",
                                    "That... that's it?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "So now will you leave my",
                                    "body?! Having to share it",
                                    "with you is ruining my life!",
                                    " ",
                                    "^71637DRight. Okay. So where am",
                                    "I gonna go, huh? You need me!^000000"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                            || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT_HIGH")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "So you're gonna represent",
                                    "the Merchants and help out",
                                    "the Assassin Guild? Great!",
                                    "You can meet your contact",
                                    "on the west side of the oasis,",
                                    "next to a hut, inside Morocc."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Hey, good luck and",
                                    "be careful, okay?",
                                    "I think something",
                                    "major is behind the",
                                    "mission that they",
                                    "have for you..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "So have you decided to",
                                    "aid the Assassin Guild on",
                                    "behalf of the Blacksmiths?",
                                    "Then please meet your contact",
                                    "at the west side of the Oasis",
                                    "inside Morocc. Good luck~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CREATOR")?))
                        {
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Ah, I almost forgot to",
                                    "tell you that your contact",
                                    "from the Assassin Guild",
                                    "will be waiting for you",
                                    "at the west side of the",
                                    "Oasis inside Morocc."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guildsman",
                                args![
                                    "Please help the",
                                    "Assassin Guild as",
                                    "much as you can on",
                                    "behalf of the Alchemists,",
                                    "and watch out for trouble..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
                            || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
                        {
                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?) {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "^71637DBe careful in your mission...",
                                        "There's something major",
                                        "going on behind all of this,",
                                        "I just know it. Keep an eye out. ",
                                        "Yeah, uh, good luck!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT_HIGH")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "So you've been working",
                                        "well with the Assassins?",
                                        "That's good news. This is a",
                                        "great chance for us to show",
                                        "the strength of Merchants!",
                                        "Still, be on your guard."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "I trust that you've been of",
                                        "great help to the Assassins.",
                                        "Stay on your guard: it seems",
                                        "that there may be powerful",
                                        "influences behind all of this... "
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CREATOR")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "I'm glad to hear that",
                                        "you're getting along with",
                                        "the Assassins. Working ",
                                        "together, I'm sure that you'll",
                                        "be able to accomplish the",
                                        "mission, whatever it may be."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?) {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "Hey, it's you!",
                                        "I guess that spirit left my",
                                        "body as soon as you finished",
                                        "your mission. I thought that",
                                        "guy was never gonna leave...",
                                        "Whoever he was in life."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "But yeah, if it weren't for",
                                        "you, that spirit would never",
                                        "have been appeased, and",
                                        "I might have been stuck with",
                                        "him forever! So... thanks, man."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT_HIGH")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?))
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "It's a nice, quiet",
                                        "day, but the stillness",
                                        "in the air is unsettling.",
                                        "It's almost as if... I feel",
                                        "like something incredible",
                                        "may happen soon, you know?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CREATOR")?))
                            {
                                ctx.lines_as(
                                    "Guildsman",
                                    args![
                                        "Ah, it's a nice day...",
                                        "Sometimes, though, I wish",
                                        "that something big would",
                                        "happen to break the monotony."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
                step = Guildsman2Step::OnTouch;
                continue 'machine;
            }
            Guildsman2Step::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
                    ctx.lines(args![
                        "^3355FFWait a second!",
                        "Right now, you're carrying",
                        "too many things with you.",
                        "Please come back after",
                        "using the Kafra Service",
                        "to store some of your items.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("BaseLevel").get()?.number()? > 59 && !(ctx.var("mao_request").get()?.is_true()) {
                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?) {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DHold it right there!",
                                "I've got a proposition",
                                "for you, so listen up!^000000",
                                " ",
                                "...N-no, you're doing it again!",
                                "Damn, g-get out of my head!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DShut up, this is important!^000000",
                                " ",
                                "No, you shut up, this is",
                                "my freaking body! Look,",
                                "just gimme a little control!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I... I can sense two",
                                "different souls within",
                                "your body. I don't mind",
                                "listening to what you have",
                                "to say. But if you both keep",
                                "talking, I'll get confused..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Fine, whatever.",
                                " ",
                                "^71637DAlright, so the Assassin",
                                "Guild is looking for outside",
                                "help in one of their missions.",
                                "Preeeeetty important stuff.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildman",
                            args![
                                "^71637DThey've been hired by some",
                                "client to... to... um, you know...^000000",
                                " ",
                                "...Investigate missing children? ",
                                "^71637DYeah, yeah. That.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildman",
                            args![
                                "^71637DAnyway, this is a pretty",
                                "big mission, so the Assassin",
                                "Guild has been soliciting for",
                                "help. I think you'd be perfect",
                                "to represent the Soul Linkers,",
                                "so I've decided to choose you.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "^71637DSo, go and help the Assassins!^000000",
                                " ",
                                "Wait, don't forget the",
                                "letter of recommendation!",
                                " ",
                                "^71637DOh, right! Here, take this!^000000"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Alright, I did what you wanted.",
                                "You can leave my body now...",
                                " ",
                                "^71637DEh, I'll think about it.^000000",
                                " ",
                                "NOOOOOOO! PLEEEEEASE~!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?)
                        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT_HIGH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Well, if it isn't",
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". Hey,")),
                                "would you wait a minute?",
                                "I've got a message for you",
                                "from the Merchant Guild."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I'm not sure why, but",
                                "the Assassin Guild has",
                                "been requesting help from",
                                "someone in the Merchant",
                                "Guild. Would you go help",
                                "them and represent us?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "I think they're working for",
                                "a client, investigating these",
                                "children that are missing",
                                "from Morocc. If you want to",
                                "help them, you'll need this",
                                "letter of recommendation, okay?"
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Oh, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...!")),
                                "I'm so glad that I finally",
                                "ran into you. I know this is",
                                "sudden, but the Blacksmith",
                                "Guild has an assignment",
                                "for you over in Morocc."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "You see, the Assassin Guild",
                                "has formally requested for our",
                                "help in a mission regarding",
                                "children that have been missing",
                                "from Morocc recently. Here, take this letter of recommendation..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "If the Assassin Guild",
                                "is asking for help, I have",
                                "no doubt that this will be",
                                "a very difficult mission.",
                                "You should prepare yourself",
                                "if you plan to get involved..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ALCHEMIST")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_CREATOR")?))
                    {
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                ((Val::from("Hey, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "I've got a message for",
                                "you from the Alchemist",
                                "Guild. Well, they're more",
                                "like orders than a message.",
                                "I'm so lucky to have found you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "The Assassin Guild has",
                                "officially asked our guild",
                                "for help in a mission regarding",
                                "children that have been missing",
                                "from Morocc, and they want us",
                                "to send somebody... you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Or... At least, you're",
                                "one of the people that the",
                                "Alchemist Guild wants to send",
                                "to represent us. So why don't",
                                "you go? You know, do it for the",
                                "children. Just think about it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guildsman",
                            args![
                                "Here, take this letter of",
                                "recommendation with you and",
                                "head over to Morocc as soon",
                                "as you can. If even the Assassin Guild needs help, I'm sure that",
                                "spells really big trouble..."
                            ],
                        )?;
                        ctx.var("mao_request").set(Val::from(1))?;
                        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn guildsman_2(ctx: &Ctx) -> Script {
    guildsman_2_run(ctx, Guildsman2Step::Start, Vec::new()).map(|_| ())
}

pub fn guildsman_2_ontouch(ctx: &Ctx) -> Script {
    guildsman_2_run(ctx, Guildsman2Step::OnTouch, Vec::new()).map(|_| ())
}

fn academy_staff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (!(ctx.var("BaseLevel").get()?.number()? > 59) || !(ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?))) {
        ctx.lines_as(
            "Academy Staff",
            args![
                "Are you headed to the",
                "Geffen Tower Dungeon?",
                "If you run into any magic",
                "users, would please tell",
                "them to come and see me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "It's difficult to gather",
                "the members of the magic",
                "community, and I have an",
                "urgent message for Mages,",
                "Wizards and Sages to hear..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.var("mao_request").get()?.is_true()) {
        ctx.lines_as(
            "Academy Staff",
            args![
                "Whoa, would you wait",
                "a minute? Actually, you",
                "might be just right for",
                "this special mission..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "The Assassin Guild has",
                "sent a formal request to the",
                "Mage and Wizard Guilds, and",
                "the Schweicherbil Academy for",
                "aid in completing a mission",
                "involving missing children..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                "I'd like you to represent",
                "the magic community by",
                "assisting the Assassins",
                "in this mission. Please take",
                "this letter of recommendation."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(1))?;
        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "If you choose to assist",
                "the Assassin Guild on our",
                "behalf, please let me know",
                "so I can tell you where",
                "you can meet your contact.",
                "I shall be waiting here..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("mao_request").get()? == 1 {
        ctx.lines_as(
            "Academy Staff",
            args![
                "Have you decided to help",
                "the Assassin Guild? Then",
                "please, head to Morocc and",
                "meet your contact that will",
                "be waiting for you west of",
                "the Oasis inside of the city."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "The Assassins don't ask",
                "for help very often, so I'm",
                "sure that this must be a very",
                "serious matter. Be careful",
                "and bring pride to the magic",
                "community. Good luck..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("mao_request").get()?.number()? > 2 && ctx.var("mao_request").get()?.number()? < 27)
        || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 125))
    {
        ctx.lines_as(
            "Academy Staff",
            args![
                "I've heard that the",
                "Assassins are very",
                "impressed with your use",
                "of magic. Cooperate with",
                "them to finish the mission,",
                "and remember to be careful."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Academy Staff",
            args![
                "The weather is certainly",
                "pleasant right now, but the",
                "worst storms come when the",
                "winds are at their calmest.",
                "Verily, the peacefulness",
                "in the air disturbs me..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn academy_staff(ctx: &Ctx) -> Script {
    academy_staff_body(ctx, Vec::new()).map(|_| ())
}

fn academy_staff_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7416), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? > 59
        && !(ctx.var("mao_request").get()?.is_true())
        && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?)
    {
        ctx.lines_as(
            "Academy Staff",
            args![
                "Whoa, would you wait",
                "a minute? Actually, you",
                "might be just right for",
                "this special mission..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "The Assassin Guild has",
                "sent a formal request to the",
                "Mage and Wizard Guilds, and",
                "the Schweicherbil Academy for",
                "aid in completing a mission",
                "involving missing children..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                "I'd like you to represent",
                "the magic community by",
                "assisting the Assassins",
                "in this mission. Please take",
                "this letter of recommendation."
            ],
        )?;
        ctx.var("mao_request").set(Val::from(1))?;
        ctx.call(Function::GetItem, vec![Val::from(7416), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Academy Staff",
            args![
                "If you choose to assist",
                "the Assassin Guild on our",
                "behalf, please let me know",
                "so I can tell you where",
                "you can meet your contact.",
                "I shall be waiting here..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn academy_staff_ontouch(ctx: &Ctx) -> Script {
    academy_staff_ontouch_body(ctx, Vec::new()).map(|_| ())
}
