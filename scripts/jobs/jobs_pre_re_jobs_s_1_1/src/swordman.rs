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

pub fn swordman_swd_1(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 1 {
        if ctx.var("Class").get()? == constants::JOB_NOVICE_HIGH
            && (ctx.var("advjob").get()? == constants::JOB_LORD_KNIGHT || ctx.var("advjob").get()? == constants::JOB_PALADIN)
        {
            ctx.lines_as("Swordman", args!["It...", "Can't be...", "You've been reborn, haven't you?"])?;
            ctx.next()?;
            ctx.lines_as("Swordman", args!["I see you're retreading the path of the Swordman! Once you've gotten used to brandishing a sword, you can never go back!!"])?;
            ctx.next()?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.lines_as(
                    "Swordman",
                    args!["Hmm? Ah, you must first master the Basic Skills before you are ready to become a Swordman."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Swordman",
                    args!["Come back to me when you have finished learning the Basic Novice Skills."],
                )?;
                return ctx.close();
            }
            ctx.lines_as("Swordman", args!["Excellent! Let me promote you to a Swordman right away!"])?;
            ctx.next()?;
            ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
            ctx.call(Function::JobChange, args![constants::JOB_SWORDMAN_HIGH])?;
            ctx.call(Function::Skill, args!["SM_MOVINGRECOVERY", 1, constants::SKILL_PERM])?;
            ctx.call(Function::Skill, args!["SM_FATALBLOW", 1, constants::SKILL_PERM])?;
            ctx.call(Function::Skill, args!["SM_AUTOBERSERK", 1, constants::SKILL_PERM])?;
            ctx.lines_as("Swordman", args!["Hmm... You look like a well-experienced Swordman. Still, I'm sure that you must train to improve your skills and gain strength!"])?;
            return ctx.close();
        }
        ctx.lines_as("Swordman", args!["Hm...?", "You're a reborn", "warrior, aren't you?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Swordman",
            args![
                "Hmmm...",
                "It seems that being",
                "a Swordman is not part",
                "of your destiny. I'm sorry,",
                "but it seems there is nothing",
                "I can do for you."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Swordman", args!["Welcome to the", "Swordman Association!"])?;
    ctx.next()?;
    ctx.lines_as("Swordman", args!["So...", "What business", "brings you to us?"])?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Job Change", "About Swordman.", "About the Job requirements.", "Cancel."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            if ctx.var("BaseJob").get()? == constants::JOB_SWORDMAN {
                ctx.lines_as(
                    "Swordman",
                    args!["Job change? Muhahaha! But you're already a Swordman! Be proud and be strong!"],
                )?;
                return ctx.close();
            }
            if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                ctx.lines_as(
                    "Swordman",
                    args!["Haha! Oh boy. I'm flattered, but you already have another job! Still, I can't blame you..."],
                )?;
                return ctx.close();
            }
            if ctx.var("job_sword_q").get()? == 0 {
                ctx.lines_as(
                    "Swordman",
                    args!["So you wish to become a proud Swordman? By all means, please sign up!"],
                )?;
                ctx.next()?;
                if ctx.menu(&["Sign up.", "Cancel."])? != 0 {
                    ctx.lines_as(
                        "Swordman",
                        args!["Hm? Alright, come back whenever you change your mind. The world can always use another Swordman!"],
                    )?;
                    return ctx.close();
                }
                ctx.call(Function::SavePoint, args!["izlude_in", 65, 165, 1, 1])?;
                ctx.var("job_sword_q").set(1)?;
                ctx.quests().start(1014)?;
                ctx.lines_as(
                    "Swordman",
                    args!["Ah, yes. Your application will be reviewed as soon as possible."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Swordman",
                    args!["If you have already met the requirements, you can take an interview right now. Would you like to?"],
                )?;
                ctx.next()?;
                if ctx.menu(&["Yes.", "No."])? != 0 {
                    ctx.lines_as("Swordman", args!["Alright then. Feel free to come back whenever you are ready. All you have to do now is meet our requirements. Good luck to you."])?;
                    return ctx.close();
                }
                ctx.lines_as("Swordman", args!["Good, good.", "Now, let's see..."])?;
                ctx.next()?;
            }
            ctx.mes("[Swordman]")?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.mes("Hm, you still haven't learned all of the Basic Skills. You need to do that before you can become a Swordman.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Swordman",
                    args!["Check the requirements for job change again, and come back when you are ready."],
                )?;
                return ctx.close();
            }
            if ctx.var("job_sword_q").get()?.number()? <= 3 {
                ctx.mes("Hm, you've learned all of the Basic Skills but didn't take the test yet. You must first pass the exam before you can change your job to Swordman.")?;
                ctx.next()?;
                ctx.lines_as("Swordman", args!["Enter the room to my right so that you can take the test. You'll need to speak to my right so you can enter the examination area."])?;
                return ctx.close();
            }
            if ctx.var("job_sword_q").get()? == 4 {
                ctx.mes("Hahaha! Congratulations! Now you are fully qualified to be a real Swordman! I will transform you right away!")?;
                ctx.next()?;
                shared::other_global_functions::job_change(ctx, args![constants::JOB_SWORDMAN])?;
                ctx.var("job_sword_q").set(0)?;
                ctx.quests().complete(1014)?;
                ctx.lines_as(
                    "Swordman",
                    args!["Once again, congratulations. I expect that you will be a good representative of the Swordman Association."],
                )?;
                return ctx.close();
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Swordman",
                args!["So you wish to know more about the mighty Swordman job? Well, then..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Swordman",
                args![
                    "Amongst the First Class jobs, the Swordman is the best melee fighter for three reasons.",
                    "There are 3 reasons why Swordy is the best to approch a fight!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Swordman", args!["First, Swordman has the benefit of additional HP. Second, Swordman generally have access to a wider selection fo weapons than the other First Class jobs."])?;
            ctx.next()?;
            ctx.lines_as("Swordman", args!["And third, most of the Swordman skills are crushing physical attacks! In my opinion, being a Swordman is the best job ever!"])?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Swordman", args!["Hmmm..."])?;
            if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                if ctx.var("BaseJob").get()? == constants::JOB_SWORDMAN {
                    ctx.mes(
                        "But there's no need to tell you the requirements. You've met them and already became a Swordman! Well, anyway...",
                    )?;
                } else {
                    ctx.mes("It's too late for you to become a Swordman. You already have another job. Still, there's no harm in telling you...")?;
                }
            }
            ctx.next()?;
            ctx.lines_as("Swordman", args!["First, you must learn all 9 of the Basic Skills. If you can't complete this requirement, you won't be able to change to any job."])?;
            ctx.next()?;
            ctx.lines_as(
                "Swordman",
                args![
                    "Second, you must pass the Swordman Test. Inquire the Test Manager located in the waiting room of the Swordman Test."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Swordman",
                args!["If you can complete these 2 requirements, you can change to a Swordman anytime you want."],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Swordman", args!["Ha ha ha!", "Ah, youth!"])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn swordman_swd_2(ctx: &Ctx) -> Script {
    ctx.mes("[Swordman]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_SWORDMAN {
        ctx.mes("Sorry guy, but I can only allow Novices to enter the Test Hall.")?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
        ctx.mes("Who the hell are you?! Nobody, other than Novices, is permitted to come in here!")?;
        return ctx.close();
    }
    if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
        ctx.mes("Stop! I can't let you in until you learn all of the Basic Skills. The Test Hall isn't for goofing off!")?;
        return ctx.close();
    }
    if ctx.var("job_sword_q").get()? == 4 {
        ctx.mes("Hey. You need to talk to the Swordman in the center of the room, not me.")?;
        return ctx.close();
    }
    if ctx.var("job_sword_q").get()? == 0 {
        ctx.mes("Stop! If you want to take the Swordman Test, you'll need to fill out an application first.")?;
        ctx.next()?;
        ctx.lines_as(
            "Swordman",
            args!["The Swordman in the center of the room can help you with that, got it?"],
        )?;
        return ctx.close();
    }
    ctx.call(Function::SavePoint, args!["izlude_in", 65, 165, 1, 1])?;
    ctx.warp("izlude_in", 39, 170)?;
    ctx.end()
}

pub fn swordman_swd_3(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Swordman",
        args!["I will tell you about the Test! Listen carefully, I won't repeat myself."],
    )?;
    ctx.next()?;
    ctx.lines_as("Swordman", args!["The purpose of this test is to confirm whether or not you are qualified to be a Swordman. As you know, a Swordman needs physical strength and spirit!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Swordman",
        args!["Without those, you won't be able to become a Swordman. Now, the conditions for completing this test are very simple."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Swordman",
        args!["You will travel through three courses and must reach the final checkpoint within ^FF000010 minutes^000000."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Swordman",
        args!["If you choose to 'Surrender,' or if you run out of time, you will not pass the test."],
    )?;
    ctx.next()?;
    ctx.lines_as("Swordman", args!["If you find that you are not strong enough to pass the test, head to the entrance of the course and talk to the checkpoint manager."])?;
    ctx.next()?;
    ctx.lines_as("Swordman", args!["As you travel through the three courses, you may fall to a random, underground area. The course is designed so that you can still find your way back."])?;
    ctx.next()?;
    ctx.lines_as(
        "Swordman",
        args!["However, be careful, as this will waste your time! Godspeed to you."],
    )?;
    ctx.close()
}

pub fn test_hall_staff_swd_1(ctx: &Ctx) -> Script {
    ctx.mes("[Test Hall Staff]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_SWORDMAN {
        ctx.mes("Hm? How did you get inside? You're not supposed to be in here, so please leave now.")?;
        ctx.close_window()?;
        ctx.warp("izlude_in", 66, 170)?;
        return ctx.end();
    }
    if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
        ctx.mes("Who are you?! This place is for the Swordman Test! You're not allowed to be in here! Leave now!")?;
        ctx.close_window()?;
        ctx.warp("izlude_in", 66, 170)?;
        return ctx.end();
    }
    if ctx.var("job_sword_q").get()? == 1 {
        ctx.mes("So are you the one who wants to be a Swordman? Alright! You look reliable!")?;
        ctx.next()?;
        ctx.lines_as(
            "Test Hall Staff",
            args!["Try to relax and do your best. This course isn't so difficult."],
        )?;
        ctx.var("job_sword_q").set(2)?;
    } else if ctx.var("job_sword_q").get()? == 2 {
        ctx.mes(
            "Retesting? Try not to worry about it. It's good that you don't back down from a challenge! Here, take these and cheer up!",
        )?;
        ctx.items().give(512, 5)?;
        ctx.var("job_sword_q").set(3)?;
    } else if ctx.var("job_sword_q").get()? == 3 {
        ctx.mes("Don't ever give up! Now retesting!")?;
    }
    ctx.close_window()?;
    ctx.warp("job_sword1", 10, 245)?;
    ctx.end()
}

pub fn medic_swd_1(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdmedic(ctx, args!["1st"])?;
    Ok(())
}

pub fn test_hall_staff_swd_2(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdstaff(ctx, args![1])?;
    Ok(())
}

pub fn medic_2swd_2(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdmedic(ctx, args!["2nd"])?;
    Ok(())
}

pub fn test_hall_staff_2swd_3(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdstaff(ctx, args![1])?;
    Ok(())
}

pub fn mae_swd_1_success(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "job_sword1",
            Val::from("Applicant ") + ctx.player().name()? + Val::from(". You successfully passed the test."),
            constants::BC_MAP
        ],
    )?;
    ctx.var("job_sword_q").set(4)?;
    ctx.lines_as(
        "Mae",
        args![
            "I sencerely congratulate you for passing the test!",
            "I already sent your test result to the Job Department. Please inquire at the Officer in Centre.Thank you."
        ],
    )?;
    ctx.close_window()?;
    ctx.warp("izlude_in", 66, 173)?;
    ctx.end()
}

pub fn test_hall_staff_swd_4(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff(ctx, args![10, 245])?;
    Ok(())
}

pub fn test_hall_staff_swd_5(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff(ctx, args![11, 207])?;
    Ok(())
}

pub fn test_hall_staff_swd_6(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff(ctx, args![11, 169])?;
    Ok(())
}

pub fn test_hall_staff_swd_7(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff2(ctx, args!["1st", 215, 244])?;
    Ok(())
}

pub fn test_hall_staff_swd_8(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff2(ctx, args!["2nd", 215, 205])?;
    ctx.warp("job_sword1", 215, 205)?;
    Ok(())
}

pub fn test_hall_staff_swd_9(ctx: &Ctx) -> Script {
    shared::pre_re_jobs_1_1_swordman::f_jobswdteststaff2(ctx, args!["3rd", 215, 167])?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum S1Blank1AStep {
    Start,
    OnTouch,
}

fn s_1_blank_1_a_run(ctx: &Ctx, mut step: S1Blank1AStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S1Blank1AStep::Start => {
                step = S1Blank1AStep::OnTouch;
                continue 'machine;
            }
            S1Blank1AStep::OnTouch => {
                match ctx.rand_range(1, 5)? {
                    1 => ctx.warp("job_sword1", 65, 56)?,
                    2 => ctx.warp("job_sword1", 29, 26)?,
                    3 => ctx.warp("job_sword1", 43, 16)?,
                    4 => ctx.warp("job_sword1", 23, 112)?,
                    5 => ctx.warp("job_sword1", 58, 83)?,
                    _ => return Ok(Val::from(0)),
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_1_blank_1_a(ctx: &Ctx) -> Script {
    s_1_blank_1_a_run(ctx, S1Blank1AStep::Start, Vec::new()).map(|_| ())
}

pub fn s_1_blank_1_a_ontouch(ctx: &Ctx) -> Script {
    s_1_blank_1_a_run(ctx, S1Blank1AStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S2Blank1AStep {
    Start,
    OnTouch,
}

fn s_2_blank_1_a_run(ctx: &Ctx, mut step: S2Blank1AStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S2Blank1AStep::Start => {
                step = S2Blank1AStep::OnTouch;
                continue 'machine;
            }
            S2Blank1AStep::OnTouch => {
                match ctx.rand_range(1, 5)? {
                    1 => ctx.warp("job_sword1", 162, 120)?,
                    2 => ctx.warp("job_sword1", 94, 120)?,
                    3 => ctx.warp("job_sword1", 94, 85)?,
                    4 => ctx.warp("job_sword1", 162, 85)?,
                    5 => ctx.warp("job_sword1", 130, 47)?,
                    _ => return Ok(Val::from(0)),
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_2_blank_1_a(ctx: &Ctx) -> Script {
    s_2_blank_1_a_run(ctx, S2Blank1AStep::Start, Vec::new()).map(|_| ())
}

pub fn s_2_blank_1_a_ontouch(ctx: &Ctx) -> Script {
    s_2_blank_1_a_run(ctx, S2Blank1AStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S3Blank1AStep {
    Start,
    OnTouch,
}

fn s_3_blank_1_a_run(ctx: &Ctx, mut step: S3Blank1AStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S3Blank1AStep::Start => {
                step = S3Blank1AStep::OnTouch;
                continue 'machine;
            }
            S3Blank1AStep::OnTouch => {
                match ctx.rand_range(1, 5)? {
                    1 => ctx.warp("job_sword1", 195, 15)?,
                    2 => ctx.warp("job_sword1", 195, 38)?,
                    3 => ctx.warp("job_sword1", 231, 30)?,
                    4 => ctx.warp("job_sword1", 198, 65)?,
                    5 => ctx.warp("job_sword1", 196, 116)?,
                    _ => return Ok(Val::from(0)),
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_3_blank_1_a(ctx: &Ctx) -> Script {
    s_3_blank_1_a_run(ctx, S3Blank1AStep::Start, Vec::new()).map(|_| ())
}

pub fn s_3_blank_1_a_ontouch(ctx: &Ctx) -> Script {
    s_3_blank_1_a_run(ctx, S3Blank1AStep::OnTouch, Vec::new()).map(|_| ())
}
