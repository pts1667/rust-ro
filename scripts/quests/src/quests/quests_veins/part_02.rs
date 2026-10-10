use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guant_prisoner_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 11 {
        ctx.lines_as(
            "Ruan",
            args![
                "This stinks. Why am",
                "I locked up in here?",
                "I didn't do anything",
                "to deserve this!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ruan",
            args![
                "It's getting to be so",
                "bad that I even miss the",
                "sound of Hianna's voice.",
                "It's freakishly loud.",
                "You couldn't outyell her",
                "even with a Megaphone."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 11 {
        ctx.lines_as(
            "Ruan",
            args![
                "This stinks. Why am",
                "I locked up in here?",
                "I didn't do anything",
                "to deserve this!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ruan",
            args![
                "It's getting to be so",
                "bad that I even miss the",
                "sound of Hianna's voice.",
                "It's freakishly loud.",
                "You couldn't outyell her",
                "even with a Megaphone."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["What...?!"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Where I can find that Megaphone?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Ruan", args!["Huh? Why would you", "want to know that?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I really need one!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Ruan",
            args![
                "Uh, you can get one from",
                "that Dancer Job Change",
                "place in Comodo. They're",
                "really hard to get, though,",
                "if not impossible. Knock",
                "yourself out, buddy."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 20 {
        ctx.lines_as(
            "Ruan",
            args![
                "Nobody knows what",
                "will happen tomorrow.",
                "I mean, this is a world of",
                "miracles and tragedies.",
                "Death, or real love...",
                "You will never know what will happen."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Ruan", args!["........"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guant_prisoner_sch(ctx: &Ctx) -> Script {
    guant_prisoner_sch_body(ctx, Vec::new()).map(|_| ())
}

fn dance_instructor_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
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
    if ctx.var("que_sch").get()?.number()? < 11 {
        ctx.lines_as(
            "Hianna",
            args![
                "Keep up the good work,",
                "everyone! There's only",
                "a few days left until the",
                "big performance! Hey...",
                "You! Can't you do it right?",
                "Turn your waist quicker!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 11 {
        ctx.lines_as(
            "Hianna",
            args![
                "Hey, who are you?",
                "We already have enough",
                "problems with too many",
                "pervs ogling the Dancers.",
                "I'd prefer it if you didn't",
                "come to watch us practice."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("I want a Megaphone.:Eeek...")])?) == 2 {
            ctx.lines_as(
                "Hianna",
                args![
                    "If you really want to",
                    "watch us dance, come to",
                    "the show and buy a ticket.",
                    "Watching for free isn't",
                    "exactly supporting the arts."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hianna",
            args![
                "You want a Megaphone?",
                "Well, I'm sorry, but it's",
                "not just something I can",
                "lend to anybody. Then again",
                "it's not like you can find one",
                "anywhere else, either."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Please! I'll do anything!:Later.")])?) == 2 {
            ctx.lines_as("Hianna", args!["Alright, then.", "I'm sorry that", "I can't help you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hianna",
            args![
                "Anything, huh?",
                "Well, you just said",
                "the magic word. Listen",
                "carefully to what I want",
                "you to do for me."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Listen:Reconsider")])?) == 1 {
            ctx.lines_as(
                "Hianna",
                args![
                    "First, I want a little",
                    "cash. Consider it a rental",
                    "fee. 500,000 zeny should be",
                    "just about enough. Then,",
                    "I want you to do me a favor."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hianna",
                args![
                    "The Schwarzwald Republic",
                    "requested me to send some",
                    "Dancers for some party, but",
                    "I don't have enough guards",
                    "to protect them on their",
                    "way over there."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hianna",
                args![
                    "If you act as bodyguard",
                    "to my Dancers on the way",
                    "to the Schwarzwald Republic,",
                    "I'll lend you my Megaphone",
                    "once you come back. So...",
                    "How does that sound?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Sounds good.:Like a ripoff.")])?) == 1 {
                ctx.lines_as(
                    "Hianna",
                    args![
                        "I'm glad you agree~",
                        "Okay, the Dancers are",
                        "waiting are the entrance,",
                        "so bring them over to the",
                        "Schwarzwald Republic as",
                        "soon as you're ready."
                    ],
                )?;
                ctx.var("que_sch").set(Val::from(12))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Hianna",
                args!["So... I guess you", "didn't need that", "Megaphone as", "badly as I thought."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Hianna", args!["Alright, take your", "time. I'm in no rush."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 17 {
        ctx.lines_as(
            "Hianna",
            args![
                "What are you still doing",
                "here? Shouldn't you be",
                "escorting the Dancers to",
                "the Schwaltvalt Republic",
                "already? Get a move on~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 17 {
        ctx.lines_as(
            "Hianna",
            args![
                "Thanks for all your",
                "hard work. The Dancers",
                "told me that you did a good",
                "job escorting them. Now...",
                "Do you have the money?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            if ctx.var("Zeny").get()?.number()? < 500000 {
                ctx.lines_as(
                    "Hianna",
                    args![
                        "What's this? Hmm...",
                        "I think you made a mistake.",
                        "This isn't enough money.",
                        "Remember, 500,000 zeny~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Hianna",
                args![
                    "Perfect. Well, here's",
                    "your Megaphone. Thanks",
                    "for everything, and I'll see",
                    "you around, adventurer~"
                ],
            )?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500000))?))?;
            ctx.var("que_sch").set(Val::from(18))?;
            ctx.call(Function::GetItem, vec![Val::from(7040), Val::from(1)])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hianna",
            args![
                "I can't have you breaking",
                "your promises, so I won't",
                "give you the Megaphone",
                "until you pay me the",
                "500,000 zeny that you",
                "said that you would."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 18 {
        ctx.lines_as(
            "Hianna",
            args![
                "Thanks for escorting",
                "my Dancers over to the",
                "Schwarzwald Republic.",
                "Anything I can help",
                "you with today?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I'd like another Megaphone.:No thanks.")],
        )?) == 1
        {
            ctx.lines_as(
                "Hianna",
                args![
                    "Well, I guess I can let",
                    "you have another one if",
                    "you pay me 500,000 zeny.",
                    "You sure you want to pay",
                    "the money for a Megaphone?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                if ctx.var("Zeny").get()?.number()? < 500000 {
                    ctx.lines_as(
                        "Hianna",
                        args![
                            "I'm sorry, but this",
                            "isn't enough money for",
                            "a Megaphone. Be sure",
                            "to bring me 500,000 zeny."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Hianna",
                    args![
                        "Here you are. The fact is...",
                        "These Megaphones are",
                        "considered guild property,",
                        "so I'm not supposed to let",
                        "you have this. Don't let",
                        "anyone know I gave you this!"
                    ],
                )?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(500000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(7040), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Hianna", args!["Alright~", "Take care, and", "travel safely~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Hianna", args!["Alright~", "Take care, and", "travel safely~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Hianna",
        args![
            "Trust me, we've made",
            "good use of the money",
            "that you've ''donated.''",
            "Thanks for taking good",
            "care of my Dancers~"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dance_instructor_sch(ctx: &Ctx) -> Script {
    dance_instructor_sch_body(ctx, Vec::new()).map(|_| ())
}

fn young_dancer_sch1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 12 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 12 {
        ctx.lines_as(
            "Dancer",
            args!["Ah, you must be the", "bodyguard. So are you", "ready to go now?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            ctx.lines_as("Dancer", args!["Alright, then.", "Here we go~"])?;
            ctx.var("que_sch").set(Val::from(13))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("airplane"), Val::from(75), Val::from(55)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Dancer",
            args!["We need to depart soon,", "so please hurry. I'll be", "waiting for you here~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "Oh, that was such a fun",
            "performance. I can still",
            "see the dazzling decorations",
            "in the ballroom where we got",
            "to dance. It was so wonderful!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_dancer_sch1(ctx: &Ctx) -> Script {
    young_dancer_sch1_body(ctx, Vec::new()).map(|_| ())
}

fn cheerful_dancer_sch1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 12 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 12 {
        ctx.lines_as(
            "Dancer",
            args!["Ah, you must be the", "bodyguard. So are you", "ready to go now?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            ctx.lines_as("Dancer", args!["Alright, then.", "Here we go~"])?;
            ctx.var("que_sch").set(Val::from(13))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("airplane"), Val::from(75), Val::from(55)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Dancer",
            args!["We need to depart soon,", "so please hurry. I'll be", "waiting for you here~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "I guess it's pretty",
            "fun to perform on stage~",
            "But I still need to practice",
            "more for the next performance."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cheerful_dancer_sch1(ctx: &Ctx) -> Script {
    cheerful_dancer_sch1_body(ctx, Vec::new()).map(|_| ())
}

fn mature_looking_dancer_s1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 12 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 12 {
        ctx.lines_as(
            "Dancer",
            args!["Ah, you must be the", "bodyguard. So are you", "ready to go now?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
            ctx.lines_as("Dancer", args!["Alright, then.", "Here we go~"])?;
            ctx.var("que_sch").set(Val::from(13))?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("airplane"), Val::from(75), Val::from(55)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Dancer",
            args!["We need to depart soon,", "so please hurry. I'll be", "waiting for you here~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "I sure learned a lot from",
            "that trip. I hope that the new",
            "girls also gained something",
            "from their experiences."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mature_looking_dancer_s1(ctx: &Ctx) -> Script {
    mature_looking_dancer_s1_body(ctx, Vec::new()).map(|_| ())
}

fn young_dancer_sch2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 13 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 13 {
        ctx.lines_as(
            "Dancer",
            args![
                "This will be my",
                "first performance...",
                "I guess that's why",
                "I have butterflies",
                "in my stomach..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "I wonder which beautiful",
            "place we'll get to perform in",
            "next time! Ooh, I can't wait!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_dancer_sch2(ctx: &Ctx) -> Script {
    young_dancer_sch2_body(ctx, Vec::new()).map(|_| ())
}

fn cheerful_dancer_sch2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 13 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 13 {
        ctx.lines_as(
            "Dancer",
            args![
                "I've performed at many",
                "venues, but this is the",
                "frist time I'll be dancing",
                "in the Schwarzwald Republic.",
                "This is also my first time on",
                "an airship. How exciting!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args!["Traveling is really", "thrilling... It's almost as", "fun as dancing on stage~"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cheerful_dancer_sch2(ctx: &Ctx) -> Script {
    cheerful_dancer_sch2_body(ctx, Vec::new()).map(|_| ())
}

fn mature_looking_dancer_s2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 13 {
        ctx.lines_as("Dancer", args!["Hi there~", "Are you enjoying", "yourself? I hope so!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 13 {
        ctx.lines_as(
            "Dancer",
            args![
                "This is the first time that",
                "the Schwarzwald Republic",
                "requested a performance",
                "from us. Isn't that amazing?",
                "I guess we earned a reputation",
                "overseas. I'll do my best!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Airship Announcement",
            args![
                "We will be arriving",
                "in Einbroch shortly.",
                "Passengers to Einbroch,",
                "please get ready to land."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Dancer",
            args!["Oh! We're finally here!", "Alright, I'll give this next", "performance my all!"],
        )?;
        ctx.close_window()?;
        ctx.var("que_sch").set(Val::from(14))?;
        ctx.call(Function::Warp, vec![Val::from("ein_in01"), Val::from(278), Val::from(223)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "I've been dancing on",
            "stage for a long time, but",
            "I always feel so nervous",
            "right beforehand. I wonder",
            "why that happens to me."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mature_looking_dancer_s2(ctx: &Ctx) -> Script {
    mature_looking_dancer_s2_body(ctx, Vec::new()).map(|_| ())
}

fn young_dancer_sch3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 17 {
        ctx.lines_as(
            "Dancer",
            args![
                "Oh, the dinner",
                "party isn't finished yet.",
                "It'll be over before",
                "you even know it~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 17 {
        ctx.lines_as(
            "Dancer",
            args![
                "Hooray! My first",
                "onstage performance",
                "was a success! Oh",
                "I was so nervous...",
                "But I'm so proud",
                "of myself now!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Dancer", args!["Without dance,", "my life has no", "meaning at all."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_dancer_sch3(ctx: &Ctx) -> Script {
    young_dancer_sch3_body(ctx, Vec::new()).map(|_| ())
}

fn cheerful_dancer_sch3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 17 {
        ctx.lines_as(
            "Dancer",
            args!["The dinner party isn't", "even finished yet, but", "I'm already exhausted~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 17 {
        ctx.lines_as(
            "Dancer",
            args![
                "^333333*Whew*^000000 We gave a good",
                "performance this time.",
                "I was worried since that",
                "airship trip really drained me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "Dancing is really",
            "hard work, and all that",
            "practicing wears you out,",
            "but it's worth it once you",
            "get up on that stage."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn cheerful_dancer_sch3(ctx: &Ctx) -> Script {
    cheerful_dancer_sch3_body(ctx, Vec::new()).map(|_| ())
}

fn mature_looking_dancer_s3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 17 {
        ctx.lines_as(
            "Dancer",
            args![
                "Oh, the dinner",
                "party isn't finished yet.",
                "It'll be over before",
                "you even know it~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 17 {
        ctx.lines_as(
            "Dancer",
            args![
                "Well well, it looks like our",
                "performance was a success.",
                "The new girls did a really",
                "great job. So let's head",
                "back home, shall we?"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("comodo"), Val::from(191), Val::from(146)])?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Dancer",
        args![
            "Dancing is so fun, but",
            "sometimes it's hard to",
            "keep up with the audience's",
            "expectations, you know?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mature_looking_dancer_s3(ctx: &Ctx) -> Script {
    mature_looking_dancer_s3_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_manager_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 15 {
        ctx.lines_as(
            "Manager",
            args![
                "It'd be a really great",
                "party if our customers",
                "were a little less rowdy",
                "Recently, they've been",
                "more than a handful..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 15 {
        ctx.lines_as(
            "Manager",
            args![
                "I'm glad our customers",
                "enjoyed the performance",
                "I was a little worried about",
                "what they were going to think,",
                "but I guess I was just being",
                "overly anxious about it all."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 16 {
        ctx.lines_as(
            "Manager",
            args![
                "Thank you so much for",
                "your services. I'll be sure to",
                "have one of my employees",
                "send you your payment",
                "Have a safe trip back~"
            ],
        )?;
        ctx.var("que_sch").set(Val::from(17))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hotel_manager_sch(ctx: &Ctx) -> Script {
    hotel_manager_sch_body(ctx, Vec::new()).map(|_| ())
}

fn hotel_manager_sch_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()? == 14 {
        ctx.lines_as(
            "Manager",
            args![
                "Oh, did you enjoy",
                "your trip? I'm glad to",
                "see that everyone arrived",
                "safely. The dinner party",
                "will start shortly, so",
                "please get ready~"
            ],
        )?;
        ctx.var("que_sch").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn hotel_manager_sch_ontouch(ctx: &Ctx) -> Script {
    hotel_manager_sch_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn employee_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 15 {
        ctx.lines_as(
            "Employee",
            args![
                "We've been so busy lately!",
                "It's just one party reservation",
                "after another! When will I be",
                "able to just take a break?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 17 {
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
            ctx.lines_as(
                "Employee",
                args![
                    "Hi, how may I help you?",
                    "Tonight, most of customers",
                    "are high ranking government",
                    "officials or public figures.",
                    "That man over there is the",
                    "Rekenber P.R. executive."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Employee",
                args![
                    "That priest over there is",
                    "actually a diplomat from",
                    "Arunafeltz on business.",
                    "These are some very",
                    "important people!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Employee",
            args![
                "How can I help you?",
                "Oh, would you like another",
                "drink? There you go! Please",
                "enjoy the dinner party~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Employee",
        args![
            "Oh, the mess left after",
            "a banquet is the biggest",
            "part of my job. It's tough",
            "work, but it needs to get done."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn employee_sch(ctx: &Ctx) -> Script {
    employee_sch_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BardTriggerSchStep {
    Start,
    OnTouch,
}

fn bard_trigger_sch_run(ctx: &Ctx, mut step: BardTriggerSchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BardTriggerSchStep::Start => {
                step = BardTriggerSchStep::OnTouch;
                continue 'machine;
            }
            BardTriggerSchStep::OnTouch => {
                if ctx.var("que_sch").get()? == 15 {
                    ctx.call(Function::DisableNpc, vec![Val::from("Corporate Figure#sch")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Arunafeltz Figure#sch")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Corporate Figure")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Arunafeltz Figure")])?;
                    ctx.lines_as(
                        "????",
                        args!["I guess the party", "will soon be over.", "Did you enjoy yourself?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "??????",
                        args![
                            "Yes, thank you, I had",
                            "a great time. I'm sorry",
                            "I gave you such short",
                            "notice of my arrival, but",
                            "you held this party anyway."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "Don't mention it.",
                            "You're a valued guest.",
                            "It would shame me if I'd",
                            "failed to entertain you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("??????", args!["Ho ho, you certainly know", "how to be a good host~"])?;
                    ctx.next()?;
                    ctx.lines_as("????", args!["Your words honor me."])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFThe man took a quick", "look around the room.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "No one's around.",
                            "I have something to",
                            "discuss with you before",
                            "we get down to business."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("??????", args!["What kind of...?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "I'll explain in detail",
                            "someplace safer. The gist",
                            "is that some rogues over in",
                            "Arunafeltz are plotting to harm",
                            "relations between Arunafeltz",
                            "and the Rekenber Corporation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "??????",
                        args![
                            "Oh... I see. Yes,",
                            "we can't talk about",
                            "that here. To tell the",
                            "truth, I've suspected that",
                            "something like that was",
                            "going on... Yes, makes sense."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "We should relocate",
                            "so that we can talk",
                            "a bit more freely.",
                            "Please follow me,",
                            "I already have",
                            "a place prepared."
                        ],
                    )?;
                    ctx.var("que_sch").set(Val::from(16))?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Corporate Figure")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Arunafeltz Figure")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Corporate Figure#sch")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Arunafeltz Figure#sch")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn bard_trigger_sch(ctx: &Ctx) -> Script {
    bard_trigger_sch_run(ctx, BardTriggerSchStep::Start, Vec::new()).map(|_| ())
}

pub fn bard_trigger_sch_ontouch(ctx: &Ctx) -> Script {
    bard_trigger_sch_run(ctx, BardTriggerSchStep::OnTouch, Vec::new()).map(|_| ())
}

fn corporate_figure_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn corporate_figure(ctx: &Ctx) -> Script {
    corporate_figure_body(ctx, Vec::new()).map(|_| ())
}

fn corporate_figure_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Corporate Figure")])?;
    return Err(Stop::End);
}

pub fn corporate_figure_oninit(ctx: &Ctx) -> Script {
    corporate_figure_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn arunafeltz_figure_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn arunafeltz_figure(ctx: &Ctx) -> Script {
    arunafeltz_figure_body(ctx, Vec::new()).map(|_| ())
}

fn arunafeltz_figure_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Arunafeltz Figure")])?;
    return Err(Stop::End);
}

pub fn arunafeltz_figure_oninit(ctx: &Ctx) -> Script {
    arunafeltz_figure_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn corporate_figure_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("????", args!["Hmm... Good.", "Everything looks", "ready to me."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn corporate_figure_sch(ctx: &Ctx) -> Script {
    corporate_figure_sch_body(ctx, Vec::new()).map(|_| ())
}

fn arunafeltz_figure_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "??????",
        args![
            "I've got to say, only",
            "Rekenber can host such",
            "a magnificent party in",
            "a city this polluted.",
            "What's going on...?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn arunafeltz_figure_sch(ctx: &Ctx) -> Script {
    arunafeltz_figure_sch_body(ctx, Vec::new()).map(|_| ())
}

fn thin_faced_bard_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
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
    if ctx.var("que_sch").get()?.number()? < 19 {
        ctx.lines_as("????", args!["............"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("que_sch").get()? == 19 {
            ctx.lines_as(
                "Vitre",
                args![
                    "Thanks for your help.",
                    "This country's government",
                    "should know better than to",
                    "put a good man like me in jail.",
                    "Listen, I'm a fugitive now.",
                    "Do you think you can help me?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Sure.:I may as well...")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Vitre",
                args![
                    "I just need you to talk",
                    "to a few people, and let",
                    "me know what they said.",
                    "I'd do it myself, but you",
                    "understand that I've got",
                    "to keep a low profile."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args![
                    "Please meet these people",
                    "in the order that I tell you.",
                    "I've sent messengers to let",
                    "them know of your arrival,",
                    "but I had to send them out",
                    "at different times... Anyway..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args![
                    "First, go to Prontera",
                    "and talk to Chada. Second,",
                    "go to Geffen and speak to",
                    "Ghez. Lastly, please go to",
                    "Comodo and meet Nosdan."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args![
                    "Each one of them will",
                    "sing you a song. Listen",
                    "carefully, and sing me their",
                    "songs when you come back.",
                    "Thanks, you have no idea how",
                    "much I appreciate your help."
                ],
            )?;
            ctx.var("que_sch").set(Val::from(20))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("que_sch").get()? == 20 {
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Chada is probably near",
                        "the middle of Prontera.",
                        "The messenger I sent to",
                        "him should have reached",
                        "him by now, so he should",
                        "be expecting your arrival."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("que_sch").get()? == 21 {
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Ah, now you've got to",
                        "talk to Ghez. He should",
                        "be in Northeast Geffen",
                        "somewhere. Hopefully",
                        "you won't have too much",
                        "trouble finding him."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("que_sch").get()? == 22 {
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Trying to find Nosdan?",
                        "I think he's probably in the",
                        "Northern Cave in Comodo.",
                        "Please talk to him, and let",
                        "me know about his song."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("que_sch").get()? == 23 {
                ctx.lines_as("Vitre", args!["Welcome back.", "So did you listen", "to all of their songs?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                    ctx.lines_as(
                        "Vitre",
                        args!["Perfect~ Here's a little", "something to show my", "gratitude. Hope you like it."],
                    )?;
                    ctx.var("que_sch").set(Val::from(24))?;
                    ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                    ctx.next()?;
                    ctx.lines_as("Vitre", args!["Now, tell me, what", "exactly did they si--"])?;
                    ctx.next()?;
                    ctx.lines_as("????", args!["Hold it!"])?;
                    ctx.next()?;
                    ctx.call(Function::EnableNpc, vec![Val::from("????#sch1")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("????#sch2")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("????#sch3")])?;
                    ctx.lines_as("Vitre", args!["What? Wh-who the", "hell are you guys?!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "Rune-Midgarts",
                            "Secret Service!",
                            "Vitre Bizlleta--",
                            "you're under arrest",
                            "for espionage!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vitre",
                        args![
                            "Again? Didn't you just",
                            "arrest me just for being",
                            "suspected of espionage?",
                            "I think it's a little unfair",
                            "to just capture me when",
                            "you don't have any proof."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "We just seized concrete",
                            "evidence of your illegal",
                            "activities. It's probably",
                            "enough to imprison you",
                            "for life. Happy now?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Vitre", args!["You're bluffing."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "????",
                        args![
                            "You shouldn't have sent",
                            "that unsuspecting adventurer",
                            "to your news sources. We've",
                            "taken them into custody",
                            "Chada, Ghez, Nosdan.",
                            "That's them, right?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Vitre", args!["Nooooooooo!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suden",
                        args![
                            "Adventurer, thank you for",
                            "your cooperation. I am",
                            "Suden Griea, Secret Service",
                            "agent. I'm sure you have",
                            "a lot of questions, but ^FF0000Lasda",
                            "Midar^000000 will answer them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suden",
                        args![
                            "I'd explain here and now,",
                            "but I better dispose of",
                            "this trash with the rest of",
                            "his scum buddies... in jail!"
                        ],
                    )?;
                    ctx.var("que_sch").set(Val::from(25))?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("????#sch1")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("????#sch2")])?;
                    ctx.call(Function::DisableNpc, vec![Val::from("????#sch3")])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Would you please hurry?",
                        "Staying in one place like",
                        "this makes me nervous...",
                        "I should be on the move..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("que_sch").get()? == 24 {
                ctx.lines_as("Vitre", args!["Now, tell me, what", "exactly did they si--"])?;
                ctx.next()?;
                ctx.lines_as("????", args!["Hold it!"])?;
                ctx.next()?;
                ctx.call(Function::EnableNpc, vec![Val::from("????#sch1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("????#sch2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("????#sch3")])?;
                ctx.lines_as("Vitre", args!["What? Wh-who the", "hell are you guys?!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args![
                        "Rune-Midgarts",
                        "Secret Service!",
                        "Vitre Bizlleta--",
                        "you're under arrest",
                        "for espionage!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Again? Didn't you just",
                        "arrest me just for being",
                        "suspected of espionage?",
                        "I think it's a little unfair",
                        "to just capture me when",
                        "you don't have any proof."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args![
                        "We just seized concrete",
                        "evidence of your illegal",
                        "activities. It's probably",
                        "enough to imprison you",
                        "for life. Happy now?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Vitre", args!["You're bluffing."])?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args![
                        "You shouldn't have sent",
                        "that unsuspecting adventurer",
                        "to your news sources. We've",
                        "taken them into custody",
                        "Chada, Ghez, Nosdan.",
                        "That's them, right?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Vitre", args!["Nooooooooo!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Suden",
                    args![
                        "Adventurer, thank you for",
                        "your cooperation. I am",
                        "Suden Griea, Secret Service",
                        "agent. I'm sure you have",
                        "a lot of questions, but ^FF0000Lasda",
                        "Midar^000000 will answer them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Suden",
                    args![
                        "I'd explain here and now,",
                        "but I better dispose of",
                        "this trash with the rest of",
                        "his scum buddies... in jail!"
                    ],
                )?;
                ctx.var("que_sch").set(Val::from(25))?;
                ctx.close_window()?;
                ctx.call(Function::DisableNpc, vec![Val::from("????#sch1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("????#sch2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("????#sch3")])?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as(
        "Bard",
        args![
            "Oh, uh...",
            "I'm not really Vitre.",
            "I'm just a lookalike that's",
            "been planted here in case",
            "his hoodlum buddies come",
            "here to find him. You know?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Bard",
        args![
            "Yeah... This is a pretty",
            "dumb job. I mean, all I do",
            "is stand here, waiting for",
            "his spy friends. What are",
            "the odds of that happening?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thin_faced_bard_sch(ctx: &Ctx) -> Script {
    thin_faced_bard_sch_body(ctx, Vec::new()).map(|_| ())
}

fn sch1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn sch1(ctx: &Ctx) -> Script {
    sch1_body(ctx, Vec::new()).map(|_| ())
}

fn sch1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("????#sch1")])?;
    return Err(Stop::End);
}

pub fn sch1_oninit(ctx: &Ctx) -> Script {
    sch1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn sch2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn sch2(ctx: &Ctx) -> Script {
    sch2_body(ctx, Vec::new()).map(|_| ())
}

fn sch2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("????#sch2")])?;
    return Err(Stop::End);
}

pub fn sch2_oninit(ctx: &Ctx) -> Script {
    sch2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn sch3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn sch3(ctx: &Ctx) -> Script {
    sch3_body(ctx, Vec::new()).map(|_| ())
}

fn sch3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("????#sch3")])?;
    return Err(Stop::End);
}

pub fn sch3_oninit(ctx: &Ctx) -> Script {
    sch3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn young_man_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 20 {
        ctx.lines_as(
            "Chada",
            args!["What a boring day...", "Perhaps I'll stave the", "dreariness with song~"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 20 {
        ctx.lines_as(
            "Chada",
            args![
                "Are you the one that",
                "Vitre sent? Good, good.",
                "Let me treat you to my",
                "wonderful song. Listen...",
                "And learn... And love..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chada",
            args![
                "La la la la la la la la~",
                "A curse plagues the",
                "royal family, and it's",
                "been passed down to their",
                "second child... la la la...",
                "No one can cure it... Oooh~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chada",
            args![
                "Hahaha! Isn't that",
                "such a wonderful song?",
                "Let Vitre know each and",
                "every word to it, okay?"
            ],
        )?;
        ctx.var("que_sch").set(Val::from(21))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 30 {
        ctx.lines_as(
            "Chada",
            args!["Did you need to hear", "the song again? Alright,", "listen carefully this time~"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chada",
            args![
                "La la la la la la la la~",
                "A curse plagues the",
                "royal family, and it's",
                "been passed down to their",
                "second child... la la la...",
                "No one can cure it... Oooh~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chada",
            args![
                "Hahaha! Isn't that",
                "such a wonderful song?",
                "Let Vitre know each and",
                "every word to it, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Chada", args!["...........", "I have nothing to say to you."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_man_sch(ctx: &Ctx) -> Script {
    young_man_sch_body(ctx, Vec::new()).map(|_| ())
}

fn young_woman_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 21 {
        ctx.lines_as("Ghez", args!["When is he going", "to send someone to", "listen to my song?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 21 {
        ctx.lines_as(
            "Ghez",
            args![
                "Oh, Vitre send you?",
                "Great, I've been waiting",
                "for you. Check out this",
                "new song I wrote. It's great."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args![
                "Sha la la la la la",
                "Prontera Knights gotta",
                "protect the palace~",
                "Sha hoo hoo haaaaa",
                "Geffen Knights gotta",
                "protect the magic tower~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args![
                "Na na na nan nan naaa",
                "Prontera! Geffen! Knights",
                "together! Protect the palace!",
                "Ooooooooooooh yeah!",
                "And crush their foes to the west!",
                "That's the secret plan! La la la~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args!["Wasn't that poetic?", "Now sing that song exactly", "as you heard it to Vitre."],
        )?;
        ctx.var("que_sch").set(Val::from(22))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 30 {
        ctx.lines_as(
            "Ghez",
            args![
                "You want to hear my song",
                "again? Fine, but make sure",
                "to memorize it all this time."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args![
                "Sha la la la la la",
                "Prontera Knights gotta",
                "protect the palace~",
                "Sha hoo hoo haaaaa",
                "Geffen Knights gotta",
                "protect the magic tower~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args![
                "Na na na nan nan naaa",
                "Prontera! Geffen! Knights",
                "together! Protect the palace!",
                "Ooooooooooooh yeah!",
                "And crush their foes to the west!",
                "That's the secret plan! La la la~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ghez",
            args!["Wasn't that poetic?", "Now sing that song exactly", "as you heard it to Vitre."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Ghez", args!["I'm doomed...", "......."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_woman_sch(ctx: &Ctx) -> Script {
    young_woman_sch_body(ctx, Vec::new()).map(|_| ())
}

fn young_man_sch2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 22 {
        ctx.lines_as(
            "Nosdan",
            args![
                "Have you come to hear",
                "my song? Ah, it's good",
                "that you've come to enjoy",
                "my melodious vo--wait,",
                "wait, where are you going?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 22 {
        ctx.lines_as(
            "Nosdan",
            args![
                "Ah, are you the one",
                "that Vitre mentioned",
                "in his message? Okay,",
                "please give me a moment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args!["^333333*Ahem ahem*^000000", "I'm ready. Now", "please listen.."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "Who dares stop the fearless",
                "warrior? Baphomet? Drake?",
                "No, they're too weak! His",
                "steps now head to the group",
                "of evil men trying to revive",
                "Satan Morocc. I mean, come on!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "Sad news from an old comrade,",
                "the evil group discovered...",
                "Time to stop Satan Morocc's",
                "revival~ La la la la la la la",
                "la la la la la la la la la la",
                "la la la la la la la la la la"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "What'd you think?",
                "Um, don't ask me about",
                "the lyrics. Just a weird",
                "artistic quirk I guess.",
                "Oh, please sing that song",
                "to Vitre for me, okay?"
            ],
        )?;
        ctx.var("que_sch").set(Val::from(23))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 30 {
        ctx.lines_as(
            "Nosdan",
            args![
                "Oh, you need to",
                "hear my song again?",
                "No problem! I guess",
                "you really like it~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "Who dares stop the fearless",
                "warrior? Baphomet? Drake?",
                "No, they're too weak! His",
                "steps now head to the group",
                "of evil men trying to revive",
                "Satan Morocc. I mean, come on!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "Sad news from an old comrade,",
                "the evil group discovered...",
                "Time to stop Satan Morocc's",
                "revival~ La la la la la la la",
                "la la la la la la la la la la",
                "la la la la la la la la la la"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nosdan",
            args![
                "What'd you think?",
                "Um, don't ask me about",
                "the lyrics. Just a weird",
                "artistic quirk I guess.",
                "Oh, please sing that song",
                "to Vitre for me, okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as("Nosdan", args!["Oh, no! I didn't!", "do anything wrong.", "I swear!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn young_man_sch2(ctx: &Ctx) -> Script {
    young_man_sch2_body(ctx, Vec::new()).map(|_| ())
}

fn extra_story_patch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Patch", args!["Yeah, you can try."])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Now:How many?")])?) == 2 {
        let (input, status) = runtime::input_number(ctx, Some(0), Some(1000))?;
        l_input = input;
        ctx.var("que_sch").set(l_input.clone())?;
    }
    ctx.lines(args![" ", (Val::from("") + ctx.var("que_sch").get()?)])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn extra_story_patch(ctx: &Ctx) -> Script {
    extra_story_patch_body(ctx, Vec::new()).map(|_| ())
}
