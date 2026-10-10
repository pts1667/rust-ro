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

pub fn cleric_aco(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 1 {
        if ctx.var("advjob").get()? == constants::JOB_HIGH_PRIEST || ctx.var("advjob").get()? == constants::JOB_CHAMPION {
            if ctx.var("Class").get()? == constants::JOB_NOVICE_HIGH {
                ctx.lines_as(
                    "Father Mareusis",
                    args![
                        "Ah, I sense you have endured",
                        "a past life experience. You must have learned many things before entering Valhalla."
                    ],
                )?;
                ctx.next()?;
                if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                    ctx.lines_as("Father Mareusis", args!["Unfortunately, I don't think you're ready to become an Acolyte yet. Please finish learning all of the Basic Skills first."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Mareusis",
                        args!["In the meantime,", "I will wait until", "you are ready.", "May God be", "with you."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Father Mareusis",
                    args![
                        "Well, I welcome you",
                        "back from Valhalla and",
                        "wish you luck on your",
                        "new life's journey."
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
                ctx.call(Function::JobChange, args![constants::JOB_ACOLYTE_HIGH])?;
                ctx.call(Function::Skill, args!["AL_HOLYLIGHT", 1, constants::SKILL_PERM])?;
                ctx.lines_as(
                    "Father Mareusis",
                    args!["Now, venture forth and seek those who need your help. May God light your path."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Father Mareusis",
                args!["Now, venture forth to seek people who need your help. May God enlighten your way."],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Father Mareusis",
            args!["I sense that you have endured a past life experience. You must have learned many things before entering Valhalla."],
        )?;
        ctx.next()?;
        ctx.lines_as("Father Mareusis", args!["However, I can tell that you are not suited to be an Acolyte. Please remember who you were in your past life and find your path."])?;
        return ctx.close();
    }
    ctx.lines_as("Father Mareusis", args!["What is it that you seek?"])?;
    ctx.next()?;
    match ctx.menu(&["Father, I want to be a Acolyte.", "Acolyte Requirements.", "Just looking around."])? {
        0 => {
            ctx.mes("[Father Mareusis]")?;
            if ctx.var("BaseJob").get()? == constants::JOB_ACOLYTE {
                ctx.mes("Are you feeling okay today? I can tell by your attire that you are already an Acolyte. You're not joking around, are you?")?;
                return ctx.close();
            } else if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                ctx.mes("I'm sorry, but we can only accept Novices as applicants for the job change to Acolyte.")?;
                return ctx.close();
            }
            if ctx.var("job_acolyte_q").get()? == 0 {
                ctx.lines(args!["Do you truly", "wish to become", "a servant of God?"])?;
                ctx.next()?;
                if ctx.menu(&["Yes Father, I do.", "Nope, I lied."])? == 0 {
                    ctx.lines_as("Father Mareusis", args![Val::from("Good. I accept ") + ctx.player().name()? + Val::from("'s will to become an Acolyte. You understand that you must do penance before you can become a servant of God, right?")])?;
                    ctx.next()?;
                    ctx.lines_as("Father Mareusis", args!["Well, I will", "give you a mission..."])?;
                    let subject2 = ctx.call(Function::Rand, args![3])?;
                    if subject2 == 1 {
                        ctx.var("job_acolyte_q").set(Val::from(3))?;
                        ctx.mes("Please visit ^000077Mother Mathilda^000000 and then return to me. She has been practicing asceticism near ^000077Morocc Town, SouthWest of Prontera City^000000.")?;
                        ctx.quests().start(1002)?;
                    } else if subject2 == 2 {
                        ctx.var("job_acolyte_q").set(Val::from(4))?;
                        ctx.mes("Please visit ^000077Father Yosuke^000000 and return here. He has been practicing asceticism around ^000077a bridge somewhere NorthWest of Prontera^000000.")?;
                        ctx.quests().start(1003)?;
                    } else {
                        ctx.var("job_acolyte_q").set(Val::from(2))?;
                        ctx.mes("Please visit ^000077Father Rubalkabara^000000, a member of the Prontera Parish, and return here. He has been practicing asceticism in the ^000077Relics NorthEast of Prontera City^000000.")?;
                        ctx.quests().start(1001)?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Father Mareusis",
                        args!["May the grace of God light your path and guide you during your journey of penance."],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Father Mareusis",
                    args![
                        "You lied?",
                        "It is good that you",
                        "have confessed your",
                        "wrongdoing. Go in",
                        "peace, my son."
                    ],
                )?;
                return ctx.close();
            }
            ctx.mes("Oh, you've come back. Let me check and see if you are ready to serve God. Let's see...")?;
            ctx.next()?;
            ctx.mes("[Father Mareusis]")?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.mes("Good Lord! Haven't you accomplished the Basic Training yet?! It's important that you finish that!")?;
                ctx.next()?;
                ctx.lines_as("Father Mareusis", args!["You should have trained more! Go back and make sure you reach Novice Job Level 9 and learn all of the Basic Skills!"])?;
                return ctx.close();
            }
            if ctx.var("job_acolyte_q").get()?.number()? < 5 {
                ctx.mes("Oh? I can't find your name on the Registration List.")?;
                ctx.next()?;
                match ctx.var("job_acolyte_q").get()?.number()? {
                    2 => {
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["Please visit ^000077Father Rubalkabara^000000, a member of the Prontera Parish, and return here."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["He has been practicing asceticism in the ^000077Relics at the NorthEast of Prontera City^000000."],
                        )?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["Please Visit ^000077Mother Mathilda^000000 and return here to me."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["She has been practicing asceticism near ^000077Morocc Town, located SouthWest of Prontera City^000000."],
                        )?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["Please visit ^000077 Father Yosuke ^000000 and return here to me."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["He has been practicing asceticism near a ^000077bridge somewhere to the NorthWest of Prontera^000000."],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as(
                    "Father Mareusis",
                    args!["May the grace of God brighten your path and guide you on your journey of penance."],
                )?;
                return ctx.close();
            }
            ctx.lines(args![
                "Hmm...",
                "Your name is on the list and you've proven your qualification."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Father Mareusis",
                args!["I am proud to say that you are now ready to become an Acolyte!"],
            )?;
            ctx.next()?;
            ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
            shared::other_global_functions::job_change(ctx, args![constants::JOB_ACOLYTE])?;
            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
            if ctx.call(Function::CheckQuest, args![1001])? != -1 {
                ctx.quests().complete(1001)?;
            } else if ctx.call(Function::CheckQuest, args![1002])? != -1 {
                ctx.quests().complete(1002)?;
            } else {
                ctx.quests().complete(1003)?;
            }
            ctx.lines_as(
                "Father Mareusis",
                args!["Always remember to be thankful to God, who is taking care of us all the time."],
            )?;
            ctx.next()?;
            ctx.lines_as("Father Mareusis", args!["Always use your gifts to serve Him by helping others. In chaos and times of difficulty, face your hardships with unwavering faith."])?;
            ctx.next()?;
            ctx.lines_as(
                "Father Mareusis",
                args!["Lastly, I want to sincerely congratulate you on persevering through your trial of penance."],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Father Mareusis",
                args!["Do you wish to become an Acolyte? You must fulfill two requirements."],
            )?;
            ctx.next()?;
            ctx.lines_as("Father Mareusis", args!["First, you have to reach at least Novice Job Level 9 and learn all of the Basic Skills. Second, you will be given a trial of penance to overcome."])?;
            ctx.next()?;
            ctx.mes("[Father Mareusis]")?;
            if ctx.var("job_acolyte_q").get()? != 0 {
                match ctx.var("job_acolyte_q").get()?.number()? {
                    2 => {
                        ctx.mes("For your trial, please visit ^000077Father Rubalkabara ^000000 and then return here to me.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["He is practicing asceticism in the ^000077Relics at the NorthEast of Prontera City^000000."],
                        )?;
                    }
                    3 => {
                        ctx.mes("For your trial, please visit ^000077Mother Mathilda^000000 and return here to me.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args![
                                "She has been practicing asceticism near ^000077Morocc, located to the SouthWest of Prontera City^000000."
                            ],
                        )?;
                    }
                    _ => {
                        ctx.mes("For your trial, please visit ^000077Father Yosuke^000000 and return here to me.")?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Father Mareusis",
                            args!["He has been practicing asceticism around a bridge somewhere ^000077NorthWest of Prontera^000000."],
                        )?;
                    }
                }
                ctx.next()?;
                ctx.lines_as(
                    "Father Mareusis",
                    args!["May the grace of God light your path and guide you on your journey of penance."],
                )?;
            } else {
                ctx.mes("The destination for this trial will be decided once you fill the application form.")?;
            }
            ctx.next()?;
            ctx.lines_as("Father Mareusis", args!["Please come back after fulfilling the two requirements I've asked of you. As long as your desire to serve God and others is sincere, you will be able to make it."])?;
            return ctx.close();
        }
        _ => ctx.close(),
    }
}

pub fn ascetic_aco(ctx: &Ctx) -> Script {
    ctx.mes("[Father Rubalkabara]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE {
        if ctx.var("job_acolyte_q").get()? == 6 {
            ctx.mes("Please take care. They should know that you've met me by the time you arrive at the Prontera Sanctuary.")?;
            ctx.next()?;
            ctx.lines_as(
                "Father Rubalkabara",
                args!["I've sent a carrier pigeon with a message. I hope it will arrive there safely..."],
            )?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 0 {
            ctx.mes("Huh? What brings you here? This is a very dangerous place for a Novice like yourself!")?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 2 {
            ctx.mes("Oh...? You must be the one who aspires to become an Acolyte. I've already received news from the Sanctuary that you might be coming.")?;
            ctx.next()?;
            ctx.lines_as(
                "Father Rubalkabara",
                args![
                    Val::from("Now, your name was ") + ctx.player().name()? + Val::from(", right? Excellent, thank you for visiting me.")
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Father Rubalkabara", args!["I believe you've been told much about Acolytes from Friar Mareusis. Plus, there's plenty of helpful people in the Prontera Sanctuary."])?;
            ctx.next()?;
            ctx.lines_as("Father Rubalkabara", args!["I guess there's really no need for me to teach you much. Besides, I'm sure your someone from your generation may have trouble listening to an old man like me. Hahaha~"])?;
            ctx.next()?;
            ctx.lines_as("Father Rubalkabara", args!["Still, lessons may come from the places you'd least expect. God loves to teach his children in strange ways. You'll see."])?;
            ctx.next()?;
            ctx.lines_as("Father Rubalkabara", args!["Well, I'll send the message telling them that you've come to visit me. So, you may now return to the Prontera Sanctuary."])?;
            ctx.next()?;
            ctx.lines_as("Father Rubalkabara", args!["Farewell."])?;
            ctx.close_window()?;
            ctx.call(Function::SavePoint, args!["prt_fild03", 361, 255, 1, 1])?;
            ctx.var("job_acolyte_q").set(Val::from(6))?;
            return ctx.end();
        }
        ctx.lines(args!["Oh...", "Are you one of the", "Acolyte applicants...?", "Let's see..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Father Rubalkabara",
            args![
                Val::from("Your name is ") + ctx.player().name()? + Val::from("?"),
                "I don't think your name",
                "is on my list. Hmmm..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Rubalkabara",
            args!["Why don't you go back to the Prontera Sanctuary and check again?"],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? == constants::JOB_ACOLYTE {
        shared::jobs_2_1_priest::f_fatherrub(ctx, vec![])?;
    } else if ctx.var("BaseJob").get()? == constants::JOB_PRIEST {
        ctx.mes("Greetings.")?;
        ctx.next()?;
        ctx.lines_as("Father Rubalkabara", args!["Welcome to the Deep. Feel free to sit and contemplate God's message with me. This place is beautiful, even if danger accompanies its sense of serenity..."])?;
        return ctx.close();
    } else {
        ctx.lines(args![
            "Oh ho...",
            "Have you come into the Deep here for training? Or are you just a Wanderer?"
        ])?;
        ctx.next()?;
        ctx.lines_as("Father Rubalkabara", args!["Whoever you are, please take care of yourself. The monsters in here are shockingly strong, contrary to their cute appearance."])?;
        return ctx.close();
    }
    Ok(())
}

pub fn ascetic_2aco(ctx: &Ctx) -> Script {
    ctx.mes("[Mother Mathilda]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE {
        if ctx.var("job_acolyte_q").get()? == 7 {
            ctx.mes("I will send a carrier pigeon to the Prontera Sanctuary. When you return, the Priest there should already have received my message.")?;
            ctx.next()?;
            ctx.lines_as(
                "Mother Mathilda",
                args!["I will pray to God, and hope that you become an Acolyte soon."],
            )?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 0 {
            ctx.mes("...")?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 3 {
            ctx.mes("Ah, you must be one of the Acolyte applicants. I sincerely welcome you.")?;
            ctx.next()?;
            ctx.lines_as(
                "Mother Mathilda",
                args![Val::from("What is your name? ") + ctx.player().name()? + Val::from("? Let's see... Ah, you're on my list.")],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mother Mathilda",
                args![
                    Val::from("I will send a message to the Sanctuary confirming that you, ")
                        + ctx.player().name()?
                        + Val::from(" visited me and completed your penance.")
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mother Mathilda",
                args!["Please return to the Prontera Sanctuary and speak to the Priest in charge."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::SavePoint, args!["moc_fild07", 35, 355, 1, 1])?;
            ctx.var("job_acolyte_q").set(Val::from(7))?;
            return ctx.end();
        }
        ctx.lines(args![
            "Ah...!",
            "You must be one",
            "of the Acolyte applicants.",
            "I sincerely welcome you."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Mother Mathilda",
            args![
                "Now, what is your name?",
                Val::from(ctx.player().name()?) + Val::from("? Let's see...")
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mother Mathilda", args!["Hmm...", "It seems your name", "is not on my list..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Mother Mathilda",
            args!["Perhaps you should return to the Prontera Sanctuary and check the destination for your penance trial once again."],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? == constants::JOB_ACOLYTE {
        shared::jobs_2_1_priest::f_mothermart(ctx, vec![])?;
    } else if ctx.var("BaseJob").get()? == constants::JOB_PRIEST {
        ctx.mes("Hello there~")?;
        ctx.next()?;
        ctx.lines_as(
            "Mother Mathilda",
            args!["How is your practice coming along? I certainly hope you're enjoying living in the grace of God."],
        )?;
        return ctx.close();
    } else {
        ctx.lines(args!["May God", "be with you..."])?;
        return ctx.close();
    }
    Ok(())
}

pub fn ascetic_3aco(ctx: &Ctx) -> Script {
    ctx.mes("[Father Yosuke]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE {
        if ctx.var("job_acolyte_q").get()? == 8 {
            ctx.mes("What?")?;
            ctx.next()?;
            ctx.lines_as(
                "Father Yosuke",
                args!["Have you any more business with me?! You don't! Go back to the Sanctuary now!"],
            )?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 0 {
            ctx.lines(args!["You...", "Novice.", "There something", "you wanna tell me?"])?;
            return ctx.close();
        }
        if ctx.var("job_acolyte_q").get()? == 4 {
            ctx.lines(args![
                "Hey.",
                "Whatever you are,",
                "you look like an",
                "Acolyte applicant.",
                "Right?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Father Yosuke",
                args![
                    "Not bad, not bad. You withstood the penance trial pretty well.",
                    "So what's your name?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Father Yosuke", args![Val::from(ctx.player().name()?) + Val::from(", huh?")])?;
            ctx.next()?;
            ctx.lines_as(
                "Father Yosuke",
                args![
                    Val::from("Okay. I'll send a message to the Sanctuary that you, ")
                        + ctx.player().name()?
                        + Val::from(", came to visit me.")
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Father Yosuke",
                args!["Now go back to the Santuary and finish becoming an Acolyte, kid."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::SavePoint, args!["prt_fild00", 206, 230, 1, 1])?;
            ctx.var("job_acolyte_q").set(Val::from(8))?;
            return ctx.end();
        }
        ctx.lines(args!["Hey.", "You look like an Acolyte Applicant. Am I right?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Father Yosuke",
            args!["Not bad at all, you've made it all the way here from Prontera. So what's your name, kid?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Yosuke",
            args![Val::from(ctx.player().name()?) + Val::from(", huh? Why isn't your name on my list?")],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Father Yosuke",
            args!["You probably made a mistake. Go back to the Santuary, and check with the Bishop."],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? == constants::JOB_ACOLYTE {
        shared::jobs_2_1_priest::f_fatheryos(ctx, vec![])?;
    } else if ctx.var("BaseJob").get()? == constants::JOB_PRIEST {
        ctx.mes("Hey...")?;
        ctx.next()?;
        ctx.lines_as(
            "Father Yosuke",
            args!["If you like, come sit here with me and meditate the great truths. God's majesty is truly inspiring..."],
        )?;
        return ctx.close();
    } else {
        ctx.lines(args![
            "Do you have anything to say? Because unfortunately for you,",
            "I don't any replies."
        ])?;
        return ctx.close();
    }
    Ok(())
}
