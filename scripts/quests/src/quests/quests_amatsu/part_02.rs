use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn soldier_ama2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jiro",
        args!["I'm Jiro, the administrator", "of this Palace. What can I do for you?"],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Issue me a Transit Permit:I want to live in Amatsu:Nothing")])? {
        1 => {
            if ctx.var("event_amatsu").get()? == 6 {
                if ctx.call(Function::CountItem, vec![Val::from(7160)])?.number()? > 0 {
                    ctx.lines_as("Jiro", args!["You already have one...", "You don't need to have two of them."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Jiro",
                    args![
                        "Did you lose your Transit Permit?",
                        "You need to pay 10,000 zeny as a tax",
                        "to issue another Transit Permit."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Issue one:Talk to you later")])?) == 1 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                    ctx.call(Function::GetItem, vec![Val::from(7160), Val::from(1)])?;
                    ctx.lines_as("Jiro", args!["There you go.", "Don't lose it this time."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Jiro", args!["Okay, then...", "Talk to me when you need help."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Gate Soldier",
                args![
                    "The lord announced that",
                    "he grants guests from Midgard the right to go anywhere.",
                    "You don't need me to issue you a Transit Permit."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Jiro",
                args![
                    "That is impossible. You're free to",
                    "go anywhere in Amatsu, but my lord doesn't want to concern himself with immigration.",
                    "Instead of that, you can stay here as long as you want."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as("Jiro", args!["Well then...", "Talk to me when you need help."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn soldier_ama2(ctx: &Ctx) -> Script {
    soldier_ama2_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Saburo",
        args![
            "This is the training ground for improving our battle skills.",
            "Please look around..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Saburo",
        args![
            "By the way, recently I've felt",
            "like the soldiers have changed",
            "lately. How do I say it...",
            "Their faces are gloomy and",
            "some of them are no longer around.",
            "Have they gone to Midgard??"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Saburo",
        args![
            "In this job, I encounter",
            "many strangers. But downstairs,",
            "there's a guest from a distant land who seems suspicious...",
            "I told my guards to watch him..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Saburo",
        args![
            "Of course, my lord precisely knows everything that's going on.",
            "Haha, don't take what I said seriously~",
            "See ya..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama3(ctx: &Ctx) -> Script {
    soldier_ama3_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Shiro",
        args![
            "*Cough, Cough* What...",
            "Don't talk to me...",
            "*Cough, Cough*... Oh, my freakin' neck."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shiro",
        args![
            "That quack's cold medicine",
            "is useless! I should never have trusted people from Midgard!",
            "*Cough, Cough*..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Shiro",
        args![
            "I don't know why but...",
            "I'm getting worse and worse...",
            "What kind of cold is this?",
            "*Cough*... *Cough*..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama4(ctx: &Ctx) -> Script {
    soldier_ama4_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Goro",
        args!["*Psst*... Please, be quiet.", "I will tell you a story, okay?", " "],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Goro",
        args![
            "There is a rumor in Amatsu.",
            "The lord of this palace isn't",
            "real... *Psst*, Quiet!",
            "Don't panic and listen to me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Goro",
        args![
            "There is a real lord behind",
            "the kind lord and he is scheming",
            "something. He is controlling",
            "our town in some hidden place.",
            "...Our kind lord is just a figurehead~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Goro",
        args![
            "It's for real! Trust me~",
            "I saw him. The lord who was",
            "laughing at the town on the",
            "TenguGak!!",
            "Just don't tell anyone that I've told you this, okay?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama5(ctx: &Ctx) -> Script {
    soldier_ama5_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Rokuro",
        args![
            "He always talks nonsense.",
            "He says that our lord",
            "is not real, but a fake.",
            "It's not even funny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rokuro",
        args![
            "However, it is true that",
            "recently, weird things are",
            "happening. There were no",
            "prohibited places before..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rokuro",
        args![
            "I hear strange sounds sometimes.",
            "The mother of the lord has been",
            "visited by doctors several times.",
            "I guess her health hasn't",
            "improve after their visits..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama6(ctx: &Ctx) -> Script {
    soldier_ama6_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Shichiro", args!["Have a good time.", "My lord prepared some", "guest rooms."])?;
    ctx.next()?;
    ctx.lines_as(
        "Shichiro",
        args![
            "If you have any problems,",
            "please call me. Also, try not to disturb the other guests.",
            " "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Shichiro", args!["Have a good day."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama7(ctx: &Ctx) -> Script {
    soldier_ama7_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hachiro",
        args!["My lord is inside.", "If you'd like to greet him, feel free to enter."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hachiro",
        args![
            "Don't be rude in front of him.",
            "He is somehow not feeling well.",
            "Usually, he greets people from",
            "other continents gladly, but...",
            "What could have happened to him...?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama8(ctx: &Ctx) -> Script {
    soldier_ama8_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama9_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kyuro",
        args![
            "This is not good.",
            "My lord is really kind, but",
            "recently he is not doing well..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kyuro",
        args![
            "I've heard about the rumors",
            "but I trust him. He made",
            "Amatsu into a great town.",
            "That is why I'm following",
            "him as a soldier."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama9(ctx: &Ctx) -> Script {
    soldier_ama9_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama10_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jyuro",
        args!["What are you doing in here?", "There is nothing interesting here."],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("That is true:Open the gate")])?) == 1 {
        ctx.lines_as(
            "Jyuro",
            args!["There are much better things to enjoy in town. Have a good time..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("event_amatsu").get()? == 6 {
        if ctx.call(Function::CountItem, vec![Val::from(7160)])?.number()? > 0 {
            ctx.lines_as(
                "Jyuro",
                args![
                    "You have the ticket...",
                    "Do you want me to send you now, or do you need a little instruction?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("I will go in:Instruction, please")])?) == 1 {
                ctx.lines_as("Jyuro", args!["I will open the gate.", "Take care of yourself..."])?;
            } else {
                ctx.lines_as(
                    "Jyuro",
                    args![
                        "The area beyond this gate is",
                        "protected by some kind of magic.",
                        "You may get killed by someone",
                        "or get lost.",
                        " "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jyuro",
                    args![
                        "I can't tell you much.",
                        "I didn't go far inside and",
                        "and just took a quick look.",
                        " ",
                        " "
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jyuro",
                    args![
                        "First.....",
                        "'Do not trust what you are",
                        "seeing.' I don't know what",
                        "it means, but I guess you shouldn't believe everything before your eyes."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jyuro",
                    args![
                        "Second.....",
                        "'There are certain rules in",
                        "magic.' Everything has a",
                        "reason to exist. Magic is not an exception. You can probably find the answer..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Jyuro", args!["Well, I will open this gate.", "Take care of yourself..."])?;
            }
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ama_dun01"), Val::from(229), Val::from(10)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Jyuro",
            args!["I'm sorry.", "You need a Transit Permit to go in.", "Please, go back."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Jyuro",
        args![
            "What gate are you talking about?",
            "A gate? On the top floor of the building??? Surely, you must been be mistaken."
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
    ctx.next()?;
    if ctx.call(Function::CountItem, vec![Val::from(7160)])?.number()? > 0 {
        if Val::from(runtime::select_values(ctx, &[Val::from("Show him the ticket:Cancel")])?) == 1 {
            ctx.lines_as(
                "Jyuro",
                args![
                    "What? That pass is...?",
                    "That Transit Permit is from",
                    "the lord but I guess it has",
                    "been issued to the wrong person."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(7160), Val::from(1)])?;
            ctx.lines_as(
                "Jyuro",
                args![
                    "I will keep this ticket because",
                    "it was issued without permission.",
                    "This is a warning.",
                    "Be careful."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Jyuro", args!["There are more things to enjoy in town. Have a good time..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Jyuro",
        args!["There are more things to enjoy in town. Have yourself a good time..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama10(ctx: &Ctx) -> Script {
    soldier_ama10_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_ama11_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Jyuro", args!["Do you want to go back?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I do:I will stay here")])?) == 1 {
        ctx.lines_as("Jyuro", args!["Take care of yourself."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("ama_in02"), Val::from(119), Val::from(181)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Jyuro", args!["Take care..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_ama11(ctx: &Ctx) -> Script {
    soldier_ama11_body(ctx, Vec::new()).map(|_| ())
}

fn lord_of_palace_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("event_amatsu").get()? == 0 {
        ctx.lines_as(
            "Ishida Yoshinaga",
            args![
                "What! A foreigner...? *Phew*",
                "I'm sorry, but I'm not in",
                "the mood to meet new people!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Yoshinaga",
            args!["Get out! I'm not interested", "in who you are.", "I'm not feeling well!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("event_amatsu").get()? == 1 {
        ctx.lines_as(
            "Ishida Yoshinaga",
            args![
                "What! A foreigner? What brings",
                "you here? If it is not urgent,",
                "come to me another time...!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("I heard about your mother...:Your last name is nice:Who are you?")],
        )? {
            1 => {
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "Oh...So you know about her disease?",
                        "I've heard that, in foreign lands,",
                        "the body of medical knowledge can",
                        "be quite amazing."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "Welcome. As you know, I'm the",
                        "lord of Toukoujyo,",
                        "Ishida Yoshinaga. Nice to",
                        "meet you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "Let's get to the point.",
                        "My mother is not doing well recently. I know you are here because of that.",
                        "Can you cure her disease?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "Until now, many famous doctors",
                        "have visited her, but they",
                        "all failed to cure her disease",
                        "and made it worse...",
                        "They disappointed me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "You, who hail from Midgard,",
                        "may be able to cure my mother's disease. I will reward you well if you succeed..."
                    ],
                )?;
                ctx.next()?;
                ctx.var("event_amatsu").set(Val::from(2))?;
                if ctx.call(Function::IsBeginQuest, vec![Val::from(8131)])? == 1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(8131), Val::from(8132)])?;
                } else {
                    ctx.call(Function::SetQuest, vec![Val::from(8132)])?;
                }
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "I beg you...Please.",
                        "My mother is living in a house outside of the palace.",
                        "Come to me when you finish your treatment."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args!["Nice!? So What?!", "Read my name until you get", "tired of it! Darn it!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "Joking, even in this critical situation... *Phew*...",
                        "Please have a good time in Amatsu.",
                        "...Whatever!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Ishida Yoshinaga",
                    args![
                        "...me? Don't you know? Huh?",
                        "I'm the lord of this palace.",
                        "If you don't know, talk to",
                        "the soldiers outside!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("event_amatsu").get()? == 5 {
        if ctx.call(Function::CountItem, vec![Val::from(1022)])?.number()? > 0 {
            ctx.lines_as(
                "Ishida Yoshinaga",
                args![
                    "I've heard the great news!",
                    "My mother seems to have gotten better. What was her disease?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ishida Yoshinaga",
                args!["A fox? Is that so? Oh...", "It wasn't a disease...!!", "Why didn't I notice?!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ishida Yoshinaga",
                args![
                    "Darn fox. To run away and",
                    "take revenge on me in such a",
                    "way...Well, then. There's no chance of revenge now... Hahaha!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Ishida Yoshinaga", args!["Hmm, Mmm. Hmm..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Ishida Yoshinaga",
                args![
                    "Anyway, thank you for helping me.",
                    "Mother will be okay now...",
                    "I want to reward you...",
                    "But what would be nice...?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ishida Yoshinaga",
                args![
                    "Alright, I will give you",
                    "a Transit Permit.",
                    "You can go anywhere",
                    "with this ticket."
                ],
            )?;
            ctx.next()?;
            ctx.var("event_amatsu").set(Val::from(6))?;
            ctx.call(Function::CompleteQuest, vec![Val::from(8135)])?;
            ctx.call(Function::DelItem, vec![Val::from(1022), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(7160), Val::from(1)])?;
            ctx.lines_as(
                "Ishida Yoshinaga",
                args![
                    "This isn't a big reward but",
                    "someday it will be useful for you.",
                    "Ask my soldier, 'Jyuro' about the details."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Ishida Yoshinaga",
            args![
                "Hmm, I heard that my mother",
                "got better...but",
                "How can I know if you cured",
                "her or not?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Yoshinaga",
            args!["Is there any evidence to prove", "that you cured her?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Yoshinaga",
            args![
                "Show me the evidence to prove",
                "your treatment. I've been",
                "meeting so many foreigners.",
                "But not all of them are trustworthy.",
                "Well... Have a good time."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("event_amatsu").get()? == 6 {
        ctx.lines_as(
            "Ishida Yoshinaga",
            args!["*Chuckle* Have a good time", "in Amatsu.....", "Foreigners are always welcome."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Ishida Yoshinaga",
        args![
            "How is my mom's status?",
            "If you find the name of the disease, please tell me.",
            "I was worrying about",
            "her all night."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ishida Yoshinaga",
        args![
            "Because you carry with you",
            "knowledge from Midgard,",
            "I have faith in your ability."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn lord_of_palace_ama(ctx: &Ctx) -> Script {
    lord_of_palace_ama_body(ctx, Vec::new()).map(|_| ())
}

fn grandma_ama2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_fox_kill = Val::from(0);
    if ctx.var("event_amatsu").get()? == 4 {
        ctx.lines_as(
            "....",
            args![
                "^FF6060Yelp, Yelp, a foolish human",
                "again! What are you going to do",
                "to get rid of me!?^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "....",
            args![
                "^FF6060You look like you learned",
                "something from somewhere, but",
                "it won't harm me!! Yelp!^000000"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Taaaah!!")])?;
        ctx.var("@menu").set(choice)?;
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_PRIEST")?) {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 1 {
                l_fox_kill = Val::from(0);
            } else {
                l_fox_kill = Val::from(1);
            }
        } else {
            if ((((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?)
                || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MONK")?))
                || ctx.call(Function::CountItem, vec![Val::from(523)])?.number()? > 0)
                || ctx.call(Function::CountItem, vec![Val::from(948)])?.number()? > 0)
                || ctx.call(Function::CountItem, vec![Val::from(1029)])?.number()? > 0)
            {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
                    l_fox_kill = Val::from(0);
                } else {
                    l_fox_kill = Val::from(1);
                }
            } else {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 1 {
                    l_fox_kill = Val::from(1);
                } else {
                    l_fox_kill = Val::from(0);
                }
            }
        }
        if l_fox_kill.clone() == 1 {
            ctx.lines_as(
                "....",
                args![
                    "^FF6060Yelp! Yelp! Yelp! Human!",
                    "To expel me from this body like",
                    "this! Curse you! Darn you!!^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "....",
                args![
                    "^FF6060I will curse the Ishida family",
                    "who made me like this! Forever!",
                    "You better watch out, human!^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(
                Function::Monster,
                vec![
                    Val::from("ama_in01"),
                    Val::from(22),
                    Val::from(111),
                    Val::from("Nine Tails"),
                    Val::from(1180),
                    Val::from(1),
                ],
            )?;
            ctx.call(Function::KillMonster, vec![Val::from("ama_in01"), Val::from("All")])?;
            ctx.var("event_amatsu").set(Val::from(5))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(8134), Val::from(8135)])?;
            ctx.call(Function::GetItem, vec![Val::from(1022), Val::from(1)])?;
            ctx.lines_as(
                "....",
                args![
                    "^FF6060Everything that",
                    "Yoshinaga does will cause you",
                    "unhappiness...^000000",
                    "^FF0000Yaaaaaaaaaaaaap!!^000000"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "....",
            args![
                "^FF6060Yelp! Haha! Yelp! Hahaha!",
                "Are you trying to expel me from this body!?",
                "Shoo!! Yelp!^000000"
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(167), Val::from(197)])?;
        return Err(Stop::End);
    } else if ctx.var("event_amatsu").get()? == 5 {
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "...Huh? Why are you here...?",
                "*Urrmmm* My head hurts...",
                "But I'm starting to remember everything..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "Thank you, traveler from a far off",
                "land. I owe you a great debt... Thank you very much..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args!["I will tell my son that", "you exorcised the fox...", "Thank you..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args!["I should get some rest.", "My head aches, Young one.....", "Go to my son..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("event_amatsu").get()? == 6 {
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "Oh, Are you...? You are the one",
                "who exorcised the fox... Welcome.",
                "Please, have a seat..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "When I think over, being possessed",
                "by the fox was my fault. I raised",
                "my son badly... Oh~",
                "He was a good boy when he was",
                "young. I wasn't strict to him..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "Long ago, this town wasn't as big",
                "as it is today. There was no big",
                "palace like Toukoujyo. Then, one",
                "day, my son brought great riches",
                "to the village. He never told me what he did to earn that fortune..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "He built the palace and helped",
                "the towners and make the town bigger.",
                "He was perfect until...",
                "he started doing strange things."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "Things...which angered God.",
                "He learned forbidden magic,",
                "performed experiments",
                "in the palace, caged monsters,",
                "did all sorts of horrible things..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "That is the reason why there are",
                "monsters in Toukoujyo... Finally,",
                "God's wrath was unleashed. Even the Priest in the shrine couldn't help..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "At last, the anger came toward",
                "me...It seems God tried to warn my son with the fox.",
                "However, my son won't stop."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ishida Saoko",
            args![
                "If it is okay, please stop my son.",
                "I don't have much time.",
                "I don't know what to do...",
                "Please save this peaceful village.",
                "I beg you please...",
                "What is he truly thinking...?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "....",
        args![
            "^FF6060Yelp, Yelp, Another foolish human!",
            "Yelp, Yelp, Who! Human!?",
            "What are you doing!",
            "Shoo... Yelp! Yelp!^000000"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "....",
        args![
            "^FF6060I will be in here until",
            "the Ishida family line dies out!",
            "Don't disturb me! Yelp!^000000"
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("amatsu"), Val::from(167), Val::from(197)])?;
    return Err(Stop::End);
}

pub fn grandma_ama2(ctx: &Ctx) -> Script {
    grandma_ama2_body(ctx, Vec::new()).map(|_| ())
}

fn kouji_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kouji",
        args!["Ralralrah Ralralrah Ralralrahralralrah~", "Ralralrah Ralralrah Ralralrahralralrah~"],
    )?;
    ctx.next()?;
    ctx.mes("[Kouji]")?;
    if ctx.var("event_amatsu").get()? == 1 {
        ctx.lines(args![
            "When you're sick, rice cakes are no good~",
            "Family is the best help, as it should~",
            "When medicine doesn't help, that's when you'll see~",
            "That Mommy's love is best for me~"
        ])?;
    } else if ctx.var("event_amatsu").get()? == 2 {
        ctx.var("event_amatsu").set(Val::from(3))?;
        ctx.lines(args![
            "Priest, Priest~",
            "A fox is following me!",
            "It's funny and a little absurd,",
            "But I'll need noodles with fried bean curd!"
        ])?;
    } else if ctx.var("event_amatsu").get()? == 3 {
        ctx.lines(args![
            "Priest, priest~",
            "A fox is following me!",
            "If he doesn't leave when I scream and shout!",
            "The North Shrine Priest should help me out~"
        ])?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8132)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8132), Val::from(8133)])?;
        }
    } else if ctx.var("event_amatsu").get()? == 4 {
        ctx.lines(args![
            "Priest, priest~",
            "A fox is following me.",
            "If shouts alone don't make Fox scared,",
            "I might need help from Tiger and Bear~!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Kouji",
            args!["Anything else I need in this fight???", "Maybe some water from an Acolyte~!"],
        )?;
    } else if ctx.var("event_amatsu").get()? == 5 {
        ctx.lines(args![
            "Scary scary harbor ship~",
            "Empty of people, full of treasure~",
            "But I don't remember~!",
            "the rest of this...song???"
        ])?;
    } else {
        ctx.lines(args![
            "Blue roof under the blue sky",
            "Blue wall on the blue lake",
            "Blue wishes in the blue minds",
            "Blue Blue Everything is Blue"
        ])?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Kouji",
        args!["Ralralrah Ralralrah Ralralrahralralrah~", "Ralralrah Ralralrah Ralralrahralralrah~"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kouji_ama(ctx: &Ctx) -> Script {
    kouji_ama_body(ctx, Vec::new()).map(|_| ())
}

fn shaman_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Tokako",
        args![
            "Er, I'm not really a shaman...",
            "My friend, Takehue-kun brought",
            "me here and invited me to try on these clothes."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tokako",
        args![
            "He asks me to do a lot of",
            "weird stuff, but he is funny.",
            "Sometimes, I don't know what",
            "he is thinking. Still, he is a good friend."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Tokako",
        args![
            "If you are curious about,",
            "something, ask Takehue-kun.",
            "He knows stuff about",
            "mysticism and the occult that other people don't know about.",
            " "
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn shaman_ama(ctx: &Ctx) -> Script {
    shaman_ama_body(ctx, Vec::new()).map(|_| ())
}

fn kitsune_mask_ama_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("event_amatsu").get()? == 3 {
        ctx.lines_as(
            "Takehue",
            args![
                "Eh, you are a foreigner.",
                "I'm sure you visited me because",
                "you are having fox troubles.",
                "I can see that in your face."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "Originally, the head priest should",
                "help you, but this shrine has been",
                "abandoned long ago."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "It is hard to expel a fox from",
                "a human body. If you have liquor",
                "and noodles with fried bean curd,",
                "it would be easy, but they are hard to find."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "I know a little about foxes, so let me tell you something...",
                "Foxes like to tease people but",
                "it is rare for them to crave",
                "vengeance.",
                " "
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "If a fox is vengeful, it means that a human caused it harm.",
                "The fox will take its revenge",
                "against, you, your family,",
                "even your close friends!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "Anyway, a stronger spirit",
                "will expel the fox from a human.",
                "Come on, yell out and show me your spirit!",
                " "
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Taaaaah!!")])?;
        ctx.var("@menu").set(choice)?;
        ctx.var("event_amatsu").set(Val::from(4))?;
        ctx.lines_as(
            "Takehue",
            args![
                "Good. Show that spiritual",
                "energy to the fox several times.",
                "Sooner or later, you'll be successful."
            ],
        )?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(8132)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8132), Val::from(8134)])?;
        } else if ctx.call(Function::IsBeginQuest, vec![Val::from(8133)])? == 1 {
            ctx.call(Function::ChangeQuest, vec![Val::from(8133), Val::from(8134)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("event_amatsu").get()? == 4 {
        ctx.lines_as(
            "Takehue",
            args![
                "Don't forget. You also need the",
                "the embodiment of animals stronger",
                "than the fox. Without these, your",
                "concentration will be of no use."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Takehue",
            args![
                "Eh? A foreigner. This shrine",
                "has been without priests for a",
                "long time. My friend, Tokako and I come here to play around."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Takehue",
            args![
                "If you have been chased by",
                "monsters, please relax.",
                "Monsters cannot come here, so",
                "take a rest in here."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn kitsune_mask_ama(ctx: &Ctx) -> Script {
    kitsune_mask_ama_body(ctx, Vec::new()).map(|_| ())
}
