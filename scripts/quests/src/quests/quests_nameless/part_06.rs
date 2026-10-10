use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn wola_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(1)])? == 0 {
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
    if ctx.var("diamond_edq").get()? == 18 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Hello? I'm here to", "see Doctor Wola."],
        )?;
        ctx.next()?;
        ctx.lines_as("Wola", args!["Oh... Ah... Um..."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Excuse me, but do you", "know where I can find her?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Wola", args!["I, um...", "Oh no! Wah!", "See what you did?!"])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I... I don't know",
                "what you're talking",
                "about. I'm just wanted",
                "to talk to Doctor Wola."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args![
                "I'm Doctor Wola!",
                "And look! You made me",
                "drop the herbal medicine",
                "I was brewing! The pot",
                "is all broken now..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Huh? Oh, I'm...", "I'm really sorry."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args!["That's... That's", "quite alright. It's", "my fault, really.", "Waaah~! My pot!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["I'm sorry, I don't know", "what's going on exactly.", "Was that pot expensive?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args![
                "Oh, I'm acting stupid",
                "again! I don't have time",
                "to cry over a broken pot!",
                "I need to check my hands",
                "for any cuts! If they're",
                "hurt, I can't help anyone!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "That's, uh, that",
                "sounds pretty serious.",
                "So how are your hands?",
                "They don't hurt, do they?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args![
                "Oooh, never mind that!",
                "They're fine, thank goodness.",
                "What's important now is that",
                "you help me fix my herbal",
                "medicine pot. You're partly",
                "responsible, you know."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Okay. Just tell",
                "me what I should do.",
                "Hold on, I came here to",
                "ask you if you'd give me",
                "some medicine. You see--"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args![
                "Perfect! All the more",
                "reason you should help",
                "me fix this pot! I can't",
                "make any medicine without",
                "this pot now, can I? Let's",
                "see... I'll need glue, glue..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wola",
            args![
                "Hurry and get",
                "^0000FF20 Fine Sands^000000,",
                "^0000FF5 Empty Bottles^000000,",
                "^0000FF10 Brigans^000000, and",
                "^0000FF10 Soft Blades of Grass^000000.",
                "There's no time to waste!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["You... You got it."],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3111), Val::from(3112)])?;
        ctx.var("diamond_edq").set(Val::from(19))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 19 {
        ctx.lines_as(
            "Wola",
            args![
                "Good, you're back.",
                "So you brought everything",
                "I need to glue my pot",
                "back together again?"
            ],
        )?;
        ctx.next()?;
        if (((ctx.call(Function::CountItem, vec![Val::from(7043)])?.number()? > 19
            && ctx.call(Function::CountItem, vec![Val::from(7054)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(7194)])?.number()? > 9)
            && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 4)
        {
            ctx.lines_as(
                "Wola",
                args![
                    "Great. Give them here.",
                    "I should grind this, mix",
                    "that... Wait, was that right?",
                    "What were these for again?",
                    "Argh, I'm acting stupid again!",
                    "You mind not watching me?"
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7043), Val::from(20)])?;
            ctx.call(Function::DelItem, vec![Val::from(7054), Val::from(10)])?;
            ctx.call(Function::DelItem, vec![Val::from(7194), Val::from(10)])?;
            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(5)])?;
            ctx.next()?;
            ctx.lines_as(
                "Wola",
                args![
                    "Just... Just sit in",
                    "that corner, don't say",
                    "anything, and wait till",
                    "I say I'm done, okay?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wola",
                args![
                    "...............................",
                    "...............................",
                    "..............................."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wola",
                args![
                    "...............................",
                    "...............................",
                    "...............................",
                    "..............................."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wola",
                args![
                    "...............................",
                    "...............................",
                    "...............................",
                    "...............................",
                    "..............................."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wola",
                args![
                    "There. It's finally",
                    "fixed. Oh wait, did you",
                    "say something about needing",
                    "some medicine? I completely",
                    "forgot about that for a while.",
                    "What did you say exactly?"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3112), Val::from(3113)])?;
            ctx.var("diamond_edq").set(Val::from(20))?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Will you go out with me?:Leblo's back medicine?")])? {
                1 => {
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.lines_as(
                            "Wola",
                            args![
                                "^666666*Blush*^000000 But it's too",
                                "early for us to just go",
                                "on a date. I'm sorry, it's...",
                                "I have someone in mind.",
                                "Still, I'm flattered that",
                                "you think of me that way."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Wola",
                            args![
                                "Wow, I can't believe",
                                "I'm turning you down--",
                                "a little surprised,",
                                "actually. But yes.",
                                "I have someone I like.",
                                "And it's not really you. Sorry."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Wola",
                        args![
                            "Leblo... Oh, that",
                            "man in Geffen? Finally...",
                            "He has the sense to send",
                            "someone to get his meds.",
                            "I already prepared it.",
                            "Where did I put it now?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wola",
                        args![
                            "I want to double check",
                            "his medicine first so",
                            "please tell him that I'll",
                            "have it delivered to him",
                            "shortly. In a while, maybe?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Um, you're not mad or", "anything? I gathered that", "you didn't like him much..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wola",
                        args![
                            "What are saying?",
                            "I'm a doctor: we're duty",
                            "bound to heal everyone we can!",
                            "That's not to say that some",
                            "patients suffer longer and",
                            "more painful treatment..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "(^666666She hates him!",
                            "But I guess Leblo's life",
                            "should be okay in her hands.^000000)"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wola",
                        args![
                            "Alright, just let Leblo",
                            "know that I'm taking care",
                            "of him, that his life should",
                            "be okay in my hands. I need",
                            "to get back to work: please",
                            "take care of yourself!"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3113), Val::from(3114)])?;
                    ctx.var("diamond_edq").set(Val::from(21))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Wola",
                args![
                    "Hurry and get",
                    "^0000FF20 Fine Sands^000000,",
                    "^0000FF5 Empty Bottles^000000,",
                    "^0000FF10 Brigans^000000, and",
                    "^0000FF10 Soft Blades of Grass^000000.",
                    "There's no time to waste!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["You... You got it."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("diamond_edq").get()? == 20 {
        ctx.lines_as(
            "Wola",
            args![
                "Sorry, I completely",
                "missed what you said",
                "earlier. Something about",
                "needing medicine from me?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Will you go out with me?:Leblo's back medicine?")])? {
            1 => {
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines(args![
                        "^666666*Blush*^000000 But it's too",
                        "early for us to just go",
                        "on a date. I'm sorry, it's...",
                        "I have someone in mind.",
                        "Still, I'm flattered that",
                        "you think of me that way."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Wola",
                        args![
                            "Wow, I can't believe",
                            "I'm turning you down--",
                            "a little surprised,",
                            "actually. But yes.",
                            "I have someone I like.",
                            "And it's not really you. Sorry."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Wola",
                    args![
                        "Leblo... Oh, that",
                        "man in Geffen? Finally...",
                        "He has the sense to send",
                        "someone to get his meds.",
                        "I already prepared it.",
                        "Where did I put it now?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wola",
                    args![
                        "I want to double check",
                        "his medicine first so",
                        "please tell him that I'll",
                        "have it delivered to him",
                        "shortly. In a while, maybe?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Um, you're not mad or", "anything? I gathered that", "you didn't like him much..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wola",
                    args![
                        "What are saying?",
                        "I'm a doctor: we're duty",
                        "bound to heal everyone we can!",
                        "That's not to say that some",
                        "patients suffer longer and",
                        "more painful treatment..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "(^666666She hates him!",
                        "But I guess Leblo's life",
                        "should be okay in her hands.^000000)"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wola",
                    args![
                        "Alright, just let Leblo",
                        "know that I'm taking care",
                        "of him, that his life should",
                        "be okay in my hands. I need",
                        "to get back to work: please",
                        "take care of yourself!"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3113), Val::from(3114)])?;
                ctx.var("diamond_edq").set(Val::from(21))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Wola",
        args![
            "Food has a dramatic",
            "effect on your body:",
            "treat it like a drug, and",
            "watch what you eat. Oh,",
            "and do your research!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Wola",
        args![
            "You know, your lifestyle",
            "and habits determine",
            "your health in the future.",
            "Take care of yourself!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wola(ctx: &Ctx) -> Script {
    wola_body(ctx, Vec::new()).map(|_| ())
}

fn wola_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("diamond_edq").get()? == 17 {
        ctx.lines_as("Wola", args!["Oh, how can this be", "happening? What ", "should I do?"])?;
        ctx.var("diamond_edq").set(Val::from(18))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn wola_ontouch(ctx: &Ctx) -> Script {
    wola_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_investigator_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 22 && ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?)) {
        ctx.lines_as(
            "Investigator",
            args![
                "Oh, remember me?",
                "It's been a long time",
                "so I can't blame you:",
                "I was there when you",
                "first joined our guild!",
                "So what brings you here?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Ask About the Z Gang")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Investigator", args!["Ah, Z Gang, you said?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "Oh, yeah, we're investigating",
                "those guys. We actually found",
                "their hideout, but then they",
                "escaped before we could even",
                "catch them. Wily bastards...",
                "I think they were tipped off!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "A few of our Rogue agents",
                "are investigating the hideout,",
                "but I doubt they can find",
                "anything useful there."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Where's the hideout?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Investigator",
            args![
                "Well, it's a large",
                "warehouse over in Comodo.",
                "You're free to scope out",
                "the place if you really want.",
                "But chances are slim that",
                "you'll find anything at all."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("diamond_edq").get()? == 22 && ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?)) {
        ctx.lines_as(
            "Investigator",
            args![
                "Hey, how's it going?",
                "You ever consider joining",
                "the Rogues? It'd be a lot",
                "of fun. You're welcome to",
                "look around, so feel free",
                "to ask me any questions."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Ask About the Z Gang")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Investigator",
            args![
                "Oh, yeah, we're investigating",
                "those guys. We actually found",
                "their hideout, but then they",
                "escaped before we could even",
                "catch them. Wily bastards...",
                "I think they were tipped off!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "A few of our Rogue agents",
                "are investigating the hideout,",
                "but I doubt they can find",
                "anything useful there."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Where's the hideout?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Investigator",
            args![
                "Well, I'm not sure if",
                "I can just tell you that",
                "kind of information. I mean,",
                "you're not a member of our",
                "guild... Yet. What can I do?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "I got it. I'll just tell my",
                "boss that you bribed me",
                "with 10,000 zeny. Not too",
                "much, is it? We Rogues are",
                "pretty bad... But not that bad."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sounds good!:Never mind, I'll find it myself...")])? {
            1 => {
                if ctx.var("Zeny").get()?.number()? > 9999 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                    ctx.lines_as(
                        "Investigator",
                        args![
                            "Thanks for the money~",
                            "Of course, if you became",
                            "a Rogue, this amount is",
                            "nothing compared to what",
                            "you can gank. ^666666*Ahem*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Investigator",
                        args![
                            "The Z Gang's old hideout",
                            "is a large warehouse over",
                            "in Comodo. I doubt you'll",
                            "find anything there, but",
                            "you can go check it out. "
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3115), Val::from(3116)])?;
                    ctx.var("diamond_edq").set(Val::from(23))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Investigator",
                        args![
                            "Aw, what am I gonna",
                            "do if my boss askes to",
                            "see the money? We've got",
                            "to make this bribe authentic!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Investigator",
                    args![
                        "Be my guest, maybe it'll be",
                        "a good learning experience.",
                        "But yeah, come back when",
                        "you think about changing",
                        "your job, okay? Be a Rogue~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("diamond_edq").get()? == 22 {
        ctx.lines_as(
            "Investigator",
            args![
                "Whoa, what are you doin'",
                "here? I've got no beef with",
                "you, but some Rogues, see,",
                "some Rogues, have got a ",
                "lot of beef. Uh, what'd you",
                "want? Not a fight, I hope."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Ask About the Z Gang")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Investigator",
            args![
                "Those guys have been",
                "bothering everyone!",
                "Yeah, I guess you can",
                "say we're united on this."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "Oh, yeah, we're investigating",
                "those guys. We actually found",
                "their hideout, but then they",
                "escaped before we could even",
                "catch them. Wily bastards...",
                "I think they were tipped off!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Investigator",
            args![
                "A few of our Rogue agents",
                "are investigating the hideout,",
                "but I doubt they can find",
                "anything useful there."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Where's the hideout?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Investigator",
            args![
                "Uh, I can't just give",
                "you that information",
                "if you're not a member",
                "of our guild. But since",
                "we're together on this,",
                "I'll just ask for 10,000 zeny."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sounds good!:Forget it...")])? {
            1 => {
                if ctx.var("Zeny").get()?.number()? > 9999 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                    ctx.lines_as(
                        "Investigator",
                        args![
                            "Right on, right on.",
                            "Alright, I didn't charge",
                            "you much since the hideout",
                            "is a large warehouse in",
                            "Comodo where I doubt you'll",
                            "find anything. But who knows? "
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3115), Val::from(3116)])?;
                    ctx.var("diamond_edq").set(Val::from(23))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Investigator",
                        args![
                            "Hey, this isn't enough",
                            "money. I thought people",
                            "in your job made more",
                            "zeny than this everyday!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Investigator",
                    args![
                        "Alright, but I think",
                        "you'll need a little",
                        "extra help finding that",
                        "place. I mean, Rogues are",
                        "hiding experts, and even",
                        "we had some trouble!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as(
        "Investigator",
        args![
            "Geez, I'm so sleepy...",
            "Everyone's out on a mission,",
            "and I'm stuck here on guard",
            "duty. When can I see some",
            "action again? Ugh! Rogues",
            "hate standing in one place!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rogue_investigator(ctx: &Ctx) -> Script {
    rogue_investigator_body(ctx, Vec::new()).map(|_| ())
}

fn investigator_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 22 || ctx.var("diamond_edq").get()? == 23) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
            ctx.lines_as(
                "Investigator",
                args![
                    "Hey, pal. You part",
                    "of the guild? Nice.",
                    "If they sent you to help",
                    "out, well, there's nothing",
                    "to do here. This place is",
                    "pretty much cleaned out."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Investigator",
                args![
                    "What do you want?",
                    "Oh, your guild musta",
                    "sent you to help out.",
                    "Yeah. We investigated.",
                    "Didn't find anything.",
                    "The Z Gang's long gone."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Investigator",
                args!["Feel free to look", "around, but if you ask", "me, it's a waste of time."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Investigator",
            args![
                "Yeesh, I'm stuck here",
                "till I receive my new",
                "orders from the guild.",
                "They didn't... They didn't",
                "forget about me, did they?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn investigator(ctx: &Ctx) -> Script {
    investigator_body(ctx, Vec::new()).map(|_| ())
}

fn small_safe_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 22 || ctx.var("diamond_edq").get()? == 23) {
        ctx.lines(args![
            "^3355FFThere is a small",
            "safe hidden under the",
            "shadows of these boxes.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Looks like the Z Gang",
                "forgot to take this with",
                "them in their rush to escape.",
                "Hmmm... How do I open this?",
                "Is there a key, a switch,",
                "something I can use?"
            ],
        )?;
        ctx.call(Function::EnableNpc, vec![Val::from("Odd Switch#Switch1")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 24 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Hey, the safe opened!",
                "Those switches must have",
                "done the trick. Now...",
                "Let's look inside."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Awesome! It's a huge",
                "red diamond! Is this",
                "Ibrahim's Diamond",
                "of Destruction?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::GetItem, vec![Val::from(7723), Val::from(1)])?;
        ctx.lines_as("Investigator", args!["Zzz... Zzz..."])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Lucky for me, this",
                "guy is asleep. I better",
                "sneak out of here, and",
                "return this big diamond",
                "to Ibrahim as soon as I can."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3117), Val::from(3118)])?;
        ctx.var("diamond_edq").set(Val::from(25))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()?.number()? < 22 && ctx.var("diamond_edq").get()?.number()? > 24) {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "There was something here",
                "fixed to the ground, but now",
                "it looks like something yanked",
                "it out forcefully. How weird."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["What is this?", "Oh well, I don't", "think it's important."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn small_safe(ctx: &Ctx) -> Script {
    small_safe_body(ctx, Vec::new()).map(|_| ())
}

fn odd_switch_switch1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 22 || ctx.var("diamond_edq").get()? == 23) {
        ctx.lines(args![
            "^3355FFThere is a tiny",
            "switch on the ground",
            "near the whiskey barrels.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Was this here before?", "What happens if I press", "this teeny little switch?"],
        )?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("Odd Switch#Switch2")])?;
        ctx.mes("^3355FF*Click Click*^000000")?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That sound...!", "Something happened,", "but what could it be?"],
        )?;
        ctx.call(Function::DisableNpc, vec![Val::from("Odd Switch#Switch1")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn odd_switch_switch1(ctx: &Ctx) -> Script {
    odd_switch_switch1_body(ctx, Vec::new()).map(|_| ())
}

fn odd_switch_switch1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Odd Switch#Switch1")])?;
    return Err(Stop::End);
}

pub fn odd_switch_switch1_oninit(ctx: &Ctx) -> Script {
    odd_switch_switch1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn odd_switch_switch2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("diamond_edq").get()? == 22 || ctx.var("diamond_edq").get()? == 23) {
        ctx.lines(args![
            "^3355FFThere is a tiny",
            "switch on the ground",
            "under the boxes' shadows.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Was this here before?", "What happens if I press", "this teeny little switch?"],
        )?;
        ctx.next()?;
        ctx.mes("^3355FF*Click Click*^000000")?;
        ctx.next()?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3116), Val::from(3117)])?;
        ctx.var("diamond_edq").set(Val::from(24))?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That sound...!", "Something happened,", "but what could it be?"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Odd Switch#Switch1")])?;
        ctx.call(Function::DisableNpc, vec![Val::from("Odd Switch#Switch2")])?;
    }
    return Err(Stop::End);
}

pub fn odd_switch_switch2(ctx: &Ctx) -> Script {
    odd_switch_switch2_body(ctx, Vec::new()).map(|_| ())
}

fn odd_switch_switch2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Odd Switch#Switch2")])?;
    return Err(Stop::End);
}

pub fn odd_switch_switch2_oninit(ctx: &Ctx) -> Script {
    odd_switch_switch2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wanted_notice_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zdan_edq").get()? == 0 {
        ctx.lines(args![
            "====National Wanted Notice====",
            "Please report any information",
            "regarding these criminals to",
            "the Rune-Midgarts Kingdom's",
            "Homeland Security Office."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "^0000ffLouis Von Silokens^000000 - Human Male",
            "^0000ffMartha Hertizan^000000 - Human Female",
            "^0000ffCatfoii^000000 - Pet Cat"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "These infamous members",
            "of the Z Gang are suspected",
            "of stealing national treasure,",
            "committing fraud, forgery,",
            "and promoting overall",
            "depravity and immorality."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "Anyone that captures",
            "the listed criminals will",
            "be rewarded with the Book",
            "of Forbidden Mystery, one",
            "of the kingdom's treasures."
        ])?;
        ctx.call(Function::SetQuest, vec![Val::from(3119)])?;
        ctx.var("zdan_edq").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zdan_edq").get()?.number()? > 18 {
        ctx.lines_as(
            "Chief Officer",
            args![
                "Ever since you captured",
                "the Z Gang, crime has gone",
                "down to an all time low.",
                "Thanks for helping us out~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chief Officer",
            args![
                "Peace never lasts long...",
                "Some other fiends will",
                "replace the Z Gang soon.",
                "When that happens, I hope",
                "we can count on you again."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "====National Wanted Notice====",
            "Please report any information",
            "regarding these criminals to",
            "the Rune-Midgarts Kingdom's",
            "Homeland Security Office."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "^0000ffLouis Von Silokens^000000 - Human Male",
            "^0000ffMartha Hertizan^000000 - Human Female",
            "^0000ffCatfoii^000000 - Pet Cat"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "These infamous members",
            "of the Z Gang are suspected",
            "of stealing national treasure,",
            "committing fraud, forgery,",
            "and promoting overall",
            "depravity and immorality."
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "====National Wanted Notice====",
            "Anyone that captures",
            "the listed criminals will",
            "be rewarded with the Book",
            "of Forbidden Mystery, one",
            "of the kingdom's treasures."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn wanted_notice_edq(ctx: &Ctx) -> Script {
    wanted_notice_edq_body(ctx, Vec::new()).map(|_| ())
}

fn wanted_notice_edq_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Wanted Notice#edq")])?;
    return Err(Stop::End);
}

pub fn wanted_notice_edq_onenable(ctx: &Ctx) -> Script {
    wanted_notice_edq_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wanted_notice_edq_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Wanted Notice#edq")])?;
    return Err(Stop::End);
}

pub fn wanted_notice_edq_ondisable(ctx: &Ctx) -> Script {
    wanted_notice_edq_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn chief_officer_edq_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Chief Officer",
            args![
                "You're carrying far too",
                "much right now. Please",
                "lighten your load by",
                "placing your items",
                "in the Kafra Storage."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("zdan_edq").get()? == 0 && ctx.var("BaseLevel").get()?.number()? > 70) {
        ctx.lines_as(
            "Chief Officer",
            args![
                "Adventurers, this is",
                "your chance to protect",
                "and serve your country!",
                "Please read the National",
                "Wanted Notice for more details."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("zdan_edq").get()? == 1 && ctx.var("BaseLevel").get()?.number()? > 70) {
        ctx.lines_as(
            "Chief Officer",
            args![
                "Attention, attention.",
                "The Homeland Security Office",
                "of the Rune-Midgarts Kingdom",
                "is looking for any information",
                "regarding the Z Gang."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chief Officer",
            args![
                "There are three members:",
                "Louis Von Silokens a.k.a.",
                "Louis, Martha Hertizan a.k.a.",
                "Martha, and Catfoii a.k.a.",
                "Catfoii must be captured!"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3119), Val::from(3120)])?;
        ctx.var("zdan_edq").set(Val::from(2))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("zdan_edq").get()? == 1 && ctx.var("BaseLevel").get()?.number()? < 70) {
        ctx.lines_as(
            "Chief Officer",
            args![
                "Oh, were you interested",
                "in pursuing the criminals",
                "listed in the Wanted notice?",
                "I'm sorry, but you're not",
                "ready for that kind of task...",
                "But I encourge you to train!",
                "Thank you."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("zdan_edq").get()? == 2 {
        ctx.lines_as("Chief Officer", args!["Hello, Rune-Midgartian.", "What brings you here?"])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("National Wanted Notice:Z Gang:What crimes did Z Gang commit?")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "Ah, were you interested",
                        "in pursuing the Z Gang?",
                        "Please carefully read the",
                        "Wanted notice posted right",
                        "next to me. We desperately",
                        "need help to catch them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "If you want to learn",
                        "more about the Z Gang,",
                        "I suggest talking to",
                        "a knight named Valdes.",
                        "He's fairly familiar with",
                        "their brand of antics."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "Unfortunately, I fear",
                        "that he may be ashamed",
                        "of his failure to catch",
                        "them. However, I am sure",
                        "that he will be of service",
                        "in your quest for justice."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "You wanted to know more",
                        "about the Z Gang members?",
                        "I don'tknow much about them,",
                        "but I can divulge what",
                        "little I've heard."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Louis:Martha:Catfoii:Book of Forbidden Mystery?")])? {
                    1 => {
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "Louis Von Silokens was",
                                "the third son of the noble",
                                "Silokens family which was",
                                "ruined many years ago.",
                                "According to record, he failed",
                                "Magic Academy three times."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "It is believed that he formed",
                                "the Z Gang as a result of",
                                "his personal failure. He",
                                "also stole the Book of",
                                "Forbidden Mystery from",
                                "the Royal Library."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "We must capture him and",
                                "retrieve that book before",
                                "he can abuse its power.",
                                "We need the help of the",
                                "people in order to find him",
                                "and bring Louis to justice!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "Martha Hertizan is the",
                                "second daughter of Alberta's",
                                "Hertizan family. No one knows",
                                "why she ran away from home",
                                "at the age of twenty."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "Martha handles the Z Gang's",
                                "finances. It's a shame that",
                                "such a talented merchant",
                                "has stooped to thievery. "
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "We initially believed",
                                "that Catfoii was just",
                                "Louis and Martha's pet",
                                "cat... But it's actually a",
                                "master criminal. Approach",
                                "it with extreme caution!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "That's all I know about",
                                "that animal. The true",
                                "nature of Catfoii is",
                                "shrouded in mystery."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Chief Officer",
                            args![
                                "Out of all their crimes,",
                                "their theft of the Book of",
                                "Forbidden Mystery poses the",
                                "greatest danger to the kingdom.",
                                "If they misuse that book, it",
                                "will cause grave disaster."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "That damned Z Gang seems",
                        "to be in the center of all the",
                        "crime that's happening in",
                        "Rune-Midgarts. They caused",
                        "a riot in Geffen by spreading",
                        "rumors about ghosts..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "They poisoned the",
                        "drinking water in the",
                        "wells of Payon, seized",
                        "countless goods, profited",
                        "off illegal trade... The list",
                        "just keeps going on and on..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Chief Officer",
                    args![
                        "Out of all their crimes,",
                        "their theft of the Book of",
                        "Forbidden Mystery poses the",
                        "greatest danger to the kingdom.",
                        "If they misuse that book, it",
                        "will cause grave disaster."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    if ctx.var("zdan_edq").get()? == 18 {
        ctx.lines_as(
            "Chief Officer",
            args![
                "Oh, you were that nice",
                "adventurer that dropped",
                "by the other day. How",
                "may I help you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chief Officer",
            args![
                "What's that?! You really",
                "found the Z Gang's hideout,",
                "captured them, and retrieved",
                "the forbidden book?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7724), Val::from(1)])?;
        ctx.lines_as(
            "Chief Officer",
            args![
                "Splendid! Just splendid!",
                "On behalf of the king,",
                "let me give you a well",
                "deserved reward. Thank",
                "you for your work on",
                "behalf of our kingdom!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chief Officer",
            args![
                "Your bravery and ardent",
                "patriotism will forever",
                "be recorded in the annals",
                "of Rune-Midgarts history.",
                "Congratulations! "
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(3134)])?;
        ctx.var("zdan_edq").set(Val::from(19))?;
        ctx.call(Function::GetExperience, vec![Val::from(1000000), Val::from(0)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Chief Officer",
        args![
            "All those that oppose",
            "the peace and safety of",
            "our kingdom will not be",
            "spared the fury of our",
            "righteous swords!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chief_officer_edq(ctx: &Ctx) -> Script {
    chief_officer_edq_body(ctx, Vec::new()).map(|_| ())
}

fn valdes_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(100)])? == 0 {
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
    if ctx.var("zdan_edq").get()? == 2 {
        ctx.lines_as(
            "Valdes",
            args![
                "^666666*Urp*^000000 What?",
                "You here to... ",
                "Um, commiserate",
                "about failing life too?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Are you Valdes?", "I came to ask you", "for your help..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valdes",
            args!["^666666*Hiccup*^000000 Why~?", "I'm-I'm no good", "to anybody anymore!"],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Ask About Z Gang:The chief officer is worried about you.:Bye!")],
        )? {
            1 => {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "That Z Gang...",
                        "Ruined my life!",
                        "My career, my pension...",
                        "It's all gone because",
                        "of those damn criminals!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "You? Capture them?",
                        "Good luck! Just... Just",
                        "try not to end up like me.",
                        "Oh, you want info-information?",
                        "^666666*Hic*^000000 Heh, scratch my back,",
                        "well, you know the rest."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Heh, I just ran out of wine!",
                        "Gimme, um, ^0000FFMorocc Fruit Wine^000000!",
                        "5 bottles! Then... Maybe...",
                        "I'll tell you want you really",
                        "wanna know. Heh heh~ ^666666*Hic!*^000000"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3120), Val::from(3121)])?;
                ctx.var("zdan_edq").set(Val::from(3))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Valdes",
                    args!["R-really? Well...", "I don't care! Just", "let me drink in peace!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Wha--? Who do you think",
                        "you're messing with, ^666666*Hic!*^000000",
                        "just comin' by to say ''bye?!''",
                        "I useta be a knight once,",
                        "you know that? I could",
                        "totally kick your--^666666*Urp!*^000000"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("zdan_edq").get()? == 3 {
        ctx.lines_as("Valdes", args!["Heeeey yoooou~", "You bring me wine?"])?;
        ctx.next()?;
        if ctx.call(Function::CountItem, vec![Val::from(12049)])?.number()? > 4 {
            ctx.call(Function::DelItem, vec![Val::from(12049), Val::from(5)])?;
            ctx.lines_as(
                "Valdes",
                args![
                    "^666666*Sniff*^000000 Yeeesh, that",
                    "smeells good. This must",
                    "be it. Oh, yeah. Knight's",
                    "honor. First I talk, then",
                    "I can drink theesh..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args!["Well, maybe...", "Just one tashte~", "^666666*Gulp Gulp Gulp*^000000"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["You just...", "That was three bottles!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args!["Ah, delicious!", "Alright, fellow,", "now I feel better", "able to speak now~"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["(^666666Whaaaat~?^000000)"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "I assume you already know",
                    "the basic information about",
                    "the Z Gang, their members,",
                    "et cetera. I'm not sure why",
                    "they've gone on such a brazen",
                    "crime spree so suddenly."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "Their success is unprecedented:",
                    "they've even managed to elude",
                    "the Rogue Guild repeatedly.",
                    "I investigated them six",
                    "months ago, but I was",
                    "dismissed for my failure."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "I am aware that the Z Gang",
                    "has many informers in their",
                    "employ, and that the Rogue",
                    "Guild are still investigating",
                    "the Z Gang on their own."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "I suggest meeting one",
                    "of the Rogue agents,",
                    "Marybell, in the Rogue",
                    "Guild. The password you",
                    "must give her is ''^0000FFThe",
                    "dawn is yet to come^000000.''"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "That is all I can tell you.",
                    "Please give my regards to",
                    "Marybell for me, and catch",
                    "those Z Gang bastards.",
                    "^666666*Sigh*^000000 I think I'll go rest",
                    "a bit now. Good luck!"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(3121), Val::from(3122)])?;
            ctx.var("zdan_edq").set(Val::from(4))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Valdes",
                args![
                    "You stinkface!",
                    "Where's my wine?!",
                    "My ^0000FF5 bottles of",
                    "Morocc Fruit Wine^000000?!",
                    "I'm not gonna talk",
                    "to you without it!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("zdan_edq").get()? == 4 {
        ctx.lines_as(
            "Valdes",
            args![
                "I suggest meeting",
                "Marybell in the Rogue",
                "Guild, and giving her the",
                "password, ''^0000FFThe dawn",
                "is yet to come^000000.''"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Valdes",
        args![
            "^666666*Sigh*^000000 It is my greatest",
            "wish that the knight master",
            "will allow me to rejoin the",
            "corps. However, I doubt that",
            "he's forgiven my failure. ^666666*Sob*^000000",
            "It's like my life is over..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn valdes(ctx: &Ctx) -> Script {
    valdes_body(ctx, Vec::new()).map(|_| ())
}

fn valdes_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zdan_edq").get()? == 2 {
        ctx.lines_as(
            "Valdes",
            args![
                "^666666*Sigh*^000000 Goddamn thieves.",
                "Why should I even care",
                "about them?! They're not",
                "my problem anymore...",
                "^666666*Hiccup*^000000"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn valdes_ontouch(ctx: &Ctx) -> Script {
    valdes_ontouch_body(ctx, Vec::new()).map(|_| ())
}
