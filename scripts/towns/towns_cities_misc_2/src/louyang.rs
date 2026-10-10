#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn girl_louyang(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Girl",
        args![
            "La la la la~",
            "I feel so good today~",
            "I'm in the mood to go",
            "on a picnic somewhere~",
            "La la la la~"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["About Luoyang.", "Go to Luoyang.", "Cancel."])? {
        0 => {
            ctx.lines_as(
                "Girl",
                args![
                    "Oh, are you",
                    "interested in Luoyang?",
                    "It's a nice place to",
                    "visit for travelers."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Girl",
                args![
                    "Luoyang has a long history",
                    "with stories of ancient magic and warriors. It's also rumored that many evil beasts roam the",
                    "Luoyang area."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Girl",
                args![
                    "You can find cure-all medicines, mysterious occurrences, and",
                    "martial artists all in one place!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Girl", args!["I used to train in the martial arts every morning back when I was in Luoyang. I might not look like it, but I'm pretty strong!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Girl",
                args![
                    "If you want to visit",
                    "Luoyang, feel free to",
                    "tell me. Just give me",
                    "some Zeny and we'll go~"
                ],
            )?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Girl",
                args![
                    "I'll guide you to",
                    "Luoyang right away.",
                    "For my service, I am",
                    "accepting 10,000 Zeny."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Girl", args!["So, are you ready?"])?;
            ctx.next()?;
            if ctx.menu(&["Yes!", "No."])? == 0 {
                if ctx.player().zeny()? > 9999 {
                    ctx.lines_as("Girl", args!["Okay~", "Ready!", "Have fun!"])?;
                    ctx.close_window()?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
                    ctx.warp("lou_fild01", 190, 101)?;
                    return ctx.end();
                }
                ctx.lines_as(
                    "Girl",
                    args!["...", "You don't seem", "to have 10,000 Zeny...", "Go get some money first!"],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Girl",
                args!["Oh...", "It's so disappointing", "to hear you say that.", "Well, have a good day!"],
            )?;
            ctx.close()
        }
        _ => {
            ctx.lines_as("Girl", args!["Oh...", "Have a good day!"])?;
            ctx.close()
        }
    }
}

pub fn girl_1lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Girl", args!["Would you", "like to go back", "to Alberta?"])?;
    ctx.next()?;
    if ctx.menu(&["Go back to Alberta.", "Cancel."])? == 0 {
        ctx.lines_as("Girl", args!["I hope to", "see you again!", "Bye bye!"])?;
        ctx.close_window()?;
        ctx.warp("alberta", 235, 45)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Girl",
        args![
            "If you like this",
            "area, why don't you",
            "stay and enjoy the",
            "the food and the sights!"
        ],
    )?;
    ctx.next()?;
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.lines_as("Girl", args!["And by sights...", "I mean girls!", "Tee hee~"])?;
    } else {
        ctx.lines_as("Girl", args!["And the boys here", "aren't bad looking~"])?;
    }
    ctx.close()
}

pub fn muscular_woman_lou(ctx: &Ctx) -> Script {
    if ctx.var("Sex").get()? == constants::SEX_FEMALE {
        ctx.lines_as(
            "Zhi Ching Li",
            args!["All the members of the Maiden Palace, including myself and our master, are all female."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zhi Ching Li",
            args![
                "Recently we've had a hard time recruiting new members, so I came here to check if there's any woman who wishes to join us."
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_THINK])?;
        return ctx.close();
    }
    ctx.lines_as("Zhi Ching Li", args!["..."])?;
    ctx.next()?;
    ctx.lines_as("Zhi Ching Li", args!["...", "......"])?;
    ctx.next()?;
    ctx.lines_as("Zhi Ching Li", args!["Please leave me", "alone, I'm busy."])?;
    ctx.close()
}

pub fn powerful_looking_guy_lou(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Akiira",
        args![
            "I am practicing my 'Claw of Dragon.' I not only need to use the power of my fists, I must also condition myself spiritually."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Akiira",
        args![
            "Every martial art requires",
            "spiritual training since the",
            "mind controls the body.",
            "If you've trained yourself spiritually, you can easily",
            "use any part of the body!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Akiira",
        args![
            "If you are considering",
            "studying the martial arts, you should first attain knowledge before jumping into the",
            "physical training."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Akiira",
        args![
            "Learn about the martial arts",
            "and meditate on life's truths. First, you must find peace of mind before you can hope to master the mind and body."
        ],
    )?;
    ctx.close()
}

pub fn fist_master_lou(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Zhiang Xiau Ji",
        args!["Finally...", "I have mastered", "the 'Claw of Dragon!'"],
    )?;
    ctx.next()?;
    ctx.lines_as("Zhiang Xiau Ji", args!["Although there are eight basic steps, I had to learn the history of this art, and meditate, focusing on my spiritual improvement,", "for three years."])?;
    ctx.next()?;
    ctx.lines_as("Zhiang Xiau Ji", args!["After that, my master finally started to give me the physical training so I could use the eight steps of the Claw of Dragon. I've devoted myself to this art for thirty years."])?;
    ctx.next()?;
    ctx.lines_as("Zhiang Xiau Ji", args!["I'm very proud that I've", "mastered this art ten years earlier than I expected. Now, I need to study this form and improve it by correcting its weak points and enhancing its strengths."])?;
    ctx.next()?;
    ctx.lines_as(
        "Zhiang Xiau Ji",
        args!["I guess that would take me about ten years. But I'm not disheartened by that at all."],
    )?;
    ctx.next()?;
    ctx.lines_as("Zhiang Xiau Ji", args!["When you're learning a martial art, you can't rush yourself and learn everything in a short period of time. It's impossible! Plus, that isn't the essence of art..."])?;
    ctx.close()
}

pub fn trainee_1lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Yeeeyap~!", "Taaaaaah~~!!", "Hooo~."])?;
    ctx.close()
}

pub fn trainee_2lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Tah Tah Tah!", "Taaaaaah~~!!", "Schwooooooo~"])?;
    ctx.close()
}

pub fn trainee_3lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Si!", "Ayah!!"])?;
    ctx.close()
}

pub fn trainee_4lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Dergh!", "Dergh!", "Schwa--!"])?;
    ctx.close()
}

pub fn trainee_5lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Yah Yah Yah!", "Taaaaaah~~!!", "Wataaaaaaaah!"])?;
    ctx.close()
}

