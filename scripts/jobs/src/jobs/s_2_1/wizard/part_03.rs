use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn white_dog_wiz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_WIZARD")?) {
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
            ctx.lines_as(
                "Maria",
                args!["Instead of sticking around here, wouldn't it be better to go out and test how strong you've become?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args!["Don't forget that Wizards grow and improve in power each and every day."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
            return Err(Stop::End);
        } else if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
            ctx.lines_as("Dog", args!["What? Kiddo!", "Is a Dog talking so amusing to you?"])?;
        } else {
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(2)])?;
            ctx.lines_as("Dog", args!["Sheesh...Why would a person that can't even cast a spell a single spell come up here?", "*Pfft* If you're that bored, do the world a favor and climb to the top of this building via the outside, then proceed to do some acrobatics if you get there."])?;
        }
        ctx.next()?;
        ctx.lines_as("Dog", args!["*Bark* Get lost!", "I don't have time for people like you!"])?;
        ctx.close_window()?;
        ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(255)])?;
        return Err(Stop::End);
    }
    if ctx.var("wiz_q").get()? == 0 {
        ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
        ctx.lines_as(
            "Dog",
            args!["Ah...I know what you're about to say. You want to change jobs to a Wizard, right?"],
        )?;
        ctx.next()?;
        ctx.lines_as("Dog", args!["Go talk to Catherine. She'll help you."])?;
        ctx.next()?;
        ctx.lines_as(
            "Dog",
            args!["Also, if you would like to know anything about the job change process, I can explain."],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from(".......:A Dog is talking to me...")])? {
            1 => {
                ctx.lines_as(
                    "Dog",
                    args![
                        "...*bark*...? What is it?? Why are you looking at me like that?!",
                        "Is it your first time seeing a Dog talk?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dog",
                    args![
                        "*Bark* It's not common that you see a Dog talking I suppose. bark~",
                        "...Yes i suppose it is a rare site...*grrr*...Stop Gawking for goodness sakes!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dog",
                    args![
                        "My name is 'Maria Splodofska'. Just call me 'Maria'.",
                        "I'm helping candidates that wish to become Wizards."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria03"), Val::from(2)])?;
                ctx.lines_as("Maria", args!["*Bark* Well, the reason I became a dog is...I was helping my boyfriend in experimenting to prepare for his Final for his Magic Degree. Well, *Grrrr* he accidentally turned me into a dog.", "Theoretically, in a couple months the chemicals should wear off and I should be returned to normal. When exactly, I have no idea?"])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                ctx.lines_as("Maria", args!["Well...it doesn't concern you anyways.", "Now, where were we."])?;
                ctx.next()?;
            }
            2 => {
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(2)])?;
                ctx.lines_as(
                    "Dog",
                    args!["*Bark* *bark* *bark* Don't state the obvious! Alright, I know I'm a dog!"],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                ctx.lines_as(
                    "Dog",
                    args![
                        "My name is 'Maria Splodofska'. People call me 'Maria'.",
                        "I'm helping little ones like you that wish to become Wizards."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria03"), Val::from(2)])?;
                ctx.lines_as("Dog called 'Maria'", args!["The reason I became a dog is...My boyfriend that was experimenting to prepare for the Magic Degree, accidentally turned me into a dog.", "Theoretically, in a couple months, the chemicals should wear off and I should be returned to normal. Exactly when, I have no idea?"])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(2)])?;
                ctx.lines_as(
                    "Dog called 'Maria'",
                    args!["I'm not a 'dog' called 'Maria'!! Oi!! Listen to me!!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Dog... 'Maria'...", args![".........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args!["...*BARK* *BARK* *BARK*... I'm upset, but whatever!! You seem busy so I'll just drop it."],
                )?;
                ctx.next()?;
            }
            _ => {}
        }
        ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
        ctx.lines_as(
            "Maria",
            args!["Like I said before, to change jobs, talk to Catherine.", "She's a new Wizard too."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Maria",
            args![
                "I can tell you more about the job change process, but I can't hold back a busy person now can I?",
                "What do you think? Should I explain some about whats in store for Mages that wish to become Wizards?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Yes, please! I would like that.:No, it's ok.:A talking dog...")],
        )? {
            1 => {
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                ctx.lines_as("Maria", args!["OK, I will explain the process for you."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "There are three tests in the job change process.",
                        "The first test is collecting magic items."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "That one begins when you submit an application to Catherine.",
                        "She'll either tell you to collect all types of gemstones, or gather stones with attributes."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "Second test is a magic quiz,",
                        "The gloomy Laurel in the corner is in charge of that part."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "There are questions about magic, monsters, and Mages.",
                        "Out of the 10 questions, if you don't get them all correct, he doesn't let you pass. In other words, you fail..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "Oddly enough, He is in charge of the 3rd test too.",
                        "The third test is eliminating monsters."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "In each room, there are monsters of certain attributes.",
                        "You must attack them with the appropriate spells."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Maria", args!["Well, that's all that I can say. Go apply now."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args!["It's better to just try it yourself than to listen to my descriptions."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                ctx.call(Function::Warp, vec![Val::from("gef_dun00"), Val::from(116), Val::from(102)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Maria", args!["Really? Ok, then go apply and do your best."])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(2)])?;
                ctx.lines_as("Maria", args!["I'm not a DOG!! HOOOOWWWWWWLLLLLLL~"])?;
                ctx.next()?;
                ctx.lines_as("Maria", args!["Dang it! I hope you FAIL!! Go get lost!!"])?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                ctx.call(Function::Warp, vec![Val::from("gef_dun00"), Val::from(116), Val::from(102)])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("wiz_q").get()? == 1 {
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
            ctx.lines_as(
                "Maria",
                args!["You seem lost...", "You've applied, and now you're looking for the items right?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "But since this is only the first test, don't depend on others.",
                    "Complete it yourself. Thats the best way."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "From what I've heard, you have to gather gemstones...",
                    "I can't help you directly, but I can give you some advice."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "First, to get Red Gemstones, go to Culverts in Prontera.",
                    "You can obtain them from the Thief Bugs and Thief Bug eggs found plentiful there."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "Yellow Gemstones are easy to find in the desert.",
                    "Condors, Picky's, and sometimes monsters like Golem's drop them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "And...*woof* to get Blue Gemstones. Try going to the Byalan Dungeons.",
                    "Cornutus, Vadon, and monsters like Mars can drop those Gems."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args![
                    "Of course you can get blue Gemstones at the magic shop here in town...",
                    "But, finding them yourself would be much more rewarding and helpful later in the test."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Maria",
                args!["Anyways, try your best.", "This is the basics of being a Wizard."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
            return Err(Stop::End);
        } else {
            if ctx.var("wiz_q").get()? == 2 {
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                ctx.lines_as(
                    "Maria",
                    args!["You seem lost...", "You've applied, and now you're looking for the items right?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "But since this is only the first test, don't depend on others for help.",
                        "Complete it yourself, thats the best way."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "From what I've heard, you have to gather elemental stones...",
                        "I can't help you directly, but I can give you some advice."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "Well, you can find the Crystal Blue in Byalan Dungeon.",
                        "Cornutus, Kukre, Marina, Vadon...these monsters drop them frequently."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "You can get Green Live from insect type monsters.",
                        "Try hunting monsters like Horn, Mantis, or Vitata."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "Oh and *woof*, Red Blood. I heard you can get a lot of those from...",
                        "Elder Willows, Metallers or Scorpions found in the desert would work well too."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args![
                        "For Wind of Verdue. Hornet, Stainer, Steel Chonchon.",
                        "If you try just a bit, you can get them really easily."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Maria",
                    args!["But anyways, always try your best.", "It's the basics of being a Wizard."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                return Err(Stop::End);
            } else {
                if ctx.var("wiz_q").get()? == 3 {
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "Don't be too relieved just after the first test.",
                            "Try your best, as you still have two more tests to go."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 4 {
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                    ctx.lines_as("Maria", args!["ZzzzZzzzZzzz..."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(2)])?;
                    ctx.lines_as("Maria", args!["*wimper*...Blizadris...you suck...Zzz..."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria02"), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 5 {
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                    ctx.lines_as("Maria", args!["Oh, you're doing well aren't you?"])?;
                    ctx.next()?;
                    ctx.lines_as("Maria", args!["Well, try your best to the very end.", "Laurel is waiting."])?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 6 {
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "*BARK*...you gave up?",
                            "*Sigh*...How can you become a Wizard with such a weak heart?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "You know that each room has monsters of the same attribute...",
                            "If you're a person that deals with magic, you need to know about the different spells."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "You also need to learn how to counter monsters that use skills.",
                            "Your best bet is to kill the monsters that are attacking you first."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "*Grrr* Anyways, continue the test.",
                            "Don't have a weak mind, *woof* and go back! *Bark* *Bark* Right this moment! *BARK*"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                    return Err(Stop::End);
                } else if ctx.var("wiz_q").get()? == 7 {
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(2)])?;
                    ctx.lines_as(
                        "Maria",
                        args![
                            "As I thought, I knew you'd be able to do it, I could smell it in yah! *Woof*",
                            "Now I can call you Wizard."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Maria",
                        args!["Congratulations. Always give your best at everything, no matter what."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Cutin, vec![Val::from("job_wizard_maria01"), Val::from(255)])?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn white_dog_wiz(ctx: &Ctx) -> Script {
    white_dog_wiz_body(ctx, Vec::new()).map(|_| ())
}
