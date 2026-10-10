use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn bill_thayer_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lv4_weapon").get()? == 55 {
        ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Pyroxene, Turquoise and Phlogopite. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 54 {
        ctx.lines_as(
            "Bill Thayer",
            args![
                "Hmm...",
                "You'll need 30 Citrin, Agate",
                "and Muscovite. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 53 {
        ctx.lines_as("Bill Thayer", args!["What...?", "Waltboughst's research?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "How do you know him?",
                "Were you a friend of his while",
                "he was still alive? This must be",
                "an act of providence..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "Hmm...",
                    "You'll need 30 Citrin, Agate",
                    "and Muscovite. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(54))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Pyroxene, Turquoise and Phlogopite. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(55))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("lv4_weapon").get()? == 49 || ctx.var("lv4_weapon").get()? == 50) {
        ctx.lines_as(
            "Bill Thayer",
            args!["Don't you have anything better to do? Quit bothering me and do your best to reach your own goals."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sigh...*^000000",
                "Still, trying your",
                "best won't always result",
                "in success though."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^660000But that's no excuse!^000000",
                "Every successful man in",
                "history has tried his best!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args!["I hope you remember that. It's something I used to tell my sons when they were alive. Ha ha ha~"],
        )?;
        ctx.var("lv4_weapon").set(Val::from(50))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 45 {
        ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Turquoise,", "Biotite and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.var("lv4_weapon").set(Val::from(45))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 46 {
        ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Phlogopite,", "Citrine and Pyroxene. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.var("lv4_weapon").set(Val::from(46))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 44 {
        ctx.lines_as("Bill Thayer", args!["What...?", "Hein's research?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "How do you know him?",
                "Were you a friend of his while",
                "he was still alive? This must be",
                "an act of providence..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Turquoise,", "Biotite and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(45))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Phlogopite,", "Citrine and Pyroxene. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(46))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("lv4_weapon").get()? == 40 || ctx.var("lv4_weapon").get()? == 41) {
        ctx.lines_as(
            "Bill Thayer",
            args!["No matter how much you're willing to pay me, I'm not going to forge you any weapons."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "Even during my weapon making days,",
                "I never accepted money from any of",
                "my clients. Do you know why?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Bill Thayer", args!["I always told my sons: 'Never accept payment to forge a good weapon. ^333333It brings bad luck.^000000' Don't ask me how that works. It just is."])?;
        ctx.var("lv4_weapon").set(Val::from(41))?;
        ctx.next()?;
        ctx.lines_as("Bill Thayer", args!["Even if you offer me something more precious than money, I won't do any weapon smithing for you. Although I might have back when my sons were still alive..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 37 {
        ctx.lines_as(
            "Bill Thayer",
            args![
                "Hmm...",
                "You'll need 30 Biotite,",
                "Agate and Citrin. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 36 {
        ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Muscovite,", "Peridot and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 35 {
        ctx.lines_as("Bill Thayer", args!["What...?", "Reyghema's research?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "How do you know him?",
                "Were you a friend of his while",
                "he was still alive? This must be",
                "an act of providence..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Muscovite,", "Peridot and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(36))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Biotite,", "Agate and Citrin. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(37))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("lv4_weapon").get()? == 31 || ctx.var("lv4_weapon").get()? == 32) {
        ctx.lines_as("Bill Thayer", args!["All of my sons broke my heart by dying too young. I remember that I always used to tell them: 'Enjoy your youth. ^333333Live without regret.^000000'"])?;
        ctx.var("lv4_weapon").set(Val::from(32))?;
        ctx.next()?;
        ctx.lines_as("Bill Thayer", args!["But I never suspected that their lives would be cut so short. Please remember to live life in such a way that regret won't haunt you later. That's the best advice I can give to you."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 27 {
        ctx.lines_as(
            "Bill Thayer",
            args![
                "Hmm...",
                "You'll need 30 Turquoise,",
                "Peridot and Agate. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 28 {
        ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Phlogopite,", "Pyroxene and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "^333333*Sniff*^000000",
                "All of my sons passed away",
                "before they could finish their research. I should have been the one to have died. They were still so young..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 26 {
        ctx.lines_as("Bill Thayer", args!["What...?", "Kayron's research?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Bill Thayer",
            args![
                "How do you know him?",
                "Were you a friend of his while",
                "he was still alive? This must be",
                "an act of providence..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Turquoise,", "Peridot and Agate. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(27))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Bill Thayer", args!["Hmm...", "You'll need 30 Phlogopite,", "Pyroxene and Rose Quartz. I'm sure those are the materials you need, but I have no idea what the end product is supposed to be..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Bill Thayer",
                args![
                    "^333333*Sniff*^000000",
                    "All of my sons passed away",
                    "before they could finish their research. I should have been the one to have died. They were still so young..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(28))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if (ctx.var("lv4_weapon").get()? == 22 || ctx.var("lv4_weapon").get()? == 23) {
        ctx.lines_as(
            "Bill Thayer",
            args![
                "Listen, I told you...",
                "There's nothing I can do.",
                "^333333I'm utterly helpless here.^000000"
            ],
        )?;
        ctx.var("lv4_weapon").set(Val::from(23))?;
        ctx.next()?;
        ctx.lines_as("Bill Thayer", args!["There's no point in asking me to do anything. Ever since my sons passed away, I haven't been able to function. Please leave me alone."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Bill Thayer", args!["Leave me alone...", "I'm just a broken old man."])?;
    ctx.next()?;
    ctx.lines_as("Bill Thayer", args!["I don't make weapons anymore for anyone. Maybe I was a well known weaponsmith once, but I just can't bring myself to do any work ever since my sons passed away..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bill_thayer_lv4(ctx: &Ctx) -> Script {
    bill_thayer_lv4_body(ctx, Vec::new()).map(|_| ())
}

fn citizen_lv4_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Citizen",
        args![
            "There was a skillful",
            "weaponsmith in Al De Baran",
            "who had 4 sons. All of them were known throughout the land for their skill in weapon crafting."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Citizen",
        args!["Tragically, all of the sons died in their attempt to create a powerful weapon. Only the father survived the accident."],
    )?;
    ctx.next()?;
    ctx.lines_as("Citizen", args!["Because of that incident, the weaponsmith retired and went into hiding. No one ever saw the weapon he and his sons were developing."])?;
    ctx.next()?;
    ctx.lines_as(
        "Citizen",
        args![
            "I don't think his sons",
            "were able to peacefully enter the afterlife after sacrificing so much and having to leave their goal unfulfilled..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn citizen_lv4_1(ctx: &Ctx) -> Script {
    citizen_lv4_1_body(ctx, Vec::new()).map(|_| ())
}

fn citizen_lv4_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Citizen", args!["Meeting a person in the realm of the dead is impossible. But I've heard that if you were able to meet a dead person, you'd find that he wouldn't have all of his memories."])?;
    ctx.next()?;
    ctx.lines_as("Citizen", args!["But if you brought him something that reminded him of his life, he'd get his memories back for a little while. Of course, you'd have to actually meet someone in the netherworld to find out for sure."])?;
    ctx.next()?;
    ctx.lines_as("Citizen", args!["Huh...", "I know what I'm talking about might be a little morbid, but even the most trivial information might come in handy someday, right?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn citizen_lv4_2(ctx: &Ctx) -> Script {
    citizen_lv4_2_body(ctx, Vec::new()).map(|_| ())
}

fn kayron_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_aekddam = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_myhand1 = Val::from(0);
    let mut l_myhand2 = Val::from(0);
    let mut l_myhand3 = Val::from(0);
    let mut l_npchand1 = Val::from(0);
    let mut l_npchand2 = Val::from(0);
    let mut l_npchand3 = Val::from(0);
    let mut l_shobu = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(300)])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? < 80 {
        ctx.lines_as(
            "Kayron",
            args!["Hey...", "How did somebody like", "you get all the way over here?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["Just by looking at that tender face of yours, I can tell you're a little weak. You better get out of here and get real strong in a hurry before you even think of coming back."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as(
            "Kayron",
            args![
                "In life, I was known as Kayron.",
                "But now I'm just another ghost wandering around this realm of",
                "the dead. Without a body, I feel",
                "so useless."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
            1 => {
                ctx.lines_as(
                    "Kayron",
                    args![
                        "Huh...?",
                        "I can't remember what I was going to say. M-my memories! I... I can't seem to recollect..."
                    ],
                )?;
                ctx.var("lv4_weapon").set(Val::from(22))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Kayron", args!["Useless...", "Absolutely useless..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 22 || ctx.var("lv4_weapon").get()?.number()? > 30) {
        ctx.lines_as(
            "Kayron",
            args![
                "I sense something from",
                "you. Almost as if you were",
                "here with some sort of goal."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "I'm surprised...",
                "And a little envious.",
                "It can be said that most",
                "in Niflheim come here with",
                "little or no purpose."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lv4_weapon").get()? == 29 || ctx.var("lv4_weapon").get()? == 30) {
        ctx.lines_as(
            "Kayron",
            args![
                "Heh heh heh...",
                "You're just in time.",
                "The time has come for",
                "me to reveal the results",
                "of my work..."
            ],
        )?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as(
                "Kayron",
                args![
                    "It's a success!",
                    "I completed the experiment",
                    "that got me killed! I almost",
                    "can't believe it! I created"
                ],
            )?;
            if ctx.var("lv4_weapon").get()? == 29 {
                ctx.mes("this... Great Axe!")?;
                ctx.call(Function::GetItem, vec![Val::from(1364), Val::from(1)])?;
            } else {
                ctx.mes("this... Longinus's Spear!")?;
                ctx.call(Function::GetItem, vec![Val::from(1469), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "I'm so glad that my years of research haven't been wasted.",
                    "If only Father were able to see this, he'd be really happy for me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Oh no...",
                    "^333333*Sigh*^000000 I can feel my memories beginning to fade again. If you ever see me, please let me",
                    "remember my past again.",
                    "Thank you..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Kayron",
                args![
                    "It's a success!",
                    "I completed the experiment",
                    "that got me killed! I almost",
                    "can't believe it! I created"
                ],
            )?;
            if ctx.var("lv4_weapon").get()? == 29 {
                ctx.mes("this... Guillotine!")?;
                ctx.call(Function::GetItem, vec![Val::from(1369), Val::from(1)])?;
            } else {
                ctx.mes("this... Brionac!")?;
                ctx.call(Function::GetItem, vec![Val::from(1470), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "I'm so glad that my years of research haven't been wasted.",
                    "If only Father were able to see this, he'd be really happy for me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Oh no...",
                    "^333333*Sigh*^000000 I can feel my memories beginning to fade again. If you ever see me, please let me",
                    "remember my past again.",
                    "Thank you..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((((ctx.var("lv4_weapon").get()? == 27 && ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7294)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
        || (((ctx.var("lv4_weapon").get()? == 28 && ctx.call(Function::CountItem, vec![Val::from(7290)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7296)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29))
    {
        ctx.lines_as(
            "Kayron",
            args![
                "What's this?",
                "The stuff you're carrying",
                "around looks awfully familiar.",
                "Hm, let me take a look, please..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "Yes, I was right!",
                "I can use these to smith a weapon! How did you know these were the materials I need? In any case, this is great!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["Oh, but first, we need to test your luck. No matter how good my skills may be, we need to make sure your luck is good today, or else we can't proceed."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "We're gonna play '^660000Rock, Paper, Scissors^000000.' We can go ahead and",
                "make the item if you win 2 out of 3 matches."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["But if you lose 2 out of 3, your luck isn't strong enough for me to even attempt smithing. We'll need to dump a huge amount of one of the materials you brought to repel your bad luck."])?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["Here's a piece of paper.", "You'll write 'Rock,' 'Paper,' or 'Scissors,' and I'll do the same. Then we'll compare our results and see what happens. Alright, let's do the first match!"])?;
        ctx.next()?;
        l_npchand1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand1 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand1.clone() == 1 && l_npchand1.clone() == 3) || (l_myhand1.clone() == 2 && l_npchand1.clone() == 1))
            || (l_myhand1.clone() == 3 && l_npchand1.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as("Kayron", args!["Now...", "Let's play", "the second match~"])?;
        ctx.next()?;
        l_npchand2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand2 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand2.clone() == 1 && l_npchand2.clone() == 3) || (l_myhand2.clone() == 2 && l_npchand2.clone() == 1))
            || (l_myhand2.clone() == 3 && l_npchand2.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as(
            "Kayron",
            args!["Alright...", "One last time.", "Write down 'Rock,'", "'Paper' or 'Scissors.'"],
        )?;
        ctx.next()?;
        l_npchand3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand3 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand3.clone() == 1 && l_npchand3.clone() == 3) || (l_myhand3.clone() == 2 && l_npchand3.clone() == 1))
            || (l_myhand3.clone() == 3 && l_npchand3.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        l_aekddam = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if l_shobu.clone().number()? > 1 {
            if ctx.var("lv4_weapon").get()? == 27 {
                ctx.call(Function::DelItem, vec![Val::from(7289), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
            } else {
                ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
            }
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(2)))?;
        } else if l_shobu.clone().number()? < 2 {
            if l_aekddam.clone() == 1 {
                if ctx.var("lv4_weapon").get()? == 27 {
                    ctx.call(Function::DelItem, vec![Val::from(7289), Val::from(30)])?;
                } else {
                    ctx.call(Function::DelItem, vec![Val::from(7290), Val::from(30)])?;
                }
            } else {
                if l_aekddam.clone() == 2 {
                    if ctx.var("lv4_weapon").get()? == 27 {
                        ctx.call(Function::DelItem, vec![Val::from(7294), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7296), Val::from(30)])?;
                    }
                } else if l_aekddam.clone() == 3 {
                    if ctx.var("lv4_weapon").get()? == 27 {
                        ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
                    }
                }
            }
        }
        ctx.lines_as("Kayron", args!["We're done!", "Now, I wrote down..."])?;
        if l_npchand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        ctx.lines_as("Kayron", args!["You wrote down..."])?;
        if l_myhand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        if l_shobu.clone().number()? > 1 {
            ctx.lines_as(
                "Kayron",
                args![
                    "Let's see...",
                    ((Val::from("You won ") + l_shobu.clone()) + Val::from(" times.")),
                    "You must be really",
                    "Without a doubt, you've",
                    "got some pretty good luck~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Alright, I'll keep my end of the bargain. Give me the materials",
                    "and some time to finish preparing. I'll talk to you later, okay?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_shobu.clone().number()? < 2 {
            ctx.lines_as(
                "Kayron",
                args![
                    "Well, I'm sorry to say that you lost. We have no choice but to drive away your bad luck with",
                    "some of the ore you brought!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Alright, let's try this again. Go and get the materials we need",
                    "and come back to me."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 26 || ctx.var("lv4_weapon").get()? == 27) || ctx.var("lv4_weapon").get()? == 28) {
        ctx.lines_as(
            "Kayron",
            args![
                "I can't seem to recall the rest",
                "of the materials I need. This is beginning to get really frustrating."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args!["I mean, me and my father used to know this stuff like it was second nature. What's wrong with me...?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("lv4_weapon").get()? == 25 && ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 1)
        && ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 19)
    {
        ctx.lines_as(
            "Kayron",
            args![
                "Oh...!",
                "I had no idea you were such a reliable person! You brought the things I've asked for pretty quickly!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args!["But I still can't remember the rest of the things that I'll be needing. I'm really sorry..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["Is there any way to get more of my memory back? In the meantime, let me keep these materials. I promise they'll be used to make a good weapon for you."])?;
        ctx.call(Function::DelItem, vec![Val::from(1005), Val::from(2)])?;
        ctx.call(Function::DelItem, vec![Val::from(989), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(710), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(20)])?;
        ctx.var("lv4_weapon").set(Val::from(26))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 25 {
        ctx.lines_as(
            "Kayron",
            args![
                "I'll need a lot of materials, but let me remind you of some of",
                "the necessary things that I can actually remember."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "So far, I know I'll need",
                "2 Hammer of Blacksmith",
                "1 Emperium Anvil",
                "1 Illusion Flower and",
                "20 Gold."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "I know there were a few more",
                "things that are crucial to this weapon's construction, but...",
                "I can't seem to concentrate hard enough to remember."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 24 {
        ctx.lines_as(
            "Kayron",
            args![
                "Alright...",
                "I just remembered",
                "most of the things",
                "I need to make you",
                "a great weapon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["I believe it's fair to let you know that I can't guarantee what weapon will be produced from this work since, quite frankly, I can't remember."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "So far, I know I'll need",
                "2 Hammer of Blacksmith",
                "1 Emperium Anvil",
                "1 Illusion Flower and",
                "20 Gold."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["Bring those items to me first. I'm pretty sure there was something else I'll be needing, but I can't seem to remember. This memory", "of mine is beginning to worry me a little bit..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args!["Anyway, I'll be waiting for you here while you go out and collect those things. Be careful, alright?"],
        )?;
        ctx.var("lv4_weapon").set(Val::from(25))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 23 {
        ctx.lines_as(
            "Kayron",
            args![
                "In life, I was known as Kayron.",
                "But now I'm just another ghost wandering around this realm of",
                "the dead. Without a body, I feel",
                "so useless."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Kayron", args!["There's nothing I can do.", "I'm... Um... I'm...?"])?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        ctx.next()?;
        if l_input_s.clone() == "I'm utterly helpless here." {
            ctx.lines_as(
                "Kayron",
                args![
                    "I'm utterly helpless here.",
                    "Right, right! My father always used to say that! I guess my brothers and I take after our father in a lot of ways."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "My mind's been so hazy.",
                    "I forgot that I even used",
                    "to be a living human.",
                    "^333333*Sigh...*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Still, that little memory has brought me a little peace of",
                    "mind. I really appreciate that."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "I used to be a weaponsmith",
                    "when I was alive, so if you like, I can forge something nice for you. You'll have to bring me the materials, though."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Kayron", args!["You understand that being", "dead sort of restricts my mobility. Still, it's actually useful to be stuck in Niflheim. This place is filled with some strange energy that will help in my smithing."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args![
                    "Even though experimental",
                    "smithing got me killed, I won't have to worry about it this time. After all, I'm already dead! Hahahah~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kayron",
                args!["Alright, just give me a little time so that I can try to remember the things that I'll need, alright?"],
            )?;
            ctx.var("lv4_weapon").set(Val::from(24))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Kayron",
                args![
                    "Huh...?",
                    "What did you just say?",
                    "I'm sorry, I'm trying to remember something my father always used",
                    "to say..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 22 {
        ctx.lines_as(
            "Kayron",
            args![
                "In life, I was known as Kayron.",
                "But now I'm just another ghost wandering around this realm of",
                "the dead. Without a body, I feel",
                "so useless."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kayron",
            args![
                "Huh...?",
                "I can't remember what I was going to say. M-my memories! I... I can't seem to recollect..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Kayron",
        args![
            "In life, I was known as Kayron.",
            "But now I'm just another ghost wandering around this realm of",
            "the dead. Without a body, I feel",
            "so useless."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
        1 => {
            ctx.lines_as(
                "Kayron",
                args![
                    "Huh...?",
                    "I can't remember what I was going to say. M-my memories! I... I can't seem to recollect..."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(22))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as("Kayron", args!["Useless...", "Absolutely useless..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn kayron_lv4(ctx: &Ctx) -> Script {
    kayron_lv4_body(ctx, Vec::new()).map(|_| ())
}

fn reyghema_lv4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_aekddam = Val::from(0);
    let mut l_input_s = Val::from("");
    let mut l_myhand1 = Val::from(0);
    let mut l_myhand2 = Val::from(0);
    let mut l_myhand3 = Val::from(0);
    let mut l_npchand1 = Val::from(0);
    let mut l_npchand2 = Val::from(0);
    let mut l_npchand3 = Val::from(0);
    let mut l_shobu = Val::from(0);
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 3000 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseLevel").get()?.number()? < 80 {
        ctx.lines_as("Reyghema", args!["How...", "the hell...?"])?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["What's a living person doing here? Stick around and I guarantee that you won't be alive much longer. Come on, you don't want to end up like me. Hurry and get out of here!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 0 {
        ctx.lines_as("Reyghema", args!["Damn it...", "I didn't want to die!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Well, nothing that's alive really wants to die, but I was so close to finishing what I wanted to do. Ah, youth..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Youth...",
                "Enjoy your youth...",
                "I remember hearing",
                "something like that,",
                "but there was more to it..."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
            1 => {
                ctx.lines_as(
                    "Reyghema",
                    args!["Enjoy your youth...", "Enjoy your youth...", "...What came after that?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Reyghema",
                    args!["Argh!", "I can't remember!", "Everything in my", "head is so cloudy!"],
                )?;
                ctx.var("lv4_weapon").set(Val::from(31))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Reyghema",
                    args!["Enjoy your youth...", "Enjoy your youth...", "...What came after that?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("lv4_weapon").get()?.number()? < 31 || ctx.var("lv4_weapon").get()?.number()? > 39) {
        ctx.lines_as(
            "Reyghema",
            args![
                "Hey...",
                "It feels like you came",
                "here to finish something.",
                "Yeah, there's a complete difference between someone who's aimless and someone with a purpose."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["I almost feel sorry for you though, if you have business to finish in a place like Niflheim."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("lv4_weapon").get()? == 38 || ctx.var("lv4_weapon").get()? == 39) {
        ctx.lines_as("Reyghema", args!["...", "...Finally."])?;
        ctx.next()?;
        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
            ctx.lines_as(
                "Reyghema",
                args!["I did it!", "I died trying to make this,", "but I finally finished it!"],
            )?;
            if ctx.var("lv4_weapon").get()? == 38 {
                ctx.mes("This is... Berserk!")?;
                ctx.call(Function::GetItem, vec![Val::from(1814), Val::from(1)])?;
            } else {
                ctx.mes("This is... Tjungkuletti!")?;
                ctx.call(Function::GetItem, vec![Val::from(1416), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["^333333*Sigh*^000000 I can feel it happening again. My memories are starting to slip away. Can't I be allowed to hold on to this memory?"])?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["I guess the dead aren't supposed to remember their past. But thank you for letting achieve my life long desire, even if this moment is short lived."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "If you ever get the chance, come back and help me remember my",
                    "past so that I can experience the joy of weapon crafting again. Farewell, adventurer..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Reyghema",
                args!["I did it!", "I died trying to make this,", "but I finally finished it!"],
            )?;
            if ctx.var("lv4_weapon").get()? == 38 {
                ctx.mes("This is... the Rudra Bow!")?;
                ctx.call(Function::GetItem, vec![Val::from(1720), Val::from(1)])?;
            } else {
                ctx.mes("This is... Brocca! Weee~ I made it!")?;
                ctx.call(Function::GetItem, vec![Val::from(1415), Val::from(1)])?;
            }
            ctx.var("lv4_weapon").set(Val::from(0))?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["^333333*Sigh*^000000 I can feel it happening again. My memories are starting to slip away. Can't I be allowed to hold on to this memory?"])?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["I guess the dead aren't supposed to remember their past. But thank you for letting achieve my life long desire, even if this moment is short lived."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "If you ever get the chance, come back and help me remember my",
                    "past so that I can experience the joy of weapon crafting again. Farewell, adventurer..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((((ctx.var("lv4_weapon").get()? == 36 && ctx.call(Function::CountItem, vec![Val::from(7292)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7293)])?.number()? > 29)
        && ctx.call(Function::CountItem, vec![Val::from(7289)])?.number()? > 29)
        || (((ctx.var("lv4_weapon").get()? == 37 && ctx.call(Function::CountItem, vec![Val::from(7297)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7291)])?.number()? > 29)
            && ctx.call(Function::CountItem, vec![Val::from(7295)])?.number()? > 29))
    {
        ctx.lines_as(
            "Reyghema",
            args![
                "Wait...",
                "The stuff that you have. Those are things I'll need for my creation! Let me take a look..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Yes, this is perfect!",
                "But how did you know exactly what materials I needed? Oh, it doesn't matter, I guess."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Now it's time for the hardest,",
                "or easiest, part. It's time to test your luck. We can't go through with this if your luck is too weak."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["Alright, we're going to play '^660000Rock, Paper, Scissors^000000.' You've gotta win 2 out of 3 for us to go ahead with the forging."])?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["If your luck is bad and you lose, we'll need to dump a bunch of one of the ores your brought to get rid of your bad luck. That's how it works."])?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Alright, here's a piece of paper. When I say so, you write down 'Rock,' 'Paper' or 'Scissors.' Alright? Let's start."],
        )?;
        ctx.next()?;
        l_npchand1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand1 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand1.clone() == 1 && l_npchand1.clone() == 3) || (l_myhand1.clone() == 2 && l_npchand1.clone() == 1))
            || (l_myhand1.clone() == 3 && l_npchand1.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as("Reyghema", args!["Okay, now let's", "try this again."])?;
        ctx.next()?;
        l_npchand2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand2 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand2.clone() == 1 && l_npchand2.clone() == 3) || (l_myhand2.clone() == 2 && l_npchand2.clone() == 1))
            || (l_myhand2.clone() == 3 && l_npchand2.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        ctx.lines_as("Reyghema", args!["Last time.", "Write down 'Rock,'", "'Paper' or 'Scissors.'"])?;
        ctx.next()?;
        l_npchand3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        l_myhand3 = Val::from(runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])?);
        if (((l_myhand3.clone() == 1 && l_npchand3.clone() == 3) || (l_myhand3.clone() == 2 && l_npchand3.clone() == 1))
            || (l_myhand3.clone() == 3 && l_npchand3.clone() == 2))
        {
            l_shobu = (l_shobu.clone() + Val::from(1));
        }
        l_aekddam = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if l_shobu.clone().number()? > 1 {
            if ctx.var("lv4_weapon").get()? == 36 {
                ctx.call(Function::DelItem, vec![Val::from(7292), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7289), Val::from(30)])?;
            } else {
                ctx.call(Function::DelItem, vec![Val::from(7297), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
                ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
            }
            ctx.var("lv4_weapon").set((ctx.var("lv4_weapon").get()? + Val::from(2)))?;
        } else if l_shobu.clone().number()? < 2 {
            if l_aekddam.clone() == 1 {
                if ctx.var("lv4_weapon").get()? == 36 {
                    ctx.call(Function::DelItem, vec![Val::from(7292), Val::from(30)])?;
                } else {
                    ctx.call(Function::DelItem, vec![Val::from(7297), Val::from(30)])?;
                }
            } else {
                if l_aekddam.clone() == 2 {
                    if ctx.var("lv4_weapon").get()? == 36 {
                        ctx.call(Function::DelItem, vec![Val::from(7293), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7291), Val::from(30)])?;
                    }
                } else if l_aekddam.clone() == 3 {
                    if ctx.var("lv4_weapon").get()? == 36 {
                        ctx.call(Function::DelItem, vec![Val::from(7289), Val::from(30)])?;
                    } else {
                        ctx.call(Function::DelItem, vec![Val::from(7295), Val::from(30)])?;
                    }
                }
            }
        }
        ctx.lines_as("Reyghema", args!["Alright, now", "I wrote down..."])?;
        if l_npchand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_npchand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_npchand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_npchand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["You wrote down..."])?;
        if l_myhand1.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand1.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand1.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand2.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand2.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand2.clone() == 3 {
            ctx.mes("Paper")?;
        }
        if l_myhand3.clone() == 1 {
            ctx.mes("Scissors")?;
        } else if l_myhand3.clone() == 2 {
            ctx.mes("Rock")?;
        } else if l_myhand3.clone() == 3 {
            ctx.mes("Paper")?;
        }
        ctx.next()?;
        if l_shobu.clone().number()? > 1 {
            ctx.lines_as(
                "Reyghema",
                args![
                    ((Val::from("Nice. You won ") + l_shobu.clone())
                        + Val::from(" times. Your luck is really good, so now we're finally finished with this nonsense."))
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "Give me some time to get some things ready. Hand me the ores",
                    "now, and talk to me a little later, alright?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_shobu.clone().number()? < 2 {
            ctx.lines_as(
                "Reyghema",
                args![
                    "Damn...",
                    "I should be happy I won, but that just means your luck is bad and",
                    "we need to do this again."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["I'm sorry, but I'll need to take a bunch of one of your ores to get rid of your bad luck. Come back with all of the ore we'll need so we can test your luck again, alright?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ((ctx.var("lv4_weapon").get()? == 35 || ctx.var("lv4_weapon").get()? == 36) || ctx.var("lv4_weapon").get()? == 37) {
        ctx.lines_as(
            "Reyghema",
            args![
                "Arrrgh!",
                "I still can't",
                "remember what",
                "I need yet! I swear,",
                "this is killing me!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Damn.", "Damn, damn, damn!", "'Killing me?!' I'm already dead!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((((ctx.var("lv4_weapon").get()? == 34 && ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 1)
        && ctx.call(Function::CountItem, vec![Val::from(989)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(710)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 19)
    {
        ctx.lines_as(
            "Reyghema",
            args!["Good work, you've brought everything I asked for. However, there's been a bit of a snafu in the plans..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["The original plan was that I'd remember everything else I need once you came back with this stuff. But I didn't."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Just give me some time to think about it some more. For now, let",
                "me hold on to the things you've brought. You've got my word that I'll be using these to make a weapon for you."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(1005), Val::from(2)])?;
        ctx.call(Function::DelItem, vec![Val::from(989), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(710), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(20)])?;
        ctx.var("lv4_weapon").set(Val::from(35))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 34 {
        ctx.lines_as(
            "Reyghema",
            args![
                "I better remind",
                "you what things I'll",
                "be needing. If you need",
                "to, write it all down."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "1 Emperium Anvil,",
                "2 Hammer of Blacksmith,",
                "1 Illusion Flower",
                "and 20 Gold..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["There were a few more critically important things, but I can't seem to recall them right now..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 33 {
        ctx.lines_as(
            "Reyghema",
            args!["Alright, you're back. I remember most of the things I'll be needing. Go ahead and note this down."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "1 Emperium Anvil,",
                "2 Hammer of Blacksmith,",
                "1 Illusion Flower",
                "and 20 Gold..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["There were a few other", "important things, but I can't seem to remember what they were. Hopefully, by the time you come back with the items I'll know by then."])?;
        ctx.next()?;
        ctx.lines_as("Reyghema", args!["For now, just go and bring back the things I asked for. The sooner, the better. Hurry, I don't know how long my memories will last..."])?;
        ctx.var("lv4_weapon").set(Val::from(34))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("lv4_weapon").get()? == 32 {
        ctx.lines_as("Reyghema", args!["Damn it...", "I didn't want to die!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Well, nothing that's alive really wants to die, but I was so close to finishing what I wanted to do. Ah, youth..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Youth...",
                "Enjoy your youth...",
                "I remember hearing",
                "something like that,",
                "but there was more to it..."
            ],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_text(ctx, None, None)?;
        l_input_s = input;
        if l_input_s.clone() == "Live without regret." {
            ctx.lines_as("Reyghema", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["...", ".......", "Enjoy your youth.", "Live without regret."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "Of course...",
                    "My father, Bill Thayer used to say that all the time! That's right!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["Hey you, thanks for refreshing my memory. It's all coming back to me. I used to be a weaponsmith, but I regret never finishing my final project..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "You just gotta help me realize",
                    "my final goal. Even if I am dead,",
                    "I really wanna complete the experiment that got me killed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Reyghema", args!["All you need to do for now is bring me the things I need. I don't think I'll be able to gather much, being here in Niflheim and all."])?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args![
                    "Come back a little later.",
                    "By then, I should remember what kinds of things I'll need."
                ],
            )?;
            ctx.var("lv4_weapon").set(Val::from(33))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Reyghema",
                args!["What...?", "You playin' games", "with me or what?", "Get outta my face~"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.var("lv4_weapon").get()? == 31 {
        ctx.lines_as("Reyghema", args!["Damn it...", "I didn't want to die!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Well, nothing that's alive really wants to die, but I was so close to finishing what I wanted to do. Ah, youth..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args![
                "Youth...",
                "Enjoy your youth...",
                "I remember hearing",
                "something like that,",
                "but there was more to it..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Enjoy your youth...", "Enjoy your youth...", "...What came after that?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Reyghema",
            args!["Argh!", "I can't remember!", "Everything in my", "head is so cloudy!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Reyghema", args!["Damn it...", "I didn't want to die!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Reyghema",
        args!["Well, nothing that's alive really wants to die, but I was so close to finishing what I wanted to do. Ah, youth..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Reyghema",
        args![
            "Youth...",
            "Enjoy your youth...",
            "I remember hearing",
            "something like that,",
            "but there was more to it..."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Show interest.:Ignore him.")])? {
        1 => {
            ctx.lines_as(
                "Reyghema",
                args!["Enjoy your youth...", "Enjoy your youth...", "...What came after that?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Reyghema",
                args!["Argh!", "I can't remember!", "Everything in my", "head is so cloudy!"],
            )?;
            ctx.var("lv4_weapon").set(Val::from(31))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Reyghema",
                args!["Enjoy your youth...", "Enjoy your youth...", "...What came after that?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn reyghema_lv4(ctx: &Ctx) -> Script {
    reyghema_lv4_body(ctx, Vec::new()).map(|_| ())
}
