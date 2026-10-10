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

fn take_mage_solution(ctx: &Ctx) -> Result<(), Stop> {
    if ctx.call(Function::CountItem, args![1071])? != 0 {
        ctx.items().take(1071, 1)?;
    } else if ctx.call(Function::CountItem, args![1085])? != 0 {
        ctx.items().take(1085, 1)?;
    } else if ctx.call(Function::CountItem, args![1086])? != 0 {
        ctx.items().take(1086, 1)?;
    } else if ctx.call(Function::CountItem, args![1087])? != 0 {
        ctx.items().take(1087, 1)?;
    } else {
        ctx.items().take(1090, 1)?;
    }
    Ok(())
}

pub fn mage_guildsman(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 1 {
        if ctx.var("advjob").get()? == constants::JOB_HIGH_WIZARD || ctx.var("advjob").get()? == constants::JOB_PROFESSOR {
            if ctx.var("Class").get()? == constants::JOB_NOVICE_HIGH {
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["Whoa, long time no see! But weren't you supposed to be dead?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["Ah, you must have been reborn. Well, I'm glad to have you back."],
                )?;
                ctx.next()?;
                if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                    ctx.lines_as("Mage Guildsman", args!["I'm sorry, but I don't think you're ready to learn magic yet. Why don't you go finish learning the Basic Skills first?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mage Guildsman",
                        args!["Take your time. The more you learn, the more ready you'll be to learn magic again."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as("Mage Guildsman", args!["Well, since you have passed the Mage test once, I will not question your qualification. You want to have your magic skills back immediately, don't you?"])?;
                ctx.next()?;
                ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
                ctx.call(Function::JobChange, args![constants::JOB_MAGE_HIGH])?;
                ctx.call(Function::Skill, args!["MG_ENERGYCOAT", 1, constants::SKILL_PERM])?;
                ctx.lines_as("Mage Guildsman", args!["Wow, for some reason, you look way better than you did before. Anyway, I believe you will do a better job being a Mage as well."])?;
                return ctx.close();
            } else {
                ctx.lines_as("Mage Guildsman", args!["Is there anything more I can help you with? If not, why don't you go test your skills? The world is waiting for you~!"])?;
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Mage Guildsman",
                args!["What, are you interested in the Mage guild? I didn't want to tell you this, but you don't belong here."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Guildsman",
                args!["I am not sure why you're still standing in front of me, but I can tell that you're not meant to be a Mage."],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as("Mage Guildsman", args!["Yo. What's up?"])?;
    ctx.next()?;
    match ctx.menu(&["I want to be a Mage.", "Tell me the Requirements.", "Pretty much nothing."])? {
        0 => {
            ctx.mes("[Mage Guildsman]")?;
            if ctx.var("BaseJob").get()? == constants::JOB_MAGE {
                ctx.mes("Hey, haven't you realized? You're aleady a Mage, silly!")?;
                ctx.next()?;
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["One of these days you'll realize the power inside of you when you can make Fire with your mind!"],
                )?;
                return ctx.close();
            }
            if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                ctx.mes("Hey~ C'mon. Quit playing games. You can't be a Mage because you already have another Job.")?;
                return ctx.close();
            }
            if ctx.var("job_magician_q").get()? == 0 {
                ctx.mes("Wanna be a Mage, eh...?")?;
                if ctx.var("Sex").get()? == constants::SEX_MALE {
                    ctx.mes("Hey, look at you! You're kinda cute~! Not my type though...")?;
                } else {
                    ctx.lines(args!["Oooh, you're such a hot babe~!", "I like girls like you~"])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["Right, you said that you wanna be a Mage? Alright then, please sign the Mage Application."],
                )?;
                ctx.next()?;
                if ctx.menu(&["Sign Up.", "Quit."])? == 0 {
                    ctx.lines_as(
                        "Mage Guildsman",
                        args![format!(
                            "Okay. Sign right there. Oh, you're very good at spelling. Alright. So your name is... {}.",
                            ctx.player().name()?
                        )],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mage Guildsman", args!["Now it's time for", "me to give you the test."])?;
                    match ctx.rand_range(0, 3)? {
                        1 => {
                            ctx.lines(args![
                                "Make me a ^3355FFMixed Solution No. 1^000000",
                                "and bring it back to me."
                            ])?;
                            ctx.var("job_magician_q").set(1)?;
                        }
                        2 => {
                            ctx.lines(args![
                                "Make me a ^3355FFMixed Solution No. 2^000000",
                                "and bring it back to me."
                            ])?;
                            ctx.var("job_magician_q").set(2)?;
                        }
                        3 => {
                            ctx.lines(args![
                                "Make me a ^3355FFMixed Solution No. 3^000000",
                                "and bring it back to me."
                            ])?;
                            ctx.var("job_magician_q").set(3)?;
                        }
                        _ => {
                            ctx.lines(args![
                                "Make me a ^3355FFMixed Solution No. 4^000000",
                                "and bring it back to me."
                            ])?;
                            ctx.var("job_magician_q").set(4)?;
                        }
                    }
                    ctx.next()?;
                    ctx.items().give(1092, 1)?;
                    ctx.lines_as("Mage Guildsman", args!["You can find the necessary ingredients inside the Guide Book in this Guild. So you better look up what you need before you go."])?;
                    ctx.next()?;
                    ctx.lines_as("Mage Guildsman", args!["Once you collect all the ingredients you, use the machine in the center of the room to mix the solution. Good luck!"])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Mage Guildsman",
                    args![
                        "Whaaaaat~?! Right after you tell me that you wanna become a Mage, you change your mind?! Be a bit more decisive!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.mes("Yeah? Ready...?")?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.mes("Oh, what a bummer. You haven't met the requirements yet.")?;
                ctx.next()?;
                ctx.lines_as("Mage Guildsman", args!["Go back and reach Novice Job level 9 first. Don't forget that you also have to learn all of the Basic Skills before you come back."])?;
                return ctx.close();
            }
            ctx.lines(args![ctx.player().name()? + "'s test was..."])?;
            if ctx.var("job_magician_q").get()? == 1 {
                ctx.mes("Making Mixed Solution No. 1.")?;
            } else if ctx.var("job_magician_q").get()? == 2 {
                ctx.mes("Making Mixed Solution No. 2.")?;
            } else if ctx.var("job_magician_q").get()? == 3 {
                ctx.mes("Making Mixed Solution No. 3.")?;
            } else {
                ctx.mes("Making Mixed Solution No. 4.")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Mage Guildsman",
                args!["Okay, let me", "check if you made your", "solution accurately..."],
            )?;
            ctx.next()?;
            ctx.mes("[Mage Guildsman]")?;
            if ctx.call(Function::CountItem, args![1071])? == 0
                && ctx.call(Function::CountItem, args![1085])? == 0
                && ctx.call(Function::CountItem, args![1086])? == 0
                && ctx.call(Function::CountItem, args![1087])? == 0
                && ctx.call(Function::CountItem, args![1090])? == 0
            {
                ctx.lines(args![
                    "Hey, where's the Solution",
                    "I asked for...? I can't check it if you don't show it to me, right?"
                ])?;
                return ctx.close();
            }
            if (ctx.var("job_magician_q").get()? == 1 && ctx.call(Function::CountItem, args![1071])? == 0)
                || (ctx.var("job_magician_q").get()? == 2 && ctx.call(Function::CountItem, args![1085])? == 0)
                || (ctx.var("job_magician_q").get()? == 3 && ctx.call(Function::CountItem, args![1086])? == 0)
                || (ctx.var("job_magician_q").get()? == 4 && ctx.call(Function::CountItem, args![1087])? == 0)
            {
                ctx.lines(args!["Wait.", "This isn't the", "Solution I asked for!"])?;
                ctx.next()?;
                ctx.mes("[Mage Guildsman]")?;
                if ctx.var("job_magician_q").get()? == 1 {
                    ctx.mes("You're supposed to make Mixed Solution No. 1 and bring it back to me. Now go and try it again.")?;
                } else if ctx.var("job_magician_q").get()? == 2 {
                    ctx.mes("You're supposed to make Mixed Solution No. 2 and bring it back to me. Now go and try it again.")?;
                } else if ctx.var("job_magician_q").get()? == 3 {
                    ctx.mes("You're supposed to make Mixed Solution No. 3 and bring it back to me. Now go and try it again.")?;
                } else {
                    ctx.mes("You're supposed to make Mixed Solution No. 4 and bring it back to me. Now go and try it again.")?;
                }
                take_mage_solution(ctx)?;
                return ctx.close();
            }
            take_mage_solution(ctx)?;
            ctx.lines(args![
                "Hmm. I can see that you tried really hard. For a beginner's attempt, this is really good.",
                "Great work!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Guildsman",
                args!["Alright! I'm pleased to say that you've passed the Mage Test. I will transform you right away!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Mage Guildsman", args!["*Ahem*", "Congratulations!", "You are now a Mage!"])?;
            ctx.next()?;
            for quest_id in 1005..=1008 {
                if ctx.call(Function::IsBeginQuest, args![quest_id])? == 1 {
                    ctx.call(Function::CompleteQuest, args![quest_id])?;
                }
            }
            shared::other_global_functions::job_change(ctx, args![constants::JOB_MAGE])?;
            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
            ctx.player().set_zeny(ctx.player().zeny()? + 50)?;
            ctx.lines_as(
                "Mage Guildsman",
                args![
                    "'Welcome to My World~'",
                    "Heh heh, I just wanted to say that. You know, it's a quote from a well-known movie~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Guildsman",
                args!["Now that you're a Mage just like us, let's be friends, okay?"],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Mage Guildsman", args!["Wanna be a Mage, eh?"])?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.mes("For a cutie like you, I'd be happy to explain the requirements!")?;
            } else {
                ctx.mes("I'd be happy to explain the requirements for a pretty girl like you!")?;
            }
            ctx.next()?;
            ctx.lines_as("Mage Guildsman", args!["First of all, you have to reach Novice Job Level 10 and learn all of the Basic Skills. Then, you'll have to pass the Mage Test."])?;
            ctx.next()?;
            if ctx.var("job_magician_q").get()? != 0 {
                ctx.lines_as("Mage Guildsman", args!["Your test is to"])?;
                match ctx.var("job_magician_q").get()?.number()? {
                    1 => {
                        ctx.lines(args![
                            "make me a",
                            "^3355FFMixed Solution No. 1^000000",
                            "and bring it back to me."
                        ])?;
                        if ctx.call(Function::IsBeginQuest, args![1005])? == 0 {
                            ctx.quests().start(1005)?;
                        }
                    }
                    2 => {
                        ctx.lines(args![
                            "make me a",
                            "^3355FFMixed Solution No. 2^000000",
                            "and bring it back to me."
                        ])?;
                        if ctx.call(Function::IsBeginQuest, args![1006])? == 0 {
                            ctx.quests().start(1006)?;
                        }
                    }
                    3 => {
                        ctx.lines(args![
                            "make me a",
                            "^3355FFMixed Solution No. 3^000000",
                            "and bring it back to me."
                        ])?;
                        if ctx.call(Function::IsBeginQuest, args![1007])? == 0 {
                            ctx.quests().start(1007)?;
                        }
                    }
                    _ => {
                        ctx.lines(args![
                            "make me a",
                            "^3355FFMixed Solution No. 4^000000",
                            "and bring it back to me."
                        ])?;
                        if ctx.call(Function::IsBeginQuest, args![1008])? == 0 {
                            ctx.quests().start(1008)?;
                        }
                    }
                }
                ctx.next()?;
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["You can look up the ingredients you'll need to make the Solution inside the Guide Book in this Guild."],
                )?;
            } else {
                ctx.lines_as(
                    "Mage Guildsman",
                    args!["You will be informed as to which Mixed Solution you will need to create after signing the application form."],
                )?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Mage Guildsman",
                args!["Let me know when you are ready to become a Mage, alright?"],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as("Mage Guildsman", args!["Nothing...?"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

fn read_mix_amount(ctx: &Ctx) -> Result<Val, Stop> {
    loop {
        let (amount, _) = runtime::input_number(ctx, None, None)?;
        if amount.number()? <= 10000 {
            return Ok(amount);
        }
        ctx.next()?;
        ctx.lines_as(
            "Mixing Machine",
            args!["Error: Item limit exceeded. Please enter values less than 10,000 try again."],
        )?;
        ctx.next()?;
    }
}

pub fn mixing_machine(ctx: &Ctx) -> Script {
    let mut ready = 0;
    let mut powder = 0;
    let mut jellopy = Val::from(0);
    let mut fluff = Val::from(0);
    let mut milk = Val::from(0);
    let mut solvent = 0;
    let mut progress = 0;
    ctx.lines_as(
        "Mixing Machine",
        args!["This machine is the property of the Geffen Mage Guild and is used only for mixing solutions for magic purposes."],
    )?;
    ctx.next()?;
    if ctx.menu(&["Use Machine.", "Cancel."])? == 1 {
        return ctx.close();
    }
    ctx.lines_as("Mixing Machine", args!["Choose the", "Solvent for", "the Solution."])?;
    ctx.next()?;
    match ctx.menu(&["Payon Solution.", "Morocc Solution.", "No Solvent."])? {
        0 => {
            if ctx.call(Function::CountItem, args![1089])? == 0 {
                ctx.lines_as(
                    "Mixing Machine",
                    args!["Error.", "Cannot find the item.", "Please check again.", "Process Halting."],
                )?;
                return ctx.close();
            }
            solvent = 1;
        }
        1 => {
            if ctx.call(Function::CountItem, args![1088])? == 0 {
                ctx.lines_as(
                    "Mixing Machine",
                    args!["Error.", "Cannot find the item.", "Please check again.", "Process Halting."],
                )?;
                return ctx.close();
            }
            solvent = 2;
        }
        2 => {
            solvent = 0;
        }
        _ => {}
    }
    'l2: loop {
        if progress == 2 {
            ctx.mes("[Mixing Machine]")?;
            if jellopy != 0 {
                ctx.lines(args![Val::from("Jellopy: ") + jellopy.clone() + Val::from(" ea.")])?;
            }
            if fluff != 0 {
                ctx.lines(args![Val::from("Fluff: ") + fluff.clone() + Val::from(" ea.")])?;
            }
            if milk != 0 {
                ctx.lines(args![Val::from("Milk: ") + milk.clone() + Val::from(" ea.")])?;
            }
            if solvent == 0 {
                ctx.mes("Solvent: None.")?;
            }
            if solvent == 1 {
                ctx.mes("Solvent: Payon Solution.")?;
            }
            if solvent == 2 {
                ctx.mes("Solvent: Morocc Solution.")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Mixing Machine",
                args!["Please choose if you wish to begin mixing, or to re-enter the number of items to be mixed."],
            )?;
            ctx.next()?;
            match ctx.menu(&["Begin Mixing.", "Re-Enter Number of Items.", "Reset."])? {
                0 => {
                    ctx.lines_as(
                        "Mixing Machine",
                        args!["Please place the items into the Mixing Receptacle. Make sure the item amounts are correct."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mixing Machine",
                        args!["You cannot adjust or restore items once they are placed into the Mixing Receptacle."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mixing Machine",
                        args!["If everything is correct, press the 'Mix' button when you are ready. Otherwise, press the 'Cancel' button."],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Press 'Mix' Button.", "Press 'Cancel' Button."])? == 0 {
                        ctx.lines_as(
                            "Mixing Machine",
                            args!["Place items into the Mixing Receptacle now. Please wait."],
                        )?;
                        ctx.next()?;
                        ctx.mes("[Mixing Machine]")?;
                        if ctx.items().count(909)? < jellopy.number()? {
                            ctx.lines(args!["Insufficient Jellopy.", "Please Check again.", "Process Halted."])?;
                            return ctx.close();
                        }
                        if ctx.items().count(914)? < fluff.number()? {
                            ctx.lines(args!["Insufficient Fluff.", "Please Check again.", "Process Halted."])?;
                            return ctx.close();
                        }
                        if ctx.items().count(519)? < milk.number()? {
                            ctx.lines(args!["Insufficient Milk.", "Please Check again.", "Process Halted."])?;
                            return ctx.close();
                        }
                        if (solvent == 1 || solvent == 2)
                            && ctx.call(Function::CountItem, args![1089])? == 0
                            && ctx.call(Function::CountItem, args![1088])? == 0
                        {
                            ctx.lines(args!["Solution not found.", "Please Check again.", "Process Halted."])?;
                            return ctx.close();
                        }
                        if jellopy != 0 {
                            ctx.call(Function::DelItem, args![909, jellopy.clone()])?;
                        }
                        if fluff != 0 {
                            ctx.call(Function::DelItem, args![914, fluff.clone()])?;
                        }
                        if milk != 0 {
                            ctx.call(Function::DelItem, args![519, milk.clone()])?;
                        }
                        if solvent == 1 {
                            ctx.items().take(1089, 1)?;
                        }
                        if solvent == 2 {
                            ctx.items().take(1088, 1)?;
                        }
                        ctx.lines(args!["Items are Ready.", "Close the Lid."])?;
                        progress = 3;
                        ctx.next()?;
                    }
                }
                1 => {
                    ready = 0;
                    ctx.next()?;
                }
                2 => {
                    jellopy = Val::from(0);
                    fluff = Val::from(0);
                    milk = Val::from(0);
                    progress = 0;
                    ready = 0;
                    ctx.lines_as("Mixing Machine", args!["Reset Complete.", "Initiate again?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes.", "No."])? == 1 {
                        ctx.lines_as("Mixing Machine", args!["Process Halted.", "Thank you."])?;
                        return ctx.close();
                    }
                }
                _ => {}
            }
            if progress == 3 {
                break 'l2;
            }
        } else if progress == 1 {
            ctx.lines_as("Mixing Machine", args!["Nothing found."])?;
            ctx.next()?;
        }
        ctx.lines_as("Mixing Machine", args!["Select items to mix."])?;
        'l4: loop {
            match ctx.menu(&["Jellopy.", "Fluff.", "Milk.", "Ready to Mix."])? {
                0 => {
                    let amount = read_mix_amount(ctx)?;
                    if ctx.items().count(909)? > 0 {
                        jellopy = jellopy + amount;
                    }
                    progress = 2;
                }
                1 => {
                    let amount = read_mix_amount(ctx)?;
                    if ctx.items().count(914)? > 0 {
                        fluff = fluff + amount;
                    }
                    progress = 2;
                }
                2 => {
                    let amount = read_mix_amount(ctx)?;
                    if ctx.items().count(519)? > 0 {
                        milk = milk + amount;
                    }
                    progress = 2;
                }
                3 => {
                    if progress != 2 {
                        progress = 1;
                    }
                    ready = 1;
                    ctx.next()?;
                }
                _ => {}
            }
            if ready != 0 {
                break 'l4;
            }
        }
    }
    ctx.lines_as(
        "Mixing Machine",
        args!["Please enter the ", "Serial Number of", "the Magic Powder."],
    )?;
    ctx.next()?;
    'l9: loop {
        let (amount, _) = runtime::input_number(ctx, None, None)?;
        if amount.number()? < 1000 || amount.number()? > 9999 {
            ctx.mes("[Mixing Machine]")?;
            if amount == 0 {
                ctx.mes("Do you want to skip this Menu?")?;
                ctx.next()?;
                if ctx.menu(&["Yes.", "No."])? == 0 {
                    break 'l9;
                }
            } else {
                ctx.lines(args!["Invalid Serial Number.", "Please try again."])?;
                ctx.next()?;
            }
        } else {
            ctx.lines_as(
                "Mixing Machine",
                args![Val::from("The Serial Number is #") + amount.clone() + Val::from(", correct?")],
            )?;
            ctx.next()?;
            if ctx.menu(&["Confirm.", "Cancel."])? == 0 {
                if amount == 8472 {
                    powder = 1;
                } else if amount == 3735 {
                    powder = 2;
                } else if amount == 2750 {
                    powder = 3;
                } else if amount == 5429 {
                    powder = 4;
                } else {
                    powder = 5;
                }
            }
            break 'l9;
        }
    }
    ctx.lines_as("Mixing Machine", args!["Choose a", "Catalyst Stone."])?;
    ctx.next()?;
    let catalyst = match ctx.menu(&["Yellow Gemstone.", "Red Gemstone.", "Blue Gemstone.", "1carat Diamond.", "Skip."])? {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        _ => 0,
    };
    ctx.lines_as(
        "Mixing Machine",
        args!["All Set.", "Initiating", "Mixing process.", "Please Wait."],
    )?;
    ctx.next()?;
    ctx.lines_as("Mixing Machine", args!["- Proverb of the Day -"])?;
    match ctx.rand_range(1, 5)? {
        1 => {
            ctx.mes("An Eye for an Eye: When you take from a person, you must replace or repay in some way.")?;
        }
        2 => {
            ctx.mes("Credibility is a Man's Currency: There's a value in genuine trust that cannot be measured.")?;
        }
        3 => {
            ctx.lines(args![
                "What Goes Around Comes Around: Ultimately, you will be treated in the way you treat others.",
                "It means 'When you harm Another you will be harmed by him in an unavoidable situation'."
            ])?;
        }
        4 => {
            ctx.mes("A good neighbor is better than a distant brother: When you need help, you can count on those close to you.")?;
        }
        _ => {
            ctx.mes("Birds of a Feather Flock Together: You can look at a person's friends as an indicator of their character.")?;
        }
    }
    ctx.next()?;
    if jellopy == 2 && fluff == 3 && milk == 1 && solvent == 1 && catalyst == 1 && powder == 1 {
        ctx.lines_as("Mixing Machine", args!["Mage Test Solution No. 1."])?;
        ctx.items().give(1071, 1)?;
        ctx.next()?;
    } else if jellopy == 3 && fluff == 1 && milk == 1 && solvent == 0 && catalyst == 2 && powder == 2 {
        ctx.lines_as("Mixing Machine", args!["Mage Test Solution No. 2."])?;
        ctx.items().give(1085, 1)?;
        ctx.next()?;
    } else if jellopy == 6 && fluff == 1 && milk == 0 && solvent == 1 && catalyst == 3 && powder == 3 {
        ctx.lines_as("Mixing Machine", args!["Mage Test Solution No. 3."])?;
        ctx.items().give(1086, 1)?;
        ctx.next()?;
    } else if jellopy == 2 && fluff == 3 && milk == 0 && solvent == 2 && catalyst == 4 && powder == 4 {
        ctx.lines_as("Mixing Machine", args!["Mage Test Solution No. 4."])?;
        ctx.items().give(1087, 1)?;
        ctx.next()?;
    } else {
        ctx.lines_as("Mixing Machine", args!["Unexpected", "Error Occurred."])?;
        ctx.items().give(1090, 1)?;
        ctx.next()?;
    }
    ctx.lines_as("Mixing Machine", args!["Mixing Complete.", "Thank you."])?;
    ctx.close()
}

pub fn bookshelf(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Guide Book",
        args!["This Guide Book is the property of the Geffen Mage Association. Please handle with care."],
    )?;
    ctx.next()?;
    match ctx.menu(&["Solution No. 1.", "Solution No. 2.", "Solution No. 3.", "Solution No. 4.", "Close."])? {
        0 => {
            ctx.lines_as(
                "Mage Test Solution No. 1",
                args!["* Ingredients List *", "2 Jellopy", "3 Fluff", "1 Milk"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 1",
                args![
                    "* Solvent Agent *",
                    "Payon Solution",
                    "Where to Find:",
                    "A small spring in Payon, the Archer Village."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Mage Test Solution No. 1", args!["* Magic Power Serial Code *", "8472"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 1",
                args!["* Catalyst *", "Yellow Gemstone", "(Provided by", "Mixing Machine)"],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Mage Test Solution No. 2",
                args!["* Ingredients List *", "3 Jellopy", "1 Fluff", "1 Milk"],
            )?;
            ctx.next()?;
            ctx.lines_as("Mage Test Solution No. 2", args!["* Solvent Agent *", "None"])?;
            ctx.next()?;
            ctx.lines_as("Mage Test Solution No. 2", args!["* Magic Power Serial Code *", "3735"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 2",
                args!["* Catalyst *", "Red Gemstone", "(Provided by", "Mixing Machine)"],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Mage Test Solution No. 3",
                args!["* Ingredients List *", "6 Jellopy", "1 Fluff"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 3",
                args![
                    "* Solvent Agent *",
                    "Payon Solution",
                    "Where to Find:",
                    "A small spring in Payon, the Archer Village."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Mage Test Solution No. 3", args!["* Magic Power Serial Code *", "2750"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 3",
                args!["* Catalyst *", "Blue Gemstone", "(Provided by", "Mixing Machine)"],
            )?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Mage Test Solution No. 4",
                args!["* Ingredients List *", "2 Jellopy", "3 Fluff"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 4",
                args![
                    "* Solvent Agent *",
                    "Morocc Solution",
                    "Where to Find:",
                    "A small spring near entrance of pyramid in Morocc."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Mage Test Solution No. 4", args!["* Magic Power Serial Code *", "5429"])?;
            ctx.next()?;
            ctx.lines_as(
                "Mage Test Solution No. 4",
                args!["* Catalyst *", "1 carat Diamond", "(Provided by", "Mixing Machine)"],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    ctx.close()
}
