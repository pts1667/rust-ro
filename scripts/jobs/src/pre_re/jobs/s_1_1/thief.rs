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

pub fn thief_guide(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 1 {
        if ctx.var("advjob").get()? == constants::JOB_ASSASSIN_CROSS || ctx.var("advjob").get()? == constants::JOB_STALKER {
            if ctx.var("Class").get()? == constants::JOB_NOVICE_HIGH {
                ctx.lines_as(
                    "Thief Guide",
                    args!["Huh? Do I know you? It's creepy that you seem so familiar. You don't have a twin, do you?"],
                )?;
                ctx.next()?;
                if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                    ctx.lines_as(
                        "Thief Guide",
                        args!["What, do you want to be a Thief? I'm sorry, but you look like you need more training."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Thief Guide",
                        args!["Take your time and learn all the Basic Skills, will you? Well then, see you later~!"],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as("Thief Guide", args!["Well, I got this feeling like you've been through a lifetime of fighting, so I'm promoting you to a Thief right this minute. I better give you tough guys what you want..."])?;
                ctx.next()?;
                ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
                ctx.call(Function::JobChange, args![constants::JOB_THIEF_HIGH])?;
                ctx.call(Function::Skill, args!["TF_SPRINKLESAND", 1, constants::SKILL_PERM])?;
                ctx.call(Function::Skill, args!["TF_BACKSLIDING", 1, constants::SKILL_PERM])?;
                ctx.call(Function::Skill, args!["TF_PICKSTONE", 1, constants::SKILL_PERM])?;
                ctx.call(Function::Skill, args!["TF_THROWSTONE", 1, constants::SKILL_PERM])?;
                ctx.lines_as(
                    "Thief Guide",
                    args!["Since you've become a Thief, live as a Thief. Now, go for it! Next~"],
                )?;
                return ctx.close();
            } else {
                ctx.mes("[Thief Guide]")?;
                if ctx.var("Sex").get()? == constants::SEX_MALE {
                    ctx.mes("Hey, dude.")?;
                } else {
                    ctx.mes("Hey, baby~")?;
                }
                return ctx.close();
            }
        } else {
            ctx.mes("[Thief Guide]")?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("Hey, dude.")?;
            } else {
                ctx.mes("Hey, baby.")?;
            }
            ctx.mes("...Hey! You look too goody-goody to want to be a Thief!! Now scram, I'm busy. Next!")?;
            return ctx.close();
        }
    }
    if ctx.var("BaseJob").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Thief Guide",
            args!["If you have a problem, feel free to speak to me anytime, alright?"],
        )?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? != constants::JOB_NOVICE && ctx.var("BaseJob").get()? != constants::JOB_THIEF {
        ctx.lines_as("Thief Guide", args!["What the heck...?"])?;
        match ctx.var("Class").get()?.number()? {
            1 => {
                ctx.lines(args!["Huh.", "Now, that's", "a big sword."])?;
                ctx.next()?;
                ctx.lines_as("Thief Guide", args!["So...", "Trying to make", "up for something", "...Buddy?"])?;
            }
            2 => {
                ctx.mes("What's a Mage doin' here? Shouldn't you be doing card tricks elsewhere? Oh well, it's a free country...")?;
                ctx.next()?;
                ctx.lines_as("Thief Guide", args!["Oh wait,", "it's not...", "Get outta here!"])?;
            }
            3 => {
                ctx.lines(args![
                    "Man, shouldn't you",
                    "Archers be playing",
                    "in the forest",
                    "or something?"
                ])?;
            }
            4 => {
                ctx.mes("You know we all steal for a living, right? What are you doing in this kinda place, Acolyte?")?;
            }
            5 => {
                ctx.lines(args![
                    "You're a Merchant,",
                    "right? Why are you",
                    "walking into a den",
                    "of Thieves?!"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Thief Guide",
                    args![
                        "It's like you're begging",
                        "us to steal from you!",
                        "Come on, hurry and",
                        "get outta here~"
                    ],
                )?;
            }
            8 => {
                ctx.lines(args!["Oh my God...", "Am I dying?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Thief Guide",
                    args!["Why else would a Priest come here? I guess I better start confessing all of my misdeeds."],
                )?;
                return ctx.close();
            }
            12 => {
                ctx.mes("Didn't you use to be one of us?! Man, you changed. You seem real dangerous now...")?;
                return ctx.close();
            }
            17 => {
                ctx.mes("Man, you got real cool all of a sudden! You must have some skills I can only dream of!")?;
                return ctx.close();
            }
            _ => {}
        }
        ctx.next()?;
        ctx.lines_as(
            "Thief Guide",
            args!["*Sigh* Look, there's really no need for you to be in this kind of place. You oughta go where you ought to go."],
        )?;
        return ctx.close();
    }
    if (ctx.var("job_thief_q").get()? == 3 && ctx.items().count(1069)? > 0) || ctx.items().count(1070)? > 0 {
        ctx.lines_as(
            "Thief Guide",
            args!["Hmmm?", "You gathered Mushrooms for", "the Thief test, right?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Thief Guide",
            args!["Here, talk to the other guy right next to me. He's the one in charge of checking your Mushrooms."],
        )?;
        return ctx.close();
    } else if ctx.var("job_thief_q").get()? == 3 {
        ctx.lines_as("Thief Guide", args!["So how was the", "Mushroom Farm?", "Have any fun?"])?;
        ctx.next()?;
        if ctx.menu(&["Yeah, kinda Cool.", "It was horrible."])? == 0 {
            ctx.lines_as(
                "Thief Guide",
                args![
                    "Heh heh! That's a good attitude. In our line of work, you gotta enjoy getting your hands dirty, one way or another."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Thief Guide", args!["Yeah? I've been there too, so I can see why that place isn't everyone's cup of tea. Still, being a Thief isn't all glamour and trendy night life."])?;
        return ctx.close();
    }
    if ctx.var("job_thief_q").get()? == 2 {
        ctx.lines_as(
            "Thief Guide",
            args!["Hey, whaddya doin' here? Aren't you supposed to be gathering Mushrooms? Or did you need it explained to you again?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes.", "No, that's okay."])? == 0 {
            ctx.lines_as(
                "Thief Guide",
                args!["*Sigh* Well, there's always one in the bunch. Alright, listen carefully."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Alright, for your test, you gotta steal Mushrooms from a farm. Don't worry, the guy who owns the farm deserves to be robbed."])?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Anyway, you gotta gather two kinds of Mushrooms: ^0000FFOrange Net Mushrooms^000000 and ^0000FFOrange Gooey Mushrooms^000000."])?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Be careful, since there are monsters are the farm that are there to protect the Mushrooms. So this will be no walk in the park."])?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["When you come back here after gathering Mushrooms, you'll be graded on the Mushrooms you've collected."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Each Orange Net Mushroom gets you 3 points, and you get 1 point for each Orange Gooey Mushroom. You need a total of 25 points to pass the test."])?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Go outside and keep going ahead toward the Eastern Field of the Pyramids. Then you will see one of our comrades between two columns."])?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["Speak to that guy, and he'll take you to the farm through the backdoor."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["On that field, I think his coordinates are '^FF0000141, 125^000000.' Just type ^3355FF/where^000000 in the right side of your chat box to check your present coordinates."])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Thief Guide",
            args!["Huh. For a second there, I thought you had something really important to tell me."],
        )?;
        return ctx.close();
    }
    ctx.mes("[Thief Guide]")?;
    if ctx.var("job_thief_q").get()? == 0 {
        ctx.lines(args!["What brings you down", "here to this rathole?"])?;
    } else {
        ctx.lines(args!["Ah...", "You came back.", "Are you sure you're", "ready to try again?"])?;
    }
    ctx.next()?;
    if ctx.menu(&["Hey, I came here to be a Thief!", "Nah, I'm just looking around."])? == 0 {
        if ctx.var("job_thief_q").get()? == 0 {
            ctx.lines_as(
                "Thief Guide",
                args!["Heh, I like your confidence. Still, you know being a Thief isn't all what it's cracked up to be."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Still...", "Do you really", "want to be", "a Thief?"])?;
            ctx.next()?;
            match ctx.menu(&["Yeah.", "No, just wasting your time.", "Why did you become a Thief?"])? {
                0 => {
                    ctx.lines_as("Thief Guide", args!["Really..."])?;
                }
                1 => {
                    ctx.lines_as("Thief Guide", args!["Yeah...", "I can see that."])?;
                }
                2 => {
                    ctx.lines_as("Thief Guide", args!["Me...?", "I had no choice at the time. It was either steal or starve. But it's not like I need to give you my life story."])?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["So do you want to", "apply to become", "a Thief or not?"])?;
            ctx.next()?;
            if ctx.menu(&["Yes, I will.", "I'm too scared to be a Thief!"])? == 0 {
                ctx.lines_as("Thief Guide", args!["Alright, tell", "me your name."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Thief Guide",
                    args![
                        Val::from("") + ctx.player().name()? + Val::from("?"),
                        Val::from("What kind of name is ") + ctx.player().name()? + Val::from("? Anyway, give me a second.")
                    ],
                )?;
                ctx.var("job_thief_q").set(Val::from(1))?;
                ctx.next()?;
                ctx.lines_as(
                    "Thief Guide",
                    args!["Alright, your registration has been processed. Okay, you can begin your test if you're ready."],
                )?;
                ctx.next()?;
                if ctx.menu(&["Yeah, I'm ready.", "No, I'm not ready yet."])? == 1 {
                    ctx.lines_as("Thief Guide", args!["Not ready?", "How can you", "not be ready?!"])?;
                    return ctx.close();
                }
            } else {
                ctx.lines_as(
                    "Thief Guide",
                    args!["Too scared?!?", "Hahahahahahah!", "Oh, please...!", "That's hilarious!"],
                )?;
                return ctx.close();
            }
        } else {
            ctx.lines_as("Thief Guide", args!["Okay...", "Give me", "one second."])?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Thief Guide",
            args![
                "Your name is...",
                Val::from(ctx.player().name()? + "? Um, where is it? Ah, here it is. Let's see...")
            ],
        )?;
        ctx.next()?;
        ctx.mes("[Thief Guide]")?;
        if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
            ctx.mes(
                "Isn't that cute? I can see you're ambitious, but you gotta learn all of the Basic Skills before you can become a Thief.",
            )?;
            return ctx.close();
        }
        ctx.mes("Alright. I looked at your Felony Record, and you seem to have a very interesting history. You might have what it takes to be a Thief.")?;
        ctx.next()?;
        ctx.lines_as(
            "Thief Guide",
            args!["Because I feel like it, I now decree that you have passed this interview. Good work!"],
        )?;
        ctx.var("job_thief_q").set(Val::from(2))?;
        ctx.quests().start(1013)?;
        ctx.next()?;
        ctx.lines_as(
            "Thief Guide",
            args!["Now, your actual abilities will need to be tested. Do you know anything about the test?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes, I do.", "Sorry, I don't."])? == 0 {
            ctx.lines_as("Thief Guide", args!["Oh yeah? Well, this makes things a lot easier."])?;
        } else {
            ctx.lines_as(
                "Thief Guide",
                args!["Alright, let me inform you then. Listen carefully. This test decides if you are worthy of becoming a Thief."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["You will be sneaking to Shibu's Farm. He is the worst Merchant, in terms of character, in Morocc."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["Alright, for your test, you gotta steal Mushrooms from his farm. Don't worry, that guy deserves to be robbed."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Anyway, you gotta gather two kinds of Mushrooms: ^0000FFOrange Net Mushrooms^000000 and ^0000FFOrange Gooey Mushrooms^000000."])?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Be careful, since there are monsters are the farm that are there to protect the Mushrooms. So this will be no walk in the park."])?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["When you come back here after gathering Mushrooms, you'll be graded on the Mushrooms you've collected."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Each Orange Net Mushroom gets you 3 points, and you get 1 point for each Orange Gooey Mushroom. You need a total of 25 points to pass the test."])?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["Go outside and keep going ahead toward the Eastern Field of the Pyramids. Then you will see one of our comrades between two columns."])?;
            ctx.next()?;
            ctx.lines_as(
                "Thief Guide",
                args!["Speak to that guy, and he'll take you to the farm through the backdoor."],
            )?;
            ctx.next()?;
            ctx.lines_as("Thief Guide", args!["On that field, I think his coordinates are '^FF0000141, 125^000000.' Just type ^3355FF/where^000000 in the right side of your chat box to check your present coordinates."])?;
        }
        ctx.next()?;
        ctx.lines_as("Thief Guide", args!["Don't forget to make plans and prepare yourself before you go inside the Mushroom Farm. Move as quickly as you can and try not to get killed, alright?"])?;
        return ctx.close();
    }
    return ctx.close();
}

pub fn comrade(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Brad",
            args!["We don't have any special events yet. Come some other time when there's news, alright?"],
        )?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? != constants::JOB_NOVICE && ctx.var("BaseJob").get()? != constants::JOB_THIEF {
        ctx.lines_as("Comrade", args!["Um...", "You don't look", "like a Thief."])?;
        ctx.next()?;
        ctx.lines_as("Comrade", args!["What the heck are", "you doing here anyway?"])?;
        return ctx.close();
    }
    if ctx.var("job_thief_q").get()? == 0 || ctx.var("job_thief_q").get()? == 1 {
        ctx.lines_as(
            "Comrade",
            args!["What's the matter? If you want to be a Thief, speak to the girl beside me."],
        )?;
        return ctx.close();
    } else if ctx.var("job_thief_q").get()? == 2 {
        ctx.lines_as(
            "Comrade",
            args!["Did you pass the interview?", "Then what are you waiting for?"],
        )?;
        return ctx.close();
    } else if ctx.var("job_thief_q").get()? == 3 {
        ctx.lines_as(
            "Comrade",
            args!["Ah, the guide told me about you. So, let me check your mushrooms..."],
        )?;
        if ctx.call(Function::CountItem, args![1069])? == 0 && ctx.call(Function::CountItem, args![1070])? == 0 {
            ctx.next()?;
            ctx.lines_as(
                "Comrade",
                args![
                    "What the hell...",
                    "You don't have any Mushrooms at all! Go back and get them. Otherwise, you won't pass the test and become a Thief!"
                ],
            )?;
            return ctx.close();
        }
        ctx.next()?;
        let net_points = ctx.call(Function::CountItem, args![1069])?.number()? * 3;
        let gooey_points = ctx.call(Function::CountItem, args![1070])?.number()?;
        let total = net_points + gooey_points;
        let money = net_points * 5 + gooey_points * 2 + 200;
        ctx.mes("[Comrade]")?;
        if ctx.call(Function::CountItem, args![1069])? != 0 {
            ctx.lines(args![
                "First, let me check the Orange Net Mushrooms you got.",
                ((Val::from("Huh, ") + ctx.call(Function::CountItem, args![1069])?) + Val::from(" of them."))
            ])?;
        }
        if ctx.call(Function::CountItem, args![1070])? != 0 {
            ctx.lines(args![
                ((Val::from("Now I'll just check your Orange Gooey Mushrooms. That's ") + ctx.call(Function::CountItem, args![1070])?)
                    + Val::from(" you gathered."))
            ])?;
        }
        ctx.next()?;
        ctx.lines_as("Comrade", args!["So that would", "bring your total to..."])?;
        ctx.next()?;
        ctx.lines_as("Comrade", args![((Val::from("Hmmm. ") + total) + Val::from(" degrees, multiplied by the speed of light, divided by the integral of pi times height plus the absolute value of politics..."))])?;
        ctx.next()?;
        ctx.lines_as("Comrade", args!["Okay!", "I got it."])?;
        ctx.next()?;
        ctx.mes("[Comrade]")?;
        if total > 25 {
            ctx.lines(args!["You got more", "than 25 points!", "Awesome!"])?;
        } else if total == 25 {
            ctx.lines(args!["Exactly 25 points!", "You did it! Badass!"])?;
        } else {
            ctx.mes("Definitely less than the 25 points you need to pass. Go out there and get me more Mushrooms!")?;
            return ctx.close();
        }
        ctx.next()?;
        ctx.lines_as(
            "Comrade",
            args![
                ctx.player().name()? + "...",
                "You have passed the official Thief Test. You are now one of us."
            ],
        )?;
        if ctx.call(Function::CountItem, args![1069])? != 0 {
            ctx.call(Function::DelItem, args![1069, ctx.call(Function::CountItem, args![1069])?])?;
        }
        if ctx.call(Function::CountItem, args![1070])? != 0 {
            ctx.call(Function::DelItem, args![1070, ctx.call(Function::CountItem, args![1070])?])?;
        }
        shared::other_global_functions::job_change(ctx, args![constants::JOB_THIEF])?;
        shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
        ctx.quests().complete(1013)?;
        ctx.next()?;
        ctx.lines_as(
            "Comrade",
            args!["Congratulations on becoming a Thief! From now, be an honorable representative of the Thief's Guild."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Comrade",
            args!["If you bring disgrace to our guild, you will be killed. I expect you to bring our comrades pride."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Brad",
            args!["*Ahem* Welcome to the Guild, comrade! I'm Brad, and I'm in charge of human resources here."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Brad",
            args!["Here is a small subsidy for a Newbie like you. Spend it whereever you want. Alright then, I'll see you around~"],
        )?;
        ctx.player().set_zeny(ctx.player().zeny()? + money)?;
        return ctx.close();
    }
    Ok(())
}

pub fn mr_irrelevant(ctx: &Ctx) -> Script {
    if ctx.var("BaseJob").get()? == constants::JOB_THIEF {
        ctx.lines_as(
            "Mr. Irrelevant",
            args!["Ah, I see that you are now a Thief. I always knew you'd join us."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Irrelevant",
            args!["Stealing from a Mushroom farm is too easy for you now. You should build up your skills and master our craft."],
        )?;
        return ctx.close();
    } else if ctx.var("BaseJob").get()? != constants::JOB_NOVICE && ctx.var("BaseJob").get()? != constants::JOB_THIEF {
        ctx.mes("[Mr. Irrelevant]")?;
        match ctx.rand(4)? {
            1 => {
                ctx.mes("I could use a good, hard drink.")?;
                return ctx.close();
            }
            2 => {
                ctx.mes("Gimme your money.")?;
                ctx.next()?;
                ctx.lines_as("Mr. Irrelevant", args!["Kidding, I'm off the clock."])?;
                return ctx.close();
            }
            3 => {
                ctx.mes("WHO YOU CALLING A PSYCHO?!?!")?;
                return ctx.close();
            }
            4 => {
                ctx.mes("I've got nothing to say to you. Would you mind leaving me alone?")?;
                return ctx.close();
            }
            _ => {
                ctx.mes("Today looks like a good day to go to the pyramids and hunt with some of my friends.")?;
                return ctx.close();
            }
        }
    }
    if ctx.var("job_thief_q").get()? == 3 {
        ctx.lines_as(
            "Mr. Irrelevant",
            args!["Hahahahaha~!", "You haven't", "passed the test yet?", "Alright, I'll let you in..."],
        )?;
        ctx.close_window()?;
        match ctx.rand(5)? {
            1 => {
                ctx.warp("job_thief1", 228, 106)?;
                return ctx.end();
            }
            2 => {
                ctx.warp("job_thief1", 38, 50)?;
                return ctx.end();
            }
            3 => {
                ctx.warp("job_thief1", 66, 331)?;
                return ctx.end();
            }
            4 => {
                ctx.warp("job_thief1", 196, 331)?;
                return ctx.end();
            }
            _ => {
                ctx.warp("job_thief1", 309, 234)?;
                return ctx.end();
            }
        }
    } else if ctx.var("job_thief_q").get()? == 2 {
        ctx.lines_as(
            "Mr. Irrelevant",
            args![
                "Hmm...",
                "You've come to take the test, right? I can see in your eyes that you know something."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mr. Irrelevant",
            args![
                Val::from("Your name is ")
                    + ctx.player().name()?
                    + Val::from(
                        "? Ah, it's on the list. Alright, I'll let you into the Mushroom Farm , but I can't guarantee your safety..."
                    )
            ],
        )?;
        ctx.close_window()?;
        ctx.var("job_thief_q").set(Val::from(3))?;
        match ctx.rand(5)? {
            1 => {
                ctx.warp("job_thief1", 228, 106)?;
                return ctx.end();
            }
            2 => {
                ctx.warp("job_thief1", 38, 50)?;
                return ctx.end();
            }
            3 => {
                ctx.warp("job_thief1", 66, 331)?;
                return ctx.end();
            }
            4 => {
                ctx.warp("job_thief1", 196, 331)?;
                return ctx.end();
            }
            _ => {
                ctx.warp("job_thief1", 309, 234)?;
                return ctx.end();
            }
        }
    } else if ctx.var("job_thief_q").get()? == 1 {
        ctx.lines_as(
            "Mr. Irrelevant",
            args!["There is this strange smell coming from... You. Now why would that be?"],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Mr. Irrelevant",
        args![
            "Hey Novice! Why don't you join the ranks of the Thief Guild? You newbies are always welcome to join us and our selfish cause."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mr. Irrelevant",
        args!["You can get more information in the Underground Room in the Pyramid 1 BF."],
    )?;
    return ctx.close();
}