pub fn trainee_6lou(ctx: &Ctx) -> Script {
    ctx.lines_as("Trainee", args!["Yeeeyap~!", "Taaaaaah~~!!", "Hooo~"])?;
    ctx.close()
}

pub fn friendly_looking_lady_lo(ctx: &Ctx) -> Script {
    ctx.lines_as("Hong Miao", args!["Welcome."])?;
    ctx.next()?;
    ctx.lines_as("Hong Miao", args!["This is an elevator which leads", "to the Observation Tower. We are providing you a safe and fast transfer service for an affordable fee. Would you like to use this service?"])?;
    ctx.next()?;
    match ctx.menu(&["Information.", "Yes.", "Maybe next time."])? {
        0 => {
            ctx.lines_as("Hong Miao", args!["After many suggestions and proposals were sent to the Luoyang tourism office, the Observation Tower was built so tourists can enjoy the sights."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hong Miao",
                args![
                    "Due to the geographical",
                    "features of Luoyang, it's difficult to enjoy the breath taking view that our land has to offer."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hong Miao", args!["You can come up to the tower by taking the elevator right here. We are providing this quick and safe transfer service for 500 zeny per person."])?;
            ctx.close()
        }
        1 => {
            if ctx.player().zeny()? < 500 {
                ctx.lines_as("Hong Miao", args!["I'm sorry, but you do not have enough zeny. I hope you'll come back later to enjoy the Observation Tower. Have a good day."])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Hong Miao",
                args![
                    "Thank you for your patronage.",
                    "We are trying to provide you with the best of service. Please",
                    "come again."
                ],
            )?;
            ctx.next()?;
            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
            ctx.warp("lou_in01", 17, 19)?;
            return ctx.end();
        }
        _ => {
            ctx.lines_as("Hong Miao", args!["Please come", "back later.", "Have a good day."])?;
            ctx.close()
        }
    }
}

pub fn exit_lou(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "^3355FFThere is some sort",
        "of descent apparatus.",
        "Would you like to use it?^000000"
    ])?;
    ctx.next()?;
    if ctx.menu(&["Yes.", "No."])? == 0 {
        if ctx.call(Function::Rand, args![1, 100])? == 34 {
            ctx.call(Function::PercentHeal, args![-99, 0])?;
            ctx.warp("louyang", 86, 269)?;
            ctx.call(
                Function::MapAnnounce,
                args![
                    "louyang",
                    Val::from("") + ctx.call(Function::StrNpcInfo, args![0])? + Val::from(" : Oh God, I'm faaaaaaaaaaaalling~~!!!!"),
                    constants::BC_MAP
                ],
            )?;
        } else {
            ctx.warp("lou_in01", 10, 18)?;
        }
        return ctx.end();
    }
    ctx.close()
}
