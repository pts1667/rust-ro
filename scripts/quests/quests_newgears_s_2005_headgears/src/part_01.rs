use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kasis_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(519)])?.number()? > 49
        && ctx.call(Function::CountItem, vec![Val::from(7031)])?.number()? > 49)
        && ctx.call(Function::CountItem, vec![Val::from(548)])?.number()? > 49)
        && ctx.call(Function::CountItem, vec![Val::from(539)])?.number()? > 49)
    {
        ctx.lines_as(
            "Kasis",
            args![
                "Milk, Cheese,",
                "Old Frying Pans,",
                "Pieces of Cake...",
                "Th-that's everything",
                "I need to make lunch for",
                "my friends in the Factory!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[
                Val::from("Well, I did promise to help you."),
                Val::from("What are you talking about?"),
            ],
        )? {
            1 => {
                ctx.lines_as(
                    "Kasis",
                    args![
                        "Finally, I can treat my",
                        "buddies to a wonderful",
                        "feast! They'll be so pleased!",
                        "Oh, I'd really like to repay you",
                        "somehow, but I'm not sure",
                        "what I could possibly give..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kasis",
                    args![
                        "Of course! Why don't",
                        "you treat yourself to",
                        "my specialty, Kasis's",
                        "Crunch Toast? It looks",
                        "simple, but it actually",
                        "requires great skill to make."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kasis",
                    args![
                        "Here you are!",
                        "Please enjoy this and always",
                        "remember that breakfast is the",
                        "most important meal of the day!",
                        "Now, I better prepare those",
                        "lunches. Goodbye, my friend~"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(519), Val::from(50)])?;
                ctx.call(Function::DelItem, vec![Val::from(7031), Val::from(50)])?;
                ctx.call(Function::DelItem, vec![Val::from(548), Val::from(50)])?;
                ctx.call(Function::DelItem, vec![Val::from(539), Val::from(50)])?;
                ctx.call(Function::GetItem, vec![Val::from(5107), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Kasis",
                    args!["Oh...", "Oh, I'm so sorry.", "I must have confused", "you with someone else."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kasis",
                    args![
                        "You see, I've been asking",
                        "adventurers that have been",
                        "passing through to provide",
                        "me with food supplies so that",
                        "I can make lunch for my friends",
                        "that are working in Einbroch."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kasis", args!["Yes...", "So...", "Sorry...", "To... Bother you."])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFAs if entranced,",
                    "Kasis continues to",
                    "stare at the Milk, Cheese",
                    "Old Frying Pans and Pieces",
                    "of Cake that you are carrying.^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("You can have these if you want."), Val::from("Um, why are you staring?")],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Kasis",
                            args![
                                "Really? Is it alright",
                                "with you for me to have",
                                "all of this Milk, Cheese,",
                                "Pieces of Cake and these",
                                "Old Frying Pans? Oh, bless",
                                "your kind heart, adventurer~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kasis",
                            args![
                                "Finally, I can treat my",
                                "buddies to a wonderful",
                                "feast! They'll be so pleased!",
                                "Oh, I'd really like to repay you",
                                "somehow, but I'm not sure",
                                "what I could possibly give..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kasis",
                            args![
                                "Of course! Why don't",
                                "you treat yourself to",
                                "my specialty, Kasis's",
                                "Crunch Toast? It looks",
                                "simple, but it actually",
                                "requires great skill to make."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kasis",
                            args![
                                "Here you are!",
                                "Please enjoy this and always",
                                "remember that breakfast is the",
                                "most important meal of the day!",
                                "Now, I better prepare those",
                                "lunches. Goodbye, my friend~"
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(519), Val::from(50)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7031), Val::from(50)])?;
                        ctx.call(Function::DelItem, vec![Val::from(548), Val::from(50)])?;
                        ctx.call(Function::DelItem, vec![Val::from(539), Val::from(50)])?;
                        ctx.call(Function::GetItem, vec![Val::from(5107), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Kasis",
                            args![
                                "Oh, let me apologize",
                                "again. But I can't help",
                                "but admire the quality of",
                                "your goods. Er, you know,",
                                "the food you're carrying. I'm",
                                "a chef by trade, after all."
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
    ctx.lines_as(
        "Kasis",
        args![
            "I'm sorry, but we're closed",
            "right now. Unfortunately, we",
            "don't have any more bread",
            "in stock. ^333333*Siiiiiiiiigh...*^000000"
        ],
    )?;
    ctx.next()?;
    'b3: {
        let subject3 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Is something wrong?"), Val::from("End Conversation.")],
        )?);
        let mut matched3 = false;
        let no_case3 = !subject3.loosely_equals(&Val::from(1)) && !subject3.loosely_equals(&Val::from(2));
        if !matched3 && subject3.loosely_equals(&Val::from(1)) {
            matched3 = true;
        }
        if matched3 {
            ctx.lines_as(
                "Kasis",
                args![
                    "Well, my current contract",
                    "with the company that's been",
                    "providing me with ingredients",
                    "has expired. Of course, it's",
                    "bad enough that I don't have",
                    "the food to run this business."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kasis",
                args![
                    "But I also feel like I'm",
                    "letting my friends down.",
                    "You see, I've been sending",
                    "them lunch every day since",
                    "they've been working in the",
                    "Factory over in Einbroch."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kasis",
                args![
                    "I'm unable to make",
                    "lunch for them at the",
                    "moment, but I really want to",
                    "help my buddies, especially",
                    "since their financial situation",
                    "seems pretty bad right now."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Do you want me to help you?"), Val::from("I'm so sorry to hear that.")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Kasis",
                        args![
                            "Sure, I'm willing to accept",
                            "help from wherever I can find",
                            "it. If you would, I'd like you to bring me some food supplies",
                            "that I can use to make lunches",
                            "for my friends at the Factory."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kasis",
                        args![
                            "Would you",
                            "please get me",
                            "^4D4DFF50 Milk^000000,",
                            "^4D4DFF50 Cheese^000000,",
                            "^4D4DFF50 Pieces of Cake^000000 and",
                            "^4D4DFF50 Old Frying Pans^000000?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kasis",
                        args![
                            "I'd really appreciate",
                            "your help. My friends seem",
                            "so depressed to be working",
                            "in the Factory and I want to",
                            "do all I can to cheer them up.",
                            "Thanks for your kind offer~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Kasis",
                        args![
                            "I'm trying to renew",
                            "my contract with the",
                            "company that's been",
                            "providing me with food",
                            "supplies, but it hasn't",
                            "been working out so far..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kasis",
                        args![
                            "I don't know what's",
                            "happening, but hopefully",
                            "I can find a new supplier",
                            "soon. There are my friends",
                            "to worry about, as well as",
                            "the sake of my business."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        if !matched3 && subject3.loosely_equals(&Val::from(2)) {
            matched3 = true;
        }
        if matched3 {
            ctx.lines_as(
                "Kasis",
                args![
                    "Anyway, please come",
                    "again. Hopefully, we'll",
                    "have some good bread",
                    "in stock next time, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn kasis_lhzhat(ctx: &Ctx) -> Script {
    kasis_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn strange_guy_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("dthatq").get()? == 0 {
        ctx.lines_as(
            "Strange Guy",
            args![
                "Wait! Don't say",
                "anything! You must be",
                "an adventurer from the",
                "Rune-Midgarts Kingdom!",
                "So... I'm right, aren't I?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Who are you?"), Val::from("How did you know?"), Val::from("Ignore him.")],
        )? {
            1 => {
                ctx.lines_as(
                    "Morris",
                    args![
                        "Allow me to",
                        "introduce myself.",
                        "My name is Morris Poe,",
                        "detective of great acclaim",
                        "and world wide fame."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[
                        Val::from("I've never heard of you."),
                        Val::from("You're kidding me."),
                        Val::from("I'm outta here."),
                    ],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Morris Poe?",
                                "Renown detective?",
                                "Advocate for justice in",
                                "the Schwarzwald Republic?",
                                "Surely you recognize one",
                                "of my esteemed titles."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Nope, I don't know you."), Val::from("You're Morris Poe?!")])? {
                            1 => {
                                ctx.lines_as(
                                    "Morris",
                                    args![
                                        "You don't...?",
                                        "But I'm a famous hero.",
                                        "Children look up to me",
                                        "and wish they'd grow",
                                        "up to be as smart as me."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Morris",
                                    args![
                                        "I don't get it.",
                                        "I'm famouser than",
                                        "that. A household name",
                                        "even. I've got a sterling",
                                        "public image, how could",
                                        "you never have heard of me...?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {}
                            _ => {}
                        }
                    }
                    2 => {
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Kidding? Ha ha!",
                                "Look at these keen,",
                                "deductive and deeply",
                                "analytical eyes and",
                                "tell me I'm joking."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("Whoa, those ARE keen eyes."), Val::from("Okay. You're joking.")],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Morris",
                                    args![
                                        "Yes, this sharp,",
                                        "penetrating, yet",
                                        "highly intelligent gaze",
                                        "belongs to the world",
                                        "detective and crime",
                                        "fighter, Morris Poe!"
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("I've never heard of you."), Val::from("Golly, Morris Poe?!")],
                                )? {
                                    1 => {
                                        ctx.lines_as(
                                            "Morris",
                                            args![
                                                "Never heard of...?",
                                                "Impossible! If there are",
                                                "two things I hate more",
                                                "than crime, they would be",
                                                "dishonesty... And rejection",
                                                "from beautiful women."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Morris",
                                            args![
                                                "When you get over your",
                                                "sense of pride and gain",
                                                "a little maturity, then we",
                                                "might share an actual",
                                                "conversation. Until then,",
                                                "you'll just have to grow up."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {}
                                    _ => {}
                                }
                            }
                            2 => {
                                ctx.lines_as(
                                    "Morris",
                                    args![
                                        "Hahahahahaah--Huh.",
                                        "If there are two things",
                                        "I hate more than criminals,",
                                        "they would be sarcasm...",
                                        "And receiving fake phone",
                                        "numbers from really cute girls."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(
                                    ctx,
                                    &[Val::from("Yeah, you're no Don Juan."), Val::from("I'm... I'm sorry.")],
                                )? {
                                    1 => {
                                        ctx.lines_as(
                                            "Morris",
                                            args![
                                                "Wah...!",
                                                "Did you just--?!",
                                                "Ugh, you're the worst",
                                                "type of person, you know that?"
                                            ],
                                        )?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Morris",
                                            args![
                                                "I think it's too late to",
                                                "apologize. Why would you",
                                                "treat someone of my stature",
                                                "with such caustic sarcasm?!",
                                                "What excuse could possibly",
                                                "exonerate your behavior?!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        'b7: {
                                            let subject7 = Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("I... I was intimidated..."), Val::from("It's how I treat all losers.")],
                                            )?);
                                            let mut matched7 = false;
                                            let no_case7 =
                                                !subject7.loosely_equals(&Val::from(1)) && !subject7.loosely_equals(&Val::from(2));
                                            if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                                                matched7 = true;
                                            }
                                            if matched7 {
                                                ctx.lines_as(
                                                    "Morris",
                                                    args![
                                                        "Of course: intense feelings",
                                                        "of awe and fear of my vast",
                                                        "intellect would result in that",
                                                        "kind of deviant behavior. I'll",
                                                        "have to forgive you, I suppose."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                            }
                                            if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                                                matched7 = true;
                                            }
                                            if matched7 {
                                                ctx.lines_as(
                                                    "Morris",
                                                    args![
                                                        "What...?!",
                                                        "Morris Poe, a loser?",
                                                        "Such defamatory remarks",
                                                        "cannot be forgiven, no",
                                                        "matter how unfounded",
                                                        "and groundless they are!"
                                                    ],
                                                )?;
                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    3 => {
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Hahaha, oh, that's",
                                "funny. Wait. You're...",
                                "You're really leaving.",
                                "N-no, come baaaack!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            2 => {
                ctx.lines_as(
                    "Strange Guy",
                    args![
                        "How did I know?",
                        "Elementary, my friend.",
                        "There was an abundance",
                        "of clues for me to make",
                        "that sort of simple deduction."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Strange Guy",
                    args![
                        "Hello...?",
                        "...............",
                        "Oh, this is embarassing.",
                        "I can't believe I'm getting",
                        "the silent treatment..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
        ctx.lines_as(
            "Morris",
            args![
                "Yes, I know it's",
                "difficult to believe that",
                "you would be fortunate",
                "enough to stand in the",
                "presence of genius. Now,",
                "you must be wondering..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Um, wondering what?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Morris",
            args![
                "You're wondering,",
                "\"How in the world",
                "can I be as brilliant",
                "as Morris Poe, the",
                "genius detective?\""
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "First, I think it's fair",
                "to let you know that it's",
                "an impossible dream. Men like",
                "myself only arise in this world",
                "perhaps once every generation.",
                "I'm sorry, but don't give up!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Now, you can try to reach",
                "a level of intelligence that",
                "is comparable to my own by",
                "joining the Young Detective's",
                "Club. To become a member, you just need one essential article."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Yes, you will need the",
                "^0000FFRenown Detective's Cap^000000!",
                "Luckily, I'm giving them",
                "away for a relatively small",
                "material cost, but you must",
                "hurry while supplies last."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Simply bring me",
                "^0000FF10 Magnifiers^000000,",
                "^0000FF1887 Tassels^000000,",
                "^0000FF1 Slotted Bucket Hat^000000",
                "and ^0000FF1,887 zeny^000000. That's all!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "But first, let me warn you",
                "not to bring a ^FF0000slotted Bucket",
                "Hat with a card compounded to",
                "it, or any upgraded Bucket Hats. I'm not responsible for any",
                "loss resulting from that."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Hurry and bring me",
                "those items! There are",
                "only 20,000,000 Renown",
                "Detective Caps left!",
                "Once they're gone,",
                "they'll be extinct!"
            ],
        )?;
        ctx.var("dthatq").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("dthatq").get()? == 1 {
        ctx.lines_as(
            "Morris",
            args![
                "So, did you want to",
                "buy, er, receive this",
                "Renown Detective's Cap",
                "and become an honored",
                "member of the Young",
                "Detective's Club?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Remember, make sure",
                "that you don't bring any",
                "^FF0000slotted Bucket Hats that",
                "have been upgraded or have",
                "cards compounded to them, or",
                "you'll lose those enhancements^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Now, if you have only",
                "1 Bucket Hat, we can begin.",
                "Young Detective's Club... Salute! "
            ],
        )?;
        ctx.next()?;
        'b8: {
            let subject8 = Val::from(runtime::select_values(ctx, &[Val::from("Salute."), Val::from("Whatever...")])?);
            let mut matched8 = false;
            let no_case8 = !subject8.loosely_equals(&Val::from(1)) && !subject8.loosely_equals(&Val::from(2));
            if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                matched8 = true;
            }
            if matched8 {
                ctx.lines_as(
                    "Morris",
                    args![
                        "Great, done like a pro!",
                        "Now, I need to check your",
                        "fee--I mean qualifications.",
                        "Let's see now, I need to",
                        "look through your Inventory..."
                    ],
                )?;
                ctx.next()?;
                ctx.var("@lhzhatfailed").set(Val::from(0))?;
                if ctx.call(Function::CountItem, vec![Val::from(611)])?.number()? < 10 {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Wait. You don't have",
                            "enough Magnifiers. You",
                            "do want to join this club,",
                            "don't you? Did you need to",
                            "hear the requirements again?"
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.call(Function::CountItem, vec![Val::from(7301)])?.number()? < 1887 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as("Morris", args!["Magnifiers. Check.", "Not enough Tassels? Check."])?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.call(Function::CountItem, vec![Val::from(5120)])?.number()? < 1 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Magnifiers. Check.",
                            "Tassels... Check.",
                            "Hm. You're missing",
                            "the slotted Bucket Hat.",
                            "You were so close..."
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.var("Zeny").get()?.number()? < 1887 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Magnifiers. Check.",
                            "Tassels... Check.",
                            "Slotted Bucket Hat. Got it.",
                            "Hey. Where's the zeny?!",
                            "That's the most important,",
                            "er, qualification of all!"
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if ctx.var("@lhzhatfailed").get()? == 1 {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "^333333*Sigh*^000000 You need",
                            "^0000FF10 Magnifiers^000000,",
                            "^0000FF1887 Tassels^000000,",
                            "^0000FF1 Slotted Bucket Hat^000000",
                            "and ^0000FF1,887 zeny^000000. That's all!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Now, remember everything",
                            "that you need this time, okay?",
                            "I'll be waiting to welcome",
                            "you into the elite ranks of ",
                            "the Young Detective's Club."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Morris",
                    args![
                        "Magnifiers. Check.",
                        "Tassels... Check.",
                        "Slotted Bucket Hat. Got it.",
                        "Aaaand zeny. Heh heh. Perfect."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Morris",
                    args![
                        "Great, that's everything!",
                        "Welcome to the club! Here's",
                        "your Renown Detective's Cap,",
                        "so wear it with pride as you",
                        "take an active part in our club's",
                        "activities and promotion!"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7301), Val::from(1887)])?;
                ctx.call(Function::DelItem, vec![Val::from(5120), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(611), Val::from(10)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1887))?))?;
                ctx.var("dthatq").set(Val::from(2))?;
                ctx.call(Function::GetItem, vec![Val::from(5108), Val::from(1)])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Wait, promotion?"), Val::from("Hooray!")])? {
                    1 => {
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Of course, isn't it obvious?",
                                "As a member, one of your duties",
                                "will be to sing accolades of this club so that more members",
                                "will join and earn their own",
                                "Renown Detective's Caps."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Alright, scurry off",
                                "now and proudly display",
                                "your new, rakishly stylish",
                                "Renown Detective's Cap",
                                "to all of your friends!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Morris",
                            args![
                                "Alright, scurry off",
                                "now and proudly display",
                                "your new, rakishly stylish",
                                "Renown Detective's Cap",
                                "to all of your friends!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                matched8 = true;
            }
            if matched8 {
                ctx.lines_as(
                    "Morris",
                    args![
                        "Huh. You just failed",
                        "the \"Test of Respect.\"",
                        "You don't deserve the",
                        "honor of wearing the",
                        "Renowned Detective's Cap!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("dthatq").get()? == 2 {
        ctx.lines_as(
            "Morris",
            args![
                "Have you come to me",
                "in order to obtain another",
                "Renown Detective's Cap?",
                "I realize that such a stylish",
                "headgear is in high demand."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "Remember, make sure",
                "that you don't bring any",
                "^FF0000slotted Bucket Hats that",
                "have been upgraded or have",
                "cards compounded to them, or",
                "you'll lose those enhancements^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Morris",
            args![
                "As a proud member of",
                "the Young Detective's Club,",
                "did you bring everything",
                "you need in order to earn",
                "a Renown Detective's Cap?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'm not sure~"), Val::from("Sure!")])? {
            1 => {
                ctx.lines_as(
                    "Morris",
                    args![
                        "What...?! Every good",
                        "detective must have a",
                        "photographic memory",
                        "and be able to recall",
                        "minute details in order",
                        "to crack the case."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Morris",
                    args![
                        "Remember...",
                        "^0000FF10 Magnifiers^000000,",
                        "^0000FF1887 Tassels^000000,",
                        "^0000FF1 Slotted Bucket Hat^000000",
                        "and ^0000FF1,887 zeny^000000. Okay?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Morris",
                    args!["Great!", "Now let me just", "check to see if you", "brought everything..."],
                )?;
                ctx.next()?;
                ctx.var("@lhzhatfailed").set(Val::from(0))?;
                if ctx.call(Function::CountItem, vec![Val::from(611)])?.number()? < 10 {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Hmm, you don't have",
                            "enough Magnifiers? But",
                            "those are essential tools",
                            "for sleuthing. Come now,",
                            "hurry back with them!"
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.call(Function::CountItem, vec![Val::from(7301)])?.number()? < 1887 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as("Morris", args!["Magnifiers. Check.", "Not enough Tassels? Check."])?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.call(Function::CountItem, vec![Val::from(5120)])?.number()? < 1 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Magnifiers. Check.",
                            "Tassels... Check.",
                            "Hm. You're missing",
                            "the slotted Bucket Hat.",
                            "You were so close..."
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if (ctx.var("Zeny").get()?.number()? < 1887 && ctx.var("@lhzhatfailed").get()? == 0) {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Magnifiers. Check.",
                            "Tassels... Check.",
                            "Slotted Bucket Hat. Got it.",
                            "Hey. Where's the zeny?!",
                            "That's the most important,",
                            "er, qualification of all!"
                        ],
                    )?;
                    ctx.var("@lhzhatfailed").set(Val::from(1))?;
                    ctx.next()?;
                }
                if ctx.var("@lhzhatfailed").get()? == 1 {
                    ctx.lines_as(
                        "Morris",
                        args![
                            "Remember...",
                            "^0000FF10 Magnifiers^000000,",
                            "^0000FF1887 Tassels^000000,",
                            "^0000FF1 Slotted Bucket Hat^000000",
                            "and ^0000FF1,887 zeny^000000. Okay?"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Morris",
                    args![
                        "Magnifiers. Check.",
                        "Tassels... Check.",
                        "Slotted Bucket Hat. Got it.",
                        "Aaaand zeny. Heh heh. Perfect."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Morris",
                    args![
                        "Great! You've just qualified",
                        "to own yet another Renowned",
                        "Detective's Cap. Congratulations! Now, let me collect all of these",
                        "goods and zeny from you..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7301), Val::from(1887)])?;
                ctx.call(Function::DelItem, vec![Val::from(5120), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(611), Val::from(10)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1887))?))?;
                ctx.var("dthatq").set(Val::from(2))?;
                ctx.call(Function::GetItem, vec![Val::from(5108), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Morris",
                    args![
                        "Hahaha! It's what",
                        "I expect from one of",
                        "our Junior Gold Members!",
                        "How does it feel like to be",
                        "the cream of the cream of",
                        "the crop? Great, isn't it?"
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

pub fn strange_guy_lhzhat(ctx: &Ctx) -> Script {
    strange_guy_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn kid_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Kid]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.mes("Hello, mister!")?;
    } else {
        ctx.mes("Hello, ma'am!")?;
    }
    ctx.lines(args!["Um, would you do", "me a favor, please?"])?;
    ctx.next()?;
    if (((ctx.call(Function::CountItem, vec![Val::from(526)])?.is_true()
        && ctx.call(Function::CountItem, vec![Val::from(7270)])?.is_true())
        && ctx.call(Function::CountItem, vec![Val::from(941)])?.is_true())
        && ctx.call(Function::CountItem, vec![Val::from(10004)])?.is_true())
    {
        ctx.lines_as(
            "Kid",
            args![
                "Oh! You brought me",
                "some Royal Jelly to",
                "eat, as well as everything",
                "I need to make a Baby Pacifier.",
                "Okay, let me get started on that right away. Just a moment..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Kid",
            args![
                "Damn.",
                "That Royal Jelly",
                "was frickin' delicious!",
                "Now I can make your",
                "Baby Pacifier. Let's see",
                "here. Ah, here we go..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kid",
            args![
                "It's done!",
                "Now, you're the",
                "proud owner of your",
                "very own Baby Pacifier!",
                "I hope you're happy with",
                "yourself. Travel safe, okay?"
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(526), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(7270), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(941), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(10004), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(5110), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("What do you need?"), Val::from("Sorry, kid...")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kid",
                args![
                    "Well, my mom has been",
                    "sick recently, so both of",
                    "us haven't been able to eat",
                    "for the past few days. But",
                    "if she doesn't eat, how does",
                    "she expect to get better?"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Kid",
                args!["So I guess what I'm", "asking is, would you be", "able to spare some food?"],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Help him."), Val::from("Refuse to help.")])? {
                1 => {
                    if !(ctx.call(Function::CountItem, vec![Val::from(526)])?.is_true()) {
                        ctx.lines_as(
                            "Kid",
                            args![
                                "If you would, do you think",
                                "you can get some Royal Jelly",
                                "that I can give to my mom? The old lady next door says that it's",
                                "really nutritious and helps you get better faster if you're sick."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kid",
                            args![
                                "My mom would be mad",
                                "at me if she found out",
                                "I was begging for it, but",
                                "Royal Jelly is too expensive",
                                "for me to get without any help!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Oh, would it be alright",
                            "if I have this Royal Jelly?",
                            "It would really help my mom",
                            "feel better. Thanks so much~"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(526), Val::from(1)])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Hey, to pay you back, why",
                            "don't I make a Baby Pacifier",
                            "for you? I learned how to make",
                            "one when my baby brother was",
                            "born. But first, I'll need some",
                            "materials to put it together..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Would you",
                            "please bring",
                            "^3131FF1 Nursing Bottle^000000,",
                            "^3131FF1 Nose Ring^000000 and",
                            "^3131FF1 Pacifier^000000? Oh, and one",
                            "more ^3131FFRoyal Jelly^000000 for me~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Kid",
                        args![
                            "Oh... Um, that's",
                            "alright. Maybe I was",
                            "asking too much from",
                            "you. I mean, it's true",
                            "that you barely know me..."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
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
                "Kid",
                args![
                    "Th-that's okay.",
                    "I guess you must",
                    "be so busy that you",
                    "don't even have the",
                    "time to listen to some",
                    "little boy's problems..."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn kid_lhzhat(ctx: &Ctx) -> Script {
    kid_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn metelle_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.call(Function::CountItem, vec![Val::from(983)])?.is_true()
        && ctx.call(Function::CountItem, vec![Val::from(7267)])?.number()? > 998)
        && ctx.call(Function::CountItem, vec![Val::from(749)])?.is_true())
        && ctx.var("Zeny").get()?.number()? > 49999)
    {
        ctx.lines_as(
            "Metelle",
            args![
                "Oh, hello, what's this?",
                "You're carrying 1 Black",
                "Dyestuffs, 1 Frozen Rose,",
                "999 Tiger Panties and even",
                "50,000 zeny with you. That's",
                "enough to make a Winter Hat..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Metelle",
            args![
                "Um, would you mind letting",
                "me have those items so that",
                "I can make a Winter Hat?",
                "I'd never be able to gather",
                "those things on my own..."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Let her have the items."), Val::from("Don't give her the items.")],
        )? {
            1 => {
                ctx.lines_as(
                    "Metelle",
                    args![
                        "Oh, thank you so much!",
                        "I've always wanted to make",
                        "this hat and try it on, even",
                        "if it's just once. But don't",
                        "worry, I'll give it to you~",
                        "Now please next; a moment..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metelle",
                    args![
                        "Let's see...",
                        "I've got to fold",
                        "the rose like this...",
                        "Be careful with the dye...",
                        "Where did I put all those",
                        "pan--Oh, here we are."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metelle",
                    args![
                        "^333333*Whew!*^000000",
                        "Finally, it's done!",
                        "Now, if you don't mind,",
                        "let me try this hat on",
                        "for just a little while."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(983), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(7267), Val::from(999)])?;
                ctx.call(Function::DelItem, vec![Val::from(749), Val::from(1)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5115), Val::from(1)])?;
                ctx.next()?;
                ctx.lines(args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Metelle",
                    args![
                        "Ahhhhhh~",
                        "It just feels...",
                        "I felt so free~",
                        "It was everything",
                        "I imagined it to be."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Metelle",
                    args![
                        "Here, please take",
                        "this Winter Hat. I really",
                        "hope that you'll enjoy it",
                        "as much as I do. Well then,",
                        "be safe in your travels, okay?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Metelle",
                    args![
                        "I'm sorry, I know that",
                        "I was asking a pretty big",
                        "favor from someone I just",
                        "met. I hope you understand",
                        "how much I really want to",
                        "make that Winter Hat..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Metelle",
        args![
            "Look at that blue",
            "sky. Don't you wish",
            "you could just soar",
            "through the heavens",
            "with your own wings?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Metelle",
        args![
            "Of course, it's an",
            "impossible dream,",
            "but with this Winter Hat,",
            "you can at least enjoy the",
            "sensation of freedom that",
            "a bird in flight must feel."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Metelle",
        args![
            "If you would like",
            "a hat like this, I would",
            "need to have some items",
            "that I don't think I can ever",
            "obtain on my very own."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Metelle",
        args![
            "^FF00001 Black Dyestuffs^000000,",
            "^FF0000999 Tiger Panties^000000,",
            "^FF00001 Frozen Rose^000000 and",
            "^FF000050,000 zeny^000000. If you can",
            "bring those to me, I shall",
            "make you a Winter Hat."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn metelle_lhzhat(ctx: &Ctx) -> Script {
    metelle_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn margaret_mary_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Margaret Mary",
        args![
            "The white rose, in its",
            "purity and simplicity, is",
            "like a woman who doesn't",
            "need jewels or fancy dresses",
            "to look noble and beautiful. It's the perfect gift for a lady."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Make a Mystic Rose."), Val::from("End Conversation.")])? {
        1 => {
            if (((ctx.call(Function::CountItem, vec![Val::from(731)])?.number()? > 9
                && ctx.call(Function::CountItem, vec![Val::from(748)])?.number()? > 2)
                && ctx.call(Function::CountItem, vec![Val::from(982)])?.is_true())
                && ctx.var("Zeny").get()?.number()? > 49999)
            {
                ctx.lines_as(
                    "Margaret Mary",
                    args![
                        "Ah, I see that you've brought",
                        "what I need to bleach the blood",
                        "red hue from these Witherless",
                        "Roses and adorn these flowers",
                        "with eternal elegance. May I use these items to make a Mystic Rose?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])? {
                    1 => {
                        ctx.lines_as(
                            "Margaret Mary",
                            args![
                                "Thank you. Please",
                                "next; a moment while",
                                "I concentrate in order to",
                                "preserve the natural beauty",
                                "of these gorgeous flowers..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Margaret Mary",
                            args![
                                "The rose truly is the",
                                "undisputed queen of all",
                                "flowers. All other flora must",
                                "humbly bow to its regal beauty."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "......", "........."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Margaret Mary",
                            args![
                                "There, it is finished.",
                                "Please remember to wear",
                                "your hair in an elegant and",
                                "refined manner when wearing",
                                "the Mystic Rose so as not to disgrace these beautiful flowers."
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(731), Val::from(10)])?;
                        ctx.call(Function::DelItem, vec![Val::from(748), Val::from(3)])?;
                        ctx.call(Function::DelItem, vec![Val::from(982), Val::from(1)])?;
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50000))?))?;
                        ctx.call(Function::GetItem, vec![Val::from(5117), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Margaret Mary",
                            args![
                                "White roses with thorns",
                                "makes my heart beat with",
                                "unceasing trepidation. What",
                                "if I prick my finger and shed",
                                "blood on its snow white petals?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            ctx.lines_as(
                "Margaret Mary",
                args![
                    "I love roses, but it makes",
                    "me sad that their beauty",
                    "fades far too soon. And so,",
                    "I've found a way to preserve",
                    "the beauty of the roses, so",
                    "that it lasts for all eternity."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Margaret Mary",
                args![
                    "Do you love roses as well?",
                    "If you like, I can make a",
                    "^3131FFMystic Rose^000000 that you can wear",
                    "upon your crown. It's not easy",
                    "for me to create, but I believe that you would enjoy it greatly."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Margaret Mary",
                args![
                    "Please bring me",
                    "^3131FF10 2 Carat Diamonds^000000,",
                    "^3131FF3 Witherless Roses^000000,",
                    "^3131FF1 White Dyestuffs^000000 and",
                    "^3131FF50,000 zeny^000000 if you would",
                    "like to have a Mystic Rose."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Margaret Mary",
                args![
                    "White roses with thorns",
                    "makes my heart beat with",
                    "unceasing trepidation. What",
                    "if I prick my finger and shed",
                    "blood on its snow white petals?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn margaret_mary_lhzhat(ctx: &Ctx) -> Script {
    margaret_mary_lhzhat_body(ctx, Vec::new()).map(|_| ())
}
